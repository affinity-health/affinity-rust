pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPracticeTeamResponseInvitations {
    pub pending: GetPracticeTeamResponseInvitationsPending,
    pub expired: GetPracticeTeamResponseInvitationsExpired,
}

impl GetPracticeTeamResponseInvitations {
    pub fn builder() -> GetPracticeTeamResponseInvitationsBuilder {
        <GetPracticeTeamResponseInvitationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamResponseInvitationsBuilder {
    pending: Option<GetPracticeTeamResponseInvitationsPending>,
    expired: Option<GetPracticeTeamResponseInvitationsExpired>,
}

impl GetPracticeTeamResponseInvitationsBuilder {
    pub fn pending(mut self, value: GetPracticeTeamResponseInvitationsPending) -> Self {
        self.pending = Some(value);
        self
    }

    pub fn expired(mut self, value: GetPracticeTeamResponseInvitationsExpired) -> Self {
        self.expired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamResponseInvitations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pending`](GetPracticeTeamResponseInvitationsBuilder::pending)
    /// - [`expired`](GetPracticeTeamResponseInvitationsBuilder::expired)
    pub fn build(self) -> Result<GetPracticeTeamResponseInvitations, BuildError> {
        Ok(GetPracticeTeamResponseInvitations {
            pending: self
                .pending
                .ok_or_else(|| BuildError::missing_field("pending"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
        })
    }
}
