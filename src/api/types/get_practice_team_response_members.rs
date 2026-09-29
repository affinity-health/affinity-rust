pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPracticeTeamResponseMembers {
    pub total: GetPracticeTeamResponseMembersTotal,
    pub active: GetPracticeTeamResponseMembersActive,
    pub disabled: GetPracticeTeamResponseMembersDisabled,
}

impl GetPracticeTeamResponseMembers {
    pub fn builder() -> GetPracticeTeamResponseMembersBuilder {
        <GetPracticeTeamResponseMembersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamResponseMembersBuilder {
    total: Option<GetPracticeTeamResponseMembersTotal>,
    active: Option<GetPracticeTeamResponseMembersActive>,
    disabled: Option<GetPracticeTeamResponseMembersDisabled>,
}

impl GetPracticeTeamResponseMembersBuilder {
    pub fn total(mut self, value: GetPracticeTeamResponseMembersTotal) -> Self {
        self.total = Some(value);
        self
    }

    pub fn active(mut self, value: GetPracticeTeamResponseMembersActive) -> Self {
        self.active = Some(value);
        self
    }

    pub fn disabled(mut self, value: GetPracticeTeamResponseMembersDisabled) -> Self {
        self.disabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamResponseMembers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](GetPracticeTeamResponseMembersBuilder::total)
    /// - [`active`](GetPracticeTeamResponseMembersBuilder::active)
    /// - [`disabled`](GetPracticeTeamResponseMembersBuilder::disabled)
    pub fn build(self) -> Result<GetPracticeTeamResponseMembers, BuildError> {
        Ok(GetPracticeTeamResponseMembers {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            disabled: self
                .disabled
                .ok_or_else(|| BuildError::missing_field("disabled"))?,
        })
    }
}
