use super::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Story struct
/// Represents a story in the project management system.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Story {
    /// The unique identifier of the story.
    pub story_uuid: Uuid,
    /// The title of the story.
    pub title: String,
    /// The description of the story.
    pub description: String,
    /// The status of the story.
    pub status: Status,    
}

impl Story {
    /// Creates a new story with the given title and description.
    pub fn new(title: String, description: String) -> Self {
        Self {
            story_uuid: Uuid::new_v4(),
            title,
            description,
            status: Status::Open,
        }
    }
}