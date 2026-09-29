pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPatientsResponseDataItemAllergySummaryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<String>,
    #[serde(default)]
    pub substance: String,
}

impl ListPatientsResponseDataItemAllergySummaryItem {
    pub fn builder() -> ListPatientsResponseDataItemAllergySummaryItemBuilder {
        <ListPatientsResponseDataItemAllergySummaryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemAllergySummaryItemBuilder {
    reaction: Option<String>,
    substance: Option<String>,
}

impl ListPatientsResponseDataItemAllergySummaryItemBuilder {
    pub fn reaction(mut self, value: impl Into<String>) -> Self {
        self.reaction = Some(value.into());
        self
    }

    pub fn substance(mut self, value: impl Into<String>) -> Self {
        self.substance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemAllergySummaryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`substance`](ListPatientsResponseDataItemAllergySummaryItemBuilder::substance)
    pub fn build(self) -> Result<ListPatientsResponseDataItemAllergySummaryItem, BuildError> {
        Ok(ListPatientsResponseDataItemAllergySummaryItem {
            reaction: self.reaction,
            substance: self
                .substance
                .ok_or_else(|| BuildError::missing_field("substance"))?,
        })
    }
}
