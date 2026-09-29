pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseIssuesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub message: String,
}

impl PreviewOrderResponseIssuesItem {
    pub fn builder() -> PreviewOrderResponseIssuesItemBuilder {
        <PreviewOrderResponseIssuesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseIssuesItemBuilder {
    code: Option<String>,
    path: Option<String>,
    message: Option<String>,
}

impl PreviewOrderResponseIssuesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseIssuesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PreviewOrderResponseIssuesItemBuilder::code)
    /// - [`path`](PreviewOrderResponseIssuesItemBuilder::path)
    /// - [`message`](PreviewOrderResponseIssuesItemBuilder::message)
    pub fn build(self) -> Result<PreviewOrderResponseIssuesItem, BuildError> {
        Ok(PreviewOrderResponseIssuesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
