pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePatientRequestMeasurementsItem {
    #[serde(rename = "heightCentimeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_centimeters: Option<UpdatePatientRequestMeasurementsItemHeightCentimeters>,
    #[serde(rename = "recordedAt")]
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "weightKilograms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kilograms: Option<UpdatePatientRequestMeasurementsItemWeightKilograms>,
}

impl UpdatePatientRequestMeasurementsItem {
    pub fn builder() -> UpdatePatientRequestMeasurementsItemBuilder {
        <UpdatePatientRequestMeasurementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestMeasurementsItemBuilder {
    height_centimeters: Option<UpdatePatientRequestMeasurementsItemHeightCentimeters>,
    recorded_at: Option<String>,
    source: Option<String>,
    weight_kilograms: Option<UpdatePatientRequestMeasurementsItemWeightKilograms>,
}

impl UpdatePatientRequestMeasurementsItemBuilder {
    pub fn height_centimeters(
        mut self,
        value: UpdatePatientRequestMeasurementsItemHeightCentimeters,
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
        value: UpdatePatientRequestMeasurementsItemWeightKilograms,
    ) -> Self {
        self.weight_kilograms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientRequestMeasurementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recorded_at`](UpdatePatientRequestMeasurementsItemBuilder::recorded_at)
    /// - [`source`](UpdatePatientRequestMeasurementsItemBuilder::source)
    pub fn build(self) -> Result<UpdatePatientRequestMeasurementsItem, BuildError> {
        Ok(UpdatePatientRequestMeasurementsItem {
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
