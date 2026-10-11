//! Experimental integration probe: run only in the disposable single-user VM.
//! Compares live Windows translations, not the generated DLL's data structures.

#[cfg(windows)]
mod probe {
    use serde_json::json;
    use std::ffi::c_void;

    type Hkl = *mut c_void;
    #[link(name = "user32")]
    extern "system" {
        fn GetKeyboardLayout(thread: u32) -> Hkl;
        fn GetKeyboardLayoutNameW(name: *mut u16) -> i32;
        fn ActivateKeyboardLayout(layout: Hkl, flags: u32) -> Hkl;
        fn MapVirtualKeyExW(key: u32, kind: u32, layout: Hkl) -> u32;
        fn ToUnicodeEx(
            key: u32,
            scan: u32,
            state: *const u8,
            output: *mut u16,
            capacity: i32,
            flags: u32,
            layout: Hkl,
        ) -> i32;
    }

    fn name() -> Result<String, String> {
        let mut buffer = [0u16; 9];
        if unsafe { GetKeyboardLayoutNameW(buffer.as_mut_ptr()) } == 0 {
            return Err("Cannot read calling-thread layout".into());
        }
        Ok(String::from_utf16_lossy(&buffer[..8]))
    }

    struct Restore(Hkl);
    impl Drop for Restore {
        fn drop(&mut self) {
            unsafe { ActivateKeyboardLayout(self.0, 0) };
        }
    }

    fn translate(layout: Hkl, key: u32, state: &[u8; 256]) -> (i32, Vec<u16>) {
        let scan = unsafe { MapVirtualKeyExW(key, 4, layout) };
        let mut buffer = [0u16; 16];
        // Bit 2 avoids changing the dead-key buffer on Windows 10 1607+.
        let count = unsafe {
            ToUnicodeEx(
                key,
                scan,
                state.as_ptr(),
                buffer.as_mut_ptr(),
                16,
                4,
                layout,
            )
        };
        let length = count.unsigned_abs().min(buffer.len() as u32) as usize;
        (count, buffer[..length].to_vec())
    }

    pub fn run() -> Result<(), String> {
        if std::env::args().skip(1).collect::<Vec<_>>() != ["--single-user-vm"] {
            return Err("Requires exactly --single-user-vm; disposable Windows VM only".into());
        }
        let original_name = name()?;
        if original_name != "0000040A" {
            return Err("Select the original Spanish Spain layout before running the probe".into());
        }
        let original = unsafe { GetKeyboardLayout(0) };
        let restore = Restore(original);
        esq_windows::select_owned()?;
        let owned_name = name()?;
        let owned = unsafe { GetKeyboardLayout(0) };
        let mut failures = Vec::new();
        let mut failed = 0;
        let mut comparisons = 0;
        let mut scan_failures = 0;
        let mut scan_comparisons = 0;
        for (kind, codes) in [
            (1, (0..128).collect::<Vec<_>>()),
            (3, (0..128).collect()),
            (3, (0..128).map(|scan| scan | 0xe000).collect()),
            (3, (0..128).map(|scan| scan | 0xe100).collect()),
            (4, (0..256).collect()),
        ] {
            for code in codes {
                scan_comparisons += 1;
                let baseline = unsafe { MapVirtualKeyExW(code, kind, original) };
                let actual = unsafe { MapVirtualKeyExW(code, kind, owned) };
                if baseline != actual {
                    scan_failures += 1;
                    if failures.len() < 20 {
                        failures.push(json!({"map_kind":kind,"code":code,
                            "expected":baseline,"actual":actual}));
                    }
                }
            }
        }
        for caps in 0..2 {
            for num in 0..2 {
                for modifiers in 0..8 {
                    let mut state = [0u8; 256];
                    state[0x14] = caps;
                    state[0x90] = num;
                    for (bit, key) in [(1, 0x10), (2, 0x11), (4, 0x12)] {
                        if modifiers & bit != 0 {
                            state[key] = 0x80;
                        }
                    }
                    for key in 0..256 {
                        comparisons += 1;
                        let baseline = translate(original, key, &state);
                        let expected = if modifiers == 6 && (key == 0x5a || key == 0x58) {
                            (1, vec![if key == 0x5a { 0x00ab } else { 0x00bb }])
                        } else {
                            baseline
                        };
                        let actual = translate(owned, key, &state);
                        if actual != expected {
                            failed += 1;
                            if failures.len() < 20 {
                                failures.push(json!({"vk":key,"modifiers":modifiers,
                                    "caps":caps,"num":num,"expected":expected,"actual":actual}));
                            }
                        }
                    }
                }
            }
        }
        drop(restore);
        let restored = name()? == original_name;
        println!(
            "{}",
            json!({"original_klid":original_name,"owned_klid":owned_name,
            "comparisons":comparisons,"failures":failed,"first_failures":failures,
                "scan_comparisons":scan_comparisons,"scan_failures":scan_failures,
            "calling_thread_restored":restored,"app_input_validation":"pending",
            "dead_key_composition_validation":"pending"})
        );
        if failed != 0 || scan_failures != 0 || !restored {
            return Err("Native translations or calling-thread restoration failed".into());
        }
        Ok(())
    }
}

fn main() {
    #[cfg(windows)]
    if let Err(error) = probe::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
    #[cfg(not(windows))]
    {
        eprintln!("This integration probe requires Windows");
        std::process::exit(1);
    }
}
