# Ironyy

A complete rewrite of the CLI Jira clone project as part of the LGR Rust Developer Bootcamp.

By Jonathan McCormick Jr. 

## Architecture
* Monolithic CLI Application
* Written in Rust
* Single-threaded, synchronous execution model
* Only supports a single user logged in at a time
* Data Persistence via Encrypted JSON Files

### Models
* User
    * username
    * UUIDv4
    * password (hashed)
    * an optional TOTP 2FA setup (using the `easy_totp` crate)
    * has personal sovereignty over their own respective data.
* Epic
    * UUIDv4
    * Title
    * Description
    * `Status`
    * Stories (stored as a `Vec<Uuid>`)
* Story
    * UUIDv4
    * Title
    * Description
    * `Status`
* `Status`
    * An epic or story can have one of the following statuses: `Open`, `InProgress`, or `Closed`.
* `Page`
    * A `Page` represents a complete screen in the CLI application, such as the Login Page, Dashboard Page, Epic Creation Page, Story Creation Page, etc.
    * The application has a stack of `Page`s to manage navigation between different screens. When a user navigates to a new screen, a new `Page` is pushed onto the stack. When they go back, the top `Page` is popped off the stack.
    * New types of `Page`s can be created by implementing the `Page` trait, which requires methods for rendering the page and handling user input. This system should be extinsible enough to allow for future addition of more complex pages and navigation flows without major refactoring.
    
### Database
* Each user has their own database file (in JSON format) stored in the `databases` folder.
* Each database file is named after the user's UUID (e.g., `<user_uuid>.json`).
* The database file contains all of the user's epics and stories, as well as their account information.
* Each database file is encrypted with a vetted postquantum algorithm (via the `rustls` crate) using a high-entropy key reproducibly derived by concatenating the user's password and their (already-random) UUID.


### Control Flow
Control flow impacts many different parts of the program. As such each function must have proper side effects for all applicable parts of the program. For the sake of simplicity, this program eliminates the in-memory abstraction of the database state, and instead reads/writes directly to the database file each time a lookup or state change is needed. Below is a list of the main user actions and their expected side effects. Some actions require reading from the database file, while other require a full read-modify-write cycle. Each action's side effects are broken down into Database Side Effects and Navigation Side Effects.

#### Pattern:
| Action Name | Example |
|-------------|---------|
| Description | A brief description of what the action does. |
| Database Side Effects | How the action affects the user's database file. |
| Navigation Side Effects | How the action affects the application's navigation stack and current page. |
| Next Actions | A list of possible next actions the user/app can take from this point. |

#### System Actions

| Action Name | `scan_for_db()` |
|-------------|-----------------|
| Description | Scans the `databases` folder for existing user database files. Each file is parsed to extract the username and UUID, which are stored in an in-memory list of existing users for login purposes. |
| Database Side Effects | None. |
| Navigation Side Effects | If database(s) are found, it displays the UUID and username of each valid file in order for the user to select to login as any of those users, or to register a new user. If no valid database file is found, then only the new registration option gets presented. |
| Next Actions | `authenticate_user()` (if existing user DB is found), or `register_new_user()` |

| Action Name | `quit_application()` |
|-------------|----------------------|
| Description | Exits the application gracefully. |
| Database Side Effects | None. |
| Navigation Side Effects | Terminates the application process. |
| Next Actions | None. Application exits. |

#### Account Actions

| Action Name | `register_new_user()` |
|-------------|-----------------------|
| Description | Prompts the user to enter a username and password to create a new user account. |
| Database Side Effects | Generates a new UUIDv4 for the user, creates a new database file in the `databases` folder named `<user_uuid>.json`, and writes the user's account information: username (plaintext), UUID (plaintext), indicator_string (plaintext), indicator string (encrypted), hashed password (encrypted), empty epics (encrypted) and stories (encrypted) to the file. Encrypted data is encrypted with their password. |
| Navigation Side Effects | After successful registration, the user is logged in and taken to the Dashboard Page. |
| Next Actions | `show_dashboard()` |

| Action Name | `authenticate_user()` |
|-------------|-----------------------|
| Description | Prompts the user to enter their username, password, and (if 2FA is enabled) a TOTP code from their authenticator app to log in. User may also quit the app without authenticating. |
| Database Side Effects | Reads the user's database file from the `databases` folder, verifies the entered password against the hashed password stored in the file, and (if applicable) verifies the entered TOTP code using the `easy_totp` crate. |
| Navigation Side Effects | If authentication is successful, the user is logged in and taken to the Dashboard Page. If authentication fails, an error message is displayed and the user is returned to the original login screen. If the user quits, then the app terminates gracefully. |
| Next Actions | `show_dashboard()` (on success), `scan_for_db()` (on failure), `quit_application()` (on quit) |

