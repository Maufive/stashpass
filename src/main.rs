use password_manager::cli::io::print;
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
 * Passwords will be stored in a file in the following format:
 * - Each line will be a password entry
 * - Each line will be in the following format:
 * - <name> <username> <password>
 *      - name: name of the password entry
 *      - username: username for the password entry
 *      - password: password for the password entry
 *
 * Example:
 * - github johndoe password123
 *
 *
 * MVP will not use encryption and decryption to store passwords
 * First version will just be getting the functionality working to store passwords
 * using a file and input from the user via the command line. After that we can
 * add encryption and decryption to the passwords.  We can also add a GUI to the
 * application with Ratatui (?).
 *
 * Features to add:
 * - Encryption and decryption of passwords
 * - Master password
 * - GUI
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
