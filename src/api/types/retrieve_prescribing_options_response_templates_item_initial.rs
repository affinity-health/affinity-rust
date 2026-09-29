pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseTemplatesItemInitial {
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
}

impl RetrievePrescribingOptionsResponseTemplatesItemInitial {
    pub fn builder() -> RetrievePrescribingOptionsResponseTemplatesItemInitialBuilder {
        <RetrievePrescribingOptionsResponseTemplatesItemInitialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseTemplatesItemInitialBuilder {
    dose: Option<String>,
    dose_unit: Option<String>,
    duration: Option<String>,
    frequency: Option<String>,
    max_daily_use: Option<String>,
    prn: Option<bool>,
    route: Option<String>,
}

impl RetrievePrescribingOptionsResponseTemplatesItemInitialBuilder {
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

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseTemplatesItemInitial`].
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseTemplatesItemInitial, BuildError> {
        Ok(RetrievePrescribingOptionsResponseTemplatesItemInitial {
            dose: self.dose,
            dose_unit: self.dose_unit,
            duration: self.duration,
            frequency: self.frequency,
            max_daily_use: self.max_daily_use,
            prn: self.prn,
            route: self.route,
        })
    }
}
