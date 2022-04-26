use crate::client::Client;
use std::collections::HashMap;

// Main state of the App
pub struct AppState {
    // Name of transaction file
    pub file_name: String,
    // Map of clients
    pub clients: HashMap<u16,Client>,
}

impl AppState {
    // Returns new App state
    pub fn new<T: AsRef<str>>(file_name: T) -> AppState {
        AppState{
            file_name: file_name.as_ref().to_string(),
            clients: HashMap::new(),
        }
    }

    // Returns the name of the file
    pub fn get_file_name(&self) -> String {
        return self.file_name.clone()
    }
}
