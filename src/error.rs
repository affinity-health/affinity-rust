use crate::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {message}")]
    BadRequestError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("UnauthorizedError: Authentication failed - {message}")]
    UnauthorizedError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("ForbiddenError: Access forbidden - {message}")]
    ForbiddenError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("NotFoundError: Resource not found - {message}")]
    NotFoundError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("ConflictError: Conflict - {message}")]
    ConflictError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("TooManyRequestsError: Rate limit exceeded - {message}")]
    TooManyRequestsError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("UnprocessableEntityError: Unprocessable entity - {message}")]
    UnprocessableEntityError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("ServiceUnavailableError: {message}")]
    ServiceUnavailableError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("BadGatewayError: {message}")]
    BadGatewayError {
        message: String,
        code: Option<String>,
        data: Option<HashMap<String, serde_json::Value>>,
        detail: Option<String>,
        instance: Option<String>,
        request_id: Option<String>,
        status: Option<i64>,
        title: Option<String>,
        trace_id: Option<String>,
        type_: Option<String>,
    },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Request executor error: {0}")]
    Executor(Box<dyn std::error::Error + Send + Sync>),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadRequestError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            403 => {
                // Parse error body for ForbiddenError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ForbiddenError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ForbiddenError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::NotFoundError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            409 => {
                // Parse error body for ConflictError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ConflictError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ConflictError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            429 => {
                // Parse error body for TooManyRequestsError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::TooManyRequestsError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::TooManyRequestsError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            422 => {
                // Parse error body for UnprocessableEntityError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnprocessableEntityError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnprocessableEntityError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            503 => {
                // Parse error body for ServiceUnavailableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ServiceUnavailableError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ServiceUnavailableError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            502 => {
                // Parse error body for BadGatewayError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadGatewayError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            code: parsed
                                .get("code")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            data: parsed.get("data").and_then(|v| {
                                serde_json::from_value::<HashMap<String, serde_json::Value>>(
                                    v.clone(),
                                )
                                .ok()
                            }),
                            detail: parsed
                                .get("detail")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            instance: parsed
                                .get("instance")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            request_id: parsed
                                .get("requestId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            status: parsed
                                .get("status")
                                .and_then(|v| serde_json::from_value::<i64>(v.clone()).ok()),
                            title: parsed
                                .get("title")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            trace_id: parsed
                                .get("traceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            type_: parsed
                                .get("type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadGatewayError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    code: None,
                    data: None,
                    detail: None,
                    instance: None,
                    request_id: None,
                    status: None,
                    title: None,
                    trace_id: None,
                    type_: None,
                };
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
