// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The number of active statements exceeds the limit.</p>
    ActiveStatementsExceededException(crate::error::ActiveStatementsExceededException),
    /// <p>An SQL statement encountered an environmental error while running.</p>
    BatchExecuteStatementException(crate::error::BatchExecuteStatementException),
    /// <p>Connection to a database failed.</p>
    DatabaseConnectionException(crate::error::DatabaseConnectionException),
    /// <p>The SQL statement encountered an environmental error while running.</p>
    ExecuteStatementException(crate::error::ExecuteStatementException),
    /// <p>The Amazon Redshift Data API operation failed due to invalid input. </p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The Amazon Redshift Data API operation failed due to a missing resource. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The Amazon Redshift Data API operation failed due to invalid input. </p>
    ValidationException(crate::error::ValidationException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ActiveStatementsExceededException(inner) => inner.fmt(f),
            Error::BatchExecuteStatementException(inner) => inner.fmt(f),
            Error::DatabaseConnectionException(inner) => inner.fmt(f),
            Error::ExecuteStatementException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchExecuteStatementError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::BatchExecuteStatementError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchExecuteStatementError> for Error {
    fn from(err: crate::error::BatchExecuteStatementError) -> Self {
        match err.kind {
            crate::error::BatchExecuteStatementErrorKind::ActiveStatementsExceededException(
                inner,
            ) => Error::ActiveStatementsExceededException(inner),
            crate::error::BatchExecuteStatementErrorKind::BatchExecuteStatementException(inner) => {
                Error::BatchExecuteStatementException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::BatchExecuteStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelStatementError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelStatementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CancelStatementError> for Error {
    fn from(err: crate::error::CancelStatementError) -> Self {
        match err.kind {
            crate::error::CancelStatementErrorKind::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::error::CancelStatementErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CancelStatementErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::CancelStatementErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CancelStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeStatementError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeStatementError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeStatementError> for Error {
    fn from(err: crate::error::DescribeStatementError) -> Self {
        match err.kind {
            crate::error::DescribeStatementErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeStatementErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeStatementErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeTableError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeTableError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeTableError> for Error {
    fn from(err: crate::error::DescribeTableError) -> Self {
        match err.kind {
            crate::error::DescribeTableErrorKind::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::error::DescribeTableErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeTableErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeTableErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExecuteStatementError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ExecuteStatementError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExecuteStatementError> for Error {
    fn from(err: crate::error::ExecuteStatementError) -> Self {
        match err.kind {
            crate::error::ExecuteStatementErrorKind::ActiveStatementsExceededException(inner) => {
                Error::ActiveStatementsExceededException(inner)
            }
            crate::error::ExecuteStatementErrorKind::ExecuteStatementException(inner) => {
                Error::ExecuteStatementException(inner)
            }
            crate::error::ExecuteStatementErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ExecuteStatementErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetStatementResultError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetStatementResultError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetStatementResultError> for Error {
    fn from(err: crate::error::GetStatementResultError) -> Self {
        match err.kind {
            crate::error::GetStatementResultErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::GetStatementResultErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetStatementResultErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::GetStatementResultErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDatabasesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDatabasesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListDatabasesError> for Error {
    fn from(err: crate::error::ListDatabasesError) -> Self {
        match err.kind {
            crate::error::ListDatabasesErrorKind::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::error::ListDatabasesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListDatabasesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListDatabasesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSchemasError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSchemasError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSchemasError> for Error {
    fn from(err: crate::error::ListSchemasError) -> Self {
        match err.kind {
            crate::error::ListSchemasErrorKind::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::error::ListSchemasErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListSchemasErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListSchemasErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStatementsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListStatementsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListStatementsError> for Error {
    fn from(err: crate::error::ListStatementsError) -> Self {
        match err.kind {
            crate::error::ListStatementsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListStatementsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListStatementsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTablesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTablesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTablesError> for Error {
    fn from(err: crate::error::ListTablesError) -> Self {
        match err.kind {
            crate::error::ListTablesErrorKind::DatabaseConnectionException(inner) => {
                Error::DatabaseConnectionException(inner)
            }
            crate::error::ListTablesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListTablesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListTablesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
