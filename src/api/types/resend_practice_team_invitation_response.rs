pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendPracticeTeamInvitationResponse {
    pub invitation: ResendPracticeTeamInvitationResponseInvitation,
    pub delivery: ResendPracticeTeamInvitationResponseDelivery,
}

impl ResendPracticeTeamInvitationResponse {
    pub fn builder() -> ResendPracticeTeamInvitationResponseBuilder {
        <ResendPracticeTeamInvitationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendPracticeTeamInvitationResponseBuilder {
    invitation: Option<ResendPracticeTeamInvitationResponseInvitation>,
    delivery: Option<ResendPracticeTeamInvitationResponseDelivery>,
}

impl ResendPracticeTeamInvitationResponseBuilder {
    pub fn invitation(mut self, value: ResendPracticeTeamInvitationResponseInvitation) -> Self {
        self.invitation = Some(value);
        self
    }

    pub fn delivery(mut self, value: ResendPracticeTeamInvitationResponseDelivery) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendPracticeTeamInvitationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invitation`](ResendPracticeTeamInvitationResponseBuilder::invitation)
    /// - [`delivery`](ResendPracticeTeamInvitationResponseBuilder::delivery)
    pub fn build(self) -> Result<ResendPracticeTeamInvitationResponse, BuildError> {
        Ok(ResendPracticeTeamInvitationResponse {
            invitation: self
                .invitation
                .ok_or_else(|| BuildError::missing_field("invitation"))?,
            delivery: self
                .delivery
                .ok_or_else(|| BuildError::missing_field("delivery"))?,
        })
    }
}
