pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig {
    #[serde(default)]
    pub dose: String,
    #[serde(rename = "doseUnit")]
    #[serde(default)]
    pub dose_unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default)]
    pub frequency: String,
    #[serde(rename = "maxDailyUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_daily_use: Option<String>,
    #[serde(default)]
    pub prn: bool,
    #[serde(default)]
    pub route: String,
    #[serde(rename = "titrationSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titration_schedule: Option<String>,
}

impl RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig {
    pub fn builder() -> RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder
    {
        <RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder {
    dose: Option<String>,
    dose_unit: Option<String>,
    duration: Option<String>,
    frequency: Option<String>,
    max_daily_use: Option<String>,
    prn: Option<bool>,
    route: Option<String>,
    titration_schedule: Option<String>,
}

impl RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder {
    pub fn dose(mut self, value: impl Into<String>) -> Self {
        self.dose = Some(value.into());
        self
    }

    pub fn dose_unit(mut self, value: impl Into<String>) -> Self {
        self.dose_unit = Some(value.into());
        self
    }

    pub fn duration(mut self, value: impl Into<String>) -> Self {
        self.duration = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn max_daily_use(mut self, value: impl Into<String>) -> Self {
        self.max_daily_use = Some(value.into());
        self
    }

    pub fn prn(mut self, value: bool) -> Self {
        self.prn = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn titration_schedule(mut self, value: impl Into<String>) -> Self {
        self.titration_schedule = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dose`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder::dose)
    /// - [`dose_unit`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder::dose_unit)
    /// - [`frequency`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder::frequency)
    /// - [`prn`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder::prn)
    /// - [`route`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSigBuilder::route)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig, BuildError>
    {
        Ok(
            RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig {
                dose: self.dose.ok_or_else(|| BuildError::missing_field("dose"))?,
                dose_unit: self
                    .dose_unit
                    .ok_or_else(|| BuildError::missing_field("dose_unit"))?,
                duration: self.duration,
                frequency: self
                    .frequency
                    .ok_or_else(|| BuildError::missing_field("frequency"))?,
                max_daily_use: self.max_daily_use,
                prn: self.prn.ok_or_else(|| BuildError::missing_field("prn"))?,
                route: self
                    .route
                    .ok_or_else(|| BuildError::missing_field("route"))?,
                titration_schedule: self.titration_schedule,
            },
        )
    }
}
