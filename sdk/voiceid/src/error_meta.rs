// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient permissions to perform this action. Check the error message and try again.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The request failed due to a conflict. Check the <code>ConflictType</code> and error message for more details.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>The request failed due to an unknown error on the server side.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The specified resource cannot be found. Check the <code>ResourceType</code> and error message for more details.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request exceeded the service quota. Refer to <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html#voiceid-quotas">Voice ID Service Quotas</a> and try your request again.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling. Please slow down your request rate. Refer to <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html##voiceid-api-quotas"> Amazon Connect Voice ID Service API throttling quotas </a> and try your request again.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The request failed one or more validations; check the error message for more details.</p>
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateDomainError> for Error {
    fn from(err: crate::error::CreateDomainError) -> Self {
        match err.kind {
            crate::error::CreateDomainErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateDomainErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::CreateDomainErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CreateDomainErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::CreateDomainErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::CreateDomainErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateDomainErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CreateDomainErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteDomainError> for Error {
    fn from(err: crate::error::DeleteDomainError) -> Self {
        match err.kind {
            crate::error::DeleteDomainErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteDomainErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteDomainErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteDomainErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteDomainErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteDomainErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteDomainErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteFraudsterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteFraudsterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteFraudsterError> for Error {
    fn from(err: crate::error::DeleteFraudsterError) -> Self {
        match err.kind {
            crate::error::DeleteFraudsterErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteFraudsterErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSpeakerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSpeakerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteSpeakerError> for Error {
    fn from(err: crate::error::DeleteSpeakerError) -> Self {
        match err.kind {
            crate::error::DeleteSpeakerErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteSpeakerErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeDomainError> for Error {
    fn from(err: crate::error::DescribeDomainError) -> Self {
        match err.kind {
            crate::error::DescribeDomainErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DescribeDomainErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeDomainErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeDomainErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeDomainErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeDomainErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFraudsterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeFraudsterError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFraudsterError> for Error {
    fn from(err: crate::error::DescribeFraudsterError) -> Self {
        match err.kind {
            crate::error::DescribeFraudsterErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DescribeFraudsterErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFraudsterErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeFraudsterErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeFraudsterErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFraudsterErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeFraudsterRegistrationJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeFraudsterRegistrationJobError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFraudsterRegistrationJobError> for Error {
    fn from(err: crate::error::DescribeFraudsterRegistrationJobError) -> Self {
        match err.kind {
            crate::error::DescribeFraudsterRegistrationJobErrorKind::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::error::DescribeFraudsterRegistrationJobErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::DescribeFraudsterRegistrationJobErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeFraudsterRegistrationJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeFraudsterRegistrationJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFraudsterRegistrationJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSpeakerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeSpeakerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeSpeakerError> for Error {
    fn from(err: crate::error::DescribeSpeakerError) -> Self {
        match err.kind {
            crate::error::DescribeSpeakerErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DescribeSpeakerErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeSpeakerErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeSpeakerErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeSpeakerErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeSpeakerErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSpeakerEnrollmentJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeSpeakerEnrollmentJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeSpeakerEnrollmentJobError> for Error {
    fn from(err: crate::error::DescribeSpeakerEnrollmentJobError) -> Self {
        match err.kind {
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeSpeakerEnrollmentJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EvaluateSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EvaluateSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::EvaluateSessionError> for Error {
    fn from(err: crate::error::EvaluateSessionError) -> Self {
        match err.kind {
            crate::error::EvaluateSessionErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::EvaluateSessionErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::EvaluateSessionErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::EvaluateSessionErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::EvaluateSessionErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::EvaluateSessionErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::EvaluateSessionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDomainsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDomainsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListDomainsError> for Error {
    fn from(err: crate::error::ListDomainsError) -> Self {
        match err.kind {
            crate::error::ListDomainsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListDomainsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListDomainsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListDomainsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListDomainsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFraudsterRegistrationJobsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListFraudsterRegistrationJobsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFraudsterRegistrationJobsError> for Error {
    fn from(err: crate::error::ListFraudsterRegistrationJobsError) -> Self {
        match err.kind {
            crate::error::ListFraudsterRegistrationJobsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListFraudsterRegistrationJobsErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::ListFraudsterRegistrationJobsErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::ListFraudsterRegistrationJobsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListFraudsterRegistrationJobsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListFraudsterRegistrationJobsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSpeakerEnrollmentJobsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListSpeakerEnrollmentJobsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSpeakerEnrollmentJobsError> for Error {
    fn from(err: crate::error::ListSpeakerEnrollmentJobsError) -> Self {
        match err.kind {
            crate::error::ListSpeakerEnrollmentJobsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListSpeakerEnrollmentJobsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListSpeakerEnrollmentJobsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListSpeakerEnrollmentJobsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListSpeakerEnrollmentJobsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListSpeakerEnrollmentJobsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSpeakersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSpeakersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSpeakersError> for Error {
    fn from(err: crate::error::ListSpeakersError) -> Self {
        match err.kind {
            crate::error::ListSpeakersErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListSpeakersErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListSpeakersErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListSpeakersErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListSpeakersErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListSpeakersErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::OptOutSpeakerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::OptOutSpeakerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::OptOutSpeakerError> for Error {
    fn from(err: crate::error::OptOutSpeakerError) -> Self {
        match err.kind {
            crate::error::OptOutSpeakerErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::OptOutSpeakerErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartFraudsterRegistrationJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartFraudsterRegistrationJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartFraudsterRegistrationJobError> for Error {
    fn from(err: crate::error::StartFraudsterRegistrationJobError) -> Self {
        match err.kind {
            crate::error::StartFraudsterRegistrationJobErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartFraudsterRegistrationJobErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StartFraudsterRegistrationJobErrorKind::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::error::StartFraudsterRegistrationJobErrorKind::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::error::StartFraudsterRegistrationJobErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::StartFraudsterRegistrationJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartFraudsterRegistrationJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartFraudsterRegistrationJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartSpeakerEnrollmentJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartSpeakerEnrollmentJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartSpeakerEnrollmentJobError> for Error {
    fn from(err: crate::error::StartSpeakerEnrollmentJobError) -> Self {
        match err.kind {
            crate::error::StartSpeakerEnrollmentJobErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::ServiceQuotaExceededException(
                inner,
            ) => Error::ServiceQuotaExceededException(inner),
            crate::error::StartSpeakerEnrollmentJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartSpeakerEnrollmentJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::TagResourceErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::TagResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::TagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::UntagResourceErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UntagResourceErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDomainError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateDomainError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateDomainError> for Error {
    fn from(err: crate::error::UpdateDomainError) -> Self {
        match err.kind {
            crate::error::UpdateDomainErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::UpdateDomainErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::UpdateDomainErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::UpdateDomainErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UpdateDomainErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::UpdateDomainErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UpdateDomainErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
