pub mod io;

use crate::cli::io::print;
use std::io::{ Write, BufRead };

use copypasta::{ ClipboardContext, ClipboardProvider };

use crate::{ store::PasswordStore, password::{ Password, PasswordEntry } };

use self::io::read_terminal_input;

pub fn handle_add_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) {
    let message = [
        format!("[{}] -> {} password\n", "1", "Generate"),
        format!("[{}] -> {} password\n", "2", "Enter"),
    ];

    let message = message.join("");
    writeln!(writer, "\nOptions:\n{message}").unwrap();
    let input = read_terminal_input(reader, writer, None);

    match input.as_str() {
        "1" | "generate" => {
            let service = read_terminal_input(reader, writer, Some("Enter service name: "));

            let is_duplicate = store.check_for_duplicate_service_entry(&service);

            if is_duplicate {
                print(
                    writer,
                    "This service already exists, please try again with a unique service name"
                );
            }

            let username = read_terminal_input(reader, writer, Some("Enter username: "));
            let password = Password::generate();
            let entry = PasswordEntry::new(service, username, password);
            // Save entry to store
            store.add_and_save_entry(entry);
        }
        "2" | "enter" => {
            let service = read_terminal_input(reader, writer, Some("Enter service name: "));
            let username = read_terminal_input(reader, writer, Some("Enter username: "));
            let password = rpassword::prompt_password("Enter password: ").unwrap();
            let verify_password = rpassword::prompt_password("Please verify password: ").unwrap();

            if password == verify_password {
                let entry = PasswordEntry::new(service, username, password);
                store.add_and_save_entry(entry);
            } else {
                print(writer, "Unfortunately the entered passwords did not match, please try again")
            }
        }
        _ => print(writer, "Invalid command"),
    }
}

pub fn handle_get_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) {
    let service = read_terminal_input(reader, writer, Some("Enter service name: "));

    match store.get(&service) {
        Some(entry) => {
            let mut ctx = ClipboardContext::new().unwrap();
            println!("Found entry for {} - password was copied to clipboard!", &service);
            ctx.set_contents(entry.password.to_owned()).unwrap();
        }
        None => println!("Could not find an entry for service: {}", &service),
    }
}

pub fn handle_update_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) {
    let service = read_terminal_input(
        reader,
        writer,
        Some("Which service would you like to update?\n")
    );

    match store.get(&service) {
        Some(entry) => {
            let message = [
                format!("[{}] -> {}\n", "1", "Update username"),
                format!("[{}] -> {}\n", "2", "Update password"),
            ];

            let message = message.join("");
            println!("\nUpdating service: {}. These are your options:\n{}", &service, message);
            let input = read_terminal_input(reader, writer, None);

            match input.as_str() {
                "1" | "username" => {
                    let username = read_terminal_input(
                        reader,
                        writer,
                        Some("Enter new username: ")
                    );
                    let password = entry.password.to_owned();
                    let entry = PasswordEntry::new(service, username, password);
                    store.update_entry(entry)
                }
                "2" | "password" => {
                    let username = entry.username.to_owned();
                    let password = rpassword::prompt_password("Enter new password: ").unwrap();
                    let verify_password = rpassword
                        ::prompt_password("Please verify password: ")
                        .unwrap();

                    if password == verify_password {
                        let entry = PasswordEntry::new(service, username, password);
                        store.update_entry(entry)
                    } else {
                        println!(
                            "Unfortunately the entered passwords did not match, please try again"
                        );
                    }
                }
                _ => print(writer, "Invalid command"),
            }
        }
        None => {
            println!("Could not find an entry for service: {}", &service);
        }
    }
}