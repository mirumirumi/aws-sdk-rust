// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_describe_services_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeServicesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_code {
        object.key("ServiceCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.format_version {
        object.key("FormatVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_attribute_values_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAttributeValuesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.service_code {
        object.key("ServiceCode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.attribute_name {
        object.key("AttributeName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_products_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetProductsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.service_code {
        object.key("ServiceCode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.filters {
        let mut array_11 = object.key("Filters").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.format_version {
        object.key("FormatVersion").string(var_14.as_str());
    }
    if let Some(var_15) = &input.next_token {
        object.key("NextToken").string(var_15.as_str());
    }
    if let Some(var_16) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.r#type {
        object.key("Type").string(var_17.as_str());
    }
    if let Some(var_18) = &input.field {
        object.key("Field").string(var_18.as_str());
    }
    if let Some(var_19) = &input.value {
        object.key("Value").string(var_19.as_str());
    }
    Ok(())
}
