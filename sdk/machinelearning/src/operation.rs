// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_tags`](crate::client::Client::add_tags).
///
/// See [`crate::client::fluent_builders::AddTags`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AddTags {
    _private: (),
}
impl AddTags {
    /// Creates a new builder-style object to manufacture [`AddTagsInput`](crate::input::AddTagsInput).
    pub fn builder() -> crate::input::add_tags_input::Builder {
        crate::input::add_tags_input::Builder::default()
    }
    /// Creates a new `AddTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddTags {
    type Output = std::result::Result<crate::output::AddTagsOutput, crate::error::AddTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_tags_error(response)
        } else {
            crate::operation_deser::parse_add_tags_response(response)
        }
    }
}

/// Operation shape for `CreateBatchPrediction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_batch_prediction`](crate::client::Client::create_batch_prediction).
///
/// See [`crate::client::fluent_builders::CreateBatchPrediction`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateBatchPrediction {
    _private: (),
}
impl CreateBatchPrediction {
    /// Creates a new builder-style object to manufacture [`CreateBatchPredictionInput`](crate::input::CreateBatchPredictionInput).
    pub fn builder() -> crate::input::create_batch_prediction_input::Builder {
        crate::input::create_batch_prediction_input::Builder::default()
    }
    /// Creates a new `CreateBatchPrediction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateBatchPrediction {
    type Output = std::result::Result<
        crate::output::CreateBatchPredictionOutput,
        crate::error::CreateBatchPredictionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_batch_prediction_error(response)
        } else {
            crate::operation_deser::parse_create_batch_prediction_response(response)
        }
    }
}

/// Operation shape for `CreateDataSourceFromRDS`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_data_source_from_rds`](crate::client::Client::create_data_source_from_rds).
///
/// See [`crate::client::fluent_builders::CreateDataSourceFromRDS`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDataSourceFromRDS {
    _private: (),
}
impl CreateDataSourceFromRDS {
    /// Creates a new builder-style object to manufacture [`CreateDataSourceFromRdsInput`](crate::input::CreateDataSourceFromRdsInput).
    pub fn builder() -> crate::input::create_data_source_from_rds_input::Builder {
        crate::input::create_data_source_from_rds_input::Builder::default()
    }
    /// Creates a new `CreateDataSourceFromRDS` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataSourceFromRDS {
    type Output = std::result::Result<
        crate::output::CreateDataSourceFromRdsOutput,
        crate::error::CreateDataSourceFromRDSError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_data_source_from_rds_error(response)
        } else {
            crate::operation_deser::parse_create_data_source_from_rds_response(response)
        }
    }
}

/// Operation shape for `CreateDataSourceFromRedshift`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_data_source_from_redshift`](crate::client::Client::create_data_source_from_redshift).
///
/// See [`crate::client::fluent_builders::CreateDataSourceFromRedshift`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDataSourceFromRedshift {
    _private: (),
}
impl CreateDataSourceFromRedshift {
    /// Creates a new builder-style object to manufacture [`CreateDataSourceFromRedshiftInput`](crate::input::CreateDataSourceFromRedshiftInput).
    pub fn builder() -> crate::input::create_data_source_from_redshift_input::Builder {
        crate::input::create_data_source_from_redshift_input::Builder::default()
    }
    /// Creates a new `CreateDataSourceFromRedshift` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataSourceFromRedshift {
    type Output = std::result::Result<
        crate::output::CreateDataSourceFromRedshiftOutput,
        crate::error::CreateDataSourceFromRedshiftError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_data_source_from_redshift_error(response)
        } else {
            crate::operation_deser::parse_create_data_source_from_redshift_response(response)
        }
    }
}

