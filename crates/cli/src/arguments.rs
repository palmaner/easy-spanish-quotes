//! Parse the entire request before inspecting or mutating platform resources.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub command: String,
    pub single_user_vm: bool,
    pub owner: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArgumentError {
    pub message: String,
    pub exit_code: i32,
}

fn invalid(message: impl Into<String>) -> ArgumentError {
    ArgumentError {
        message: message.into(),
        exit_code: 2,
    }
}

pub fn parse(args: &[String], windows: bool) -> Result<Request, ArgumentError> {
    let command = args
        .first()
        .map(String::as_str)
        .unwrap_or(if windows { "gui" } else { "help" });
    let command = match command {
        "--help" | "-h" => "help",
        "--version" => "version",
        command => command,
    };
    let lifecycle = matches!(
        command,
        "install" | "enable" | "disable" | "uninstall" | "repair"
    );
    if !lifecycle && !matches!(command, "status" | "doctor" | "gui" | "help" | "version") {
        return Err(ArgumentError {
            message: format!("Operación no disponible en este prototipo: {command}"),
            exit_code: 3,
        });
    }
    let mut request = Request {
        command: command.into(),
        single_user_vm: false,
        owner: None,
    };
    let mut json = false;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--json" if (lifecycle || matches!(command, "status" | "doctor")) && !json => {
                json = true
            }
            "--single-user-vm" if (lifecycle || command == "gui") && !request.single_user_vm => {
                request.single_user_vm = true;
            }
            "--owner" if lifecycle && request.owner.is_none() => {
                index += 1;
                let owner = args
                    .get(index)
                    .ok_or_else(|| invalid("Falta el SID después de --owner."))?;
                if !owner.starts_with("S-1-")
                    || owner[4..]
                        .split('-')
                        .any(|part| part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()))
                {
                    return Err(invalid("--owner requiere un SID de Windows."));
                }
                request.owner = Some(owner.clone());
            }
            other => {
                return Err(invalid(format!(
                    "Argumento desconocido, duplicado o incompatible: {other}"
                )))
            }
        }
        index += 1;
    }
    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| (*s).into()).collect()
    }

    #[test]
    fn unknown_and_duplicate_arguments_fail_before_dispatch() {
        for input in [
            vec!["install", "--single-user-vm", "--force"],
            vec!["install", "--single-user-vm", "--single-user-vm"],
            vec!["status", "--owner", "S-1-5-21-123"],
            vec!["gui", "--json"],
            vec!["doctor", "--json", "--json"],
            vec!["install", "unexpected"],
        ] {
            assert_eq!(parse(&args(&input), true).unwrap_err().exit_code, 2);
        }
    }
    #[test]
    fn owner_value_cannot_consume_the_vm_guard() {
        for input in [
            vec!["install", "--owner"],
            vec!["install", "--owner", "--single-user-vm"],
            vec!["install", "--owner", "S-1-5-"],
        ] {
            assert!(parse(&args(&input), true).is_err());
        }
        let r = parse(
            &args(&["uninstall", "--owner", "S-1-5-21-123", "--single-user-vm"]),
            true,
        )
        .unwrap();
        assert!(r.single_user_vm);
        assert_eq!(r.owner.as_deref(), Some("S-1-5-21-123"));
    }
    #[test]
    fn defaults_and_unsupported_commands_are_explicit() {
        assert_eq!(parse(&[], true).unwrap().command, "gui");
        assert_eq!(parse(&[], false).unwrap().command, "help");
        assert_eq!(parse(&args(&["configure"]), true).unwrap_err().exit_code, 3);
        assert_eq!(
            parse(&args(&["--version"]), false).unwrap().command,
            "version"
        );
    }
}
