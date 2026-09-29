pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPatientsResponseDataItemClinicalProfile {
    #[serde(rename = "currentMedications")]
    #[serde(default)]
    pub current_medications: Vec<String>,
    #[serde(rename = "heightInches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_inches: Option<ListPatientsResponseDataItemClinicalProfileHeightInches>,
    #[serde(rename = "reviewedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[serde(rename = "weightPounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_pounds: Option<ListPatientsResponseDataItemClinicalProfileWeightPounds>,
}

impl ListPatientsResponseDataItemClinicalProfile {
    pub fn builder() -> ListPatientsResponseDataItemClinicalProfileBuilder {
        <ListPatientsResponseDataItemClinicalProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemClinicalProfileBuilder {
    current_medications: Option<Vec<String>>,
    height_inches: Option<ListPatientsResponseDataItemClinicalProfileHeightInches>,
    reviewed_at: Option<String>,
    weight_pounds: Option<ListPatientsResponseDataItemClinicalProfileWeightPounds>,
}

impl ListPatientsResponseDataItemClinicalProfileBuilder {
    pub fn current_medications(mut self, value: Vec<String>) -> Self {
        self.current_medications = Some(value);
        self
    }

    pub fn height_inches(
        mut self,
        value: ListPatientsResponseDataItemClinicalProfileHeightInches,
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
        value: ListPatientsResponseDataItemClinicalProfileWeightPounds,
    ) -> Self {
        self.weight_pounds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemClinicalProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current_medications`](ListPatientsResponseDataItemClinicalProfileBuilder::current_medications)
    pub fn build(self) -> Result<ListPatientsResponseDataItemClinicalProfile, BuildError> {
        Ok(ListPatientsResponseDataItemClinicalProfile {
            current_medications: self
                .current_medications
                .ok_or_else(|| BuildError::missing_field("current_medications"))?,
            height_inches: self.height_inches,
            reviewed_at: self.reviewed_at,
            weight_pounds: self.weight_pounds,
        })
    }
}
