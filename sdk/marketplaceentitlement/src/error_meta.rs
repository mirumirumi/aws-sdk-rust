// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceErrorException(crate::error::InternalServiceErrorException),
    /// <p>One or more parameters in your request was invalid.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>The calls to the GetEntitlements API are throttled.</p>
    ThrottlingException(crate::error::ThrottlingException),
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
            Error::InternalServiceErrorException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEntitlementsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEntitlementsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetEntitlementsError> for Error {
    fn from(err: crate::error::GetEntitlementsError) -> Self {
        match err.kind {
            crate::error::GetEntitlementsErrorKind::InternalServiceErrorException(inner) => {
                Error::InternalServiceErrorException(inner)
            }
            crate::error::GetEntitlementsErrorKind::InvalidParameterException(inner) => {
                Error::InvalidParameterException(inner)
            }
            crate::error::GetEntitlementsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::GetEntitlementsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
