// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_attributes_to_findings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddAttributesToFindingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.finding_arns {
        let mut array_2 = object.key("findingArns").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.attributes {
        let mut array_5 = object.key("attributes").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_attribute(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_assessment_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAssessmentTargetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.assessment_target_name {
        object.key("assessmentTargetName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.resource_group_arn {
        object.key("resourceGroupArn").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_assessment_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAssessmentTemplateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.assessment_target_arn {
        object.key("assessmentTargetArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.assessment_template_name {
        object.key("assessmentTemplateName").string(var_11.as_str());
    }
    {
        object.key("durationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.duration_in_seconds).into()),
        );
    }
    if let Some(var_12) = &input.rules_package_arns {
        let mut array_13 = object.key("rulesPackageArns").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.user_attributes_for_findings {
        let mut array_16 = object.key("userAttributesForFindings").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_attribute(
                    &mut object_18,
                    item_17,
                )?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_exclusions_preview_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateExclusionsPreviewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.assessment_template_arn {
        object.key("assessmentTemplateArn").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_resource_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResourceGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.resource_group_tags {
        let mut array_21 = object.key("resourceGroupTags").start_array();
        for item_22 in var_20 {
            {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_group_tag(
                    &mut object_23,
                    item_22,
                )?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_assessment_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAssessmentRunInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_assessment_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAssessmentTargetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.assessment_target_arn {
        object.key("assessmentTargetArn").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_assessment_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAssessmentTemplateInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.assessment_template_arn {
        object.key("assessmentTemplateArn").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_assessment_runs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAssessmentRunsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.assessment_run_arns {
        let mut array_28 = object.key("assessmentRunArns").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_assessment_targets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAssessmentTargetsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.assessment_target_arns {
        let mut array_31 = object.key("assessmentTargetArns").start_array();
        for item_32 in var_30 {
            {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_assessment_templates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAssessmentTemplatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.assessment_template_arns {
        let mut array_34 = object.key("assessmentTemplateArns").start_array();
        for item_35 in var_33 {
            {
                array_34.value().string(item_35.as_str());
            }
        }
        array_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_exclusions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeExclusionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.exclusion_arns {
        let mut array_37 = object.key("exclusionArns").start_array();
        for item_38 in var_36 {
            {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    if let Some(var_39) = &input.locale {
        object.key("locale").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_findings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeFindingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.finding_arns {
        let mut array_41 = object.key("findingArns").start_array();
        for item_42 in var_40 {
            {
                array_41.value().string(item_42.as_str());
            }
        }
        array_41.finish();
    }
    if let Some(var_43) = &input.locale {
        object.key("locale").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_resource_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeResourceGroupsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.resource_group_arns {
        let mut array_45 = object.key("resourceGroupArns").start_array();
        for item_46 in var_44 {
            {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_rules_packages_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeRulesPackagesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.rules_package_arns {
        let mut array_48 = object.key("rulesPackageArns").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.locale {
        object.key("locale").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_assessment_report_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetAssessmentReportInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_51.as_str());
    }
    if let Some(var_52) = &input.report_file_format {
        object.key("reportFileFormat").string(var_52.as_str());
    }
    if let Some(var_53) = &input.report_type {
        object.key("reportType").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_exclusions_preview_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetExclusionsPreviewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.assessment_template_arn {
        object.key("assessmentTemplateArn").string(var_54.as_str());
    }
    if let Some(var_55) = &input.preview_token {
        object.key("previewToken").string(var_55.as_str());
    }
    if let Some(var_56) = &input.next_token {
        object.key("nextToken").string(var_56.as_str());
    }
    if let Some(var_57) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.locale {
        object.key("locale").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_telemetry_metadata_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTelemetryMetadataInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_assessment_run_agents_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAssessmentRunAgentsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_60.as_str());
    }
    if let Some(var_61) = &input.filter {
        #[allow(unused_mut)]
        let mut object_62 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_agent_filter(&mut object_62, var_61)?;
        object_62.finish();
    }
    if let Some(var_63) = &input.next_token {
        object.key("nextToken").string(var_63.as_str());
    }
    if let Some(var_64) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_64).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_assessment_runs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAssessmentRunsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.assessment_template_arns {
        let mut array_66 = object.key("assessmentTemplateArns").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67.as_str());
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.filter {
        #[allow(unused_mut)]
        let mut object_69 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_assessment_run_filter(
            &mut object_69,
            var_68,
        )?;
        object_69.finish();
    }
    if let Some(var_70) = &input.next_token {
        object.key("nextToken").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_assessment_targets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAssessmentTargetsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.filter {
        #[allow(unused_mut)]
        let mut object_73 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_assessment_target_filter(
            &mut object_73,
            var_72,
        )?;
        object_73.finish();
    }
    if let Some(var_74) = &input.next_token {
        object.key("nextToken").string(var_74.as_str());
    }
    if let Some(var_75) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_75).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_assessment_templates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAssessmentTemplatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.assessment_target_arns {
        let mut array_77 = object.key("assessmentTargetArns").start_array();
        for item_78 in var_76 {
            {
                array_77.value().string(item_78.as_str());
            }
        }
        array_77.finish();
    }
    if let Some(var_79) = &input.filter {
        #[allow(unused_mut)]
        let mut object_80 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_assessment_template_filter(
            &mut object_80,
            var_79,
        )?;
        object_80.finish();
    }
    if let Some(var_81) = &input.next_token {
        object.key("nextToken").string(var_81.as_str());
    }
    if let Some(var_82) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_82).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_event_subscriptions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListEventSubscriptionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_83) = &input.resource_arn {
        object.key("resourceArn").string(var_83.as_str());
    }
    if let Some(var_84) = &input.next_token {
        object.key("nextToken").string(var_84.as_str());
    }
    if let Some(var_85) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_85).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_exclusions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListExclusionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_86.as_str());
    }
    if let Some(var_87) = &input.next_token {
        object.key("nextToken").string(var_87.as_str());
    }
    if let Some(var_88) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_88).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_findings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListFindingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.assessment_run_arns {
        let mut array_90 = object.key("assessmentRunArns").start_array();
        for item_91 in var_89 {
            {
                array_90.value().string(item_91.as_str());
            }
        }
        array_90.finish();
    }
    if let Some(var_92) = &input.filter {
        #[allow(unused_mut)]
        let mut object_93 = object.key("filter").start_object();
        crate::json_ser::serialize_structure_crate_model_finding_filter(&mut object_93, var_92)?;
        object_93.finish();
    }
    if let Some(var_94) = &input.next_token {
        object.key("nextToken").string(var_94.as_str());
    }
    if let Some(var_95) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_95).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_rules_packages_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRulesPackagesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.next_token {
        object.key("nextToken").string(var_96.as_str());
    }
    if let Some(var_97) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_97).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_98) = &input.resource_arn {
        object.key("resourceArn").string(var_98.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_preview_agents_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PreviewAgentsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.preview_agents_arn {
        object.key("previewAgentsArn").string(var_99.as_str());
    }
    if let Some(var_100) = &input.next_token {
        object.key("nextToken").string(var_100.as_str());
    }
    if let Some(var_101) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_101).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_cross_account_access_role_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterCrossAccountAccessRoleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_102) = &input.role_arn {
        object.key("roleArn").string(var_102.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_attributes_from_findings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveAttributesFromFindingsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_103) = &input.finding_arns {
        let mut array_104 = object.key("findingArns").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    if let Some(var_106) = &input.attribute_keys {
        let mut array_107 = object.key("attributeKeys").start_array();
        for item_108 in var_106 {
            {
                array_107.value().string(item_108.as_str());
            }
        }
        array_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_set_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SetTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.resource_arn {
        object.key("resourceArn").string(var_109.as_str());
    }
    if let Some(var_110) = &input.tags {
        let mut array_111 = object.key("tags").start_array();
        for item_112 in var_110 {
            {
                #[allow(unused_mut)]
                let mut object_113 = array_111.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_113, item_112)?;
                object_113.finish();
            }
        }
        array_111.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_assessment_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartAssessmentRunInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_114) = &input.assessment_template_arn {
        object.key("assessmentTemplateArn").string(var_114.as_str());
    }
    if let Some(var_115) = &input.assessment_run_name {
        object.key("assessmentRunName").string(var_115.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_assessment_run_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopAssessmentRunInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_116) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_116.as_str());
    }
    if let Some(var_117) = &input.stop_action {
        object.key("stopAction").string(var_117.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_subscribe_to_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SubscribeToEventInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_118) = &input.resource_arn {
        object.key("resourceArn").string(var_118.as_str());
    }
    if let Some(var_119) = &input.event {
        object.key("event").string(var_119.as_str());
    }
    if let Some(var_120) = &input.topic_arn {
        object.key("topicArn").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_unsubscribe_from_event_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UnsubscribeFromEventInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.resource_arn {
        object.key("resourceArn").string(var_121.as_str());
    }
    if let Some(var_122) = &input.event {
        object.key("event").string(var_122.as_str());
    }
    if let Some(var_123) = &input.topic_arn {
        object.key("topicArn").string(var_123.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_assessment_target_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAssessmentTargetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_124) = &input.assessment_target_arn {
        object.key("assessmentTargetArn").string(var_124.as_str());
    }
    if let Some(var_125) = &input.assessment_target_name {
        object.key("assessmentTargetName").string(var_125.as_str());
    }
    if let Some(var_126) = &input.resource_group_arn {
        object.key("resourceGroupArn").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_attribute(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Attribute,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.key {
        object.key("key").string(var_127.as_str());
    }
    if let Some(var_128) = &input.value {
        object.key("value").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_group_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceGroupTag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.key {
        object.key("key").string(var_129.as_str());
    }
    if let Some(var_130) = &input.value {
        object.key("value").string(var_130.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_agent_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AgentFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_131) = &input.agent_healths {
        let mut array_132 = object.key("agentHealths").start_array();
        for item_133 in var_131 {
            {
                array_132.value().string(item_133.as_str());
            }
        }
        array_132.finish();
    }
    if let Some(var_134) = &input.agent_health_codes {
        let mut array_135 = object.key("agentHealthCodes").start_array();
        for item_136 in var_134 {
            {
                array_135.value().string(item_136.as_str());
            }
        }
        array_135.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_assessment_run_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssessmentRunFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_137) = &input.name_pattern {
        object.key("namePattern").string(var_137.as_str());
    }
    if let Some(var_138) = &input.states {
        let mut array_139 = object.key("states").start_array();
        for item_140 in var_138 {
            {
                array_139.value().string(item_140.as_str());
            }
        }
        array_139.finish();
    }
    if let Some(var_141) = &input.duration_range {
        #[allow(unused_mut)]
        let mut object_142 = object.key("durationRange").start_object();
        crate::json_ser::serialize_structure_crate_model_duration_range(&mut object_142, var_141)?;
        object_142.finish();
    }
    if let Some(var_143) = &input.rules_package_arns {
        let mut array_144 = object.key("rulesPackageArns").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145.as_str());
            }
        }
        array_144.finish();
    }
    if let Some(var_146) = &input.start_time_range {
        #[allow(unused_mut)]
        let mut object_147 = object.key("startTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_147, var_146)?;
        object_147.finish();
    }
    if let Some(var_148) = &input.completion_time_range {
        #[allow(unused_mut)]
        let mut object_149 = object.key("completionTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_149, var_148)?;
        object_149.finish();
    }
    if let Some(var_150) = &input.state_change_time_range {
        #[allow(unused_mut)]
        let mut object_151 = object.key("stateChangeTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_151, var_150)?;
        object_151.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_assessment_target_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssessmentTargetFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_152) = &input.assessment_target_name_pattern {
        object
            .key("assessmentTargetNamePattern")
            .string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_assessment_template_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AssessmentTemplateFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.name_pattern {
        object.key("namePattern").string(var_153.as_str());
    }
    if let Some(var_154) = &input.duration_range {
        #[allow(unused_mut)]
        let mut object_155 = object.key("durationRange").start_object();
        crate::json_ser::serialize_structure_crate_model_duration_range(&mut object_155, var_154)?;
        object_155.finish();
    }
    if let Some(var_156) = &input.rules_package_arns {
        let mut array_157 = object.key("rulesPackageArns").start_array();
        for item_158 in var_156 {
            {
                array_157.value().string(item_158.as_str());
            }
        }
        array_157.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_finding_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FindingFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_159) = &input.agent_ids {
        let mut array_160 = object.key("agentIds").start_array();
        for item_161 in var_159 {
            {
                array_160.value().string(item_161.as_str());
            }
        }
        array_160.finish();
    }
    if let Some(var_162) = &input.auto_scaling_groups {
        let mut array_163 = object.key("autoScalingGroups").start_array();
        for item_164 in var_162 {
            {
                array_163.value().string(item_164.as_str());
            }
        }
        array_163.finish();
    }
    if let Some(var_165) = &input.rule_names {
        let mut array_166 = object.key("ruleNames").start_array();
        for item_167 in var_165 {
            {
                array_166.value().string(item_167.as_str());
            }
        }
        array_166.finish();
    }
    if let Some(var_168) = &input.severities {
        let mut array_169 = object.key("severities").start_array();
        for item_170 in var_168 {
            {
                array_169.value().string(item_170.as_str());
            }
        }
        array_169.finish();
    }
    if let Some(var_171) = &input.rules_package_arns {
        let mut array_172 = object.key("rulesPackageArns").start_array();
        for item_173 in var_171 {
            {
                array_172.value().string(item_173.as_str());
            }
        }
        array_172.finish();
    }
    if let Some(var_174) = &input.attributes {
        let mut array_175 = object.key("attributes").start_array();
        for item_176 in var_174 {
            {
                #[allow(unused_mut)]
                let mut object_177 = array_175.value().start_object();
                crate::json_ser::serialize_structure_crate_model_attribute(
                    &mut object_177,
                    item_176,
                )?;
                object_177.finish();
            }
        }
        array_175.finish();
    }
    if let Some(var_178) = &input.user_attributes {
        let mut array_179 = object.key("userAttributes").start_array();
        for item_180 in var_178 {
            {
                #[allow(unused_mut)]
                let mut object_181 = array_179.value().start_object();
                crate::json_ser::serialize_structure_crate_model_attribute(
                    &mut object_181,
                    item_180,
                )?;
                object_181.finish();
            }
        }
        array_179.finish();
    }
    if let Some(var_182) = &input.creation_time_range {
        #[allow(unused_mut)]
        let mut object_183 = object.key("creationTimeRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_183, var_182)?;
        object_183.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_184) = &input.key {
        object.key("key").string(var_184.as_str());
    }
    if let Some(var_185) = &input.value {
        object.key("value").string(var_185.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_duration_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DurationRange,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.min_seconds != 0 {
        object.key("minSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_seconds).into()),
        );
    }
    if input.max_seconds != 0 {
        object.key("maxSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timestamp_range(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimestampRange,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_186) = &input.begin_date {
        object
            .key("beginDate")
            .date_time(var_186, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_187) = &input.end_date {
        object
            .key("endDate")
            .date_time(var_187, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}
