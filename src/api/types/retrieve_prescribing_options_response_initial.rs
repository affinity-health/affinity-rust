pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseInitial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dose: Option<String>,
    #[serde(rename = "doseUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dose_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "maxDailyUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_daily_use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(rename = "titrationSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titration_schedule: Option<String>,
}

impl RetrievePrescribingOptionsResponseInitial {
    pub fn builder() -> RetrievePrescribingOptionsResponseInitialBuilder {
        <RetrievePrescribingOptionsResponseInitialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseInitialBuilder {
    dose: Option<String>,
    dose_unit: Option<String>,
    duration: Option<String>,
    frequency: Option<String>,
    max_daily_use: Option<String>,
    prn: Option<bool>,
    route: Option<String>,
    titration_schedule: Option<String>,
}

impl RetrievePrescribingOptionsResponseInitialBuilder {
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

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseInitial`].
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseInitial, BuildError> {
        Ok(RetrievePrescribingOptionsResponseInitial {
            dose: self.dose,
            dose_unit: self.dose_unit,
            duration: self.duration,
            frequency: self.frequency,
            max_daily_use: self.max_daily_use,
            prn: self.prn,
            route: self.route,
            titration_schedule: self.titration_schedule,
        })
    }
}
