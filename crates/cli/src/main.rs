pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args
        .first()
        .map(String::as_str)
        .unwrap_or(if cfg!(windows) { "gui" } else { "help" });
    #[cfg(windows)]
    if command == "gui" {
        if let Err(error) = esq_windows::gui(args.iter().any(|s| s == "--single-user-vm")) {
            eprintln!("{error}");
            std::process::exit(6);
        }
        return;
    }
    let result = match command {
        "status" | "doctor" => esq_windows::inspect(),
        #[cfg(windows)]
        "install" | "enable" | "disable" | "uninstall" | "repair" => {
            if !args.iter().any(|s| s == "--single-user-vm") {
                eprintln!("Instalación experimental: solo máquinas virtuales desechables con un único usuario. Se requiere --single-user-vm.");
                std::process::exit(3);
            }
            let owner = args
                .windows(2)
                .find(|w| w[0] == "--owner")
                .map(|w| w[1].clone())
                .unwrap_or_else(|| esq_windows::identity().map(|x| x.0).unwrap_or_default());
            esq_windows::lifecycle(command, &owner)
        }
        "help" | "--help" | "-h" => {
            println!("easy-spanish-quotes — prototipo experimental\n\nstatus | doctor [--json]\ngui\ninstall | enable | disable | uninstall | repair --single-user-vm\n\nAltGr+Z → «  AltGr+X → »\nLas operaciones nativas requieren un terminal elevado y validación en Windows desechable.");
            return;
        }
        _ => {
            eprintln!("Operación no disponible en este prototipo: {command}");
            std::process::exit(3);
        }
    };
    match result {
        Ok(status) => println!("{}", serde_json::to_string_pretty(&status).unwrap()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(3);
        }
    }
}
