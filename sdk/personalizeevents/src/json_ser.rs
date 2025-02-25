// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_put_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEventsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_list {
        let mut array_2 = object.key("eventList").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_event(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.session_id {
        object.key("sessionId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tracking_id {
        object.key("trackingId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.user_id {
        object.key("userId").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutItemsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.dataset_arn {
        object.key("datasetArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.items {
        let mut array_10 = object.key("items").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_item(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_users_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutUsersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.dataset_arn {
        object.key("datasetArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.users {
        let mut array_15 = object.key("users").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_user(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Event,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.event_id {
        object.key("eventId").string(var_18.as_str());
    }
    if let Some(var_19) = &input.event_type {
        object.key("eventType").string(var_19.as_str());
    }
    if let Some(var_20) = &input.event_value {
        object.key("eventValue").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.item_id {
        object.key("itemId").string(var_21.as_str());
    }
    if let Some(var_22) = &input.properties {
        object.key("properties").string(var_22.as_str());
    }
    if let Some(var_23) = &input.sent_at {
        object
            .key("sentAt")
            .date_time(var_23, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_24) = &input.recommendation_id {
        object.key("recommendationId").string(var_24.as_str());
    }
    if let Some(var_25) = &input.impression {
        let mut array_26 = object.key("impression").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Item,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.item_id {
        object.key("itemId").string(var_28.as_str());
    }
    if let Some(var_29) = &input.properties {
        object.key("properties").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_user(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::User,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.user_id {
        object.key("userId").string(var_30.as_str());
    }
    if let Some(var_31) = &input.properties {
        object.key("properties").string(var_31.as_str());
    }
    Ok(())
}
