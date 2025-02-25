// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_get_role_credentials(
    input: &crate::input::GetRoleCredentialsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.access_token {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "access_token",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-sso_bearer_token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_account_roles(
    input: &crate::input::ListAccountRolesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_3) = &input.access_token {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "access_token",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-sso_bearer_token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_accounts(
    input: &crate::input::ListAccountsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_5) = &input.access_token {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "access_token",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-sso_bearer_token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_logout(
    input: &crate::input::LogoutInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_7) = &input.access_token {
        let formatted_8 = inner_7.as_str();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::error::BuildError::invalid_field(
                        "access_token",
                        format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    )
                })?;
            builder = builder.header("x-amz-sso_bearer_token", header_value);
        }
    }
    Ok(builder)
}
