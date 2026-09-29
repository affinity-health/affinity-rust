pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponsePrescriptionsItemStructuredSig {
    #[serde(default)]
    pub dose: String,
    #[serde(rename = "doseUnit")]
    #[serde(default)]
    pub dose_unit: String,
    #[serde(default)]
    pub frequency: String,
    #[serde(default)]
    pub route: String,
    #[serde(default)]
    pub prn: bool,
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

impl PreviewOrderResponsePrescriptionsItemStructuredSig {
    pub fn builder() -> PreviewOrderResponsePrescriptionsItemStructuredSigBuilder {
        <PreviewOrderResponsePrescriptionsItemStructuredSigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponsePrescriptionsItemStructuredSigBuilder {
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

impl PreviewOrderResponsePrescriptionsItemStructuredSigBuilder {
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

    /// Consumes the builder and constructs a [`PreviewOrderResponsePrescriptionsItemStructuredSig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dose`](PreviewOrderResponsePrescriptionsItemStructuredSigBuilder::dose)
    /// - [`dose_unit`](PreviewOrderResponsePrescriptionsItemStructuredSigBuilder::dose_unit)
    /// - [`frequency`](PreviewOrderResponsePrescriptionsItemStructuredSigBuilder::frequency)
    /// - [`route`](PreviewOrderResponsePrescriptionsItemStructuredSigBuilder::route)
    /// - [`prn`](PreviewOrderResponsePrescriptionsItemStructuredSigBuilder::prn)
    pub fn build(self) -> Result<PreviewOrderResponsePrescriptionsItemStructuredSig, BuildError> {
        Ok(PreviewOrderResponsePrescriptionsItemStructuredSig {
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
            prn: self.prn.ok_or_else(|| BuildError::missing_field("prn"))?,
            duration: self.duration,
            indication: self.indication,
            max_daily_use: self.max_daily_use,
            titration_schedule: self.titration_schedule,
        })
    }
}
