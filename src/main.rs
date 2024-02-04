use std::io::{stdout, Write};
use std::path::Path;
use std::process::Command;
use std::process::{Child, Stdio};
use std::{env, io};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // default to "~"
                    let new_dir = args.peekable().peek().map_or("~", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one, prepare to send output to next command
                        Stdio::piped()
                    } else {
                        // there are no commands behind this one, send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
