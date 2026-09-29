pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequestPrescriptionsItem {
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(rename = "expectedRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<PreviewOrderRequestPrescriptionsItemOverrides>,
}

impl PreviewOrderRequestPrescriptionsItem {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemBuilder {
        <PreviewOrderRequestPrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemBuilder {
    medication_id: Option<String>,
    external_prescription_id: Option<String>,
    preset: Option<String>,
    expected_revision: Option<String>,
    overrides: Option<PreviewOrderRequestPrescriptionsItemOverrides>,
}

impl PreviewOrderRequestPrescriptionsItemBuilder {
    pub fn medication_id(mut self, value: impl Into<String>) -> Self {
        self.medication_id = Some(value.into());
        self
    }

    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn preset(mut self, value: impl Into<String>) -> Self {
        self.preset = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: impl Into<String>) -> Self {
        self.expected_revision = Some(value.into());
        self
    }

    pub fn overrides(mut self, value: PreviewOrderRequestPrescriptionsItemOverrides) -> Self {
        self.overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`medication_id`](PreviewOrderRequestPrescriptionsItemBuilder::medication_id)
    pub fn build(self) -> Result<PreviewOrderRequestPrescriptionsItem, BuildError> {
        Ok(PreviewOrderRequestPrescriptionsItem {
            medication_id: self
                .medication_id
                .ok_or_else(|| BuildError::missing_field("medication_id"))?,
            external_prescription_id: self.external_prescription_id,
            preset: self.preset,
            expected_revision: self.expected_revision,
            overrides: self.overrides,
        })
    }
}
