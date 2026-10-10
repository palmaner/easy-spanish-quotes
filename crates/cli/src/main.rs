mod arguments;

pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let request = arguments::parse(&args, cfg!(windows)).unwrap_or_else(|error| {
        eprintln!("{}", error.message);
        std::process::exit(error.exit_code);
    });
    let command = request.command.as_str();
    #[cfg(windows)]
    if command == "gui" {
        if let Err(error) = esq_windows::gui(request.single_user_vm) {
            eprintln!("{error}");
            std::process::exit(6);
        }
        return;
    }
    let result = match command {
        "status" => esq_windows::inspect(),
        "doctor" => esq_windows::inspect().map(|mut status| {
            status["doctor"] = serde_json::json!({
                "read_only": true,
                "findings": status["installation"]["findings"].as_array().cloned().unwrap_or_default(),
                "native_validation": "pending",
                "supported_release": false
            });
            status
        }),
        #[cfg(windows)]
        "install" | "enable" | "disable" | "uninstall" | "repair" => {
            if !request.single_user_vm {
                eprintln!("Instalación experimental: solo máquinas virtuales desechables con un único usuario. Se requiere --single-user-vm.");
                std::process::exit(3);
            }
            let owner = match request.owner {
                Some(owner) => Ok(owner),
                None => esq_windows::identity().map(|x| x.0),
            };
            owner.and_then(|owner| esq_windows::lifecycle(command, &owner))
        }
        "version" => {
            println!("easy-spanish-quotes {} — experimental", env!("CARGO_PKG_VERSION"));
            return;
        }
        "help" => {
            println!("easy-spanish-quotes — prototipo experimental\n\nstatus | doctor [--json]\ngui\ninstall | enable | disable | uninstall | repair --single-user-vm\n\nAltGr+Z → «  AltGr+X → »\nLas operaciones nativas requieren un terminal elevado y validación en Windows desechable.");
            return;
        }
        _ => {
            eprintln!("Operación no disponible en este prototipo: {command}");
            std::process::exit(3);
        }
    };
    match result {
        Ok(status) => {
            println!("{}", serde_json::to_string_pretty(&status).unwrap());
            if command != "status" {
                let state = status.get("installation").unwrap_or(&status)["state"].as_str();
                match state {
                    Some("repair-required") => std::process::exit(7),
                    Some("pending-reboot") => std::process::exit(8),
                    _ => (),
                }
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(3);
        }
    }
}
