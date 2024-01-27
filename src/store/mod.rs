use std::{
    fs::{ self, OpenOptions, read_to_string },
    collections::HashMap,
    path::PathBuf,
    io::{ BufWriter, Write },
};

use crate::password::PasswordEntry;

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

    pub fn load(&mut self) {
        let contents = fs
            ::read_to_string(self.file_path.clone())
            .expect("Something went wrong reading the file");

        for line in contents.lines() {
            let mut split = line.split_whitespace();

            let service = split.next().unwrap().to_string();
            let username = split.next().unwrap().to_string();
            let password = split.next().unwrap().to_string();

            let entry = PasswordEntry::new(service, username, password);
            self.add(entry);
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

    pub fn check_for_duplicate_service_entry(&self, service: &str) -> bool {
        let is_duplicate = self.passwords.iter().any(|(_, e)| e.service == service);

        is_duplicate
    }

    /** Saves entry to file */
    fn save(&self, entry: PasswordEntry) -> Result<PasswordEntry, &'static str> {
        println!("Saving entry for service: {} to file...", &entry.service);
        let file = OpenOptions::new().write(true).append(true).open(&self.file_path).unwrap();

        let mut writer = BufWriter::new(file);

        if
            let Err(_) = writeln!(
                writer,
                "{} {} {}",
                &entry.service,
                &entry.username,
                &entry.password
            )
        {
            return Err("Couldn't write to file");
        } else {
            return Ok(entry);
        }
    }

    pub fn add_and_save_entry(&mut self, entry: PasswordEntry) {
        self.add(entry.clone());
        let save_result = self.save(entry);

        match save_result {
            Ok(entry) => println!("Successfully saved entry for {} to file! 🎉", &entry.service),
            Err(_) => println!("Couldn't save entry to file"),
        }
    }

    pub fn update_entry(&mut self, entry: PasswordEntry) {
        let content = read_to_string(&self.file_path);
        // let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&self.file_path);

        match content {
            Ok(content) => {
                content
                    .lines()
                    .find(|line| line.contains(&entry.service))
                    .map(|line| {
                        // Replace the line with the new entry
                        let new_content = content.replace(
                            line,
                            &format!("{} {} {}", &entry.service, &entry.username, &entry.password)
                        );
                        // Write the new content to the file
                        fs::write(&self.file_path, new_content).expect("Unable to write file");
                    });
            }
            Err(error) => eprintln!("Was not able to update entry.\n {}", error),
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
}
