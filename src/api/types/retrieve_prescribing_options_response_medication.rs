pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseMedication {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rxnorm: Option<RetrievePrescribingOptionsResponseMedicationRxnorm>,
}

impl RetrievePrescribingOptionsResponseMedication {
    pub fn builder() -> RetrievePrescribingOptionsResponseMedicationBuilder {
        <RetrievePrescribingOptionsResponseMedicationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseMedicationBuilder {
    name: Option<String>,
    rxnorm: Option<RetrievePrescribingOptionsResponseMedicationRxnorm>,
}

impl RetrievePrescribingOptionsResponseMedicationBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rxnorm(mut self, value: RetrievePrescribingOptionsResponseMedicationRxnorm) -> Self {
        self.rxnorm = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseMedication`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RetrievePrescribingOptionsResponseMedicationBuilder::name)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseMedication, BuildError> {
        Ok(RetrievePrescribingOptionsResponseMedication {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rxnorm: self.rxnorm,
        })
    }
}
