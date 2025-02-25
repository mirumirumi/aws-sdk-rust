// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_activate_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ActivatePipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_id {
        object.key("pipelineId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.parameter_values {
        let mut array_3 = object.key("parameterValues").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_value(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.start_timestamp {
        object
            .key("startTimestamp")
            .date_time(var_6, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_add_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.pipeline_id {
        object.key("pipelineId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("tags").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.name {
        object.key("name").string(var_12.as_str());
    }
    if let Some(var_13) = &input.unique_id {
        object.key("uniqueId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.description {
        object.key("description").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("tags").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deactivate_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeactivatePipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.pipeline_id {
        object.key("pipelineId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.cancel_active {
        object.key("cancelActive").boolean(*var_20);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_pipeline_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeletePipelineInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.pipeline_id {
        object.key("pipelineId").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_objects_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeObjectsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.pipeline_id {
        object.key("pipelineId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.object_ids {
        let mut array_24 = object.key("objectIds").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if input.evaluate_expressions {
        object
            .key("evaluateExpressions")
            .boolean(input.evaluate_expressions);
    }
    if let Some(var_26) = &input.marker {
        object.key("marker").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_pipelines_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribePipelinesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.pipeline_ids {
        let mut array_28 = object.key("pipelineIds").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_evaluate_expression_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EvaluateExpressionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.pipeline_id {
        object.key("pipelineId").string(var_30.as_str());
    }
    if let Some(var_31) = &input.object_id {
        object.key("objectId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.expression {
        object.key("expression").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_pipeline_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPipelineDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.pipeline_id {
        object.key("pipelineId").string(var_33.as_str());
    }
    if let Some(var_34) = &input.version {
        object.key("version").string(var_34.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_pipelines_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPipelinesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.marker {
        object.key("marker").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_poll_for_task_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PollForTaskInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.worker_group {
        object.key("workerGroup").string(var_36.as_str());
    }
    if let Some(var_37) = &input.hostname {
        object.key("hostname").string(var_37.as_str());
    }
    if let Some(var_38) = &input.instance_identity {
        #[allow(unused_mut)]
        let mut object_39 = object.key("instanceIdentity").start_object();
        crate::json_ser::serialize_structure_crate_model_instance_identity(&mut object_39, var_38)?;
        object_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_pipeline_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPipelineDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.pipeline_id {
        object.key("pipelineId").string(var_40.as_str());
    }
    if let Some(var_41) = &input.pipeline_objects {
        let mut array_42 = object.key("pipelineObjects").start_array();
        for item_43 in var_41 {
            {
                #[allow(unused_mut)]
                let mut object_44 = array_42.value().start_object();
                crate::json_ser::serialize_structure_crate_model_pipeline_object(
                    &mut object_44,
                    item_43,
                )?;
                object_44.finish();
            }
        }
        array_42.finish();
    }
    if let Some(var_45) = &input.parameter_objects {
        let mut array_46 = object.key("parameterObjects").start_array();
        for item_47 in var_45 {
            {
                #[allow(unused_mut)]
                let mut object_48 = array_46.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_object(
                    &mut object_48,
                    item_47,
                )?;
                object_48.finish();
            }
        }
        array_46.finish();
    }
    if let Some(var_49) = &input.parameter_values {
        let mut array_50 = object.key("parameterValues").start_array();
        for item_51 in var_49 {
            {
                #[allow(unused_mut)]
                let mut object_52 = array_50.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_value(
                    &mut object_52,
                    item_51,
                )?;
                object_52.finish();
            }
        }
        array_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_query_objects_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::QueryObjectsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.pipeline_id {
        object.key("pipelineId").string(var_53.as_str());
    }
    if let Some(var_54) = &input.query {
        #[allow(unused_mut)]
        let mut object_55 = object.key("query").start_object();
        crate::json_ser::serialize_structure_crate_model_query(&mut object_55, var_54)?;
        object_55.finish();
    }
    if let Some(var_56) = &input.sphere {
        object.key("sphere").string(var_56.as_str());
    }
    if let Some(var_57) = &input.marker {
        object.key("marker").string(var_57.as_str());
    }
    if let Some(var_58) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.pipeline_id {
        object.key("pipelineId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.tag_keys {
        let mut array_61 = object.key("tagKeys").start_array();
        for item_62 in var_60 {
            {
                array_61.value().string(item_62.as_str());
            }
        }
        array_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_report_task_progress_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ReportTaskProgressInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.task_id {
        object.key("taskId").string(var_63.as_str());
    }
    if let Some(var_64) = &input.fields {
        let mut array_65 = object.key("fields").start_array();
        for item_66 in var_64 {
            {
                #[allow(unused_mut)]
                let mut object_67 = array_65.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field(&mut object_67, item_66)?;
                object_67.finish();
            }
        }
        array_65.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_report_task_runner_heartbeat_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ReportTaskRunnerHeartbeatInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.taskrunner_id {
        object.key("taskrunnerId").string(var_68.as_str());
    }
    if let Some(var_69) = &input.worker_group {
        object.key("workerGroup").string(var_69.as_str());
    }
    if let Some(var_70) = &input.hostname {
        object.key("hostname").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.pipeline_id {
        object.key("pipelineId").string(var_71.as_str());
    }
    if let Some(var_72) = &input.object_ids {
        let mut array_73 = object.key("objectIds").start_array();
        for item_74 in var_72 {
            {
                array_73.value().string(item_74.as_str());
            }
        }
        array_73.finish();
    }
    if let Some(var_75) = &input.status {
        object.key("status").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_task_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetTaskStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.task_id {
        object.key("taskId").string(var_76.as_str());
    }
    if let Some(var_77) = &input.task_status {
        object.key("taskStatus").string(var_77.as_str());
    }
    if let Some(var_78) = &input.error_id {
        object.key("errorId").string(var_78.as_str());
    }
    if let Some(var_79) = &input.error_message {
        object.key("errorMessage").string(var_79.as_str());
    }
    if let Some(var_80) = &input.error_stack_trace {
        object.key("errorStackTrace").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_validate_pipeline_definition_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ValidatePipelineDefinitionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.pipeline_id {
        object.key("pipelineId").string(var_81.as_str());
    }
    if let Some(var_82) = &input.pipeline_objects {
        let mut array_83 = object.key("pipelineObjects").start_array();
        for item_84 in var_82 {
            {
                #[allow(unused_mut)]
                let mut object_85 = array_83.value().start_object();
                crate::json_ser::serialize_structure_crate_model_pipeline_object(
                    &mut object_85,
                    item_84,
                )?;
                object_85.finish();
            }
        }
        array_83.finish();
    }
    if let Some(var_86) = &input.parameter_objects {
        let mut array_87 = object.key("parameterObjects").start_array();
        for item_88 in var_86 {
            {
                #[allow(unused_mut)]
                let mut object_89 = array_87.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_object(
                    &mut object_89,
                    item_88,
                )?;
                object_89.finish();
            }
        }
        array_87.finish();
    }
    if let Some(var_90) = &input.parameter_values {
        let mut array_91 = object.key("parameterValues").start_array();
        for item_92 in var_90 {
            {
                #[allow(unused_mut)]
                let mut object_93 = array_91.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_value(
                    &mut object_93,
                    item_92,
                )?;
                object_93.finish();
            }
        }
        array_91.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.id {
        object.key("id").string(var_94.as_str());
    }
    if let Some(var_95) = &input.string_value {
        object.key("stringValue").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.key {
        object.key("key").string(var_96.as_str());
    }
    if let Some(var_97) = &input.value {
        object.key("value").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_instance_identity(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InstanceIdentity,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_98) = &input.document {
        object.key("document").string(var_98.as_str());
    }
    if let Some(var_99) = &input.signature {
        object.key("signature").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_pipeline_object(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PipelineObject,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.id {
        object.key("id").string(var_100.as_str());
    }
    if let Some(var_101) = &input.name {
        object.key("name").string(var_101.as_str());
    }
    if let Some(var_102) = &input.fields {
        let mut array_103 = object.key("fields").start_array();
        for item_104 in var_102 {
            {
                #[allow(unused_mut)]
                let mut object_105 = array_103.value().start_object();
                crate::json_ser::serialize_structure_crate_model_field(&mut object_105, item_104)?;
                object_105.finish();
            }
        }
        array_103.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_object(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterObject,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_106) = &input.id {
        object.key("id").string(var_106.as_str());
    }
    if let Some(var_107) = &input.attributes {
        let mut array_108 = object.key("attributes").start_array();
        for item_109 in var_107 {
            {
                #[allow(unused_mut)]
                let mut object_110 = array_108.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_attribute(
                    &mut object_110,
                    item_109,
                )?;
                object_110.finish();
            }
        }
        array_108.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_query(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Query,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_111) = &input.selectors {
        let mut array_112 = object.key("selectors").start_array();
        for item_113 in var_111 {
            {
                #[allow(unused_mut)]
                let mut object_114 = array_112.value().start_object();
                crate::json_ser::serialize_structure_crate_model_selector(
                    &mut object_114,
                    item_113,
                )?;
                object_114.finish();
            }
        }
        array_112.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_field(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Field,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.key {
        object.key("key").string(var_115.as_str());
    }
    if let Some(var_116) = &input.string_value {
        object.key("stringValue").string(var_116.as_str());
    }
    if let Some(var_117) = &input.ref_value {
        object.key("refValue").string(var_117.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_attribute(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterAttribute,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_118) = &input.key {
        object.key("key").string(var_118.as_str());
    }
    if let Some(var_119) = &input.string_value {
        object.key("stringValue").string(var_119.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Selector,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_120) = &input.field_name {
        object.key("fieldName").string(var_120.as_str());
    }
    if let Some(var_121) = &input.operator {
        #[allow(unused_mut)]
        let mut object_122 = object.key("operator").start_object();
        crate::json_ser::serialize_structure_crate_model_operator(&mut object_122, var_121)?;
        object_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_operator(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Operator,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.r#type {
        object.key("type").string(var_123.as_str());
    }
    if let Some(var_124) = &input.values {
        let mut array_125 = object.key("values").start_array();
        for item_126 in var_124 {
            {
                array_125.value().string(item_126.as_str());
            }
        }
        array_125.finish();
    }
    Ok(())
}
