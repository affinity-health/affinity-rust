pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreviewOrderResponse {
    #[serde(rename = "clinicalRequirementsSatisfied")]
    #[serde(default)]
    pub clinical_requirements_satisfied: bool,
    #[serde(rename = "clinicalIssues")]
    #[serde(default)]
    pub clinical_issues: Vec<PreviewOrderResponseClinicalIssuesItem>,
    #[serde(rename = "clinicalRequirements")]
    #[serde(default)]
    pub clinical_requirements: Vec<PreviewOrderResponseClinicalRequirementsItem>,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<PreviewOrderResponseOtcItemsItem>,
    #[serde(rename = "shippingGroups")]
    #[serde(default)]
    pub shipping_groups: Vec<PreviewOrderResponseShippingGroupsItem>,
    pub totals: PreviewOrderResponseTotals,
    pub object: PreviewOrderResponseObject,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub prescriptions: Vec<PreviewOrderResponsePrescriptionsItem>,
    #[serde(default)]
    pub issues: Vec<PreviewOrderResponseIssuesItem>,
    pub status: PreviewOrderResponseStatus,
    #[serde(rename = "orderInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_input: Option<PreviewOrderResponseOrderInput>,
}

impl PreviewOrderResponse {
    pub fn builder() -> PreviewOrderResponseBuilder {
        <PreviewOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseBuilder {
    clinical_requirements_satisfied: Option<bool>,
    clinical_issues: Option<Vec<PreviewOrderResponseClinicalIssuesItem>>,
    clinical_requirements: Option<Vec<PreviewOrderResponseClinicalRequirementsItem>>,
    otc_items: Option<Vec<PreviewOrderResponseOtcItemsItem>>,
    shipping_groups: Option<Vec<PreviewOrderResponseShippingGroupsItem>>,
    totals: Option<PreviewOrderResponseTotals>,
    object: Option<PreviewOrderResponseObject>,
    livemode: Option<bool>,
    prescriptions: Option<Vec<PreviewOrderResponsePrescriptionsItem>>,
    issues: Option<Vec<PreviewOrderResponseIssuesItem>>,
    status: Option<PreviewOrderResponseStatus>,
    order_input: Option<PreviewOrderResponseOrderInput>,
}

impl PreviewOrderResponseBuilder {
    pub fn clinical_requirements_satisfied(mut self, value: bool) -> Self {
        self.clinical_requirements_satisfied = Some(value);
        self
    }

    pub fn clinical_issues(mut self, value: Vec<PreviewOrderResponseClinicalIssuesItem>) -> Self {
        self.clinical_issues = Some(value);
        self
    }

    pub fn clinical_requirements(
        mut self,
        value: Vec<PreviewOrderResponseClinicalRequirementsItem>,
    ) -> Self {
        self.clinical_requirements = Some(value);
        self
    }

    pub fn otc_items(mut self, value: Vec<PreviewOrderResponseOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn shipping_groups(mut self, value: Vec<PreviewOrderResponseShippingGroupsItem>) -> Self {
        self.shipping_groups = Some(value);
        self
    }

    pub fn totals(mut self, value: PreviewOrderResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    pub fn object(mut self, value: PreviewOrderResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn prescriptions(mut self, value: Vec<PreviewOrderResponsePrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn issues(mut self, value: Vec<PreviewOrderResponseIssuesItem>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn status(mut self, value: PreviewOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn order_input(mut self, value: PreviewOrderResponseOrderInput) -> Self {
        self.order_input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clinical_requirements_satisfied`](PreviewOrderResponseBuilder::clinical_requirements_satisfied)
    /// - [`clinical_issues`](PreviewOrderResponseBuilder::clinical_issues)
    /// - [`clinical_requirements`](PreviewOrderResponseBuilder::clinical_requirements)
    /// - [`otc_items`](PreviewOrderResponseBuilder::otc_items)
    /// - [`shipping_groups`](PreviewOrderResponseBuilder::shipping_groups)
    /// - [`totals`](PreviewOrderResponseBuilder::totals)
    /// - [`object`](PreviewOrderResponseBuilder::object)
    /// - [`livemode`](PreviewOrderResponseBuilder::livemode)
    /// - [`prescriptions`](PreviewOrderResponseBuilder::prescriptions)
    /// - [`issues`](PreviewOrderResponseBuilder::issues)
    /// - [`status`](PreviewOrderResponseBuilder::status)
    pub fn build(self) -> Result<PreviewOrderResponse, BuildError> {
        Ok(PreviewOrderResponse {
            clinical_requirements_satisfied: self
                .clinical_requirements_satisfied
                .ok_or_else(|| BuildError::missing_field("clinical_requirements_satisfied"))?,
            clinical_issues: self
                .clinical_issues
                .ok_or_else(|| BuildError::missing_field("clinical_issues"))?,
            clinical_requirements: self
                .clinical_requirements
                .ok_or_else(|| BuildError::missing_field("clinical_requirements"))?,
            otc_items: self
                .otc_items
                .ok_or_else(|| BuildError::missing_field("otc_items"))?,
            shipping_groups: self
                .shipping_groups
                .ok_or_else(|| BuildError::missing_field("shipping_groups"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            order_input: self.order_input,
        })
    }
}
