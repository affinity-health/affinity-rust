pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
    pub fn builder() -> AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
        <AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder::code)
    /// - [`display`](AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem, BuildError> {
        Ok(
            AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
            },
        )
    }
}
