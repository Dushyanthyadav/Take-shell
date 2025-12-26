use std::env;
use std::fs::File;
use std::io::{ErrorKind, Write, stdin, stdout};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    ctrlc::set_handler(move || {
        print!("\n");
        print!("> ");
        stdout().flush().unwrap();
    }).expect("Error setting Ctrl-C handler");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(error) => {
                if error.kind() == ErrorKind::Interrupted {
                    continue;
                }
                
                eprintln!("Error reading input: {}", error);
                continue;
            }
        }

        let mut commands = input.trim().split("|").map(|x| x.trim()).peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            if command.contains(">") {
                let stdin = previous_command.map_or(Stdio::inherit(),|output: Child| Stdio::from(output.stdout.unwrap()));

                let mut process_file = command.split(">").map(|x| x.trim());

                let process_args = process_file.next().unwrap();

                let mut process_args = process_args.split_whitespace().map(|x| x.trim()).peekable();

                let process = process_args.next().unwrap();
                let args = process_args;


                let file_name = process_file.next().unwrap();

                let file = File::create(file_name).unwrap();

                let output = Command::new(process)
                    .args(args)
                    .stdin(stdin)
                    .stdout(Stdio::from(file))
                    .spawn();

                match output {
                    Ok(output) => {previous_command = Some(output);},
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    },
                };
            
            } else if command.contains("<") {
                let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                let mut process_file = command.split("<").map(|x| x.trim());

                let process_args = process_file.next().unwrap();

                let mut process_args = process_args.split_whitespace().map(|x| x.trim()).peekable();

                let process = process_args.next().unwrap();
                let args = process_args;


                let file_name = process_file.next().unwrap();
                println!("file name: {}", file_name);

                let file = File::open(file_name).unwrap();

                let output = Command::new(process)
                    .args(args)
                    .stdin(Stdio::from(file))
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => {previous_command = Some(output);},
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    },
                };
            } else {
                let mut parts = command.trim().split_whitespace();
                let command = match parts.next() {
                Some(c) => c,
                None => continue,
                };
                let args = parts;
                match command {
                    "cd" => {
                        let new_dir = args.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e);
                        }
                        previous_command = None;
                    }
                    "exit" => return,
                    command => {
                        let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                        
                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();

                        match output {
                            Ok(output) => {previous_command = Some(output);},
                            Err(e) => {
                                previous_command = None;
                                eprintln!("{}", e);
                            },
                        };
                    }
                }
            
        }

    }
    if let Some(mut final_command) = previous_command {
        final_command.wait().unwrap();
    }
}
}




