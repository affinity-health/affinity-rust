pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderRequestPatientClinicalProfile {
    #[serde(rename = "currentMedications")]
    #[serde(default)]
    pub current_medications: Vec<String>,
    #[serde(rename = "heightInches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_inches: Option<CreateOrderRequestPatientClinicalProfileHeightInches>,
    #[serde(rename = "reviewedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(rename = "weightPounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_pounds: Option<CreateOrderRequestPatientClinicalProfileWeightPounds>,
}

impl CreateOrderRequestPatientClinicalProfile {
    pub fn builder() -> CreateOrderRequestPatientClinicalProfileBuilder {
        <CreateOrderRequestPatientClinicalProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPatientClinicalProfileBuilder {
    current_medications: Option<Vec<String>>,
    height_inches: Option<CreateOrderRequestPatientClinicalProfileHeightInches>,
    reviewed_at: Option<String>,
    weight_pounds: Option<CreateOrderRequestPatientClinicalProfileWeightPounds>,
}

impl CreateOrderRequestPatientClinicalProfileBuilder {
    pub fn current_medications(mut self, value: Vec<String>) -> Self {
        self.current_medications = Some(value);
        self
    }

    pub fn height_inches(
        mut self,
        value: CreateOrderRequestPatientClinicalProfileHeightInches,
    ) -> Self {
        self.height_inches = Some(value);
        self
    }

    pub fn reviewed_at(mut self, value: impl Into<String>) -> Self {
        self.reviewed_at = Some(value.into());
        self
    }

    pub fn weight_pounds(
        mut self,
        value: CreateOrderRequestPatientClinicalProfileWeightPounds,
    ) -> Self {
        self.weight_pounds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPatientClinicalProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current_medications`](CreateOrderRequestPatientClinicalProfileBuilder::current_medications)
    pub fn build(self) -> Result<CreateOrderRequestPatientClinicalProfile, BuildError> {
        Ok(CreateOrderRequestPatientClinicalProfile {
            current_medications: self
                .current_medications
                .ok_or_else(|| BuildError::missing_field("current_medications"))?,
            height_inches: self.height_inches,
            reviewed_at: self.reviewed_at,
            weight_pounds: self.weight_pounds,
        })
    }
}
