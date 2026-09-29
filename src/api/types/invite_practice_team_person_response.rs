pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponse {
    pub person: InvitePracticeTeamPersonResponsePerson,
    pub delivery: InvitePracticeTeamPersonResponseDelivery,
}

impl InvitePracticeTeamPersonResponse {
    pub fn builder() -> InvitePracticeTeamPersonResponseBuilder {
        <InvitePracticeTeamPersonResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponseBuilder {
    person: Option<InvitePracticeTeamPersonResponsePerson>,
    delivery: Option<InvitePracticeTeamPersonResponseDelivery>,
}

impl InvitePracticeTeamPersonResponseBuilder {
    pub fn person(mut self, value: InvitePracticeTeamPersonResponsePerson) -> Self {
        self.person = Some(value);
        self
    }

    pub fn delivery(mut self, value: InvitePracticeTeamPersonResponseDelivery) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`person`](InvitePracticeTeamPersonResponseBuilder::person)
    /// - [`delivery`](InvitePracticeTeamPersonResponseBuilder::delivery)
    pub fn build(self) -> Result<InvitePracticeTeamPersonResponse, BuildError> {
        Ok(InvitePracticeTeamPersonResponse {
            person: self
                .person
                .ok_or_else(|| BuildError::missing_field("person"))?,
            delivery: self
                .delivery
                .ok_or_else(|| BuildError::missing_field("delivery"))?,
        })
    }
}
