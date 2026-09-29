pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAccountResponseAccount {
    #[serde(rename = "allowedReturnUrls")]
    #[serde(default)]
    pub allowed_return_urls: Vec<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    pub object: GetAccountResponseAccountObject,
    #[serde(default)]
    pub slug: String,
    pub status: GetAccountResponseAccountStatus,
    #[serde(rename = "supportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "websiteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

impl GetAccountResponseAccount {
    pub fn builder() -> GetAccountResponseAccountBuilder {
        <GetAccountResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseAccountBuilder {
    allowed_return_urls: Option<Vec<String>>,
    display_name: Option<String>,
    id: Option<String>,
    object: Option<GetAccountResponseAccountObject>,
    slug: Option<String>,
    status: Option<GetAccountResponseAccountStatus>,
    support_email: Option<String>,
    website_url: Option<String>,
}

impl GetAccountResponseAccountBuilder {
    pub fn allowed_return_urls(mut self, value: Vec<String>) -> Self {
        self.allowed_return_urls = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: GetAccountResponseAccountObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetAccountResponseAccountStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn support_email(mut self, value: impl Into<String>) -> Self {
        self.support_email = Some(value.into());
        self
    }

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed_return_urls`](GetAccountResponseAccountBuilder::allowed_return_urls)
    /// - [`display_name`](GetAccountResponseAccountBuilder::display_name)
    /// - [`id`](GetAccountResponseAccountBuilder::id)
    /// - [`object`](GetAccountResponseAccountBuilder::object)
    /// - [`slug`](GetAccountResponseAccountBuilder::slug)
    /// - [`status`](GetAccountResponseAccountBuilder::status)
    pub fn build(self) -> Result<GetAccountResponseAccount, BuildError> {
        Ok(GetAccountResponseAccount {
            allowed_return_urls: self
                .allowed_return_urls
                .ok_or_else(|| BuildError::missing_field("allowed_return_urls"))?,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            support_email: self.support_email,
            website_url: self.website_url,
        })
    }
}
