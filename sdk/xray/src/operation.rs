// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchGetTraces`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_traces`](crate::client::Client::batch_get_traces).
///
/// See [`crate::client::fluent_builders::BatchGetTraces`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetTraces {
    _private: (),
}
impl BatchGetTraces {
    /// Creates a new builder-style object to manufacture [`BatchGetTracesInput`](crate::input::BatchGetTracesInput).
    pub fn builder() -> crate::input::batch_get_traces_input::Builder {
        crate::input::batch_get_traces_input::Builder::default()
    }
    /// Creates a new `BatchGetTraces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetTraces {
    type Output =
        std::result::Result<crate::output::BatchGetTracesOutput, crate::error::BatchGetTracesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_traces_error(response)
        } else {
            crate::operation_deser::parse_batch_get_traces_response(response)
        }
    }
}

/// Operation shape for `CreateGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_group`](crate::client::Client::create_group).
///
/// See [`crate::client::fluent_builders::CreateGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateGroup {
    _private: (),
}
impl CreateGroup {
    /// Creates a new builder-style object to manufacture [`CreateGroupInput`](crate::input::CreateGroupInput).
    pub fn builder() -> crate::input::create_group_input::Builder {
        crate::input::create_group_input::Builder::default()
    }
    /// Creates a new `CreateGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateGroup {
    type Output =
        std::result::Result<crate::output::CreateGroupOutput, crate::error::CreateGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_group_error(response)
        } else {
            crate::operation_deser::parse_create_group_response(response)
        }
    }
}

/// Operation shape for `CreateSamplingRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_sampling_rule`](crate::client::Client::create_sampling_rule).
///
/// See [`crate::client::fluent_builders::CreateSamplingRule`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSamplingRule {
    _private: (),
}
impl CreateSamplingRule {
    /// Creates a new builder-style object to manufacture [`CreateSamplingRuleInput`](crate::input::CreateSamplingRuleInput).
    pub fn builder() -> crate::input::create_sampling_rule_input::Builder {
        crate::input::create_sampling_rule_input::Builder::default()
    }
    /// Creates a new `CreateSamplingRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSamplingRule {
    type Output = std::result::Result<
        crate::output::CreateSamplingRuleOutput,
        crate::error::CreateSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_create_sampling_rule_response(response)
        }
    }
}

/// Operation shape for `DeleteGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_group`](crate::client::Client::delete_group).
///
/// See [`crate::client::fluent_builders::DeleteGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteGroup {
    _private: (),
}
impl DeleteGroup {
    /// Creates a new builder-style object to manufacture [`DeleteGroupInput`](crate::input::DeleteGroupInput).
    pub fn builder() -> crate::input::delete_group_input::Builder {
        crate::input::delete_group_input::Builder::default()
    }
    /// Creates a new `DeleteGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteGroup {
    type Output =
        std::result::Result<crate::output::DeleteGroupOutput, crate::error::DeleteGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_group_error(response)
        } else {
            crate::operation_deser::parse_delete_group_response(response)
        }
    }
}

/// Operation shape for `DeleteSamplingRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_sampling_rule`](crate::client::Client::delete_sampling_rule).
///
/// See [`crate::client::fluent_builders::DeleteSamplingRule`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSamplingRule {
    _private: (),
}
impl DeleteSamplingRule {
    /// Creates a new builder-style object to manufacture [`DeleteSamplingRuleInput`](crate::input::DeleteSamplingRuleInput).
    pub fn builder() -> crate::input::delete_sampling_rule_input::Builder {
        crate::input::delete_sampling_rule_input::Builder::default()
    }
    /// Creates a new `DeleteSamplingRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSamplingRule {
    type Output = std::result::Result<
        crate::output::DeleteSamplingRuleOutput,
        crate::error::DeleteSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_delete_sampling_rule_response(response)
        }
    }
}

