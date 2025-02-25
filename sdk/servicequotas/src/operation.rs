// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateServiceQuotaTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_service_quota_template`](crate::client::Client::associate_service_quota_template).
///
/// See [`crate::client::fluent_builders::AssociateServiceQuotaTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateServiceQuotaTemplate {
    _private: (),
}
impl AssociateServiceQuotaTemplate {
    /// Creates a new builder-style object to manufacture [`AssociateServiceQuotaTemplateInput`](crate::input::AssociateServiceQuotaTemplateInput).
    pub fn builder() -> crate::input::associate_service_quota_template_input::Builder {
        crate::input::associate_service_quota_template_input::Builder::default()
    }
    /// Creates a new `AssociateServiceQuotaTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateServiceQuotaTemplate {
    type Output = std::result::Result<
        crate::output::AssociateServiceQuotaTemplateOutput,
        crate::error::AssociateServiceQuotaTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_service_quota_template_error(response)
        } else {
            crate::operation_deser::parse_associate_service_quota_template_response(response)
        }
    }
}

/// Operation shape for `DeleteServiceQuotaIncreaseRequestFromTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_service_quota_increase_request_from_template`](crate::client::Client::delete_service_quota_increase_request_from_template).
///
/// See [`crate::client::fluent_builders::DeleteServiceQuotaIncreaseRequestFromTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplate {
    _private: (),
}
impl DeleteServiceQuotaIncreaseRequestFromTemplate {
    /// Creates a new builder-style object to manufacture [`DeleteServiceQuotaIncreaseRequestFromTemplateInput`](crate::input::DeleteServiceQuotaIncreaseRequestFromTemplateInput).
    pub fn builder(
    ) -> crate::input::delete_service_quota_increase_request_from_template_input::Builder {
        crate::input::delete_service_quota_increase_request_from_template_input::Builder::default()
    }
    /// Creates a new `DeleteServiceQuotaIncreaseRequestFromTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for DeleteServiceQuotaIncreaseRequestFromTemplate
{
    type Output = std::result::Result<
        crate::output::DeleteServiceQuotaIncreaseRequestFromTemplateOutput,
        crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_service_quota_increase_request_from_template_error(
                response,
            )
        } else {
            crate::operation_deser::parse_delete_service_quota_increase_request_from_template_response(response)
        }
    }
}

/// Operation shape for `DisassociateServiceQuotaTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_service_quota_template`](crate::client::Client::disassociate_service_quota_template).
///
/// See [`crate::client::fluent_builders::DisassociateServiceQuotaTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateServiceQuotaTemplate {
    _private: (),
}
impl DisassociateServiceQuotaTemplate {
    /// Creates a new builder-style object to manufacture [`DisassociateServiceQuotaTemplateInput`](crate::input::DisassociateServiceQuotaTemplateInput).
    pub fn builder() -> crate::input::disassociate_service_quota_template_input::Builder {
        crate::input::disassociate_service_quota_template_input::Builder::default()
    }
    /// Creates a new `DisassociateServiceQuotaTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateServiceQuotaTemplate {
    type Output = std::result::Result<
        crate::output::DisassociateServiceQuotaTemplateOutput,
        crate::error::DisassociateServiceQuotaTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_service_quota_template_error(response)
        } else {
            crate::operation_deser::parse_disassociate_service_quota_template_response(response)
        }
    }
}

/// Operation shape for `GetAssociationForServiceQuotaTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_association_for_service_quota_template`](crate::client::Client::get_association_for_service_quota_template).
///
/// See [`crate::client::fluent_builders::GetAssociationForServiceQuotaTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAssociationForServiceQuotaTemplate {
    _private: (),
}
impl GetAssociationForServiceQuotaTemplate {
    /// Creates a new builder-style object to manufacture [`GetAssociationForServiceQuotaTemplateInput`](crate::input::GetAssociationForServiceQuotaTemplateInput).
    pub fn builder() -> crate::input::get_association_for_service_quota_template_input::Builder {
        crate::input::get_association_for_service_quota_template_input::Builder::default()
    }
    /// Creates a new `GetAssociationForServiceQuotaTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAssociationForServiceQuotaTemplate {
    type Output = std::result::Result<
        crate::output::GetAssociationForServiceQuotaTemplateOutput,
        crate::error::GetAssociationForServiceQuotaTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_association_for_service_quota_template_error(response)
        } else {
            crate::operation_deser::parse_get_association_for_service_quota_template_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetAWSDefaultServiceQuota`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_aws_default_service_quota`](crate::client::Client::get_aws_default_service_quota).