| Action Name | `logout_user()` |
|-------------|-----------------|
| Description | Logs the user out of their account. |
| Database Side Effects | None. The user's database file remains intact. |
| Navigation Side Effects | Clears the user's data from the in-memory application and returns to the login screen. |
| Next Actions | `scan_for_db()` |

| Action Name | `export_data(encryption_password: Option(&str), export_path: &str)` |
|-------------|---------------------------------------------------------------------|
| Description | Exports the user's data (account info, epics, and stories) to a JSON file at the specified export path. The user can choose to export the data with or without encryption. |
| Database Side Effects | Reads the user's database file from the `databases` folder. If an encryption password is provided, the data is encrypted with that password before being written to the export file. If no encryption password is provided, the data is written in plaintext. |
| Navigation Side Effects | After successful export, the user is presented with a confirmation message and given the option to return to the Dashboard Page. |
| Next Actions | `show_dashboard()` |

| Action Name | `enable_2fa()` |
|-------------|----------------|
| Description | Enables TOTP-based 2FA for the user's account. |
| Database Side Effects | Updates the user's database file to include the TOTP instance using the `easy_totp` crate. |
| Navigation Side Effects | After successful setup, the user is presented with a confirmation message and given the option to return to the Dashboard Page. |
| Next Actions | `show_dashboard()` |

| Action Name | `disable_2fa()` |
|-------------|-----------------|
| Description | Disables TOTP-based 2FA for the user's account. |
| Database Side Effects | Updates the user's database file to remove the TOTP instance. |
| Navigation Side Effects | After successful disablement, the user is presented with a confirmation message and given the option to return to the Dashboard Page. |
| Next Actions | `show_dashboard()` |

| Action Name | `change_password()` |
|-------------|---------------------|
| Description | Changes the user's account password. |
| Database Side Effects | Reads the user's database file, decrypts it with the old password, and re-encrypts it with the new password. Updates the hashed password in the database file. |
| Navigation Side Effects | After successful password change, the user is presented with a confirmation message and given the option to return to the Dashboard Page. |
| Next Actions | `show_dashboard()` |

| Action Name | `delete_account()` |
|-------------|--------------------|
| Description | Deletes the user's account and all associated data. |
| Database Side Effects | Deletes the user's database file from the `databases` folder. |
| Navigation Side Effects | Clears the user's data from the in-memory application and returns to the login screen. |
| Next Actions | `scan_for_db()` |

#### Navigation Actions

| Action Name | `show_dashboard()` |
|-------------|--------------------|
| Description | Displays the Dashboard Page, which shows an overview of the user's epics, stories, and account settings. |
| Database Side Effects | Reads the user's database file from the `databases` folder to retrieve the latest data for display. |
| Navigation Side Effects | Clears the page stack and sets the current page to the Dashboard Page. |
| Next Actions | `create_epic()`, `create_story()`, `view_epics()`, `export_data()`, `delete_account()`, `logout_user()`, `enable_2fa()`/`disable_2fa()`, `change_password()`, `quit_application()` |

| Action Name | `go_back()` |
|-------------|-------------|
| Description | Navigates back to the previous page in the application's navigation stack. |
| Database Side Effects | None. |
| Navigation Side Effects | Pops the current page off the navigation stack and sets the previous page as the current page. |
| Next Actions | Depends on the previous page in the navigation stack. |

#### Epic & Story Actions

| Action Name | `create_epic()` |
|-------------|-----------------|
| Description | Creates a new epic in the user's database. |
| Database Side Effects | Reads the user's database file, adds a new epic with a unique UUIDv4, title, description, `Open` status, and an empty collection of stories, then writes the updated data back to the file. |
| Navigation Side Effects | After successful creation, the user is presented with a confirmation message and given the option to return to the Dashboard Page or view details for their newly formed epic. |
| Next Actions | `show_dashboard()`, `view_epic_details(epic_id: Uuid)` |

| Action Name | `create_story()` |
|-------------|------------------|
| Description | Creates a new story in the user's database and associates it with an epic. |
| Database Side Effects | Reads the user's database file, adds a new story with a unique UUIDv4, title, description, `Open` status, then associates it with the selected epic by adding the story's UUID to the epic's collection of stories, then writes the updated data back to the file. |
| Navigation Side Effects | After successful creation, the user is presented with a confirmation message and given the option to return to the Dashboard Page, view details for the containing epic, or view details for their newly formed story. |
| Next Actions | `show_dashboard()`, `view_epic_details(epic_id: Uuid)`, `view_story_details(story_id: Uuid)` |

| Action Name | `edit_epic(epic_id: Uuid)` |
|-------------|---------------|
| Description | Edits an existing epic in the user's database. |
| Database Side Effects | Reads the user's database file, updates the selected epic's title, description, or status, then writes the updated data back to the file. |
| Navigation Side Effects | After successful edit, the user is presented with a confirmation message and given the option to return to the Dashboard Page or view details for the edited epic. |
| Next Actions | `show_dashboard()`, `view_epic_details(epic_id: Uuid)` |

