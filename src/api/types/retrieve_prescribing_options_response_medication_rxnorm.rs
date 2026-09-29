pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseMedicationRxnorm {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
    #[serde(rename = "doseForm")]
    #[serde(default)]
    pub dose_form: String,
    #[serde(default)]
    pub route: String,
    pub system: RetrievePrescribingOptionsResponseMedicationRxnormSystem,
}

impl RetrievePrescribingOptionsResponseMedicationRxnorm {
    pub fn builder() -> RetrievePrescribingOptionsResponseMedicationRxnormBuilder {
        <RetrievePrescribingOptionsResponseMedicationRxnormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseMedicationRxnormBuilder {
    code: Option<String>,
    display: Option<String>,
    dose_form: Option<String>,
    route: Option<String>,
    system: Option<RetrievePrescribingOptionsResponseMedicationRxnormSystem>,
}

impl RetrievePrescribingOptionsResponseMedicationRxnormBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    pub fn dose_form(mut self, value: impl Into<String>) -> Self {
        self.dose_form = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn system(
        mut self,
        value: RetrievePrescribingOptionsResponseMedicationRxnormSystem,
    ) -> Self {
        self.system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseMedicationRxnorm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RetrievePrescribingOptionsResponseMedicationRxnormBuilder::code)
    /// - [`display`](RetrievePrescribingOptionsResponseMedicationRxnormBuilder::display)
    /// - [`dose_form`](RetrievePrescribingOptionsResponseMedicationRxnormBuilder::dose_form)
    /// - [`route`](RetrievePrescribingOptionsResponseMedicationRxnormBuilder::route)
    /// - [`system`](RetrievePrescribingOptionsResponseMedicationRxnormBuilder::system)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseMedicationRxnorm, BuildError> {
        Ok(RetrievePrescribingOptionsResponseMedicationRxnorm {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
            dose_form: self
                .dose_form
                .ok_or_else(|| BuildError::missing_field("dose_form"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
        })
    }
}
