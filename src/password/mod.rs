use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String, //TODO remove pub
}

impl PasswordEntry {
    pub fn new(service: String, username: String, password: String) -> PasswordEntry {
        PasswordEntry {
            service,
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Passwords(Vec<PasswordEntry>);

impl Passwords {
    pub fn new() -> Passwords {
        Passwords(Vec::new())
    }
}

pub struct Password();

impl Password {
    pub fn generate() -> String {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        rand::thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passwords() {
        assert_eq!(Passwords::new(), Passwords(vec![]));
    }
}