| Action Name | `edit_story(story_id: Uuid)` |
|-------------|----------------|
| Description | Edits an existing story in the user's database. |
| Database Side Effects | Reads the user's database file, updates the selected story's title, description, or status, then writes the updated data back to the file. |
| Navigation Side Effects | After successful edit, the user is presented with a confirmation message and given the option to return to the Dashboard Page, view details for the containing epic, or view details for the edited story. |
| Next Actions | `show_dashboard()`, `view_epic_details(epic_id: Uuid)`, `view_story_details(story_id: Uuid)` |

| Action Name | `view_epics()` |
|-------------|----------------|
| Description | Displays a list of all epics in the user's database, along with their titles & statuses. |
| Database Side Effects | Reads the user's database file to retrieve the list of epics. |
| Navigation Side Effects | Sets the current page to the Epics List Page. |
| Next Actions | `view_epic_details(epic_id: Uuid)`, `go_back()` |

| Action Name | `view_epic_details(epic_id: Uuid)` |
|-------------|------------------------------------|
| Description | Displays the details of a selected epic, including title, description, status, and associated stories. |
| Database Side Effects | Reads the user's database file to retrieve the details of the selected epic and its associated stories. |
| Navigation Side Effects | Sets the current page to the Epic Details Page. |
| Next Actions | `edit_epic()`, `view_stories(epic_id: Uuid)`, `delete_epic()`, `go_back()` |

| Action Name | `view_stories(epic_id: Uuid)` |
|-------------|-------------------------------|
| Description | Displays a list of all stories in the user's database associated with a selected epic, along with their titles & statuses. |
| Database Side Effects | Reads the user's database file to retrieve the list of stories associated with the selected epic. |
| Navigation Side Effects | Sets the current page to the Stories List Page for the selected epic. |
| Next Actions | `view_story_details(story_id: Uuid)`, `go_back()` |

| Action Name | `view_story_details(story_id: Uuid)` |
|-------------|-------------------------------------|
| Description | Displays the details of a selected story, including title, description, and status. |
| Database Side Effects | Reads the user's database file to retrieve the details of the selected story.
| Navigation Side Effects | Sets the current page to the Story Details Page. |
| Next Actions | `edit_story()`, `delete_story()`, `go_back()` |

| Action Name | `delete_epic(epic_id: Uuid)` |
|-------------|------------------------------|
| Description | Deletes a selected epic and all associated stories from the user's database. |
| Database Side Effects | Reads the user's database file, removes the selected epic and all associated stories, then writes the updated data back to the file. |
| Navigation Side Effects | After successful deletion, the user is presented with a confirmation message and given the option to return to the Epics List Page or the Dashboard Page. |
| Next Actions | `view_epics()`, `show_dashboard()` |

| Action Name | `delete_story(story_id: Uuid)` |
|-------------|--------------------------------|
| Description | Deletes a selected story from the user's database. |
| Database Side Effects | Reads the user's database file, removes the selected story, then writes the updated data back to the file. |
| Navigation Side Effects | After successful deletion, the user is presented with a confirmation message and given the option to return to the Stories List Page for the containing epic or the Dashboard Page. |
| Next Actions | `view_stories(epic_id: Uuid)`, `show_dashboard()` |

### Helper Functions

Various helper functions are implemented to support the main actions, such as:
* `hash_password(password: &str, user_uuid: Uuid) -> String` - Hashes a password using Argon2 with the user's UUID as the salt.
* `verify_password(password: &str, hashed_password: &str, user_uuid: Uuid) -> bool` - Verifies a password against a hashed password using Argon2. Internally calls `hash_password()` and compares the result.
* `generate_encryption_key(password: &str, user_uuid: Uuid) -> [u8; 64]` - Generates a high-entropy encryption key by concatenating the user's password and UUID, then hashing the result with SHA-512.
* `encrypt_data(data: &str, key: &[u8; 64]) -> Vec<u8>` - Encrypts data using the provided encryption key with a vetted postquantum algorithm (via the `rustls` crate).
* `decrypt_data(encrypted_data: &[u8], key: &[u8; 64]) -> String` - Decrypts data using the provided encryption key with a vetted postquantum algorithm (via the `rustls` crate).
* `atomic_write_to_file(file_path: &str, data: &[u8]) -> Result<(), std::io::Error>` - Writes data to a file atomically to prevent data corruption. Uses a temporary file and renames it to the target file path upon successful write.
* `create_databases_folder_if_none_exists()` - Checks if the `databases` folder exists, and creates it if it doesn't.
* `clear_stack()` - Clears the page stack to reset navigation at the Dashboard Page.
* `push_page(page: Box<dyn Page>)` - Pushes a new page onto the navigation stack.
* `pop_page() -> Option<Box<dyn Page>>` - Pops the top page off the navigation stack and returns it.
