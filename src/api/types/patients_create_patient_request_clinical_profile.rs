pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePatientRequestClinicalProfile {
    #[serde(rename = "currentMedications")]
    #[serde(default)]
    pub current_medications: Vec<String>,
    #[serde(rename = "heightInches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_inches: Option<CreatePatientRequestClinicalProfileHeightInches>,
    #[serde(rename = "reviewedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(rename = "weightPounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_pounds: Option<CreatePatientRequestClinicalProfileWeightPounds>,
}

impl CreatePatientRequestClinicalProfile {
    pub fn builder() -> CreatePatientRequestClinicalProfileBuilder {
        <CreatePatientRequestClinicalProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientRequestClinicalProfileBuilder {
    current_medications: Option<Vec<String>>,
    height_inches: Option<CreatePatientRequestClinicalProfileHeightInches>,
    reviewed_at: Option<String>,
    weight_pounds: Option<CreatePatientRequestClinicalProfileWeightPounds>,
}

impl CreatePatientRequestClinicalProfileBuilder {
    pub fn current_medications(mut self, value: Vec<String>) -> Self {
        self.current_medications = Some(value);
        self
    }

    pub fn height_inches(mut self, value: CreatePatientRequestClinicalProfileHeightInches) -> Self {
        self.height_inches = Some(value);
        self
    }

    pub fn reviewed_at(mut self, value: impl Into<String>) -> Self {
        self.reviewed_at = Some(value.into());
        self
    }

    pub fn weight_pounds(mut self, value: CreatePatientRequestClinicalProfileWeightPounds) -> Self {
        self.weight_pounds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientRequestClinicalProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current_medications`](CreatePatientRequestClinicalProfileBuilder::current_medications)
    pub fn build(self) -> Result<CreatePatientRequestClinicalProfile, BuildError> {
        Ok(CreatePatientRequestClinicalProfile {
            current_medications: self
                .current_medications
                .ok_or_else(|| BuildError::missing_field("current_medications"))?,
            height_inches: self.height_inches,
            reviewed_at: self.reviewed_at,
            weight_pounds: self.weight_pounds,
        })
    }
}
