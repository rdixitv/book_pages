use std::process::Command;

pub enum BookPagesError {
    EndBeforeStart,
}

pub fn run_pages(start: i32, total: i32, ascii: bool) -> Result<(), BookPagesError> {
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
