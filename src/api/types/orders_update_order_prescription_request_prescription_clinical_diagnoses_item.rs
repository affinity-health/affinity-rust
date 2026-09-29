pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
    pub fn builder() -> UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
        <UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder::code)
    /// - [`display`](UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem, BuildError> {
        Ok(
            UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
            },
        )
    }
}
