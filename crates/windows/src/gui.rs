//! Native Spanish prototype UI. Ordinary mode is read-only; VM mode is explicit.
use std::{ffi::c_void, ptr};
use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::{GetStockObject, COLOR_WINDOW, DEFAULT_GUI_FONT},
    System::LibraryLoader::GetModuleHandleW,
    UI::{Shell::ShellExecuteW, WindowsAndMessaging::*},
};

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
struct Ui {
    status: HWND,
    vm: bool,
}

unsafe fn control(
    window: HWND,
    class: &str,
    text: &str,
    style: u32,
    bounds: (i32, i32, i32, i32),
    id: usize,
) -> HWND {
    let class = wide(class);
    let text = wide(text);
    let (x, y, w, h) = bounds;
    let result = CreateWindowExW(
        0,
        class.as_ptr(),
        text.as_ptr(),
        WS_CHILD | WS_VISIBLE | style,
        x,
        y,
        w,
        h,
        window,
        id as *mut c_void,
        GetModuleHandleW(ptr::null()),
        ptr::null(),
    );
    SendMessageW(
        result,
        WM_SETFONT,
        GetStockObject(DEFAULT_GUI_FONT) as usize,
        1,
    );
    result
}
fn status_text() -> String {
    match crate::inspect() {
        Ok(s) => {
            let original = s["calling_thread_klid"].as_str().unwrap_or("desconocido");
            let state = s["installation"]["state"].as_str().unwrap_or("desconocido");
            let label = match state {
                "absent" => "Sin instalar",
                "installed-disabled" => "Instalado, desactivado",
                "enabled" => "Habilitado; comprueba la selección",
                "pending-reboot" => "Pendiente de reiniciar",
                "repair-required" => "Requiere revisión",
                _ => "Estado desconocido",
            };
            format!("Teclado de esta ventana: {original}\r\n{label}\r\n\r\nPrototipo sin validar: instalación, reinicio y retirada pendientes de pruebas reales.")
        }
        Err(e) => format!("No se pudo comprobar el teclado: {e}"),
    }
}

unsafe extern "system" fn procedure(window: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if msg == WM_CREATE {
        let create = &*(l as *const CREATESTRUCTW);
        let ui = &mut *(create.lpCreateParams as *mut Ui);
        SetWindowLongPtrW(window, GWLP_USERDATA, create.lpCreateParams as isize);
        control(
            window,
            "STATIC",
            "Comillas españolas « »",
            0,
            (24, 20, 540, 28),
            0,
        );
        control(
            window,
            "STATIC",
            "Abrir: AltGr + Z       Cerrar: AltGr + X",
            0,
            (24, 56, 540, 28),
            0,
        );
        ui.status = control(window, "STATIC", &status_text(), 0, (24, 100, 540, 100), 0);
        control(
            window,
            "BUTTON",
            "Actualizar estado",
            WS_TABSTOP,
            (24, 212, 160, 32),
            1,
        );
        if ui.vm {
            control(
                window,
                "BUTTON",
                "Activar (prueba)",
                WS_TABSTOP,
                (200, 212, 160, 32),
                2,
            );
            control(
                window,
                "BUTTON",
                "Desactivar",
                WS_TABSTOP,
                (376, 212, 160, 32),
                3,
            );
            control(
                window,
                "BUTTON",
                "Quitar",
                WS_TABSTOP,
                (24, 260, 160, 32),
                4,
            );
            control(
                window,
                "BUTTON",
                "Seleccionar y probar",
                WS_TABSTOP,
                (200, 260, 220, 32),
                5,
            );
        } else {
            control(
                window,
                "STATIC",
                "La instalación experimental solo se ofrece en una VM desechable.",
                0,
                (24, 260, 540, 44),
                0,
            );
        }
        control(
            window,
            "STATIC",
            "Escribe aquí para probar las comillas:",
            0,
            (24, 322, 540, 24),
            0,
        );
        control(
            window,
            "EDIT",
            "",
            WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL as u32,
            (24, 352, 540, 36),
            6,
        );
        return 0;
    }
    let ui = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Ui;
    if msg == WM_COMMAND && !ui.is_null() {
        let id = w & 0xffff;
        if id == 1 {
            SetWindowTextW((*ui).status, wide(&status_text()).as_ptr());
        } else if id == 5 && (*ui).vm {
            let result=crate::select_owned().map(|_|"Selección comprobada en esta ventana. Prueba «Hola» en el campo y en otras aplicaciones.".into())
                .unwrap_or_else(|e|format!("No se pudo seleccionar: {e}"));
            SetWindowTextW((*ui).status, wide(&result).as_ptr());
        } else if (*ui).vm && (2..=4).contains(&id) {
            let command = match id {
                2 => "install",
                3 => "disable",
                _ => "uninstall",
            };
            let result = (|| {
                let (owner, _) = crate::identity()?;
                let exe = std::env::current_exe().map_err(|e| e.to_string())?;
                let exe = wide(exe.to_str().ok_or("Ruta de aplicación no válida")?);
                let args = wide(&format!("{command} --single-user-vm --owner {owner}"));
                let verb = wide("runas");
                let result = ShellExecuteW(
                    window,
                    verb.as_ptr(),
                    exe.as_ptr(),
                    args.as_ptr(),
                    ptr::null(),
                    SW_SHOWNORMAL,
                );
                if result as usize <= 32 {
                    return Err("Elevación cancelada o no disponible".to_string());
                }
                Ok(
                    "Se ha abierto la operación elevada. Al terminar, pulsa Actualizar estado."
                        .to_string(),
                )
            })()
            .unwrap_or_else(|e| e);
            SetWindowTextW((*ui).status, wide(&result).as_ptr());
        }
        return 0;
    }
    if msg == WM_DESTROY {
        PostQuitMessage(0);
        return 0;
    }
    DefWindowProcW(window, msg, w, l)
}

pub fn gui(vm: bool) -> Result<(), String> {
    unsafe {
        let instance = GetModuleHandleW(ptr::null());
        let class = wide("EasySpanishQuotes");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(procedure),
            hInstance: instance,
            lpszClassName: class.as_ptr(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: ((COLOR_WINDOW + 1) as usize) as _,
            ..std::mem::zeroed()
        };
        if RegisterClassW(&wc) == 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let mut ui = Box::new(Ui {
            status: ptr::null_mut(),
            vm,
        });
        let window = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("easy-spanish-quotes — prototipo").as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            620,
            470,
            ptr::null_mut(),
            ptr::null_mut(),
            instance,
            (&mut *ui as *mut Ui).cast(),
        );
        if window.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        ShowWindow(window, SW_SHOW);
        let mut msg: MSG = std::mem::zeroed();
        loop {
            let status = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
            if status == 0 {
                break;
            }
            if status < 0 {
                return Err(std::io::Error::last_os_error().to_string());
            }
            if IsDialogMessageW(window, &msg) == 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        Ok(())
    }
}
