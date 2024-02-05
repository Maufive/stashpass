use std::{
    fs::{ self, OpenOptions, File },
    collections::HashMap,
    path::PathBuf,
    io::{ BufWriter, Write, BufReader },
};

use crate::password::PasswordEntry;
use serde_json::{ Value, Map, json };

/**
 * Password Store
 * The PasswordStore is responsible for managing the passwords, saving them to file and
 * reading into memory.
 *
 * It exposes methods to work with password entries and the file.
 */
#[derive(Debug)]
pub struct PasswordStore {
    passwords: HashMap<String, PasswordEntry>,
    file_path: PathBuf,
}

impl PasswordStore {
    pub fn new(file_path: PathBuf) -> Result<PasswordStore, &'static str> {
        let mut store = PasswordStore {
            passwords: HashMap::new(),
            file_path: file_path.clone(),
        };

        if file_path.exists() {
            store.load();
        } else {
            fs::File::create(file_path).expect("Unable to create file");
        }

        Ok(store)
    }

    /**
     * Load passwords from file into memory
     *
     * The method will read the file and parse the content into a PasswordEntry object.
     * The PasswordEntry object will then be added to the in-memory store.
     */
    pub fn load(&mut self) {
        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);
        let json_obj: Map<String, Value> = serde_json::from_reader(reader).unwrap();

        for (service, entry) in json_obj.iter() {
            let username = entry["username"].as_str().unwrap().to_string();
            let password = entry["password"].as_str().unwrap().to_string();
            let password_entry = PasswordEntry::new(service.clone(), username, password);

            self.add(password_entry);
        }
    }

    fn add(&mut self, entry: PasswordEntry) {
        self.passwords.insert(entry.service.clone(), entry);
    }

    pub fn get(&self, service: &str) -> Option<&PasswordEntry> {
        self.passwords.get(service)
    }

    pub fn get_file_path(&self) -> PathBuf {
        self.file_path.clone()
    }

    /**
     * List all passwords
     * The method will loop over the in-memory store and print the service and username for each entry.
     */
    pub fn list_all(&self) {
        for (service, entry) in self.passwords.iter() {
            println!("Service: {}, Username: {}", service, entry.username);
        }
    }

    /**
     * Check for duplicate service entry
     * The method will loop over the in-memory store and check if the service already exists.
     */
    pub fn check_for_duplicate_service_entry(&self, service: &str) -> bool {
        let is_duplicate = self.passwords.iter().any(|(_, e)| e.service == service);

        is_duplicate
    }

    /**
     * Save entry to file
     * The method will read the existing JSON file, add the new entry to the JSON object and then
     * write the updated JSON object to the file.
     *
     * @param entry: PasswordEntry
     * @return Result<PasswordEntry, &'static str>
     */
    fn save_entry(&self, entry: PasswordEntry) -> Result<PasswordEntry, &'static str> {
        println!("Saving entry for service: {} to file...", &entry.service);

        // Read the existing JSON file
        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);
        let mut json_obj: Map<String, Value> = match serde_json::from_reader(reader) {
            Ok(json) => json,
            Err(_) => Map::new(), // In case the file is empty
        };

        // Add new entry to the JSON object
        json_obj.insert(
            entry.service.clone(),
            json!({
                "username": &entry.username,
                "password": &entry.password
            })
        );

        // Write the updated JSON object to the file
        let file = OpenOptions::new().write(true).truncate(true).open(&self.file_path).unwrap();
        let mut writer = BufWriter::new(file);
        let result = writer.write_all(serde_json::to_string_pretty(&json_obj).unwrap().as_bytes());

        match result {
            Ok(_) => Ok(entry),
            Err(_) => Err("Failed to save entry to file"),
        }
    }

    /**
     * Add and save entry
     * The method will add the entry to the in-memory store and then save the entry to the file.
     * If the entry is successfully saved to the file, the method will print a success message.
     * If the entry fails to save to the file, the method will print an error message.
     *
     * @param entry: PasswordEntry
     * @return Result<&str, &'static str>
     */
    pub fn add_and_save_entry(&mut self, entry: PasswordEntry) -> Result<&str, &'static str> {
        // Add to the in-memory store
        self.add(entry.clone());
        // Save to file
        let save_result = self.save_entry(entry);

        match save_result {
            Ok(_) => Ok("Password entry was successfully saved to file"),
            Err(err) => Err(err),
        }
    }

    /**
     * Update entry in file
     * The method will read the existing JSON file, update the entry with the new password and then
     * write the updated JSON object to the file.
     *
     * @param entry: PasswordEntry
     * @return Result<(), &'static str>
     */
    pub fn update_entry(&mut self, entry: PasswordEntry) -> Result<(), &'static str> {
        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);

        // Read the existing JSON file
        let mut json_obj: Map<String, Value> = match serde_json::from_reader(reader) {
            Ok(json) => json,
            Err(_) => Map::new(), // In case the file is empty
        };

        // Update the JSON object with the new entry
        json_obj.insert(
            entry.service.clone(),
            json!({
                "username": &entry.username,
                "password": &entry.password
            })
        );

        // Write the updated JSON object to the file
        let file = OpenOptions::new().write(true).truncate(true).open(&self.file_path).unwrap();
        let mut writer = BufWriter::new(file);
        let result = writer.write_all(serde_json::to_string_pretty(&json_obj).unwrap().as_bytes());

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to update entry in file"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_password_store() {
        let store = PasswordStore::new(PathBuf::from("passwords.txt")).unwrap();

        assert_eq!(store.passwords.len(), 0);
    }

    #[test]
    fn test_check_for_duplicate() {
        let mut store = PasswordStore::new(PathBuf::from("passwords.txt")).unwrap();

        let entry = PasswordEntry::new(
            "service".to_string(),
            "username".to_string(),
            "password".to_string()
        );
        store.add(entry.clone());
        let is_duplicate = store.check_for_duplicate_service_entry("service");

        assert_eq!(is_duplicate, true);
    }

    #[test]
    fn test_get_file_path() {
        let store = PasswordStore::new(PathBuf::from("passwords.txt")).unwrap();

        assert_eq!(store.get_file_path(), PathBuf::from("passwords.txt"));
    }

    #[test]
    fn test_update_service_password() {
        let mut store = PasswordStore::new(PathBuf::from("test_passwords.json")).unwrap();

        let entry = PasswordEntry::new(
            "service".to_string(),
            "username".to_string(),
            "password".to_string()
        );

        let result = store.add_and_save_entry(entry.clone());

        assert_eq!(result, Ok("Password entry was successfully saved to file"));

        let updated_entry = PasswordEntry::new(
            "service".to_string(),
            "username".to_string(),
            "new_password".to_string()
        );

        let result = store.update_entry(updated_entry.clone());

        assert_eq!(result, Ok(()));

        let file_content = fs::read_to_string(store.get_file_path()).unwrap();
        let json_obj: Map<String, Value> = serde_json::from_str(&file_content).unwrap();
        let password = json_obj.get("service").unwrap();

        assert_eq!(password["password"], "new_password");
    }
}
