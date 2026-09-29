pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestShipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<PreviewOrderRequestShippingSelection>,
}

impl PreviewOrderRequestShipping {
    pub fn builder() -> PreviewOrderRequestShippingBuilder {
        <PreviewOrderRequestShippingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestShippingBuilder {
    selection: Option<PreviewOrderRequestShippingSelection>,
}

impl PreviewOrderRequestShippingBuilder {
    pub fn selection(mut self, value: PreviewOrderRequestShippingSelection) -> Self {
        self.selection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestShipping`].
    pub fn build(self) -> Result<PreviewOrderRequestShipping, BuildError> {
        Ok(PreviewOrderRequestShipping {
            selection: self.selection,
        })
    }
}
