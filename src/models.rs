//! # Models Module
//! This module contains data models used throughout the application.

pub mod epics;
pub mod stories;

use serde::{Deserialize, Serialize};

/// # Status struct 
/// 
/// (applies to Epics and Stories)
/// 
/// An epic or story can have one of the following statuses: `Open`, `InProgress`, or `Closed`.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    /// The item is freshly created and not yet started.
    #[default]
    Open = 0,
    /// The item is currently in progress.
    InProgress = 1,
    /// The item has been completed.
    Closed = 255,
}

