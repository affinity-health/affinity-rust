pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig {
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

impl RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig {
    pub fn builder() -> RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder {
        <RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder {
    dose: Option<String>,
    dose_unit: Option<String>,
    duration: Option<String>,
    frequency: Option<String>,
    max_daily_use: Option<String>,
    prn: Option<bool>,
    route: Option<String>,
    titration_schedule: Option<String>,
}

impl RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder {
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

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dose`](RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder::dose)
    /// - [`dose_unit`](RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder::dose_unit)
    /// - [`frequency`](RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder::frequency)
    /// - [`prn`](RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder::prn)
    /// - [`route`](RetrievePrescribingOptionsResponseFormulationDefaultStructuredSigBuilder::route)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig, BuildError> {
        Ok(
            RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig {
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