/// Operation shape for `CreateDataSourceFromS3`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_data_source_from_s3`](crate::client::Client::create_data_source_from_s3).
///
/// See [`crate::client::fluent_builders::CreateDataSourceFromS3`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateDataSourceFromS3 {
    _private: (),
}
impl CreateDataSourceFromS3 {
    /// Creates a new builder-style object to manufacture [`CreateDataSourceFromS3Input`](crate::input::CreateDataSourceFromS3Input).
    pub fn builder() -> crate::input::create_data_source_from_s3_input::Builder {
        crate::input::create_data_source_from_s3_input::Builder::default()
    }
    /// Creates a new `CreateDataSourceFromS3` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataSourceFromS3 {
    type Output = std::result::Result<
        crate::output::CreateDataSourceFromS3Output,
        crate::error::CreateDataSourceFromS3Error,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_data_source_from_s3_error(response)
        } else {
            crate::operation_deser::parse_create_data_source_from_s3_response(response)
        }
    }
}

/// Operation shape for `CreateEvaluation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_evaluation`](crate::client::Client::create_evaluation).
///
/// See [`crate::client::fluent_builders::CreateEvaluation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateEvaluation {
    _private: (),
}
impl CreateEvaluation {
    /// Creates a new builder-style object to manufacture [`CreateEvaluationInput`](crate::input::CreateEvaluationInput).
    pub fn builder() -> crate::input::create_evaluation_input::Builder {
        crate::input::create_evaluation_input::Builder::default()
    }
    /// Creates a new `CreateEvaluation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEvaluation {
    type Output = std::result::Result<
        crate::output::CreateEvaluationOutput,
        crate::error::CreateEvaluationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_evaluation_error(response)
        } else {
            crate::operation_deser::parse_create_evaluation_response(response)
        }
    }
}

/// Operation shape for `CreateMLModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_ml_model`](crate::client::Client::create_ml_model).
///
/// See [`crate::client::fluent_builders::CreateMLModel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateMLModel {
    _private: (),
}
impl CreateMLModel {
    /// Creates a new builder-style object to manufacture [`CreateMlModelInput`](crate::input::CreateMlModelInput).
    pub fn builder() -> crate::input::create_ml_model_input::Builder {
        crate::input::create_ml_model_input::Builder::default()
    }
    /// Creates a new `CreateMLModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateMLModel {
    type Output =
        std::result::Result<crate::output::CreateMlModelOutput, crate::error::CreateMLModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_ml_model_error(response)
        } else {
            crate::operation_deser::parse_create_ml_model_response(response)
        }
    }
}

/// Operation shape for `CreateRealtimeEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_realtime_endpoint`](crate::client::Client::create_realtime_endpoint).
///
/// See [`crate::client::fluent_builders::CreateRealtimeEndpoint`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRealtimeEndpoint {
    _private: (),
}
impl CreateRealtimeEndpoint {
    /// Creates a new builder-style object to manufacture [`CreateRealtimeEndpointInput`](crate::input::CreateRealtimeEndpointInput).
    pub fn builder() -> crate::input::create_realtime_endpoint_input::Builder {
        crate::input::create_realtime_endpoint_input::Builder::default()
    }
    /// Creates a new `CreateRealtimeEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRealtimeEndpoint {
    type Output = std::result::Result<
        crate::output::CreateRealtimeEndpointOutput,
        crate::error::CreateRealtimeEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_realtime_endpoint_error(response)
        } else {
            crate::operation_deser::parse_create_realtime_endpoint_response(response)
        }
    }
}

/// Operation shape for `DeleteBatchPrediction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_batch_prediction`](crate::client::Client::delete_batch_prediction).
///
/// See [`crate::client::fluent_builders::DeleteBatchPrediction`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteBatchPrediction {
    _private: (),
}
impl DeleteBatchPrediction {
    /// Creates a new builder-style object to manufacture [`DeleteBatchPredictionInput`](crate::input::DeleteBatchPredictionInput).
    pub fn builder() -> crate::input::delete_batch_prediction_input::Builder {
        crate::input::delete_batch_prediction_input::Builder::default()
    }
    /// Creates a new `DeleteBatchPrediction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteBatchPrediction {
    type Output = std::result::Result<
        crate::output::DeleteBatchPredictionOutput,
        crate::error::DeleteBatchPredictionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_batch_prediction_error(response)
        } else {
            crate::operation_deser::parse_delete_batch_prediction_response(response)
        }
    }
}

