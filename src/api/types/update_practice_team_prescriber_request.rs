pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamPrescriberRequest {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePracticeTeamPrescriberRequestAddress>,
    #[serde(rename = "practiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_status: Option<UpdatePracticeTeamPrescriberRequestPracticeStatus>,
}

impl UpdatePracticeTeamPrescriberRequest {
    pub fn builder() -> UpdatePracticeTeamPrescriberRequestBuilder {
        <UpdatePracticeTeamPrescriberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamPrescriberRequestBuilder {
    display_name: Option<String>,
    legal_name: Option<String>,
    credentials: Option<String>,
    phone: Option<String>,
    address: Option<UpdatePracticeTeamPrescriberRequestAddress>,
    practice_status: Option<UpdatePracticeTeamPrescriberRequestPracticeStatus>,
}

impl UpdatePracticeTeamPrescriberRequestBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn address(mut self, value: UpdatePracticeTeamPrescriberRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn practice_status(
        mut self,
        value: UpdatePracticeTeamPrescriberRequestPracticeStatus,
    ) -> Self {
        self.practice_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamPrescriberRequest`].
    pub fn build(self) -> Result<UpdatePracticeTeamPrescriberRequest, BuildError> {
        Ok(UpdatePracticeTeamPrescriberRequest {
            display_name: self.display_name,
            legal_name: self.legal_name,
            credentials: self.credentials,
            phone: self.phone,
            address: self.address,
            practice_status: self.practice_status,
        })
    }
}
