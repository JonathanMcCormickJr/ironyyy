#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![forbid(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Ironyyy is a secure, offline-first project management application focused on epics and stories.

mod db;
mod models;
mod nav;
mod pages;
pub mod security;
mod users;

/// Runs the Ironyyy application.
/// 
/// # Examples
/// ```
/// use ironyyy::run_app;
/// run_app();
/// ```
pub fn run_app() {
    // Application entry point
    todo!()
}
