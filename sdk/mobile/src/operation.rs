// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_project`](crate::client::Client::create_project).
///
/// See [`crate::client::fluent_builders::CreateProject`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateProject {
    _private: (),
}
impl CreateProject {
    /// Creates a new builder-style object to manufacture [`CreateProjectInput`](crate::input::CreateProjectInput).
    pub fn builder() -> crate::input::create_project_input::Builder {
        crate::input::create_project_input::Builder::default()
    }
    /// Creates a new `CreateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateProject {
    type Output =
        std::result::Result<crate::output::CreateProjectOutput, crate::error::CreateProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_project_error(response)
        } else {
            crate::operation_deser::parse_create_project_response(response)
        }
    }
}

/// Operation shape for `DeleteProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_project`](crate::client::Client::delete_project).
///
/// See [`crate::client::fluent_builders::DeleteProject`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteProject {
    _private: (),
}
impl DeleteProject {
    /// Creates a new builder-style object to manufacture [`DeleteProjectInput`](crate::input::DeleteProjectInput).
    pub fn builder() -> crate::input::delete_project_input::Builder {
        crate::input::delete_project_input::Builder::default()
    }
    /// Creates a new `DeleteProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteProject {
    type Output =
        std::result::Result<crate::output::DeleteProjectOutput, crate::error::DeleteProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_project_error(response)
        } else {
            crate::operation_deser::parse_delete_project_response(response)
        }
    }
}

/// Operation shape for `DescribeBundle`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_bundle`](crate::client::Client::describe_bundle).
///
/// See [`crate::client::fluent_builders::DescribeBundle`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBundle {
    _private: (),
}
impl DescribeBundle {
    /// Creates a new builder-style object to manufacture [`DescribeBundleInput`](crate::input::DescribeBundleInput).
    pub fn builder() -> crate::input::describe_bundle_input::Builder {
        crate::input::describe_bundle_input::Builder::default()
    }
    /// Creates a new `DescribeBundle` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBundle {
    type Output =
        std::result::Result<crate::output::DescribeBundleOutput, crate::error::DescribeBundleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_bundle_error(response)
        } else {
            crate::operation_deser::parse_describe_bundle_response(response)
        }
    }
}

/// Operation shape for `DescribeProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_project`](crate::client::Client::describe_project).
///
/// See [`crate::client::fluent_builders::DescribeProject`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeProject {
    _private: (),
}
impl DescribeProject {
    /// Creates a new builder-style object to manufacture [`DescribeProjectInput`](crate::input::DescribeProjectInput).
    pub fn builder() -> crate::input::describe_project_input::Builder {
        crate::input::describe_project_input::Builder::default()
    }
    /// Creates a new `DescribeProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeProject {
    type Output = std::result::Result<
        crate::output::DescribeProjectOutput,
        crate::error::DescribeProjectError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_project_error(response)
        } else {
            crate::operation_deser::parse_describe_project_response(response)
        }
    }
}

/// Operation shape for `ExportBundle`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_bundle`](crate::client::Client::export_bundle).
///
/// See [`crate::client::fluent_builders::ExportBundle`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportBundle {
    _private: (),
}
impl ExportBundle {
    /// Creates a new builder-style object to manufacture [`ExportBundleInput`](crate::input::ExportBundleInput).
    pub fn builder() -> crate::input::export_bundle_input::Builder {
        crate::input::export_bundle_input::Builder::default()
    }
    /// Creates a new `ExportBundle` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportBundle {
    type Output =
        std::result::Result<crate::output::ExportBundleOutput, crate::error::ExportBundleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_bundle_error(response)
        } else {
            crate::operation_deser::parse_export_bundle_response(response)
        }
    }
}

/// Operation shape for `ExportProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_project`](crate::client::Client::export_project).
///
/// See [`crate::client::fluent_builders::ExportProject`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportProject {
    _private: (),
}
impl ExportProject {
    /// Creates a new builder-style object to manufacture [`ExportProjectInput`](crate::input::ExportProjectInput).
    pub fn builder() -> crate::input::export_project_input::Builder {
        crate::input::export_project_input::Builder::default()
    }
    /// Creates a new `ExportProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportProject {
    type Output =
        std::result::Result<crate::output::ExportProjectOutput, crate::error::ExportProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_project_error(response)
        } else {
            crate::operation_deser::parse_export_project_response(response)
        }
    }
}

/// Operation shape for `ListBundles`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_bundles`](crate::client::Client::list_bundles).
///
/// See [`crate::client::fluent_builders::ListBundles`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListBundles {
    _private: (),
}
impl ListBundles {
    /// Creates a new builder-style object to manufacture [`ListBundlesInput`](crate::input::ListBundlesInput).
    pub fn builder() -> crate::input::list_bundles_input::Builder {
        crate::input::list_bundles_input::Builder::default()
    }
    /// Creates a new `ListBundles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListBundles {
    type Output =
        std::result::Result<crate::output::ListBundlesOutput, crate::error::ListBundlesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_bundles_error(response)
        } else {
            crate::operation_deser::parse_list_bundles_response(response)
        }
    }
}

/// Operation shape for `ListProjects`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_projects`](crate::client::Client::list_projects).
///
/// See [`crate::client::fluent_builders::ListProjects`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListProjects {
    _private: (),
}
impl ListProjects {
    /// Creates a new builder-style object to manufacture [`ListProjectsInput`](crate::input::ListProjectsInput).
    pub fn builder() -> crate::input::list_projects_input::Builder {
        crate::input::list_projects_input::Builder::default()
    }
    /// Creates a new `ListProjects` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProjects {
    type Output =
        std::result::Result<crate::output::ListProjectsOutput, crate::error::ListProjectsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_projects_error(response)
        } else {
            crate::operation_deser::parse_list_projects_response(response)
        }
    }
}

/// Operation shape for `UpdateProject`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_project`](crate::client::Client::update_project).
///
/// See [`crate::client::fluent_builders::UpdateProject`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateProject {
    _private: (),
}
impl UpdateProject {
    /// Creates a new builder-style object to manufacture [`UpdateProjectInput`](crate::input::UpdateProjectInput).
    pub fn builder() -> crate::input::update_project_input::Builder {
        crate::input::update_project_input::Builder::default()
    }
    /// Creates a new `UpdateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateProject {
    type Output =
        std::result::Result<crate::output::UpdateProjectOutput, crate::error::UpdateProjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_project_error(response)
        } else {
            crate::operation_deser::parse_update_project_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
