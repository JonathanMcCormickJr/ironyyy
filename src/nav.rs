//! # Navigation Module
//! This module handles navigation between different pages/screens of the CLI application.

use crate::pages::Page;

/// # Navigator type alias
/// Manages the current page and navigation history.
pub type Navigator<'a> = Vec<&'a dyn Page>;

