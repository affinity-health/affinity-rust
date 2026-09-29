pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPatientsResponseDataItemMeasurementsItem {
    #[serde(rename = "heightCentimeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_centimeters: Option<ListPatientsResponseDataItemMeasurementsItemHeightCentimeters>,
    #[serde(rename = "recordedAt")]
    #[serde(default)]
    pub recorded_at: String,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "weightKilograms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kilograms: Option<ListPatientsResponseDataItemMeasurementsItemWeightKilograms>,
}

impl ListPatientsResponseDataItemMeasurementsItem {
    pub fn builder() -> ListPatientsResponseDataItemMeasurementsItemBuilder {
        <ListPatientsResponseDataItemMeasurementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemMeasurementsItemBuilder {
    height_centimeters: Option<ListPatientsResponseDataItemMeasurementsItemHeightCentimeters>,
    recorded_at: Option<String>,
    source: Option<String>,
    weight_kilograms: Option<ListPatientsResponseDataItemMeasurementsItemWeightKilograms>,
}

impl ListPatientsResponseDataItemMeasurementsItemBuilder {
    pub fn height_centimeters(
        mut self,
        value: ListPatientsResponseDataItemMeasurementsItemHeightCentimeters,
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
        value: ListPatientsResponseDataItemMeasurementsItemWeightKilograms,
    ) -> Self {
        self.weight_kilograms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemMeasurementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recorded_at`](ListPatientsResponseDataItemMeasurementsItemBuilder::recorded_at)
    /// - [`source`](ListPatientsResponseDataItemMeasurementsItemBuilder::source)
    pub fn build(self) -> Result<ListPatientsResponseDataItemMeasurementsItem, BuildError> {
        Ok(ListPatientsResponseDataItemMeasurementsItem {
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
