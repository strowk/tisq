use crate::components::SentTree;

use super::connection::DbResponse;

#[derive(PartialOrd, Clone, Eq, Debug)]
pub(crate) enum TisqEvent {
    TreeReloaded(SentTree),
    DbResponse(DbResponse),
}

// For the purposes of subscriptions we only care about the type of the event
// , not about the content, so we have to implement PartialEq manually
impl PartialEq for TisqEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TreeReloaded(_), Self::TreeReloaded(_)) => true,
            (Self::DbResponse(_), Self::DbResponse(_)) => true,
            _ => false,
        }
    }
}
