pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePatientResponseMeasurementsItem {
    #[serde(rename = "heightCentimeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_centimeters: Option<UpdatePatientResponseMeasurementsItemHeightCentimeters>,
    #[serde(rename = "recordedAt")]
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "weightKilograms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kilograms: Option<UpdatePatientResponseMeasurementsItemWeightKilograms>,
}

impl UpdatePatientResponseMeasurementsItem {
    pub fn builder() -> UpdatePatientResponseMeasurementsItemBuilder {
        <UpdatePatientResponseMeasurementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseMeasurementsItemBuilder {
    height_centimeters: Option<UpdatePatientResponseMeasurementsItemHeightCentimeters>,
    recorded_at: Option<String>,
    source: Option<String>,
    weight_kilograms: Option<UpdatePatientResponseMeasurementsItemWeightKilograms>,
}

impl UpdatePatientResponseMeasurementsItemBuilder {
    pub fn height_centimeters(
        mut self,
        value: UpdatePatientResponseMeasurementsItemHeightCentimeters,
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
        value: UpdatePatientResponseMeasurementsItemWeightKilograms,
    ) -> Self {
        self.weight_kilograms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponseMeasurementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recorded_at`](UpdatePatientResponseMeasurementsItemBuilder::recorded_at)
    /// - [`source`](UpdatePatientResponseMeasurementsItemBuilder::source)
    pub fn build(self) -> Result<UpdatePatientResponseMeasurementsItem, BuildError> {
        Ok(UpdatePatientResponseMeasurementsItem {
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
