pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Problem {
    #[serde(default)]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub instance: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub status: i64,
    #[serde(default)]
    pub title: String,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default)]
    pub r#type: String,
}

impl Problem {
    pub fn builder() -> ProblemBuilder {
        <ProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemBuilder {
    code: Option<String>,
    data: Option<HashMap<String, serde_json::Value>>,
    detail: Option<String>,
    instance: Option<String>,
    request_id: Option<String>,
    status: Option<i64>,
    title: Option<String>,
    trace_id: Option<String>,
    r#type: Option<String>,
}

impl ProblemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn instance(mut self, value: impl Into<String>) -> Self {
        self.instance = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Problem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ProblemBuilder::code)
    /// - [`detail`](ProblemBuilder::detail)
    /// - [`instance`](ProblemBuilder::instance)
    /// - [`request_id`](ProblemBuilder::request_id)
    /// - [`status`](ProblemBuilder::status)
    /// - [`title`](ProblemBuilder::title)
    /// - [`r#type`](ProblemBuilder::r#type)
    pub fn build(self) -> Result<Problem, BuildError> {
        Ok(Problem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            data: self.data,
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            instance: self
                .instance
                .ok_or_else(|| BuildError::missing_field("instance"))?,
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            trace_id: self.trace_id,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
