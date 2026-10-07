use serde_json::{json, Value};
use std::ffi::c_void;

#[link(name = "user32")]
extern "system" {
    fn GetKeyboardLayout(thread: u32) -> *mut c_void;
    fn GetKeyboardLayoutNameW(name: *mut u16) -> i32;
    fn GetForegroundWindow() -> *mut c_void;
    fn GetWindowThreadProcessId(window: *mut c_void, process: *mut u32) -> u32;
    fn MapVirtualKeyExW(code: u32, kind: u32, layout: *mut c_void) -> u32;
    fn ToUnicodeEx(
        vk: u32,
        scan: u32,
        state: *const u8,
        output: *mut u16,
        capacity: i32,
        flags: u32,
        layout: *mut c_void,
    ) -> i32;
}

pub fn inspect() -> Result<Value, String> {
    unsafe {
        let hkl = GetKeyboardLayout(0);
        let mut name = [0u16; 9];
        if GetKeyboardLayoutNameW(name.as_mut_ptr()) == 0 {
            return Err(format!(
                "GetKeyboardLayoutNameW: {}",
                std::io::Error::last_os_error()
            ));
        }
        let klid = String::from_utf16_lossy(&name[..8]);
        let window = GetForegroundWindow();
        let thread = if window.is_null() {
            0
        } else {
            GetWindowThreadProcessId(window, std::ptr::null_mut())
        };
        let foreground = if thread == 0 {
            None
        } else {
            Some(format!("{:X}", GetKeyboardLayout(thread) as usize))
        };
        let mut conflicts = vec![];
        for (vk, label) in [(0x5a, "AltGr+Z"), (0x58, "AltGr+X")] {
            let mut state = [0u8; 256];
            state[0x11] = 0x80;
            state[0x12] = 0x80;
            state[0xa5] = 0x80;
            let mut output = [0u16; 16];
            let count = ToUnicodeEx(
                vk,
                MapVirtualKeyExW(vk, 0, hkl),
                state.as_ptr(),
                output.as_mut_ptr(),
                16,
                4,
                hkl,
            );
            conflicts.push(json!({"combination":label, "kind":if count < 0 {"dead-key"}
                else if count == 0 {"empty"} else {"printable"},
                "output":String::from_utf16_lossy(&output[..count.unsigned_abs().min(16) as usize])}));
        }
        Ok(
            json!({"schema":1, "experimental":true, "platform":"windows",
            "calling_thread_klid":klid, "calling_thread_hkl":format!("{:X}",hkl as usize),
            "foreground_thread_hkl":foreground, "activation":"unknown",
            "supported_baseline":klid == "0000040A", "conflicts":conflicts,
            "profile":esq_core::PROFILE, "integration_validation":"untested",
            "installation":crate::installation_status()?, "enabled_profiles":crate::enabled_profiles()?}),
        )
    }
}