/// Operation shape for `DeleteDataSource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_data_source`](crate::client::Client::delete_data_source).
///
/// See [`crate::client::fluent_builders::DeleteDataSource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteDataSource {
    _private: (),
}
impl DeleteDataSource {
    /// Creates a new builder-style object to manufacture [`DeleteDataSourceInput`](crate::input::DeleteDataSourceInput).
    pub fn builder() -> crate::input::delete_data_source_input::Builder {
        crate::input::delete_data_source_input::Builder::default()
    }
    /// Creates a new `DeleteDataSource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDataSource {
    type Output = std::result::Result<
        crate::output::DeleteDataSourceOutput,
        crate::error::DeleteDataSourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_data_source_error(response)
        } else {
            crate::operation_deser::parse_delete_data_source_response(response)
        }
    }
}

/// Operation shape for `DeleteEvaluation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_evaluation`](crate::client::Client::delete_evaluation).
///
/// See [`crate::client::fluent_builders::DeleteEvaluation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteEvaluation {
    _private: (),
}
impl DeleteEvaluation {
    /// Creates a new builder-style object to manufacture [`DeleteEvaluationInput`](crate::input::DeleteEvaluationInput).
    pub fn builder() -> crate::input::delete_evaluation_input::Builder {
        crate::input::delete_evaluation_input::Builder::default()
    }
    /// Creates a new `DeleteEvaluation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEvaluation {
    type Output = std::result::Result<
        crate::output::DeleteEvaluationOutput,
        crate::error::DeleteEvaluationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_evaluation_error(response)
        } else {
            crate::operation_deser::parse_delete_evaluation_response(response)
        }
    }
}

/// Operation shape for `DeleteMLModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_ml_model`](crate::client::Client::delete_ml_model).
///
/// See [`crate::client::fluent_builders::DeleteMLModel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteMLModel {
    _private: (),
}
impl DeleteMLModel {
    /// Creates a new builder-style object to manufacture [`DeleteMlModelInput`](crate::input::DeleteMlModelInput).
    pub fn builder() -> crate::input::delete_ml_model_input::Builder {
        crate::input::delete_ml_model_input::Builder::default()
    }
    /// Creates a new `DeleteMLModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMLModel {
    type Output =
        std::result::Result<crate::output::DeleteMlModelOutput, crate::error::DeleteMLModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_ml_model_error(response)
        } else {
            crate::operation_deser::parse_delete_ml_model_response(response)
        }
    }
}

/// Operation shape for `DeleteRealtimeEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_realtime_endpoint`](crate::client::Client::delete_realtime_endpoint).
///
/// See [`crate::client::fluent_builders::DeleteRealtimeEndpoint`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRealtimeEndpoint {
    _private: (),
}
impl DeleteRealtimeEndpoint {
    /// Creates a new builder-style object to manufacture [`DeleteRealtimeEndpointInput`](crate::input::DeleteRealtimeEndpointInput).
    pub fn builder() -> crate::input::delete_realtime_endpoint_input::Builder {
        crate::input::delete_realtime_endpoint_input::Builder::default()
    }
    /// Creates a new `DeleteRealtimeEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRealtimeEndpoint {
    type Output = std::result::Result<
        crate::output::DeleteRealtimeEndpointOutput,
        crate::error::DeleteRealtimeEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_realtime_endpoint_error(response)
        } else {
            crate::operation_deser::parse_delete_realtime_endpoint_response(response)
        }
    }
}

