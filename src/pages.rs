//! # Pages Module
//! This module contains different pages/screens of the CLI application.

use uuid::Uuid;

/// # Page trait
/// Represents a page/screen in the CLI application.
pub trait Page {
    fn render(&self) -> Vec<String>;
    fn handle_input(&self, input: &str);
}

// TODO: Implement specific pages like LoginPage, DashboardPage, EpicCreationPage, StoryCreationPage, etc.

/// # DetectedUsers type alias
/// A list of detected users represented by their UUID and username.
type DetectedUsers = Vec<(Uuid, String)>;

/// # LoginOrRegisterPage struct
/// Represents the page for the user to select whether to log into an existing account or register a new one.
pub struct LoginOrRegisterPage(DetectedUsers);

