pub mod io;

use crate::cli::io::print;
use std::io::{ Write, BufRead };

use copypasta::{ ClipboardContext, ClipboardProvider };

use crate::{ store::PasswordStore, password::{ Password, PasswordEntry } };

use self::io::read_terminal_input;

/** Get input from the user for the username */
fn read_username<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> String {
    read_terminal_input(reader, writer, Some("Enter username: "))
}

/** Get input from the user for the password */
// fn read_password<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> String {
//     rpassword::prompt_password("Enter password: ").unwrap()
// }

/**
 * Get input from the user for a service.
 * A service a website, app, or whatever you want to associate a password with.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 * @return String
 */
fn read_service_name<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) -> String {
    loop {
        let service = read_terminal_input(reader, writer, Some("Enter service name: "));
        if !store.check_for_duplicate_service_entry(&service) {
            return service;
        }
        print(writer, "This service already exists, please try again with a unique service name");
    }
}

/**
 * Get input from the user for a password and verify it.
 * This method will keep asking for a password until the user enters the same password twice.
 *
 * It also uses the rpassword crate to hide the password input for the users privacy.
 *
 * @param writer: &mut W
 * @return Option<String>
 */
fn read_and_confirm_password<W: Write>(writer: &mut W) -> Option<String> {
    loop {
        let password = rpassword::prompt_password("Enter password: ").unwrap();
        let verify_password = rpassword::prompt_password("Please verify password: ").unwrap();

        if password == verify_password {
            return Some(password);
        }

        print(writer, "Unfortunately the entered passwords did not match, please try again");
    }
}

/**
 * Handle the user input for entering their own password.
 * The method will verify the entered password to make sure the user entered the correct password.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 *
 */
fn handle_enter_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) {
    let service = read_service_name(reader, writer, store);
    let username = read_username(reader, writer);
    let password = read_and_confirm_password(writer);

    if let Some(password) = password {
        let entry = PasswordEntry::new(service, username, password);
        store.add_and_save_entry(entry);
    } else {
        print(writer, "Unfortunately the entered passwords did not match, please try again")
    }
}

/**
 * Handle the user input for generating a password.
 * The method will generate a password and add it to the password store.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 */
fn handle_generate_password<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) {
    let service = read_service_name(reader, writer, store);
    let username = read_username(reader, writer);
    let password = Password::generate();
    let entry = PasswordEntry::new(service, username, password);
    store.add_and_save_entry(entry);
}

/**
 * Starts the dialog to add a password.
 * The user can choose to generate a password or enter their own password.
 * The method will then call the appropriate method to handle the user input.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 */
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
        "1" | "generate" => handle_generate_password(reader, writer, store),
        "2" | "enter" => handle_enter_password(reader, writer, store),
        _ => print(writer, "Invalid command"),
    }
}

/**
 * Starts the dialog to get a password.
 * The user can enter a service name and the method will then try to find the password for that service.
 * If the password is found it will be copied to the clipboard.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 */
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

/**
 * Handle updating a username for a service.
 * The method will ask the user for a new username and then update the entry in the store.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 * @param service: &str
 * @param entry: PasswordEntry
 */
fn update_username<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore,
    service: &str,
    entry: PasswordEntry
) -> Result<(), &'static str> {
    let username = read_terminal_input(reader, writer, Some("Enter new username: "));
    let password = entry.password.to_owned();
    let entry = PasswordEntry::new(service.to_owned(), username, password);
    store.update_entry(entry);
    Ok(())
}

/**
 * Handle updating a password for a service.
 * The method will ask the user for a new password and then update the entry in the store.
 *
 * @param store: &mut PasswordStore
 * @param service: &str
 * @param entry: PasswordEntry
 */
fn update_password(
    store: &mut PasswordStore,
    service: &str,
    entry: PasswordEntry
) -> Result<(), &'static str> {
    let username = entry.username.to_owned();
    let password = rpassword::prompt_password("Enter new password: ").unwrap();
    let verify_password = rpassword::prompt_password("Please verify password: ").unwrap();

    if password == verify_password {
        let entry = PasswordEntry::new(service.to_owned(), username, password);
        store.update_entry(entry);
        Ok(())
    } else {
        Err("Unfortunately the entered passwords did not match, please try again")
    }
}

/**
 * Starts the dialog to update a service.
 * The user can enter a service name and the method will ask the user if they want to update the username or password.
 *
 * @param reader: &mut R
 * @param writer: &mut W
 * @param store: &mut PasswordStore
 */
pub fn handle_update_service<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    store: &mut PasswordStore
) -> Result<(), &'static str> {
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

            let entry_clone = entry.clone();

            match input.as_str() {
                "1" | "username" => update_username(reader, writer, store, &service, entry_clone),
                "2" | "password" => update_password(store, &service, entry_clone),
                _ => Err("Invalid command"),
            }
        }
        None => { Err("Could not find an entry for service") }
    }
}
