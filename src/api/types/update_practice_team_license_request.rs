pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamLicenseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "licenseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl UpdatePracticeTeamLicenseRequest {
    pub fn builder() -> UpdatePracticeTeamLicenseRequestBuilder {
        <UpdatePracticeTeamLicenseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamLicenseRequestBuilder {
    state: Option<String>,
    license_number: Option<String>,
    expires_at: Option<String>,
}

impl UpdatePracticeTeamLicenseRequestBuilder {
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn license_number(mut self, value: impl Into<String>) -> Self {
        self.license_number = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamLicenseRequest`].
    pub fn build(self) -> Result<UpdatePracticeTeamLicenseRequest, BuildError> {
        Ok(UpdatePracticeTeamLicenseRequest {
            state: self.state,
            license_number: self.license_number,
            expires_at: self.expires_at,
        })
    }
}
