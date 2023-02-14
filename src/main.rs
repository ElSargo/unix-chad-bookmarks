fn main() {
    let home_dir = dirs::home_dir().expect("Unable to read home dir");
    let file = bytes_to_string(
        &std::fs::read(format!("{}/.config/bookmarks", home_dir.display()))
            .expect("No bookmark file found (~/.config/bookmarks)"),
    );

    let mut dmenu_input = String::new();
    let mut map = std::collections::HashMap::new();

    for line in file.split("\n") {
        let contents = match line.split_once("|") {
            Some(str) => str,
            None => continue,
        };

        dmenu_input.push_str(contents.0);
        dmenu_input.push_str("\n");
        map.insert(contents.0, contents.1);
    }

    if let Some(output) = dmenu(dmenu_input.into()) {
        let output_as_string: String = bytes_to_string(&output);
        let end = match output_as_string.len() {
            0 => std::process::exit(1),
            n => n,
        };
        let key = &output_as_string[0..end - 1];
        let command = map.get(key).unwrap();
        run(command);
    };
}

fn bytes_to_string(o: &Vec<u8>) -> String {
    o.iter().map(|b| *b as char).collect()
}

/// Invoke dmenu, piping in the input and getting the output,
/// Returns None if any part of the proccess fails
fn dmenu(input: Vec<u8>) -> Option<Vec<u8>> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut dmenu = Command::new("dmenu");
    dmenu
        .arg("-l")
        .arg("30")
        .arg("-p")
        .arg("Open")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = match dmenu.spawn() {
        Ok(child) => child,
        _ => return None,
    };
    let ps = child.stdin.as_mut();
    match ps {
        Some(buff) => match buff.write_all(&input) {
            Ok(_) => match child.wait_with_output() {
                Ok(output) => Some(output.stdout),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// Run the command with the fish shell
fn run(cmd: &str) -> Option<Vec<u8>> {
    use std::process::{Command, Stdio};
    // Use the shell to run the commaand so we have acces to our config
    let shell = std::env::var("SHELL").ok()?;
    let mut command = Command::new(shell);
    command
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    match command.spawn() {
        Err(_) => None,
        Ok(child) => match child.wait_with_output() {
            Ok(output) => Some(output.stdout.iter().map(|x| *x).collect()),
            Err(_) => None,
        },
    }
}
