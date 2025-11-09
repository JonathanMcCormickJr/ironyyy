//! # Database Module
//! This module handles data persistence using encrypted JSON files.
//! 
//! * Each user has their own database file (in JSON format) stored in the `databases` folder.
//! * Each database file is named after the user's UUID (e.g., `<user_uuid>.json`).
//! * The database file contains all of the user's epics and stories, as well as their account information.
//! * Each database file is encrypted with a vetted postquantum algorithm (via the `rustls` crate) using a high-entropy key reproducibly derived by concatenating the user's password and their (already-random) UUID.

use rand_core::{TryRngCore, OsRng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{epics::Epic, stories::Story};
use crate::users::User;

/// # Clear Text Database State struct
/// Represents the state of a user's database, including their account info, epics, and stories.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClearTextDBState {
    /// The user account information.
    pub user: User,
    /// The list of epics associated with the user.
    pub epics: Vec<Epic>,
    /// The list of stories associated with the user.
    pub stories: Vec<Story>,
}

impl ClearTextDBState {
    /// Creates a new, empty ClearTextDBState for the given user.
    pub fn new(user: User) -> Self {
        Self {
            user,
            epics: Vec::new(),
            stories: Vec::new(),
        }
    }

    /// Converts the ClearTextDBState into a CypherTextDBState by encrypting the data.
    pub fn to_cypher_text(self) -> Result<CypherTextDBState, Box<dyn std::error::Error>> {
        
        let serialized_data = serde_json::to_vec(&self)?;
        let encrypted_data = todo!("Implement encryption of serialized_data using a postquantum algorithm");
        // Bytes to indicate whether later decryption was successful or not
        let mut indicator = [0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut indicator)?;
        
        Ok(CypherTextDBState {
            user_uuid: self.user.user_uuid,
            username: self.user.username,
            indicator,
            encrypted_data, // In a real implementation, this would be encrypted data
        })
    }
}

/// # Cypher Text Database State struct
/// Represents the encrypted state of a user's database as a vector of bytes.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CypherTextDBState {
    /// UUID in clear
    pub user_uuid: Uuid,
    /// Username in clear
    pub username: String,
    /// Indicator bytes in clear
    pub indicator: [u8; 16],
    /// The encrypted data as a vector of bytes.
    pub encrypted_data: Vec<u8>,
}

impl CypherTextDBState {
    /// Converts the CypherTextDBState back into a ClearTextDBState by decrypting the data.
    pub fn to_clear_text(self) -> Result<ClearTextDBState, Box<dyn std::error::Error>> {
        let decrypted_data = todo!("Implement decryption of self.encrypted_data using a postquantum algorithm");
        let clear_text_db_state: ClearTextDBState = serde_json::from_slice(&decrypted_data)?;
        Ok(clear_text_db_state)
    }
}


/// # Scan for DB function
/// Scans the `databases` folder for existing user database files. Each file is parsed to extract the username and UUID, which are stored in an in-memory list of existing users for login purposes.
/// 
/// ## Example
/// ```rust
/// use crate::db::scan_for_db;
/// scan_for_db();
/// ```
/// 
/// ## Navigation side effects
/// Takes the user to the LoginOrRegister page after scanning for existing databases.
pub fn scan_for_db() -> std::io::Result<()> {
    // Scan the `databases` folder for existing user database files.
    // For each file found, parse it to extract the username and UUID.
    // Store the extracted information in an in-memory list of existing users.
    let target_folder = "databases";
    
    // Check if the target folder exists
    match std::fs::read_dir(target_folder) {
        Ok(_) => (),
        Err(_) => {
            // If the folder does not exist, create it
            std::fs::create_dir(target_folder)?;
        }
    };

    for entry in std::fs::read_dir(target_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            // Here, you would typically open the file, decrypt it, and parse the JSON
            // to extract the username and UUID. For simplicity, we'll just print the file name.
            todo!("Implement database file parsing to extract username and UUID")
        }
    }


    Ok(())
}