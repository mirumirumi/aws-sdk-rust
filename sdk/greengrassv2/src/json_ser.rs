// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_service_role_to_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateServiceRoleToAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.role_arn {
        object.key("RoleArn").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_associate_client_device_with_core_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchAssociateClientDeviceWithCoreDeviceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.entries {
        let mut array_3 = object.key("entries").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_associate_client_device_with_core_device_entry(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_disassociate_client_device_from_core_device_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDisassociateClientDeviceFromCoreDeviceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.entries {
        let mut array_7 = object.key("entries").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_disassociate_client_device_from_core_device_entry(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_component_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateComponentVersionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.client_token {
        object.key("clientToken").string(var_10.as_str());
    }
    if let Some(var_11) = &input.inline_recipe {
        object
            .key("inlineRecipe")
            .string_unchecked(&aws_smithy_types::base64::encode(var_11));
    }
    if let Some(var_12) = &input.lambda_function {
        #[allow(unused_mut)]
        let mut object_13 = object.key("lambdaFunction").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_function_recipe_source(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.tags {
        #[allow(unused_mut)]
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_deployment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeploymentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.client_token {
        object.key("clientToken").string(var_18.as_str());
    }
    if let Some(var_19) = &input.components {
        #[allow(unused_mut)]
        let mut object_20 = object.key("components").start_object();
        for (key_21, value_22) in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_23 = object_20.key(key_21.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_component_deployment_specification(&mut object_23, value_22)?;
                object_23.finish();
            }
        }
        object_20.finish();
    }
    if let Some(var_24) = &input.deployment_name {
        object.key("deploymentName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.deployment_policies {
        #[allow(unused_mut)]
        let mut object_26 = object.key("deploymentPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_policies(
            &mut object_26,
            var_25,
        )?;
        object_26.finish();
    }
    if let Some(var_27) = &input.iot_job_configuration {
        #[allow(unused_mut)]
        let mut object_28 = object.key("iotJobConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_io_t_job_configuration(
            &mut object_28,
            var_27,
        )?;
        object_28.finish();
    }
    if let Some(var_29) = &input.tags {
        #[allow(unused_mut)]
        let mut object_30 = object.key("tags").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31.as_str()).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.target_arn {
        object.key("targetArn").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resolve_component_candidates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResolveComponentCandidatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.component_candidates {
        let mut array_35 = object.key("componentCandidates").start_array();
        for item_36 in var_34 {
            {
                #[allow(unused_mut)]
                let mut object_37 = array_35.value().start_object();
                crate::json_ser::serialize_structure_crate_model_component_candidate(
                    &mut object_37,
                    item_36,
                )?;
                object_37.finish();
            }
        }
        array_35.finish();
    }
    if let Some(var_38) = &input.platform {
        #[allow(unused_mut)]
        let mut object_39 = object.key("platform").start_object();
        crate::json_ser::serialize_structure_crate_model_component_platform(
            &mut object_39,
            var_38,
        )?;
        object_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.tags {
        #[allow(unused_mut)]
        let mut object_41 = object.key("tags").start_object();
        for (key_42, value_43) in var_40 {
            {
                object_41.key(key_42.as_str()).string(value_43.as_str());
            }
        }
        object_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_connectivity_info_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConnectivityInfoInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.connectivity_info {
        let mut array_45 = object.key("ConnectivityInfo").start_array();
        for item_46 in var_44 {
            {
                #[allow(unused_mut)]
                let mut object_47 = array_45.value().start_object();
                crate::json_ser::serialize_structure_crate_model_connectivity_info(
                    &mut object_47,
                    item_46,
                )?;
                object_47.finish();
            }
        }
        array_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_associate_client_device_with_core_device_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssociateClientDeviceWithCoreDeviceEntry,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.thing_name {
        object.key("thingName").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_disassociate_client_device_from_core_device_entry(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DisassociateClientDeviceFromCoreDeviceEntry,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_49) = &input.thing_name {
        object.key("thingName").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_function_recipe_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaFunctionRecipeSource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.lambda_arn {
        object.key("lambdaArn").string(var_50.as_str());
    }
    if let Some(var_51) = &input.component_name {
        object.key("componentName").string(var_51.as_str());
    }
    if let Some(var_52) = &input.component_version {
        object.key("componentVersion").string(var_52.as_str());
    }
    if let Some(var_53) = &input.component_platforms {
        let mut array_54 = object.key("componentPlatforms").start_array();
        for item_55 in var_53 {
            {
                #[allow(unused_mut)]
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_component_platform(
                    &mut object_56,
                    item_55,
                )?;
                object_56.finish();
            }
        }
        array_54.finish();
    }
    if let Some(var_57) = &input.component_dependencies {
        #[allow(unused_mut)]
        let mut object_58 = object.key("componentDependencies").start_object();
        for (key_59, value_60) in var_57 {
            {
                #[allow(unused_mut)]
                let mut object_61 = object_58.key(key_59.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_component_dependency_requirement(
                    &mut object_61,
                    value_60,
                )?;
                object_61.finish();
            }
        }
        object_58.finish();
    }
    if let Some(var_62) = &input.component_lambda_parameters {
        #[allow(unused_mut)]
        let mut object_63 = object.key("componentLambdaParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_execution_parameters(
            &mut object_63,
            var_62,
        )?;
        object_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_deployment_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentDeploymentSpecification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.component_version {
        object.key("componentVersion").string(var_64.as_str());
    }
    if let Some(var_65) = &input.configuration_update {
        #[allow(unused_mut)]
        let mut object_66 = object.key("configurationUpdate").start_object();
        crate::json_ser::serialize_structure_crate_model_component_configuration_update(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.run_with {
        #[allow(unused_mut)]
        let mut object_68 = object.key("runWith").start_object();
        crate::json_ser::serialize_structure_crate_model_component_run_with(
            &mut object_68,
            var_67,
        )?;
        object_68.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_policies(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentPolicies,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.failure_handling_policy {
        object.key("failureHandlingPolicy").string(var_69.as_str());
    }
    if let Some(var_70) = &input.component_update_policy {
        #[allow(unused_mut)]
        let mut object_71 = object.key("componentUpdatePolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_component_update_policy(
            &mut object_71,
            var_70,
        )?;
        object_71.finish();
    }
    if let Some(var_72) = &input.configuration_validation_policy {
        #[allow(unused_mut)]
        let mut object_73 = object.key("configurationValidationPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_deployment_configuration_validation_policy(&mut object_73, var_72)?;
        object_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_io_t_job_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentIoTJobConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.job_executions_rollout_config {
        #[allow(unused_mut)]
        let mut object_75 = object.key("jobExecutionsRolloutConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_executions_rollout_config(
            &mut object_75,
            var_74,
        )?;
        object_75.finish();
    }
    if let Some(var_76) = &input.abort_config {
        #[allow(unused_mut)]
        let mut object_77 = object.key("abortConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_abort_config(
            &mut object_77,
            var_76,
        )?;
        object_77.finish();
    }
    if let Some(var_78) = &input.timeout_config {
        #[allow(unused_mut)]
        let mut object_79 = object.key("timeoutConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_timeout_config(
            &mut object_79,
            var_78,
        )?;
        object_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_candidate(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentCandidate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.component_name {
        object.key("componentName").string(var_80.as_str());
    }
    if let Some(var_81) = &input.component_version {
        object.key("componentVersion").string(var_81.as_str());
    }
    if let Some(var_82) = &input.version_requirements {
        #[allow(unused_mut)]
        let mut object_83 = object.key("versionRequirements").start_object();
        for (key_84, value_85) in var_82 {
            {
                object_83.key(key_84.as_str()).string(value_85.as_str());
            }
        }
        object_83.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_platform(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentPlatform,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.name {
        object.key("name").string(var_86.as_str());
    }
    if let Some(var_87) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_88 = object.key("attributes").start_object();
        for (key_89, value_90) in var_87 {
            {
                object_88.key(key_89.as_str()).string(value_90.as_str());
            }
        }
        object_88.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_connectivity_info(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectivityInfo,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.id {
        object.key("Id").string(var_91.as_str());
    }
    if let Some(var_92) = &input.host_address {
        object.key("HostAddress").string(var_92.as_str());
    }
    if input.port_number != 0 {
        object.key("PortNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.port_number).into()),
        );
    }
    if let Some(var_93) = &input.metadata {
        object.key("Metadata").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_dependency_requirement(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentDependencyRequirement,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.version_requirement {
        object.key("versionRequirement").string(var_94.as_str());
    }
    if let Some(var_95) = &input.dependency_type {
        object.key("dependencyType").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_execution_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaExecutionParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.event_sources {
        let mut array_97 = object.key("eventSources").start_array();
        for item_98 in var_96 {
            {
                #[allow(unused_mut)]
                let mut object_99 = array_97.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_event_source(
                    &mut object_99,
                    item_98,
                )?;
                object_99.finish();
            }
        }
        array_97.finish();
    }
    if let Some(var_100) = &input.max_queue_size {
        object.key("maxQueueSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    if let Some(var_101) = &input.max_instances_count {
        object.key("maxInstancesCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_101).into()),
        );
    }
    if let Some(var_102) = &input.max_idle_time_in_seconds {
        object.key("maxIdleTimeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_102).into()),
        );
    }
    if let Some(var_103) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_103).into()),
        );
    }
    if let Some(var_104) = &input.status_timeout_in_seconds {
        object.key("statusTimeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_104).into()),
        );
    }
    if let Some(var_105) = &input.pinned {
        object.key("pinned").boolean(*var_105);
    }
    if let Some(var_106) = &input.input_payload_encoding_type {
        object
            .key("inputPayloadEncodingType")
            .string(var_106.as_str());
    }
    if let Some(var_107) = &input.exec_args {
        let mut array_108 = object.key("execArgs").start_array();
        for item_109 in var_107 {
            {
                array_108.value().string(item_109.as_str());
            }
        }
        array_108.finish();
    }
    if let Some(var_110) = &input.environment_variables {
        #[allow(unused_mut)]
        let mut object_111 = object.key("environmentVariables").start_object();
        for (key_112, value_113) in var_110 {
            {
                object_111.key(key_112.as_str()).string(value_113.as_str());
            }
        }
        object_111.finish();
    }
    if let Some(var_114) = &input.linux_process_params {
        #[allow(unused_mut)]
        let mut object_115 = object.key("linuxProcessParams").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_linux_process_params(
            &mut object_115,
            var_114,
        )?;
        object_115.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_configuration_update(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentConfigurationUpdate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_116) = &input.merge {
        object.key("merge").string(var_116.as_str());
    }
    if let Some(var_117) = &input.reset {
        let mut array_118 = object.key("reset").start_array();
        for item_119 in var_117 {
            {
                array_118.value().string(item_119.as_str());
            }
        }
        array_118.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_component_run_with(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComponentRunWith,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_120) = &input.posix_user {
        object.key("posixUser").string(var_120.as_str());
    }
    if let Some(var_121) = &input.system_resource_limits {
        #[allow(unused_mut)]
        let mut object_122 = object.key("systemResourceLimits").start_object();
        crate::json_ser::serialize_structure_crate_model_system_resource_limits(
            &mut object_122,
            var_121,
        )?;
        object_122.finish();
    }
    if let Some(var_123) = &input.windows_user {
        object.key("windowsUser").string(var_123.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_component_update_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentComponentUpdatePolicy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_124) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_124).into()),
        );
    }
    if let Some(var_125) = &input.action {
        object.key("action").string(var_125.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_deployment_configuration_validation_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeploymentConfigurationValidationPolicy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_126) = &input.timeout_in_seconds {
        object.key("timeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_126).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_executions_rollout_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobExecutionsRolloutConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.exponential_rate {
        #[allow(unused_mut)]
        let mut object_128 = object.key("exponentialRate").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_exponential_rollout_rate(
            &mut object_128,
            var_127,
        )?;
        object_128.finish();
    }
    if let Some(var_129) = &input.maximum_per_minute {
        object.key("maximumPerMinute").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_129).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_abort_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobAbortConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_130) = &input.criteria_list {
        let mut array_131 = object.key("criteriaList").start_array();
        for item_132 in var_130 {
            {
                #[allow(unused_mut)]
                let mut object_133 = array_131.value().start_object();
                crate::json_ser::serialize_structure_crate_model_io_t_job_abort_criteria(
                    &mut object_133,
                    item_132,
                )?;
                object_133.finish();
            }
        }
        array_131.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_timeout_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobTimeoutConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_134) = &input.in_progress_timeout_in_minutes {
        object.key("inProgressTimeoutInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_134).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_event_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaEventSource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_135) = &input.topic {
        object.key("topic").string(var_135.as_str());
    }
    if let Some(var_136) = &input.r#type {
        object.key("type").string(var_136.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_linux_process_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaLinuxProcessParams,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_137) = &input.isolation_mode {
        object.key("isolationMode").string(var_137.as_str());
    }
    if let Some(var_138) = &input.container_params {
        #[allow(unused_mut)]
        let mut object_139 = object.key("containerParams").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_container_params(
            &mut object_139,
            var_138,
        )?;
        object_139.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_system_resource_limits(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SystemResourceLimits,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.memory != 0 {
        object.key("memory").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.memory).into()),
        );
    }
    if input.cpus != 0.0 {
        object.key("cpus").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.cpus).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_exponential_rollout_rate(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobExponentialRolloutRate,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("baseRatePerMinute").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.base_rate_per_minute).into()),
        );
    }
    {
        object.key("incrementFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.increment_factor).into()),
        );
    }
    if let Some(var_140) = &input.rate_increase_criteria {
        #[allow(unused_mut)]
        let mut object_141 = object.key("rateIncreaseCriteria").start_object();
        crate::json_ser::serialize_structure_crate_model_io_t_job_rate_increase_criteria(
            &mut object_141,
            var_140,
        )?;
        object_141.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_abort_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobAbortCriteria,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_142) = &input.failure_type {
        object.key("failureType").string(var_142.as_str());
    }
    if let Some(var_143) = &input.action {
        object.key("action").string(var_143.as_str());
    }
    {
        object.key("thresholdPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.threshold_percentage).into()),
        );
    }
    {
        object.key("minNumberOfExecutedThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_number_of_executed_things).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_container_params(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaContainerParams,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_144) = &input.memory_size_in_kb {
        object.key("memorySizeInKB").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_144).into()),
        );
    }
    if let Some(var_145) = &input.mount_ro_sysfs {
        object.key("mountROSysfs").boolean(*var_145);
    }
    if let Some(var_146) = &input.volumes {
        let mut array_147 = object.key("volumes").start_array();
        for item_148 in var_146 {
            {
                #[allow(unused_mut)]
                let mut object_149 = array_147.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_volume_mount(
                    &mut object_149,
                    item_148,
                )?;
                object_149.finish();
            }
        }
        array_147.finish();
    }
    if let Some(var_150) = &input.devices {
        let mut array_151 = object.key("devices").start_array();
        for item_152 in var_150 {
            {
                #[allow(unused_mut)]
                let mut object_153 = array_151.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lambda_device_mount(
                    &mut object_153,
                    item_152,
                )?;
                object_153.finish();
            }
        }
        array_151.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_io_t_job_rate_increase_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IoTJobRateIncreaseCriteria,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_154) = &input.number_of_notified_things {
        object.key("numberOfNotifiedThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_154).into()),
        );
    }
    if let Some(var_155) = &input.number_of_succeeded_things {
        object.key("numberOfSucceededThings").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_155).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_volume_mount(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaVolumeMount,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_156) = &input.source_path {
        object.key("sourcePath").string(var_156.as_str());
    }
    if let Some(var_157) = &input.destination_path {
        object.key("destinationPath").string(var_157.as_str());
    }
    if let Some(var_158) = &input.permission {
        object.key("permission").string(var_158.as_str());
    }
    if let Some(var_159) = &input.add_group_owner {
        object.key("addGroupOwner").boolean(*var_159);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_device_mount(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaDeviceMount,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_160) = &input.path {
        object.key("path").string(var_160.as_str());
    }
    if let Some(var_161) = &input.permission {
        object.key("permission").string(var_161.as_str());
    }
    if let Some(var_162) = &input.add_group_owner {
        object.key("addGroupOwner").boolean(*var_162);
    }
    Ok(())
}
