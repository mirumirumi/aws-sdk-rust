// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Tags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_configuration_profile_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConfigurationProfileInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.description {
        object.key("Description").string(var_7.as_str());
    }
    if let Some(var_8) = &input.location_uri {
        object.key("LocationUri").string(var_8.as_str());
    }
    if let Some(var_9) = &input.name {
        object.key("Name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.retrieval_role_arn {
        object.key("RetrievalRoleArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.r#type {
        object.key("Type").string(var_15.as_str());
    }
    if let Some(var_16) = &input.validators {
        let mut array_17 = object.key("Validators").start_array();
        for item_18 in var_16 {
            {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::json_ser::serialize_structure_crate_model_validator(
                    &mut object_19,
                    item_18,
                )?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_deployment_strategy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDeploymentStrategyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.deployment_duration_in_minutes {
        object.key("DeploymentDurationInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.description {
        object.key("Description").string(var_21.as_str());
    }
    if input.final_bake_time_in_minutes != 0 {
        object.key("FinalBakeTimeInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.final_bake_time_in_minutes).into()),
        );
    }
    if let Some(var_22) = &input.growth_factor {
        object.key("GrowthFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_22).into()),
        );
    }
    if let Some(var_23) = &input.growth_type {
        object.key("GrowthType").string(var_23.as_str());
    }
    if let Some(var_24) = &input.name {
        object.key("Name").string(var_24.as_str());
    }
    if let Some(var_25) = &input.replicate_to {
        object.key("ReplicateTo").string(var_25.as_str());
    }
    if let Some(var_26) = &input.tags {
        #[allow(unused_mut)]
        let mut object_27 = object.key("Tags").start_object();
        for (key_28, value_29) in var_26 {
            {
                object_27.key(key_28.as_str()).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.description {
        object.key("Description").string(var_30.as_str());
    }
    if let Some(var_31) = &input.monitors {
        let mut array_32 = object.key("Monitors").start_array();
        for item_33 in var_31 {
            {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                crate::json_ser::serialize_structure_crate_model_monitor(&mut object_34, item_33)?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.name {
        object.key("Name").string(var_35.as_str());
    }
    if let Some(var_36) = &input.tags {
        #[allow(unused_mut)]
        let mut object_37 = object.key("Tags").start_object();
        for (key_38, value_39) in var_36 {
            {
                object_37.key(key_38.as_str()).string(value_39.as_str());
            }
        }
        object_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_extension_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExtensionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.actions {
        #[allow(unused_mut)]
        let mut object_41 = object.key("Actions").start_object();
        for (key_42, value_43) in var_40 {
            {
                let mut array_44 = object_41.key(key_42.as_str()).start_array();
                for item_45 in value_43 {
                    {
                        #[allow(unused_mut)]
                        let mut object_46 = array_44.value().start_object();
                        crate::json_ser::serialize_structure_crate_model_action(
                            &mut object_46,
                            item_45,
                        )?;
                        object_46.finish();
                    }
                }
                array_44.finish();
            }
        }
        object_41.finish();
    }
    if let Some(var_47) = &input.description {
        object.key("Description").string(var_47.as_str());
    }
    if let Some(var_48) = &input.name {
        object.key("Name").string(var_48.as_str());
    }
    if let Some(var_49) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_50 = object.key("Parameters").start_object();
        for (key_51, value_52) in var_49 {
            {
                #[allow(unused_mut)]
                let mut object_53 = object_50.key(key_51.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_parameter(
                    &mut object_53,
                    value_52,
                )?;
                object_53.finish();
            }
        }
        object_50.finish();
    }
    if let Some(var_54) = &input.tags {
        #[allow(unused_mut)]
        let mut object_55 = object.key("Tags").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56.as_str()).string(value_57.as_str());
            }
        }
        object_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_extension_association_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExtensionAssociationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.extension_identifier {
        object.key("ExtensionIdentifier").string(var_58.as_str());
    }
    if let Some(var_59) = &input.extension_version_number {
        object.key("ExtensionVersionNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    if let Some(var_60) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_61 = object.key("Parameters").start_object();
        for (key_62, value_63) in var_60 {
            {
                object_61.key(key_62.as_str()).string(value_63.as_str());
            }
        }
        object_61.finish();
    }
    if let Some(var_64) = &input.resource_identifier {
        object.key("ResourceIdentifier").string(var_64.as_str());
    }
    if let Some(var_65) = &input.tags {
        #[allow(unused_mut)]
        let mut object_66 = object.key("Tags").start_object();
        for (key_67, value_68) in var_65 {
            {
                object_66.key(key_67.as_str()).string(value_68.as_str());
            }
        }
        object_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_deployment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDeploymentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.configuration_profile_id {
        object.key("ConfigurationProfileId").string(var_69.as_str());
    }
    if let Some(var_70) = &input.configuration_version {
        object.key("ConfigurationVersion").string(var_70.as_str());
    }
    if let Some(var_71) = &input.deployment_strategy_id {
        object.key("DeploymentStrategyId").string(var_71.as_str());
    }
    if let Some(var_72) = &input.description {
        object.key("Description").string(var_72.as_str());
    }
    if let Some(var_73) = &input.tags {
        #[allow(unused_mut)]
        let mut object_74 = object.key("Tags").start_object();
        for (key_75, value_76) in var_73 {
            {
                object_74.key(key_75.as_str()).string(value_76.as_str());
            }
        }
        object_74.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_77) = &input.tags {
        #[allow(unused_mut)]
        let mut object_78 = object.key("Tags").start_object();
        for (key_79, value_80) in var_77 {
            {
                object_78.key(key_79.as_str()).string(value_80.as_str());
            }
        }
        object_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.description {
        object.key("Description").string(var_81.as_str());
    }
    if let Some(var_82) = &input.name {
        object.key("Name").string(var_82.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_configuration_profile_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConfigurationProfileInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_83) = &input.description {
        object.key("Description").string(var_83.as_str());
    }
    if let Some(var_84) = &input.name {
        object.key("Name").string(var_84.as_str());
    }
    if let Some(var_85) = &input.retrieval_role_arn {
        object.key("RetrievalRoleArn").string(var_85.as_str());
    }
    if let Some(var_86) = &input.validators {
        let mut array_87 = object.key("Validators").start_array();
        for item_88 in var_86 {
            {
                #[allow(unused_mut)]
                let mut object_89 = array_87.value().start_object();
                crate::json_ser::serialize_structure_crate_model_validator(
                    &mut object_89,
                    item_88,
                )?;
                object_89.finish();
            }
        }
        array_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_deployment_strategy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDeploymentStrategyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.deployment_duration_in_minutes {
        object.key("DeploymentDurationInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    if let Some(var_91) = &input.description {
        object.key("Description").string(var_91.as_str());
    }
    if let Some(var_92) = &input.final_bake_time_in_minutes {
        object.key("FinalBakeTimeInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_92).into()),
        );
    }
    if let Some(var_93) = &input.growth_factor {
        object.key("GrowthFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((*var_93).into()),
        );
    }
    if let Some(var_94) = &input.growth_type {
        object.key("GrowthType").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.description {
        object.key("Description").string(var_95.as_str());
    }
    if let Some(var_96) = &input.monitors {
        let mut array_97 = object.key("Monitors").start_array();
        for item_98 in var_96 {
            {
                #[allow(unused_mut)]
                let mut object_99 = array_97.value().start_object();
                crate::json_ser::serialize_structure_crate_model_monitor(&mut object_99, item_98)?;
                object_99.finish();
            }
        }
        array_97.finish();
    }
    if let Some(var_100) = &input.name {
        object.key("Name").string(var_100.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_extension_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateExtensionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_101) = &input.actions {
        #[allow(unused_mut)]
        let mut object_102 = object.key("Actions").start_object();
        for (key_103, value_104) in var_101 {
            {
                let mut array_105 = object_102.key(key_103.as_str()).start_array();
                for item_106 in value_104 {
                    {
                        #[allow(unused_mut)]
                        let mut object_107 = array_105.value().start_object();
                        crate::json_ser::serialize_structure_crate_model_action(
                            &mut object_107,
                            item_106,
                        )?;
                        object_107.finish();
                    }
                }
                array_105.finish();
            }
        }
        object_102.finish();
    }
    if let Some(var_108) = &input.description {
        object.key("Description").string(var_108.as_str());
    }
    if let Some(var_109) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_110 = object.key("Parameters").start_object();
        for (key_111, value_112) in var_109 {
            {
                #[allow(unused_mut)]
                let mut object_113 = object_110.key(key_111.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_parameter(
                    &mut object_113,
                    value_112,
                )?;
                object_113.finish();
            }
        }
        object_110.finish();
    }
    if let Some(var_114) = &input.version_number {
        object.key("VersionNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_114).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_extension_association_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateExtensionAssociationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_116 = object.key("Parameters").start_object();
        for (key_117, value_118) in var_115 {
            {
                object_116.key(key_117.as_str()).string(value_118.as_str());
            }
        }
        object_116.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_validator(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Validator,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.r#type {
        object.key("Type").string(var_119.as_str());
    }
    if let Some(var_120) = &input.content {
        object.key("Content").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_monitor(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Monitor,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.alarm_arn {
        object.key("AlarmArn").string(var_121.as_str());
    }
    if let Some(var_122) = &input.alarm_role_arn {
        object.key("AlarmRoleArn").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Action,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.name {
        object.key("Name").string(var_123.as_str());
    }
    if let Some(var_124) = &input.description {
        object.key("Description").string(var_124.as_str());
    }
    if let Some(var_125) = &input.uri {
        object.key("Uri").string(var_125.as_str());
    }
    if let Some(var_126) = &input.role_arn {
        object.key("RoleArn").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Parameter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.description {
        object.key("Description").string(var_127.as_str());
    }
    if input.required {
        object.key("Required").boolean(input.required);
    }
    Ok(())
}
