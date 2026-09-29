pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseOptions {
    #[serde(rename = "doseUnits")]
    #[serde(default)]
    pub dose_units: Vec<RetrievePrescribingOptionsResponseOptionsDoseUnitsItem>,
    #[serde(default)]
    pub doses: Vec<RetrievePrescribingOptionsResponseOptionsDosesItem>,
    #[serde(default)]
    pub frequencies: Vec<RetrievePrescribingOptionsResponseOptionsFrequenciesItem>,
    #[serde(default)]
    pub routes: Vec<RetrievePrescribingOptionsResponseOptionsRoutesItem>,
}

impl RetrievePrescribingOptionsResponseOptions {
    pub fn builder() -> RetrievePrescribingOptionsResponseOptionsBuilder {
        <RetrievePrescribingOptionsResponseOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseOptionsBuilder {
    dose_units: Option<Vec<RetrievePrescribingOptionsResponseOptionsDoseUnitsItem>>,
    doses: Option<Vec<RetrievePrescribingOptionsResponseOptionsDosesItem>>,
    frequencies: Option<Vec<RetrievePrescribingOptionsResponseOptionsFrequenciesItem>>,
    routes: Option<Vec<RetrievePrescribingOptionsResponseOptionsRoutesItem>>,
}

impl RetrievePrescribingOptionsResponseOptionsBuilder {
    pub fn dose_units(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseOptionsDoseUnitsItem>,
    ) -> Self {
        self.dose_units = Some(value);
        self
    }

    pub fn doses(mut self, value: Vec<RetrievePrescribingOptionsResponseOptionsDosesItem>) -> Self {
        self.doses = Some(value);
        self
    }

    pub fn frequencies(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseOptionsFrequenciesItem>,
    ) -> Self {
        self.frequencies = Some(value);
        self
    }

    pub fn routes(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseOptionsRoutesItem>,
    ) -> Self {
        self.routes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseOptions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dose_units`](RetrievePrescribingOptionsResponseOptionsBuilder::dose_units)
    /// - [`doses`](RetrievePrescribingOptionsResponseOptionsBuilder::doses)
    /// - [`frequencies`](RetrievePrescribingOptionsResponseOptionsBuilder::frequencies)
    /// - [`routes`](RetrievePrescribingOptionsResponseOptionsBuilder::routes)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseOptions, BuildError> {
        Ok(RetrievePrescribingOptionsResponseOptions {
            dose_units: self
                .dose_units
                .ok_or_else(|| BuildError::missing_field("dose_units"))?,
            doses: self
                .doses
                .ok_or_else(|| BuildError::missing_field("doses"))?,
            frequencies: self
                .frequencies
                .ok_or_else(|| BuildError::missing_field("frequencies"))?,
            routes: self
                .routes
                .ok_or_else(|| BuildError::missing_field("routes"))?,
        })
    }
}
