use crate::commands;

pub fn process_command(input: &str) {
    let mut parts = input.split_whitespace();
    if let Some(command) = parts.next() {
        match command {
            "echo" => commands::echo::run(input),
            _ => println!("Minishell >Unknown command: {}", command),
        }
    }
}