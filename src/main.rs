use password_manager::cli::{ io::print, handle_list_services };
use std::{ io::{ Write, BufRead }, path::PathBuf };

use password_manager::{
    store::PasswordStore,
    cli::{
        io::read_terminal_input,
        handle_add_password,
        handle_get_password,
        handle_update_service,
    },
};

/**
 * Password manager written in Rust
 *
 * MVP Features:
 * - Add new password
 * - Get password
 * - Update password
 * - Delete password
 * - List all passwords
 *
 */

/**
 * Loops over the basic commands of the application:
 * Add, Get, Update, Delete and List
 *
 * Wait for user input to get direction on which commands to run
 * Each command will fan out to it's sub-dialogs that encapsulates feature specific logic
 */
fn run_dialog<R: BufRead, W: Write>(reader: &mut R, writer: &mut W, store: &mut PasswordStore) {
    loop {
        let message = [
            format!("[{}] -> {} password\n", "1", "Add"),
            format!("[{}] -> {} password\n", "2", "Get"),
            format!("[{}] -> {} service\n", "3", "Update"),
            format!("[{}] -> {} all services\n", "4", "List"),
        ];

        let message = message.join("");
        writeln!(writer, "\nCommands:\n{message}").unwrap();
        let input = read_terminal_input(reader, writer, None);

        match input.as_str() {
            "1" | "add" => {
                handle_add_password(reader, writer, store);
            }
            "2" | "get" => {
                handle_get_password(reader, writer, store);
            }
            "3" | "update" => {
                handle_update_service(reader, writer, store);
            }
            "4" | "list" => {
                handle_list_services(store);
            }
            _ => {
                print(writer, "Invalid command");
            }
        }
    }
}

fn initialize_application<R: BufRead, W: Write>(read: &mut R, write: &mut W) {
    print(write, "Welcome to the password manager! 👋");

    // Initialize the password store
    let file_path = PathBuf::from("passwords.json");
    let mut store = match PasswordStore::new(file_path) {
        Ok(store) => store,
        Err(err) => {
            print(write, err);
            return;
        }
    };

    run_dialog(read, write, &mut store)
}

fn main() {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();

    initialize_application(&mut input, &mut output);
}