///
/// See [`crate::client::fluent_builders::GetAWSDefaultServiceQuota`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAWSDefaultServiceQuota {
    _private: (),
}
impl GetAWSDefaultServiceQuota {
    /// Creates a new builder-style object to manufacture [`GetAwsDefaultServiceQuotaInput`](crate::input::GetAwsDefaultServiceQuotaInput).
    pub fn builder() -> crate::input::get_aws_default_service_quota_input::Builder {
        crate::input::get_aws_default_service_quota_input::Builder::default()
    }
    /// Creates a new `GetAWSDefaultServiceQuota` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAWSDefaultServiceQuota {
    type Output = std::result::Result<
        crate::output::GetAwsDefaultServiceQuotaOutput,
        crate::error::GetAWSDefaultServiceQuotaError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_aws_default_service_quota_error(response)
        } else {
            crate::operation_deser::parse_get_aws_default_service_quota_response(response)
        }
    }
}

/// Operation shape for `GetRequestedServiceQuotaChange`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_requested_service_quota_change`](crate::client::Client::get_requested_service_quota_change).
///
/// See [`crate::client::fluent_builders::GetRequestedServiceQuotaChange`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRequestedServiceQuotaChange {
    _private: (),
}
impl GetRequestedServiceQuotaChange {
    /// Creates a new builder-style object to manufacture [`GetRequestedServiceQuotaChangeInput`](crate::input::GetRequestedServiceQuotaChangeInput).
    pub fn builder() -> crate::input::get_requested_service_quota_change_input::Builder {
        crate::input::get_requested_service_quota_change_input::Builder::default()
    }
    /// Creates a new `GetRequestedServiceQuotaChange` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRequestedServiceQuotaChange {
    type Output = std::result::Result<
        crate::output::GetRequestedServiceQuotaChangeOutput,
        crate::error::GetRequestedServiceQuotaChangeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_requested_service_quota_change_error(response)
        } else {
            crate::operation_deser::parse_get_requested_service_quota_change_response(response)
        }
    }
}

/// Operation shape for `GetServiceQuota`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service_quota`](crate::client::Client::get_service_quota).
///
/// See [`crate::client::fluent_builders::GetServiceQuota`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServiceQuota {
    _private: (),
}
impl GetServiceQuota {
    /// Creates a new builder-style object to manufacture [`GetServiceQuotaInput`](crate::input::GetServiceQuotaInput).
    pub fn builder() -> crate::input::get_service_quota_input::Builder {
        crate::input::get_service_quota_input::Builder::default()
    }
    /// Creates a new `GetServiceQuota` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServiceQuota {
    type Output = std::result::Result<
        crate::output::GetServiceQuotaOutput,
        crate::error::GetServiceQuotaError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_quota_error(response)
        } else {
            crate::operation_deser::parse_get_service_quota_response(response)
        }
    }
}

/// Operation shape for `GetServiceQuotaIncreaseRequestFromTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service_quota_increase_request_from_template`](crate::client::Client::get_service_quota_increase_request_from_template).
///
/// See [`crate::client::fluent_builders::GetServiceQuotaIncreaseRequestFromTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServiceQuotaIncreaseRequestFromTemplate {
    _private: (),
}
impl GetServiceQuotaIncreaseRequestFromTemplate {
    /// Creates a new builder-style object to manufacture [`GetServiceQuotaIncreaseRequestFromTemplateInput`](crate::input::GetServiceQuotaIncreaseRequestFromTemplateInput).
    pub fn builder() -> crate::input::get_service_quota_increase_request_from_template_input::Builder
    {
        crate::input::get_service_quota_increase_request_from_template_input::Builder::default()
    }
    /// Creates a new `GetServiceQuotaIncreaseRequestFromTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServiceQuotaIncreaseRequestFromTemplate {
    type Output = std::result::Result<
        crate::output::GetServiceQuotaIncreaseRequestFromTemplateOutput,
        crate::error::GetServiceQuotaIncreaseRequestFromTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_quota_increase_request_from_template_error(
                response,
            )
        } else {
            crate::operation_deser::parse_get_service_quota_increase_request_from_template_response(
                response,
            )
        }
    }
}

