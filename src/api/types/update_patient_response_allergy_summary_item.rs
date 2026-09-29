pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientResponseAllergySummaryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,
    #[serde(default)]
    pub substance: String,
}

impl UpdatePatientResponseAllergySummaryItem {
    pub fn builder() -> UpdatePatientResponseAllergySummaryItemBuilder {
        <UpdatePatientResponseAllergySummaryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseAllergySummaryItemBuilder {
    reaction: Option<String>,
    substance: Option<String>,
}

impl UpdatePatientResponseAllergySummaryItemBuilder {
    pub fn reaction(mut self, value: impl Into<String>) -> Self {
        self.reaction = Some(value.into());
        self
    }

    pub fn substance(mut self, value: impl Into<String>) -> Self {
        self.substance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponseAllergySummaryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`substance`](UpdatePatientResponseAllergySummaryItemBuilder::substance)
    pub fn build(self) -> Result<UpdatePatientResponseAllergySummaryItem, BuildError> {
        Ok(UpdatePatientResponseAllergySummaryItem {
            reaction: self.reaction,
            substance: self
                .substance
                .ok_or_else(|| BuildError::missing_field("substance"))?,
        })
    }
}
