#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]

//! Ironyyy is a secure, offline-first project management application focused on epics and stories.

mod db;
mod models;
mod nav;
mod pages;
mod security;
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