/// Operation shape for `ListAWSDefaultServiceQuotas`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_aws_default_service_quotas`](crate::client::Client::list_aws_default_service_quotas).
///
/// See [`crate::client::fluent_builders::ListAWSDefaultServiceQuotas`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAWSDefaultServiceQuotas {
    _private: (),
}
impl ListAWSDefaultServiceQuotas {
    /// Creates a new builder-style object to manufacture [`ListAwsDefaultServiceQuotasInput`](crate::input::ListAwsDefaultServiceQuotasInput).
    pub fn builder() -> crate::input::list_aws_default_service_quotas_input::Builder {
        crate::input::list_aws_default_service_quotas_input::Builder::default()
    }
    /// Creates a new `ListAWSDefaultServiceQuotas` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAWSDefaultServiceQuotas {
    type Output = std::result::Result<
        crate::output::ListAwsDefaultServiceQuotasOutput,
        crate::error::ListAWSDefaultServiceQuotasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_aws_default_service_quotas_error(response)
        } else {
            crate::operation_deser::parse_list_aws_default_service_quotas_response(response)
        }
    }
}

/// Operation shape for `ListRequestedServiceQuotaChangeHistory`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_requested_service_quota_change_history`](crate::client::Client::list_requested_service_quota_change_history).
///
/// See [`crate::client::fluent_builders::ListRequestedServiceQuotaChangeHistory`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRequestedServiceQuotaChangeHistory {
    _private: (),
}
impl ListRequestedServiceQuotaChangeHistory {
    /// Creates a new builder-style object to manufacture [`ListRequestedServiceQuotaChangeHistoryInput`](crate::input::ListRequestedServiceQuotaChangeHistoryInput).
    pub fn builder() -> crate::input::list_requested_service_quota_change_history_input::Builder {
        crate::input::list_requested_service_quota_change_history_input::Builder::default()
    }
    /// Creates a new `ListRequestedServiceQuotaChangeHistory` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRequestedServiceQuotaChangeHistory {
    type Output = std::result::Result<
        crate::output::ListRequestedServiceQuotaChangeHistoryOutput,
        crate::error::ListRequestedServiceQuotaChangeHistoryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_requested_service_quota_change_history_error(
                response,
            )
        } else {
            crate::operation_deser::parse_list_requested_service_quota_change_history_response(
                response,
            )
        }
    }
}

/// Operation shape for `ListRequestedServiceQuotaChangeHistoryByQuota`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_requested_service_quota_change_history_by_quota`](crate::client::Client::list_requested_service_quota_change_history_by_quota).
///
/// See [`crate::client::fluent_builders::ListRequestedServiceQuotaChangeHistoryByQuota`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRequestedServiceQuotaChangeHistoryByQuota {
    _private: (),
}
impl ListRequestedServiceQuotaChangeHistoryByQuota {
    /// Creates a new builder-style object to manufacture [`ListRequestedServiceQuotaChangeHistoryByQuotaInput`](crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInput).
    pub fn builder(
    ) -> crate::input::list_requested_service_quota_change_history_by_quota_input::Builder {
        crate::input::list_requested_service_quota_change_history_by_quota_input::Builder::default()
    }
    /// Creates a new `ListRequestedServiceQuotaChangeHistoryByQuota` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for ListRequestedServiceQuotaChangeHistoryByQuota
{
    type Output = std::result::Result<
        crate::output::ListRequestedServiceQuotaChangeHistoryByQuotaOutput,
        crate::error::ListRequestedServiceQuotaChangeHistoryByQuotaError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_requested_service_quota_change_history_by_quota_error(
                response,
            )
        } else {
            crate::operation_deser::parse_list_requested_service_quota_change_history_by_quota_response(response)
        }
    }
}

/// Operation shape for `ListServiceQuotaIncreaseRequestsInTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_service_quota_increase_requests_in_template`](crate::client::Client::list_service_quota_increase_requests_in_template).
///
/// See [`crate::client::fluent_builders::ListServiceQuotaIncreaseRequestsInTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListServiceQuotaIncreaseRequestsInTemplate {
    _private: (),
}
impl ListServiceQuotaIncreaseRequestsInTemplate {
    /// Creates a new builder-style object to manufacture [`ListServiceQuotaIncreaseRequestsInTemplateInput`](crate::input::ListServiceQuotaIncreaseRequestsInTemplateInput).
    pub fn builder() -> crate::input::list_service_quota_increase_requests_in_template_input::Builder
    {
        crate::input::list_service_quota_increase_requests_in_template_input::Builder::default()
    }
    /// Creates a new `ListServiceQuotaIncreaseRequestsInTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServiceQuotaIncreaseRequestsInTemplate {
    type Output = std::result::Result<
        crate::output::ListServiceQuotaIncreaseRequestsInTemplateOutput,
        crate::error::ListServiceQuotaIncreaseRequestsInTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_service_quota_increase_requests_in_template_error(
                response,
            )
        } else {
            crate::operation_deser::parse_list_service_quota_increase_requests_in_template_response(
                response,
            )
        }
    }
}