/// Operation shape for `GetEncryptionConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_encryption_config`](crate::client::Client::get_encryption_config).
///
/// See [`crate::client::fluent_builders::GetEncryptionConfig`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEncryptionConfig {
    _private: (),
}
impl GetEncryptionConfig {
    /// Creates a new builder-style object to manufacture [`GetEncryptionConfigInput`](crate::input::GetEncryptionConfigInput).
    pub fn builder() -> crate::input::get_encryption_config_input::Builder {
        crate::input::get_encryption_config_input::Builder::default()
    }
    /// Creates a new `GetEncryptionConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEncryptionConfig {
    type Output = std::result::Result<
        crate::output::GetEncryptionConfigOutput,
        crate::error::GetEncryptionConfigError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_encryption_config_error(response)
        } else {
            crate::operation_deser::parse_get_encryption_config_response(response)
        }
    }
}

/// Operation shape for `GetGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_group`](crate::client::Client::get_group).
///
/// See [`crate::client::fluent_builders::GetGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGroup {
    _private: (),
}
impl GetGroup {
    /// Creates a new builder-style object to manufacture [`GetGroupInput`](crate::input::GetGroupInput).
    pub fn builder() -> crate::input::get_group_input::Builder {
        crate::input::get_group_input::Builder::default()
    }
    /// Creates a new `GetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroup {
    type Output = std::result::Result<crate::output::GetGroupOutput, crate::error::GetGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_group_error(response)
        } else {
            crate::operation_deser::parse_get_group_response(response)
        }
    }
}

/// Operation shape for `GetGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_groups`](crate::client::Client::get_groups).
///
/// See [`crate::client::fluent_builders::GetGroups`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGroups {
    _private: (),
}
impl GetGroups {
    /// Creates a new builder-style object to manufacture [`GetGroupsInput`](crate::input::GetGroupsInput).
    pub fn builder() -> crate::input::get_groups_input::Builder {
        crate::input::get_groups_input::Builder::default()
    }
    /// Creates a new `GetGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroups {
    type Output = std::result::Result<crate::output::GetGroupsOutput, crate::error::GetGroupsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_groups_error(response)
        } else {
            crate::operation_deser::parse_get_groups_response(response)
        }
    }
}

/// Operation shape for `GetInsight`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_insight`](crate::client::Client::get_insight).
///
/// See [`crate::client::fluent_builders::GetInsight`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetInsight {
    _private: (),
}
impl GetInsight {
    /// Creates a new builder-style object to manufacture [`GetInsightInput`](crate::input::GetInsightInput).
    pub fn builder() -> crate::input::get_insight_input::Builder {
        crate::input::get_insight_input::Builder::default()
    }
    /// Creates a new `GetInsight` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInsight {
    type Output =
        std::result::Result<crate::output::GetInsightOutput, crate::error::GetInsightError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_error(response)
        } else {
            crate::operation_deser::parse_get_insight_response(response)
        }
    }
}

/// Operation shape for `GetInsightEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_insight_events`](crate::client::Client::get_insight_events).
///
/// See [`crate::client::fluent_builders::GetInsightEvents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetInsightEvents {
    _private: (),
}
impl GetInsightEvents {
    /// Creates a new builder-style object to manufacture [`GetInsightEventsInput`](crate::input::GetInsightEventsInput).
    pub fn builder() -> crate::input::get_insight_events_input::Builder {
        crate::input::get_insight_events_input::Builder::default()
    }
    /// Creates a new `GetInsightEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInsightEvents {
    type Output = std::result::Result<
        crate::output::GetInsightEventsOutput,
        crate::error::GetInsightEventsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_events_error(response)
        } else {
            crate::operation_deser::parse_get_insight_events_response(response)
        }
    }
}

