pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookGrantsResponseDataItem {
    /// Platform account ID; use as the pagination cursor within this owner's grants.
    #[serde(default)]
    pub id: String,
    pub object: ListWebhookGrantsResponseDataItemObject,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub scopes: Vec<ListWebhookGrantsResponseDataItemScopesItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl ListWebhookGrantsResponseDataItem {
    pub fn builder() -> ListWebhookGrantsResponseDataItemBuilder {
        <ListWebhookGrantsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookGrantsResponseDataItemBuilder {
    id: Option<String>,
    object: Option<ListWebhookGrantsResponseDataItemObject>,
    organization_id: Option<String>,
    platform_id: Option<String>,
    livemode: Option<bool>,
    scopes: Option<Vec<ListWebhookGrantsResponseDataItemScopesItem>>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl ListWebhookGrantsResponseDataItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListWebhookGrantsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn platform_id(mut self, value: impl Into<String>) -> Self {
        self.platform_id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<ListWebhookGrantsResponseDataItemScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookGrantsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListWebhookGrantsResponseDataItemBuilder::id)
    /// - [`object`](ListWebhookGrantsResponseDataItemBuilder::object)
    /// - [`organization_id`](ListWebhookGrantsResponseDataItemBuilder::organization_id)
    /// - [`platform_id`](ListWebhookGrantsResponseDataItemBuilder::platform_id)
    /// - [`livemode`](ListWebhookGrantsResponseDataItemBuilder::livemode)
    /// - [`scopes`](ListWebhookGrantsResponseDataItemBuilder::scopes)
    /// - [`created_at`](ListWebhookGrantsResponseDataItemBuilder::created_at)
    /// - [`updated_at`](ListWebhookGrantsResponseDataItemBuilder::updated_at)
    pub fn build(self) -> Result<ListWebhookGrantsResponseDataItem, BuildError> {
        Ok(ListWebhookGrantsResponseDataItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            platform_id: self
                .platform_id
                .ok_or_else(|| BuildError::missing_field("platform_id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
