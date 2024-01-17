use std::process::{self, Command};

enum BookPagesError {
    EndBeforeStart,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_int = |s: &String, mes| s.parse().expect(mes);

    let res: Result<(), BookPagesError>;
    match args.len() {
        2 => {
            res = run_pages(0, parse_int(&args[1], "Invalid number of pages"), true);
        }
        3 => {
            res = run_pages(
                parse_int(&args[1], "Invalid current page"),
                parse_int(&args[2], "Invalid number of pages"),
                true,
            );
        }
        4 => {
            res = run_pages(
                parse_int(&args[1], "Invalid current page"),
                parse_int(&args[2], "Invalid number of pages"),
                if args[3].trim() == "-f" { false } else { true },
            );
        }
        _ => {
            println!("Usage: ./book_pages (current page) [number of pages] (-f)");
            process::exit(1);
        }
    }

    match res {
        Ok(_) => (),
        Err(BookPagesError::EndBeforeStart) => {
            println!("The starting page should come before the last page.");
            process::exit(1);
        }
    }
}

fn run_pages(start: i32, total: i32, ascii: bool) -> Result<(), BookPagesError> {
    if start > total {
        return Err(BookPagesError::EndBeforeStart);
    }

    let mut input = String::new();

    let mut command = Command::new("figlet");

    for i in start..=total {
        print!("{0}[2J{0}[1;1H", 27 as char);
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "" {
            if ascii {
                println!(
                    "{} / {}: {}%",
                    i,
                    total,
                    (i as f32) / (total as f32) * 100.0
                );
            } else {
                command
                    .arg("-t")
                    .arg(format!(
                        "{} / {}: {:.1}%\n",
                        i,
                        total,
                        (i as f32) / (total as f32) * 100.0
                    ))
                    .spawn()
                    .expect("figlet not available");
            }
        }
    }

    Ok(())
}
