pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePatientRequestClinicalProfile {
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(rename = "heightInches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_inches: Option<UpdatePatientRequestClinicalProfileHeightInches>,
    #[serde(rename = "reviewedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(rename = "weightPounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_pounds: Option<UpdatePatientRequestClinicalProfileWeightPounds>,
}

impl UpdatePatientRequestClinicalProfile {
    pub fn builder() -> UpdatePatientRequestClinicalProfileBuilder {
        <UpdatePatientRequestClinicalProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestClinicalProfileBuilder {
    current_medications: Option<Vec<String>>,
    height_inches: Option<UpdatePatientRequestClinicalProfileHeightInches>,
    reviewed_at: Option<String>,
    weight_pounds: Option<UpdatePatientRequestClinicalProfileWeightPounds>,
}

impl UpdatePatientRequestClinicalProfileBuilder {
    pub fn current_medications(mut self, value: Vec<String>) -> Self {
        self.current_medications = Some(value);
        self
    }

    pub fn height_inches(mut self, value: UpdatePatientRequestClinicalProfileHeightInches) -> Self {
        self.height_inches = Some(value);
        self
    }

    pub fn reviewed_at(mut self, value: impl Into<String>) -> Self {
        self.reviewed_at = Some(value.into());
        self
    }

    pub fn weight_pounds(mut self, value: UpdatePatientRequestClinicalProfileWeightPounds) -> Self {
        self.weight_pounds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientRequestClinicalProfile`].
    pub fn build(self) -> Result<UpdatePatientRequestClinicalProfile, BuildError> {
        Ok(UpdatePatientRequestClinicalProfile {
            current_medications: self.current_medications,
            height_inches: self.height_inches,
            reviewed_at: self.reviewed_at,
            weight_pounds: self.weight_pounds,
        })
    }
}
