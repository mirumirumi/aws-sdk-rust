// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_media_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetMediaInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.start_selector {
        #[allow(unused_mut)]
        let mut object_2 = object.key("StartSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_start_selector(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.stream_arn {
        object.key("StreamARN").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stream_name {
        object.key("StreamName").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_start_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StartSelector,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.start_selector_type {
        object.key("StartSelectorType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.after_fragment_number {
        object.key("AfterFragmentNumber").string(var_6.as_str());
    }
    if let Some(var_7) = &input.start_timestamp {
        object
            .key("StartTimestamp")
            .date_time(var_7, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_8) = &input.continuation_token {
        object.key("ContinuationToken").string(var_8.as_str());
    }
    Ok(())
}