/// Operation shape for `ListServiceQuotas`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_service_quotas`](crate::client::Client::list_service_quotas).
///
/// See [`crate::client::fluent_builders::ListServiceQuotas`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListServiceQuotas {
    _private: (),
}
impl ListServiceQuotas {
    /// Creates a new builder-style object to manufacture [`ListServiceQuotasInput`](crate::input::ListServiceQuotasInput).
    pub fn builder() -> crate::input::list_service_quotas_input::Builder {
        crate::input::list_service_quotas_input::Builder::default()
    }
    /// Creates a new `ListServiceQuotas` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServiceQuotas {
    type Output = std::result::Result<
        crate::output::ListServiceQuotasOutput,
        crate::error::ListServiceQuotasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_service_quotas_error(response)
        } else {
            crate::operation_deser::parse_list_service_quotas_response(response)
        }
    }
}

/// Operation shape for `ListServices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_services`](crate::client::Client::list_services).
///
/// See [`crate::client::fluent_builders::ListServices`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListServices {
    _private: (),
}
impl ListServices {
    /// Creates a new builder-style object to manufacture [`ListServicesInput`](crate::input::ListServicesInput).
    pub fn builder() -> crate::input::list_services_input::Builder {
        crate::input::list_services_input::Builder::default()
    }
    /// Creates a new `ListServices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServices {
    type Output =
        std::result::Result<crate::output::ListServicesOutput, crate::error::ListServicesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_services_error(response)
        } else {
            crate::operation_deser::parse_list_services_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `PutServiceQuotaIncreaseRequestIntoTemplate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_service_quota_increase_request_into_template`](crate::client::Client::put_service_quota_increase_request_into_template).
///
/// See [`crate::client::fluent_builders::PutServiceQuotaIncreaseRequestIntoTemplate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutServiceQuotaIncreaseRequestIntoTemplate {
    _private: (),
}
impl PutServiceQuotaIncreaseRequestIntoTemplate {
    /// Creates a new builder-style object to manufacture [`PutServiceQuotaIncreaseRequestIntoTemplateInput`](crate::input::PutServiceQuotaIncreaseRequestIntoTemplateInput).
    pub fn builder() -> crate::input::put_service_quota_increase_request_into_template_input::Builder
    {
        crate::input::put_service_quota_increase_request_into_template_input::Builder::default()
    }
    /// Creates a new `PutServiceQuotaIncreaseRequestIntoTemplate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutServiceQuotaIncreaseRequestIntoTemplate {
    type Output = std::result::Result<
        crate::output::PutServiceQuotaIncreaseRequestIntoTemplateOutput,
        crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_service_quota_increase_request_into_template_error(
                response,
            )
        } else {
            crate::operation_deser::parse_put_service_quota_increase_request_into_template_response(
                response,
            )
        }
    }
}

/// Operation shape for `RequestServiceQuotaIncrease`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`request_service_quota_increase`](crate::client::Client::request_service_quota_increase).
///
/// See [`crate::client::fluent_builders::RequestServiceQuotaIncrease`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RequestServiceQuotaIncrease {
    _private: (),
}
impl RequestServiceQuotaIncrease {
    /// Creates a new builder-style object to manufacture [`RequestServiceQuotaIncreaseInput`](crate::input::RequestServiceQuotaIncreaseInput).
    pub fn builder() -> crate::input::request_service_quota_increase_input::Builder {
        crate::input::request_service_quota_increase_input::Builder::default()
    }
    /// Creates a new `RequestServiceQuotaIncrease` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RequestServiceQuotaIncrease {
    type Output = std::result::Result<
        crate::output::RequestServiceQuotaIncreaseOutput,
        crate::error::RequestServiceQuotaIncreaseError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_request_service_quota_increase_error(response)
        } else {
            crate::operation_deser::parse_request_service_quota_increase_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
