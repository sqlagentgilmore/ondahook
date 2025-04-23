use std::hint::spin_loop;
use std::process::Stdio;
use std::time::Duration;
use std::env;

pub fn commandment(args: &[&str]) -> std::io::Result<()> {
    if args.is_empty() {
        Ok(())
    } else {
        match
        {
            if args.len() == 1 {
                std::process::Command::new(args[0]).stdout(Stdio::piped()).spawn()
            } else {
                std::process::Command::new(args[0]).args(&args[1..]).stdout(Stdio::piped()).spawn()
            }
        } {
            Ok(mut child) => {
                match child.wait_with_output() {
                    Ok(output) => {
                        if output.status.success() {
                            Ok(())
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                String::from_utf8_lossy(&output.stderr),
                            ))
                        }
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}

fn main() {
    let mut args = std::env::args();
    args.next(); // Skip the program name

    if let Some(dir_path) = args.next() {
        
        if let Err(e) = env::set_current_dir(&dir_path) {
            panic!("Error changing directory: {}", e);
        }

        loop {
            spin_loop();
            match commandment(&["git", "pull"]) {
                Ok(_) => {
                    // Successfully pulled
                },
                Err(e) => {
                    println!("Error updating: {}", e);
                }
            };
            std::thread::sleep(Duration::new(5, 0));
        }
    } else {
        panic!("No directory path provided");
    }
}