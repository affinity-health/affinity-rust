pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponseOrderInputPatientMeasurementsItem {
    #[serde(rename = "heightCentimeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_centimeters:
        Option<PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters>,
    #[serde(rename = "recordedAt")]
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "weightKilograms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kilograms:
        Option<PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms>,
}

impl PreviewOrderResponseOrderInputPatientMeasurementsItem {
    pub fn builder() -> PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder {
        <PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder {
    height_centimeters:
        Option<PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters>,
    recorded_at: Option<String>,
    source: Option<String>,
    weight_kilograms: Option<PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms>,
}

impl PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder {
    pub fn height_centimeters(
        mut self,
        value: PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters,
    ) -> Self {
        self.height_centimeters = Some(value);
        self
    }

    pub fn recorded_at(mut self, value: impl Into<String>) -> Self {
        self.recorded_at = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn weight_kilograms(
        mut self,
        value: PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms,
    ) -> Self {
        self.weight_kilograms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPatientMeasurementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recorded_at`](PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder::recorded_at)
    /// - [`source`](PreviewOrderResponseOrderInputPatientMeasurementsItemBuilder::source)
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPatientMeasurementsItem, BuildError> {
        Ok(PreviewOrderResponseOrderInputPatientMeasurementsItem {
            height_centimeters: self.height_centimeters,
            recorded_at: self
                .recorded_at
                .ok_or_else(|| BuildError::missing_field("recorded_at"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            weight_kilograms: self.weight_kilograms,
        })
    }
}
