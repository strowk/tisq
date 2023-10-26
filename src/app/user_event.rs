use crate::components::SentTree;

use super::{connection::DbResponse, EditorId};

#[derive(PartialOrd, Clone, Eq, Debug)]
pub(crate) enum TisqEvent {
    TreeReloaded(SentTree),
    DbResponse(DbResponse),
    EditorContentReset(EditorId, String), // TODO: use attr instead of UserEvent
}

// For the purposes of subscriptions we only care about the type of the event
// , not about the content, so we have to implement PartialEq manually
impl PartialEq for TisqEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TreeReloaded(_), Self::TreeReloaded(_)) => true,
            (Self::DbResponse(_), Self::DbResponse(_)) => true,
            (Self::EditorContentReset(_, _), Self::EditorContentReset(_, _)) => true,
            _ => false,
        }
    }
}