/// Operation shape for `DeleteTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_tags`](crate::client::Client::delete_tags).
///
/// See [`crate::client::fluent_builders::DeleteTags`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteTags {
    _private: (),
}
impl DeleteTags {
    /// Creates a new builder-style object to manufacture [`DeleteTagsInput`](crate::input::DeleteTagsInput).
    pub fn builder() -> crate::input::delete_tags_input::Builder {
        crate::input::delete_tags_input::Builder::default()
    }
    /// Creates a new `DeleteTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTags {
    type Output =
        std::result::Result<crate::output::DeleteTagsOutput, crate::error::DeleteTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_tags_error(response)
        } else {
            crate::operation_deser::parse_delete_tags_response(response)
        }
    }
}

/// Operation shape for `DescribeBatchPredictions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_batch_predictions`](crate::client::Client::describe_batch_predictions).
///
/// See [`crate::client::fluent_builders::DescribeBatchPredictions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBatchPredictions {
    _private: (),
}
impl DescribeBatchPredictions {
    /// Creates a new builder-style object to manufacture [`DescribeBatchPredictionsInput`](crate::input::DescribeBatchPredictionsInput).
    pub fn builder() -> crate::input::describe_batch_predictions_input::Builder {
        crate::input::describe_batch_predictions_input::Builder::default()
    }
    /// Creates a new `DescribeBatchPredictions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBatchPredictions {
    type Output = std::result::Result<
        crate::output::DescribeBatchPredictionsOutput,
        crate::error::DescribeBatchPredictionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_batch_predictions_error(response)
        } else {
            crate::operation_deser::parse_describe_batch_predictions_response(response)
        }
    }
}

/// Operation shape for `DescribeDataSources`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_data_sources`](crate::client::Client::describe_data_sources).
///
/// See [`crate::client::fluent_builders::DescribeDataSources`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeDataSources {
    _private: (),
}
impl DescribeDataSources {
    /// Creates a new builder-style object to manufacture [`DescribeDataSourcesInput`](crate::input::DescribeDataSourcesInput).
    pub fn builder() -> crate::input::describe_data_sources_input::Builder {
        crate::input::describe_data_sources_input::Builder::default()
    }
    /// Creates a new `DescribeDataSources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDataSources {
    type Output = std::result::Result<
        crate::output::DescribeDataSourcesOutput,
        crate::error::DescribeDataSourcesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_data_sources_error(response)
        } else {
            crate::operation_deser::parse_describe_data_sources_response(response)
        }
    }
}

/// Operation shape for `DescribeEvaluations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_evaluations`](crate::client::Client::describe_evaluations).
///
/// See [`crate::client::fluent_builders::DescribeEvaluations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEvaluations {
    _private: (),
}
impl DescribeEvaluations {
    /// Creates a new builder-style object to manufacture [`DescribeEvaluationsInput`](crate::input::DescribeEvaluationsInput).
    pub fn builder() -> crate::input::describe_evaluations_input::Builder {
        crate::input::describe_evaluations_input::Builder::default()
    }
    /// Creates a new `DescribeEvaluations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEvaluations {
    type Output = std::result::Result<
        crate::output::DescribeEvaluationsOutput,
        crate::error::DescribeEvaluationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_evaluations_error(response)
        } else {
            crate::operation_deser::parse_describe_evaluations_response(response)
        }
    }
}

/// Operation shape for `DescribeMLModels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_ml_models`](crate::client::Client::describe_ml_models).
///
/// See [`crate::client::fluent_builders::DescribeMLModels`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeMLModels {
    _private: (),
}
impl DescribeMLModels {
    /// Creates a new builder-style object to manufacture [`DescribeMlModelsInput`](crate::input::DescribeMlModelsInput).
    pub fn builder() -> crate::input::describe_ml_models_input::Builder {
        crate::input::describe_ml_models_input::Builder::default()
    }
    /// Creates a new `DescribeMLModels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeMLModels {
    type Output = std::result::Result<
        crate::output::DescribeMlModelsOutput,
        crate::error::DescribeMLModelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_ml_models_error(response)
        } else {
            crate::operation_deser::parse_describe_ml_models_response(response)
        }
    }
}

