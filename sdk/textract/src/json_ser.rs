// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_analyze_document_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AnalyzeDocumentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.document {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.feature_types {
        let mut array_4 = object.key("FeatureTypes").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.human_loop_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("HumanLoopConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_human_loop_config(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.queries_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("QueriesConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_queries_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_analyze_expense_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AnalyzeExpenseInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.document {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_analyze_id_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AnalyzeIdInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.document_pages {
        let mut array_13 = object.key("DocumentPages").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_document(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_detect_document_text_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DetectDocumentTextInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.document {
        #[allow(unused_mut)]
        let mut object_17 = object.key("Document").start_object();
        crate::json_ser::serialize_structure_crate_model_document(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_document_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDocumentAnalysisInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.job_id {
        object.key("JobId").string(var_18.as_str());
    }
    if let Some(var_19) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.next_token {
        object.key("NextToken").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_document_text_detection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDocumentTextDetectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.job_id {
        object.key("JobId").string(var_21.as_str());
    }
    if let Some(var_22) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_expense_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetExpenseAnalysisInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.job_id {
        object.key("JobId").string(var_24.as_str());
    }
    if let Some(var_25) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_25).into()),
        );
    }
    if let Some(var_26) = &input.next_token {
        object.key("NextToken").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_document_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDocumentAnalysisInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.document_location {
        #[allow(unused_mut)]
        let mut object_28 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.feature_types {
        let mut array_30 = object.key("FeatureTypes").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31.as_str());
            }
        }
        array_30.finish();
    }
    if let Some(var_32) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_32.as_str());
    }
    if let Some(var_33) = &input.job_tag {
        object.key("JobTag").string(var_33.as_str());
    }
    if let Some(var_34) = &input.notification_channel {
        #[allow(unused_mut)]
        let mut object_35 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_35,
            var_34,
        )?;
        object_35.finish();
    }
    if let Some(var_36) = &input.output_config {
        #[allow(unused_mut)]
        let mut object_37 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_38.as_str());
    }
    if let Some(var_39) = &input.queries_config {
        #[allow(unused_mut)]
        let mut object_40 = object.key("QueriesConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_queries_config(&mut object_40, var_39)?;
        object_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_document_text_detection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDocumentTextDetectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.document_location {
        #[allow(unused_mut)]
        let mut object_42 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_42, var_41)?;
        object_42.finish();
    }
    if let Some(var_43) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_43.as_str());
    }
    if let Some(var_44) = &input.job_tag {
        object.key("JobTag").string(var_44.as_str());
    }
    if let Some(var_45) = &input.notification_channel {
        #[allow(unused_mut)]
        let mut object_46 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_46,
            var_45,
        )?;
        object_46.finish();
    }
    if let Some(var_47) = &input.output_config {
        #[allow(unused_mut)]
        let mut object_48 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_expense_analysis_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartExpenseAnalysisInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.document_location {
        #[allow(unused_mut)]
        let mut object_51 = object.key("DocumentLocation").start_object();
        crate::json_ser::serialize_structure_crate_model_document_location(&mut object_51, var_50)?;
        object_51.finish();
    }
    if let Some(var_52) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_52.as_str());
    }
    if let Some(var_53) = &input.job_tag {
        object.key("JobTag").string(var_53.as_str());
    }
    if let Some(var_54) = &input.notification_channel {
        #[allow(unused_mut)]
        let mut object_55 = object.key("NotificationChannel").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_channel(
            &mut object_55,
            var_54,
        )?;
        object_55.finish();
    }
    if let Some(var_56) = &input.output_config {
        #[allow(unused_mut)]
        let mut object_57 = object.key("OutputConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_config(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.kms_key_id {
        object.key("KMSKeyId").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_document(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Document,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.bytes {
        object
            .key("Bytes")
            .string_unchecked(&aws_smithy_types::base64::encode(var_59));
    }
    if let Some(var_60) = &input.s3_object {
        #[allow(unused_mut)]
        let mut object_61 = object.key("S3Object").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_object(&mut object_61, var_60)?;
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_human_loop_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.human_loop_name {
        object.key("HumanLoopName").string(var_62.as_str());
    }
    if let Some(var_63) = &input.flow_definition_arn {
        object.key("FlowDefinitionArn").string(var_63.as_str());
    }
    if let Some(var_64) = &input.data_attributes {
        #[allow(unused_mut)]
        let mut object_65 = object.key("DataAttributes").start_object();
        crate::json_ser::serialize_structure_crate_model_human_loop_data_attributes(
            &mut object_65,
            var_64,
        )?;
        object_65.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_queries_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::QueriesConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.queries {
        let mut array_67 = object.key("Queries").start_array();
        for item_68 in var_66 {
            {
                #[allow(unused_mut)]
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_crate_model_query(&mut object_69, item_68)?;
                object_69.finish();
            }
        }
        array_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_document_location(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DocumentLocation,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.s3_object {
        #[allow(unused_mut)]
        let mut object_71 = object.key("S3Object").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_object(&mut object_71, var_70)?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_channel(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationChannel,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.sns_topic_arn {
        object.key("SNSTopicArn").string(var_72.as_str());
    }
    if let Some(var_73) = &input.role_arn {
        object.key("RoleArn").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_output_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.s3_bucket {
        object.key("S3Bucket").string(var_74.as_str());
    }
    if let Some(var_75) = &input.s3_prefix {
        object.key("S3Prefix").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_object(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Object,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.bucket {
        object.key("Bucket").string(var_76.as_str());
    }
    if let Some(var_77) = &input.name {
        object.key("Name").string(var_77.as_str());
    }
    if let Some(var_78) = &input.version {
        object.key("Version").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_human_loop_data_attributes(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopDataAttributes,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.content_classifiers {
        let mut array_80 = object.key("ContentClassifiers").start_array();
        for item_81 in var_79 {
            {
                array_80.value().string(item_81.as_str());
            }
        }
        array_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_query(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Query,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.text {
        object.key("Text").string(var_82.as_str());
    }
    if let Some(var_83) = &input.alias {
        object.key("Alias").string(var_83.as_str());
    }
    if let Some(var_84) = &input.pages {
        let mut array_85 = object.key("Pages").start_array();
        for item_86 in var_84 {
            {
                array_85.value().string(item_86.as_str());
            }
        }
        array_85.finish();
    }
    Ok(())
}
