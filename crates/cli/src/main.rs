fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    let result = match command {
        "status" | "doctor" => esq_windows::inspect(),
        "help" | "--help" | "-h" => {
            println!("easy-spanish-quotes — prototipo experimental\n\nstatus | doctor [--json]\n\nAltGr+Z → «  AltGr+X → »\nLas operaciones nativas requieren validación en Windows desechable.");
            return;
        }
        _ => { eprintln!("Operación no disponible en este prototipo: {command}"); std::process::exit(3); }
    };
    match result {
        Ok(status) => println!("{}", serde_json::to_string_pretty(&status).unwrap()),
        Err(error) => { eprintln!("{error}"); std::process::exit(3); }
    }
}