/// Operation shape for `GetInsightImpactGraph`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_insight_impact_graph`](crate::client::Client::get_insight_impact_graph).
///
/// See [`crate::client::fluent_builders::GetInsightImpactGraph`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetInsightImpactGraph {
    _private: (),
}
impl GetInsightImpactGraph {
    /// Creates a new builder-style object to manufacture [`GetInsightImpactGraphInput`](crate::input::GetInsightImpactGraphInput).
    pub fn builder() -> crate::input::get_insight_impact_graph_input::Builder {
        crate::input::get_insight_impact_graph_input::Builder::default()
    }
    /// Creates a new `GetInsightImpactGraph` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInsightImpactGraph {
    type Output = std::result::Result<
        crate::output::GetInsightImpactGraphOutput,
        crate::error::GetInsightImpactGraphError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_impact_graph_error(response)
        } else {
            crate::operation_deser::parse_get_insight_impact_graph_response(response)
        }
    }
}

/// Operation shape for `GetInsightSummaries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_insight_summaries`](crate::client::Client::get_insight_summaries).
///
/// See [`crate::client::fluent_builders::GetInsightSummaries`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetInsightSummaries {
    _private: (),
}
impl GetInsightSummaries {
    /// Creates a new builder-style object to manufacture [`GetInsightSummariesInput`](crate::input::GetInsightSummariesInput).
    pub fn builder() -> crate::input::get_insight_summaries_input::Builder {
        crate::input::get_insight_summaries_input::Builder::default()
    }
    /// Creates a new `GetInsightSummaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInsightSummaries {
    type Output = std::result::Result<
        crate::output::GetInsightSummariesOutput,
        crate::error::GetInsightSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_insight_summaries_response(response)
        }
    }
}

/// Operation shape for `GetSamplingRules`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_sampling_rules`](crate::client::Client::get_sampling_rules).
///
/// See [`crate::client::fluent_builders::GetSamplingRules`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSamplingRules {
    _private: (),
}
impl GetSamplingRules {
    /// Creates a new builder-style object to manufacture [`GetSamplingRulesInput`](crate::input::GetSamplingRulesInput).
    pub fn builder() -> crate::input::get_sampling_rules_input::Builder {
        crate::input::get_sampling_rules_input::Builder::default()
    }
    /// Creates a new `GetSamplingRules` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSamplingRules {
    type Output = std::result::Result<
        crate::output::GetSamplingRulesOutput,
        crate::error::GetSamplingRulesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_rules_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_rules_response(response)
        }
    }
}

/// Operation shape for `GetSamplingStatisticSummaries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_sampling_statistic_summaries`](crate::client::Client::get_sampling_statistic_summaries).
///
/// See [`crate::client::fluent_builders::GetSamplingStatisticSummaries`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSamplingStatisticSummaries {
    _private: (),
}
impl GetSamplingStatisticSummaries {
    /// Creates a new builder-style object to manufacture [`GetSamplingStatisticSummariesInput`](crate::input::GetSamplingStatisticSummariesInput).
    pub fn builder() -> crate::input::get_sampling_statistic_summaries_input::Builder {
        crate::input::get_sampling_statistic_summaries_input::Builder::default()
    }
    /// Creates a new `GetSamplingStatisticSummaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSamplingStatisticSummaries {
    type Output = std::result::Result<
        crate::output::GetSamplingStatisticSummariesOutput,
        crate::error::GetSamplingStatisticSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_statistic_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_statistic_summaries_response(response)
        }
    }
}

/// Operation shape for `GetSamplingTargets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_sampling_targets`](crate::client::Client::get_sampling_targets).
///
/// See [`crate::client::fluent_builders::GetSamplingTargets`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSamplingTargets {
    _private: (),
}
impl GetSamplingTargets {
    /// Creates a new builder-style object to manufacture [`GetSamplingTargetsInput`](crate::input::GetSamplingTargetsInput).
    pub fn builder() -> crate::input::get_sampling_targets_input::Builder {
        crate::input::get_sampling_targets_input::Builder::default()
    }
    /// Creates a new `GetSamplingTargets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSamplingTargets {
    type Output = std::result::Result<
        crate::output::GetSamplingTargetsOutput,
        crate::error::GetSamplingTargetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_sampling_targets_error(response)
        } else {
            crate::operation_deser::parse_get_sampling_targets_response(response)
        }
    }
}

