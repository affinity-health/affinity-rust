pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields {
    #[serde(default)]
    pub dose: String,
    #[serde(rename = "doseUnit")]
    #[serde(default)]
    pub dose_unit: String,
    #[serde(default)]
    pub frequency: String,
    #[serde(default)]
    pub route: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<String>,
    #[serde(rename = "maxDailyUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_daily_use: Option<String>,
    #[serde(rename = "titrationSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titration_schedule: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder {
    dose: Option<String>,
    dose_unit: Option<String>,
    frequency: Option<String>,
    route: Option<String>,
    prn: Option<bool>,
    duration: Option<String>,
    indication: Option<String>,
    max_daily_use: Option<String>,
    titration_schedule: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder {
    pub fn dose(mut self, value: impl Into<String>) -> Self {
        self.dose = Some(value.into());
        self
    }

    pub fn dose_unit(mut self, value: impl Into<String>) -> Self {
        self.dose_unit = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn prn(mut self, value: bool) -> Self {
        self.prn = Some(value);
        self
    }

    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    pub fn indication(mut self, value: impl Into<String>) -> Self {
        self.indication = Some(value.into());
        self
    }

    pub fn max_daily_use(mut self, value: impl Into<String>) -> Self {
        self.max_daily_use = Some(value.into());
        self
    }

    pub fn titration_schedule(mut self, value: impl Into<String>) -> Self {
        self.titration_schedule = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dose`](PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder::dose)
    /// - [`dose_unit`](PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder::dose_unit)
    /// - [`frequency`](PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder::frequency)
    /// - [`route`](PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFieldsBuilder::route)
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields, BuildError> {
        Ok(
            PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields {
                dose: self.dose.ok_or_else(|| BuildError::missing_field("dose"))?,
                dose_unit: self
                    .dose_unit
                    .ok_or_else(|| BuildError::missing_field("dose_unit"))?,
                frequency: self
                    .frequency
                    .ok_or_else(|| BuildError::missing_field("frequency"))?,
                route: self
                    .route
                    .ok_or_else(|| BuildError::missing_field("route"))?,
                prn: self.prn,
                duration: self.duration,
                indication: self.indication,
                max_daily_use: self.max_daily_use,
                titration_schedule: self.titration_schedule,
            },
        )
    }
}