/// Operation shape for `DescribeTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_tags`](crate::client::Client::describe_tags).
///
/// See [`crate::client::fluent_builders::DescribeTags`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTags {
    _private: (),
}
impl DescribeTags {
    /// Creates a new builder-style object to manufacture [`DescribeTagsInput`](crate::input::DescribeTagsInput).
    pub fn builder() -> crate::input::describe_tags_input::Builder {
        crate::input::describe_tags_input::Builder::default()
    }
    /// Creates a new `DescribeTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTags {
    type Output =
        std::result::Result<crate::output::DescribeTagsOutput, crate::error::DescribeTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_tags_error(response)
        } else {
            crate::operation_deser::parse_describe_tags_response(response)
        }
    }
}

/// Operation shape for `GetBatchPrediction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_batch_prediction`](crate::client::Client::get_batch_prediction).
///
/// See [`crate::client::fluent_builders::GetBatchPrediction`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetBatchPrediction {
    _private: (),
}
impl GetBatchPrediction {
    /// Creates a new builder-style object to manufacture [`GetBatchPredictionInput`](crate::input::GetBatchPredictionInput).
    pub fn builder() -> crate::input::get_batch_prediction_input::Builder {
        crate::input::get_batch_prediction_input::Builder::default()
    }
    /// Creates a new `GetBatchPrediction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetBatchPrediction {
    type Output = std::result::Result<
        crate::output::GetBatchPredictionOutput,
        crate::error::GetBatchPredictionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_batch_prediction_error(response)
        } else {
            crate::operation_deser::parse_get_batch_prediction_response(response)
        }
    }
}

/// Operation shape for `GetDataSource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_data_source`](crate::client::Client::get_data_source).
///
/// See [`crate::client::fluent_builders::GetDataSource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDataSource {
    _private: (),
}
impl GetDataSource {
    /// Creates a new builder-style object to manufacture [`GetDataSourceInput`](crate::input::GetDataSourceInput).
    pub fn builder() -> crate::input::get_data_source_input::Builder {
        crate::input::get_data_source_input::Builder::default()
    }
    /// Creates a new `GetDataSource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDataSource {
    type Output =
        std::result::Result<crate::output::GetDataSourceOutput, crate::error::GetDataSourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_data_source_error(response)
        } else {
            crate::operation_deser::parse_get_data_source_response(response)
        }
    }
}

/// Operation shape for `GetEvaluation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_evaluation`](crate::client::Client::get_evaluation).
///
/// See [`crate::client::fluent_builders::GetEvaluation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEvaluation {
    _private: (),
}
impl GetEvaluation {
    /// Creates a new builder-style object to manufacture [`GetEvaluationInput`](crate::input::GetEvaluationInput).
    pub fn builder() -> crate::input::get_evaluation_input::Builder {
        crate::input::get_evaluation_input::Builder::default()
    }
    /// Creates a new `GetEvaluation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEvaluation {
    type Output =
        std::result::Result<crate::output::GetEvaluationOutput, crate::error::GetEvaluationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_evaluation_error(response)
        } else {
            crate::operation_deser::parse_get_evaluation_response(response)
        }
    }
}

/// Operation shape for `GetMLModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ml_model`](crate::client::Client::get_ml_model).
///
/// See [`crate::client::fluent_builders::GetMLModel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetMLModel {
    _private: (),
}
impl GetMLModel {
    /// Creates a new builder-style object to manufacture [`GetMlModelInput`](crate::input::GetMlModelInput).
    pub fn builder() -> crate::input::get_ml_model_input::Builder {
        crate::input::get_ml_model_input::Builder::default()
    }
    /// Creates a new `GetMLModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetMLModel {
    type Output =
        std::result::Result<crate::output::GetMlModelOutput, crate::error::GetMLModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ml_model_error(response)
        } else {
            crate::operation_deser::parse_get_ml_model_response(response)
        }
    }
}

