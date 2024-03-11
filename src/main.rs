use std::process;
mod run;
use run::BookPagesError;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_int = |s: &String, mes| s.parse().expect(mes);

    let res: Result<(), BookPagesError>;
    match args.len() {
        2 => {
            res = run::run_pages(0, parse_int(&args[1], "Invalid number of pages"), true);
        }
        3 => {
            res = run::run_pages(
                parse_int(&args[1], "Invalid current page"),
                parse_int(&args[2], "Invalid number of pages"),
                true,
            );
        }
        4 => {
            res = run::run_pages(
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