/// Operation shape for `GetServiceGraph`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service_graph`](crate::client::Client::get_service_graph).
///
/// See [`crate::client::fluent_builders::GetServiceGraph`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServiceGraph {
    _private: (),
}
impl GetServiceGraph {
    /// Creates a new builder-style object to manufacture [`GetServiceGraphInput`](crate::input::GetServiceGraphInput).
    pub fn builder() -> crate::input::get_service_graph_input::Builder {
        crate::input::get_service_graph_input::Builder::default()
    }
    /// Creates a new `GetServiceGraph` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServiceGraph {
    type Output = std::result::Result<
        crate::output::GetServiceGraphOutput,
        crate::error::GetServiceGraphError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_graph_error(response)
        } else {
            crate::operation_deser::parse_get_service_graph_response(response)
        }
    }
}

/// Operation shape for `GetTimeSeriesServiceStatistics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_time_series_service_statistics`](crate::client::Client::get_time_series_service_statistics).
///
/// See [`crate::client::fluent_builders::GetTimeSeriesServiceStatistics`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTimeSeriesServiceStatistics {
    _private: (),
}
impl GetTimeSeriesServiceStatistics {
    /// Creates a new builder-style object to manufacture [`GetTimeSeriesServiceStatisticsInput`](crate::input::GetTimeSeriesServiceStatisticsInput).
    pub fn builder() -> crate::input::get_time_series_service_statistics_input::Builder {
        crate::input::get_time_series_service_statistics_input::Builder::default()
    }
    /// Creates a new `GetTimeSeriesServiceStatistics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTimeSeriesServiceStatistics {
    type Output = std::result::Result<
        crate::output::GetTimeSeriesServiceStatisticsOutput,
        crate::error::GetTimeSeriesServiceStatisticsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_time_series_service_statistics_error(response)
        } else {
            crate::operation_deser::parse_get_time_series_service_statistics_response(response)
        }
    }
}

/// Operation shape for `GetTraceGraph`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_trace_graph`](crate::client::Client::get_trace_graph).
///
/// See [`crate::client::fluent_builders::GetTraceGraph`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTraceGraph {
    _private: (),
}
impl GetTraceGraph {
    /// Creates a new builder-style object to manufacture [`GetTraceGraphInput`](crate::input::GetTraceGraphInput).
    pub fn builder() -> crate::input::get_trace_graph_input::Builder {
        crate::input::get_trace_graph_input::Builder::default()
    }
    /// Creates a new `GetTraceGraph` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTraceGraph {
    type Output =
        std::result::Result<crate::output::GetTraceGraphOutput, crate::error::GetTraceGraphError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trace_graph_error(response)
        } else {
            crate::operation_deser::parse_get_trace_graph_response(response)
        }
    }
}

/// Operation shape for `GetTraceSummaries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_trace_summaries`](crate::client::Client::get_trace_summaries).
///
/// See [`crate::client::fluent_builders::GetTraceSummaries`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTraceSummaries {
    _private: (),
}
impl GetTraceSummaries {
    /// Creates a new builder-style object to manufacture [`GetTraceSummariesInput`](crate::input::GetTraceSummariesInput).
    pub fn builder() -> crate::input::get_trace_summaries_input::Builder {
        crate::input::get_trace_summaries_input::Builder::default()
    }
    /// Creates a new `GetTraceSummaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTraceSummaries {
    type Output = std::result::Result<
        crate::output::GetTraceSummariesOutput,
        crate::error::GetTraceSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trace_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_trace_summaries_response(response)
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

/// Operation shape for `PutEncryptionConfig`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_encryption_config`](crate::client::Client::put_encryption_config).
///
/// See [`crate::client::fluent_builders::PutEncryptionConfig`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutEncryptionConfig {
    _private: (),
}
impl PutEncryptionConfig {
    /// Creates a new builder-style object to manufacture [`PutEncryptionConfigInput`](crate::input::PutEncryptionConfigInput).
    pub fn builder() -> crate::input::put_encryption_config_input::Builder {
        crate::input::put_encryption_config_input::Builder::default()
    }
    /// Creates a new `PutEncryptionConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutEncryptionConfig {
    type Output = std::result::Result<
        crate::output::PutEncryptionConfigOutput,
        crate::error::PutEncryptionConfigError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_encryption_config_error(response)
        } else {
            crate::operation_deser::parse_put_encryption_config_response(response)
        }
    }
}

