use super::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Epic struct
/// Represents an epic in the project management system.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Epic {
    /// The unique identifier of the epic.
    pub epic_uuid: Uuid,
    /// The title of the epic.
    pub title: String,
    /// The description of the epic.
    pub description: String,
    /// The status of the epic.
    pub status: Status,    
    /// The list of story UUIDs associated with this epic.
    pub story_uuids: Vec<Uuid>,
}

impl Epic {
    /// Creates a new epic with the given title and description.
    pub fn new(title: String, description: String) -> Self {
        Self {
            epic_uuid: Uuid::new_v4(),
            title,
            description,
            status: Status::Open,
            story_uuids: Vec::new(),
        }
    }
}