use pauli_repl::repl::render_once;
use std::io::{self, Write};

fn main() {
    println!("Pauli REPL");
    println!("Type help for commands, examples for demos, exit to quit.");

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!("> ");
        let _ = stdout.flush();

        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {}
            Err(err) => {
                eprintln!("input error: {err}");
                break;
            }
        }

        let input = buffer.trim();
        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" => break,
            "help" => {
                println!("Commands: help, examples, exit");
                println!("Or enter an expression like X*Y, [X,Y], (X+Y)*(X+Y)");
            }
            "examples" => {
                println!("X*Y");
                println!("[X,Y]");
                println!("(X+Y)*(X+Y)");
            }
            _ => match render_once(input) {
                Ok(output) => println!("{output}"),
                Err(err) => println!("error: {err}"),
            },
        }
    }
}