/// Operation shape for `Predict`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`predict`](crate::client::Client::predict).
///
/// See [`crate::client::fluent_builders::Predict`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Predict {
    _private: (),
}
impl Predict {
    /// Creates a new builder-style object to manufacture [`PredictInput`](crate::input::PredictInput).
    pub fn builder() -> crate::input::predict_input::Builder {
        crate::input::predict_input::Builder::default()
    }
    /// Creates a new `Predict` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Predict {
    type Output = std::result::Result<crate::output::PredictOutput, crate::error::PredictError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_predict_error(response)
        } else {
            crate::operation_deser::parse_predict_response(response)
        }
    }
}

/// Operation shape for `UpdateBatchPrediction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_batch_prediction`](crate::client::Client::update_batch_prediction).
///
/// See [`crate::client::fluent_builders::UpdateBatchPrediction`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateBatchPrediction {
    _private: (),
}
impl UpdateBatchPrediction {
    /// Creates a new builder-style object to manufacture [`UpdateBatchPredictionInput`](crate::input::UpdateBatchPredictionInput).
    pub fn builder() -> crate::input::update_batch_prediction_input::Builder {
        crate::input::update_batch_prediction_input::Builder::default()
    }
    /// Creates a new `UpdateBatchPrediction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateBatchPrediction {
    type Output = std::result::Result<
        crate::output::UpdateBatchPredictionOutput,
        crate::error::UpdateBatchPredictionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_batch_prediction_error(response)
        } else {
            crate::operation_deser::parse_update_batch_prediction_response(response)
        }
    }
}

/// Operation shape for `UpdateDataSource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_data_source`](crate::client::Client::update_data_source).
///
/// See [`crate::client::fluent_builders::UpdateDataSource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateDataSource {
    _private: (),
}
impl UpdateDataSource {
    /// Creates a new builder-style object to manufacture [`UpdateDataSourceInput`](crate::input::UpdateDataSourceInput).
    pub fn builder() -> crate::input::update_data_source_input::Builder {
        crate::input::update_data_source_input::Builder::default()
    }
    /// Creates a new `UpdateDataSource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDataSource {
    type Output = std::result::Result<
        crate::output::UpdateDataSourceOutput,
        crate::error::UpdateDataSourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_data_source_error(response)
        } else {
            crate::operation_deser::parse_update_data_source_response(response)
        }
    }
}

/// Operation shape for `UpdateEvaluation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_evaluation`](crate::client::Client::update_evaluation).
///
/// See [`crate::client::fluent_builders::UpdateEvaluation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateEvaluation {
    _private: (),
}
impl UpdateEvaluation {
    /// Creates a new builder-style object to manufacture [`UpdateEvaluationInput`](crate::input::UpdateEvaluationInput).
    pub fn builder() -> crate::input::update_evaluation_input::Builder {
        crate::input::update_evaluation_input::Builder::default()
    }
    /// Creates a new `UpdateEvaluation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEvaluation {
    type Output = std::result::Result<
        crate::output::UpdateEvaluationOutput,
        crate::error::UpdateEvaluationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_evaluation_error(response)
        } else {
            crate::operation_deser::parse_update_evaluation_response(response)
        }
    }
}

/// Operation shape for `UpdateMLModel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_ml_model`](crate::client::Client::update_ml_model).
///
/// See [`crate::client::fluent_builders::UpdateMLModel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateMLModel {
    _private: (),
}
impl UpdateMLModel {
    /// Creates a new builder-style object to manufacture [`UpdateMlModelInput`](crate::input::UpdateMlModelInput).
    pub fn builder() -> crate::input::update_ml_model_input::Builder {
        crate::input::update_ml_model_input::Builder::default()
    }
    /// Creates a new `UpdateMLModel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateMLModel {
    type Output =
        std::result::Result<crate::output::UpdateMlModelOutput, crate::error::UpdateMLModelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_ml_model_error(response)
        } else {
            crate::operation_deser::parse_update_ml_model_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
