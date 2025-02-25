// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You don't have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The request couldn't be completed because it conflicted with the current state of the resource.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>An exception occurred with the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The resource couldn't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request failed because a service quota is exceeded.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The request was denied because of request throttling. Retry the request.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>A parameter is not valid.</p>
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
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssociateUserError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AssociateUserError> for Error {
    fn from(err: crate::error::AssociateUserError) -> Self {
        match err.kind {
            crate::error::AssociateUserErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::AssociateUserErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::AssociateUserErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::AssociateUserErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::AssociateUserErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::AssociateUserErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::AssociateUserErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::AssociateUserErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeregisterIdentityProviderError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeregisterIdentityProviderError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeregisterIdentityProviderError> for Error {
    fn from(err: crate::error::DeregisterIdentityProviderError) -> Self {
        match err.kind {
            crate::error::DeregisterIdentityProviderErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::DeregisterIdentityProviderErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeregisterIdentityProviderErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DisassociateUserError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisassociateUserError> for Error {
    fn from(err: crate::error::DisassociateUserError) -> Self {
        match err.kind {
            crate::error::DisassociateUserErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DisassociateUserErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DisassociateUserErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DisassociateUserErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DisassociateUserErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::DisassociateUserErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DisassociateUserErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DisassociateUserErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListIdentityProvidersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListIdentityProvidersError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListIdentityProvidersError> for Error {
    fn from(err: crate::error::ListIdentityProvidersError) -> Self {
        match err.kind {
            crate::error::ListIdentityProvidersErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListIdentityProvidersErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListInstancesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListInstancesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListInstancesError> for Error {
    fn from(err: crate::error::ListInstancesError) -> Self {
        match err.kind {
            crate::error::ListInstancesErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListInstancesErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::ListInstancesErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListInstancesErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListInstancesErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::ListInstancesErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListInstancesErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListInstancesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProductSubscriptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListProductSubscriptionsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListProductSubscriptionsError> for Error {
    fn from(err: crate::error::ListProductSubscriptionsError) -> Self {
        match err.kind {
            crate::error::ListProductSubscriptionsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::ListProductSubscriptionsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListProductSubscriptionsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListUserAssociationsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListUserAssociationsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListUserAssociationsError> for Error {
    fn from(err: crate::error::ListUserAssociationsError) -> Self {
        match err.kind {
            crate::error::ListUserAssociationsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListUserAssociationsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterIdentityProviderError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RegisterIdentityProviderError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RegisterIdentityProviderError> for Error {
    fn from(err: crate::error::RegisterIdentityProviderError) -> Self {
        match err.kind {
            crate::error::RegisterIdentityProviderErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::RegisterIdentityProviderErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::RegisterIdentityProviderErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartProductSubscriptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartProductSubscriptionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartProductSubscriptionError> for Error {
    fn from(err: crate::error::StartProductSubscriptionError) -> Self {
        match err.kind {
            crate::error::StartProductSubscriptionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::StartProductSubscriptionErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartProductSubscriptionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopProductSubscriptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StopProductSubscriptionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopProductSubscriptionError> for Error {
    fn from(err: crate::error::StopProductSubscriptionError) -> Self {
        match err.kind {
            crate::error::StopProductSubscriptionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::StopProductSubscriptionErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StopProductSubscriptionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
