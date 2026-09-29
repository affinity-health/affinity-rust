pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPracticeTeamResponsePrescribers {
    pub total: GetPracticeTeamResponsePrescribersTotal,
    pub active: GetPracticeTeamResponsePrescribersActive,
}

impl GetPracticeTeamResponsePrescribers {
    pub fn builder() -> GetPracticeTeamResponsePrescribersBuilder {
        <GetPracticeTeamResponsePrescribersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamResponsePrescribersBuilder {
    total: Option<GetPracticeTeamResponsePrescribersTotal>,
    active: Option<GetPracticeTeamResponsePrescribersActive>,
}

impl GetPracticeTeamResponsePrescribersBuilder {
    pub fn total(mut self, value: GetPracticeTeamResponsePrescribersTotal) -> Self {
        self.total = Some(value);
        self
    }

    pub fn active(mut self, value: GetPracticeTeamResponsePrescribersActive) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamResponsePrescribers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](GetPracticeTeamResponsePrescribersBuilder::total)
    /// - [`active`](GetPracticeTeamResponsePrescribersBuilder::active)
    pub fn build(self) -> Result<GetPracticeTeamResponsePrescribers, BuildError> {
        Ok(GetPracticeTeamResponsePrescribers {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
        })
    }
}