/// Operation shape for `PutTelemetryRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_telemetry_records`](crate::client::Client::put_telemetry_records).
///
/// See [`crate::client::fluent_builders::PutTelemetryRecords`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutTelemetryRecords {
    _private: (),
}
impl PutTelemetryRecords {
    /// Creates a new builder-style object to manufacture [`PutTelemetryRecordsInput`](crate::input::PutTelemetryRecordsInput).
    pub fn builder() -> crate::input::put_telemetry_records_input::Builder {
        crate::input::put_telemetry_records_input::Builder::default()
    }
    /// Creates a new `PutTelemetryRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutTelemetryRecords {
    type Output = std::result::Result<
        crate::output::PutTelemetryRecordsOutput,
        crate::error::PutTelemetryRecordsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_telemetry_records_error(response)
        } else {
            crate::operation_deser::parse_put_telemetry_records_response(response)
        }
    }
}

/// Operation shape for `PutTraceSegments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_trace_segments`](crate::client::Client::put_trace_segments).
///
/// See [`crate::client::fluent_builders::PutTraceSegments`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutTraceSegments {
    _private: (),
}
impl PutTraceSegments {
    /// Creates a new builder-style object to manufacture [`PutTraceSegmentsInput`](crate::input::PutTraceSegmentsInput).
    pub fn builder() -> crate::input::put_trace_segments_input::Builder {
        crate::input::put_trace_segments_input::Builder::default()
    }
    /// Creates a new `PutTraceSegments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutTraceSegments {
    type Output = std::result::Result<
        crate::output::PutTraceSegmentsOutput,
        crate::error::PutTraceSegmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_trace_segments_error(response)
        } else {
            crate::operation_deser::parse_put_trace_segments_response(response)
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

/// Operation shape for `UpdateGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_group`](crate::client::Client::update_group).
///
/// See [`crate::client::fluent_builders::UpdateGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateGroup {
    _private: (),
}
impl UpdateGroup {
    /// Creates a new builder-style object to manufacture [`UpdateGroupInput`](crate::input::UpdateGroupInput).
    pub fn builder() -> crate::input::update_group_input::Builder {
        crate::input::update_group_input::Builder::default()
    }
    /// Creates a new `UpdateGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateGroup {
    type Output =
        std::result::Result<crate::output::UpdateGroupOutput, crate::error::UpdateGroupError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_group_error(response)
        } else {
            crate::operation_deser::parse_update_group_response(response)
        }
    }
}

/// Operation shape for `UpdateSamplingRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_sampling_rule`](crate::client::Client::update_sampling_rule).
///
/// See [`crate::client::fluent_builders::UpdateSamplingRule`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateSamplingRule {
    _private: (),
}
impl UpdateSamplingRule {
    /// Creates a new builder-style object to manufacture [`UpdateSamplingRuleInput`](crate::input::UpdateSamplingRuleInput).
    pub fn builder() -> crate::input::update_sampling_rule_input::Builder {
        crate::input::update_sampling_rule_input::Builder::default()
    }
    /// Creates a new `UpdateSamplingRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSamplingRule {
    type Output = std::result::Result<
        crate::output::UpdateSamplingRuleOutput,
        crate::error::UpdateSamplingRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_sampling_rule_error(response)
        } else {
            crate::operation_deser::parse_update_sampling_rule_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
