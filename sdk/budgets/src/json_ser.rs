// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.budget {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Budget").start_object();
        crate::json_ser::serialize_structure_crate_model_budget(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.notifications_with_subscribers {
        let mut array_5 = object.key("NotificationsWithSubscribers").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_notification_with_subscribers(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.account_id {
        object.key("AccountId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.budget_name {
        object.key("BudgetName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.notification_type {
        object.key("NotificationType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.action_type {
        object.key("ActionType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.action_threshold {
        #[allow(unused_mut)]
        let mut object_13 = object.key("ActionThreshold").start_object();
        crate::json_ser::serialize_structure_crate_model_action_threshold(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.definition {
        #[allow(unused_mut)]
        let mut object_15 = object.key("Definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_16.as_str());
    }
    if let Some(var_17) = &input.approval_model {
        object.key("ApprovalModel").string(var_17.as_str());
    }
    if let Some(var_18) = &input.subscribers {
        let mut array_19 = object.key("Subscribers").start_array();
        for item_20 in var_18 {
            {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_21,
                    item_20,
                )?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNotificationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.account_id {
        object.key("AccountId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.budget_name {
        object.key("BudgetName").string(var_23.as_str());
    }
    if let Some(var_24) = &input.notification {
        #[allow(unused_mut)]
        let mut object_25 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.subscribers {
        let mut array_27 = object.key("Subscribers").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSubscriberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.account_id {
        object.key("AccountId").string(var_30.as_str());
    }
    if let Some(var_31) = &input.budget_name {
        object.key("BudgetName").string(var_31.as_str());
    }
    if let Some(var_32) = &input.notification {
        #[allow(unused_mut)]
        let mut object_33 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_33, var_32)?;
        object_33.finish();
    }
    if let Some(var_34) = &input.subscriber {
        #[allow(unused_mut)]
        let mut object_35 = object.key("Subscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_35, var_34)?;
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.account_id {
        object.key("AccountId").string(var_36.as_str());
    }
    if let Some(var_37) = &input.budget_name {
        object.key("BudgetName").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.account_id {
        object.key("AccountId").string(var_38.as_str());
    }
    if let Some(var_39) = &input.budget_name {
        object.key("BudgetName").string(var_39.as_str());
    }
    if let Some(var_40) = &input.action_id {
        object.key("ActionId").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteNotificationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.account_id {
        object.key("AccountId").string(var_41.as_str());
    }
    if let Some(var_42) = &input.budget_name {
        object.key("BudgetName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.notification {
        #[allow(unused_mut)]
        let mut object_44 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_44, var_43)?;
        object_44.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSubscriberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_45) = &input.account_id {
        object.key("AccountId").string(var_45.as_str());
    }
    if let Some(var_46) = &input.budget_name {
        object.key("BudgetName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.notification {
        #[allow(unused_mut)]
        let mut object_48 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.subscriber {
        #[allow(unused_mut)]
        let mut object_50 = object.key("Subscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_50, var_49)?;
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.account_id {
        object.key("AccountId").string(var_51.as_str());
    }
    if let Some(var_52) = &input.budget_name {
        object.key("BudgetName").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.account_id {
        object.key("AccountId").string(var_53.as_str());
    }
    if let Some(var_54) = &input.budget_name {
        object.key("BudgetName").string(var_54.as_str());
    }
    if let Some(var_55) = &input.action_id {
        object.key("ActionId").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_action_histories_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionHistoriesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_56) = &input.account_id {
        object.key("AccountId").string(var_56.as_str());
    }
    if let Some(var_57) = &input.budget_name {
        object.key("BudgetName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.action_id {
        object.key("ActionId").string(var_58.as_str());
    }
    if let Some(var_59) = &input.time_period {
        #[allow(unused_mut)]
        let mut object_60 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_60, var_59)?;
        object_60.finish();
    }
    if let Some(var_61) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.next_token {
        object.key("NextToken").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_actions_for_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionsForAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.account_id {
        object.key("AccountId").string(var_63.as_str());
    }
    if let Some(var_64) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_64).into()),
        );
    }
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_actions_for_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionsForBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.account_id {
        object.key("AccountId").string(var_66.as_str());
    }
    if let Some(var_67) = &input.budget_name {
        object.key("BudgetName").string(var_67.as_str());
    }
    if let Some(var_68) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.next_token {
        object.key("NextToken").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_notifications_for_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetNotificationsForAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.account_id {
        object.key("AccountId").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_71).into()),
        );
    }
    if let Some(var_72) = &input.next_token {
        object.key("NextToken").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_performance_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetPerformanceHistoryInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.account_id {
        object.key("AccountId").string(var_73.as_str());
    }
    if let Some(var_74) = &input.budget_name {
        object.key("BudgetName").string(var_74.as_str());
    }
    if let Some(var_75) = &input.time_period {
        #[allow(unused_mut)]
        let mut object_76 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_76, var_75)?;
        object_76.finish();
    }
    if let Some(var_77) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    if let Some(var_78) = &input.next_token {
        object.key("NextToken").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budgets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.account_id {
        object.key("AccountId").string(var_79.as_str());
    }
    if let Some(var_80) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    if let Some(var_81) = &input.next_token {
        object.key("NextToken").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_notifications_for_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeNotificationsForBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.account_id {
        object.key("AccountId").string(var_82.as_str());
    }
    if let Some(var_83) = &input.budget_name {
        object.key("BudgetName").string(var_83.as_str());
    }
    if let Some(var_84) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    if let Some(var_85) = &input.next_token {
        object.key("NextToken").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_subscribers_for_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSubscribersForNotificationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.account_id {
        object.key("AccountId").string(var_86.as_str());
    }
    if let Some(var_87) = &input.budget_name {
        object.key("BudgetName").string(var_87.as_str());
    }
    if let Some(var_88) = &input.notification {
        #[allow(unused_mut)]
        let mut object_89 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_89, var_88)?;
        object_89.finish();
    }
    if let Some(var_90) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    if let Some(var_91) = &input.next_token {
        object.key("NextToken").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_execute_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExecuteBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.account_id {
        object.key("AccountId").string(var_92.as_str());
    }
    if let Some(var_93) = &input.budget_name {
        object.key("BudgetName").string(var_93.as_str());
    }
    if let Some(var_94) = &input.action_id {
        object.key("ActionId").string(var_94.as_str());
    }
    if let Some(var_95) = &input.execution_type {
        object.key("ExecutionType").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBudgetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.account_id {
        object.key("AccountId").string(var_96.as_str());
    }
    if let Some(var_97) = &input.new_budget {
        #[allow(unused_mut)]
        let mut object_98 = object.key("NewBudget").start_object();
        crate::json_ser::serialize_structure_crate_model_budget(&mut object_98, var_97)?;
        object_98.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.account_id {
        object.key("AccountId").string(var_99.as_str());
    }
    if let Some(var_100) = &input.budget_name {
        object.key("BudgetName").string(var_100.as_str());
    }
    if let Some(var_101) = &input.action_id {
        object.key("ActionId").string(var_101.as_str());
    }
    if let Some(var_102) = &input.notification_type {
        object.key("NotificationType").string(var_102.as_str());
    }
    if let Some(var_103) = &input.action_threshold {
        #[allow(unused_mut)]
        let mut object_104 = object.key("ActionThreshold").start_object();
        crate::json_ser::serialize_structure_crate_model_action_threshold(
            &mut object_104,
            var_103,
        )?;
        object_104.finish();
    }
    if let Some(var_105) = &input.definition {
        #[allow(unused_mut)]
        let mut object_106 = object.key("Definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition(&mut object_106, var_105)?;
        object_106.finish();
    }
    if let Some(var_107) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_107.as_str());
    }
    if let Some(var_108) = &input.approval_model {
        object.key("ApprovalModel").string(var_108.as_str());
    }
    if let Some(var_109) = &input.subscribers {
        let mut array_110 = object.key("Subscribers").start_array();
        for item_111 in var_109 {
            {
                #[allow(unused_mut)]
                let mut object_112 = array_110.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_112,
                    item_111,
                )?;
                object_112.finish();
            }
        }
        array_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNotificationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.account_id {
        object.key("AccountId").string(var_113.as_str());
    }
    if let Some(var_114) = &input.budget_name {
        object.key("BudgetName").string(var_114.as_str());
    }
    if let Some(var_115) = &input.old_notification {
        #[allow(unused_mut)]
        let mut object_116 = object.key("OldNotification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_116, var_115)?;
        object_116.finish();
    }
    if let Some(var_117) = &input.new_notification {
        #[allow(unused_mut)]
        let mut object_118 = object.key("NewNotification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_118, var_117)?;
        object_118.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSubscriberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.account_id {
        object.key("AccountId").string(var_119.as_str());
    }
    if let Some(var_120) = &input.budget_name {
        object.key("BudgetName").string(var_120.as_str());
    }
    if let Some(var_121) = &input.notification {
        #[allow(unused_mut)]
        let mut object_122 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_122, var_121)?;
        object_122.finish();
    }
    if let Some(var_123) = &input.old_subscriber {
        #[allow(unused_mut)]
        let mut object_124 = object.key("OldSubscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_124, var_123)?;
        object_124.finish();
    }
    if let Some(var_125) = &input.new_subscriber {
        #[allow(unused_mut)]
        let mut object_126 = object.key("NewSubscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_126, var_125)?;
        object_126.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_budget(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Budget,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.budget_name {
        object.key("BudgetName").string(var_127.as_str());
    }
    if let Some(var_128) = &input.budget_limit {
        #[allow(unused_mut)]
        let mut object_129 = object.key("BudgetLimit").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_129, var_128)?;
        object_129.finish();
    }
    if let Some(var_130) = &input.planned_budget_limits {
        #[allow(unused_mut)]
        let mut object_131 = object.key("PlannedBudgetLimits").start_object();
        for (key_132, value_133) in var_130 {
            {
                #[allow(unused_mut)]
                let mut object_134 = object_131.key(key_132.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_spend(&mut object_134, value_133)?;
                object_134.finish();
            }
        }
        object_131.finish();
    }
    if let Some(var_135) = &input.cost_filters {
        #[allow(unused_mut)]
        let mut object_136 = object.key("CostFilters").start_object();
        for (key_137, value_138) in var_135 {
            {
                let mut array_139 = object_136.key(key_137.as_str()).start_array();
                for item_140 in value_138 {
                    {
                        array_139.value().string(item_140.as_str());
                    }
                }
                array_139.finish();
            }
        }
        object_136.finish();
    }
    if let Some(var_141) = &input.cost_types {
        #[allow(unused_mut)]
        let mut object_142 = object.key("CostTypes").start_object();
        crate::json_ser::serialize_structure_crate_model_cost_types(&mut object_142, var_141)?;
        object_142.finish();
    }
    if let Some(var_143) = &input.time_unit {
        object.key("TimeUnit").string(var_143.as_str());
    }
    if let Some(var_144) = &input.time_period {
        #[allow(unused_mut)]
        let mut object_145 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_145, var_144)?;
        object_145.finish();
    }
    if let Some(var_146) = &input.calculated_spend {
        #[allow(unused_mut)]
        let mut object_147 = object.key("CalculatedSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_calculated_spend(
            &mut object_147,
            var_146,
        )?;
        object_147.finish();
    }
    if let Some(var_148) = &input.budget_type {
        object.key("BudgetType").string(var_148.as_str());
    }
    if let Some(var_149) = &input.last_updated_time {
        object
            .key("LastUpdatedTime")
            .date_time(var_149, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_150) = &input.auto_adjust_data {
        #[allow(unused_mut)]
        let mut object_151 = object.key("AutoAdjustData").start_object();
        crate::json_ser::serialize_structure_crate_model_auto_adjust_data(
            &mut object_151,
            var_150,
        )?;
        object_151.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_with_subscribers(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationWithSubscribers,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_152) = &input.notification {
        #[allow(unused_mut)]
        let mut object_153 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_153, var_152)?;
        object_153.finish();
    }
    if let Some(var_154) = &input.subscribers {
        let mut array_155 = object.key("Subscribers").start_array();
        for item_156 in var_154 {
            {
                #[allow(unused_mut)]
                let mut object_157 = array_155.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_157,
                    item_156,
                )?;
                object_157.finish();
            }
        }
        array_155.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_action_threshold(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActionThreshold,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("ActionThresholdValue").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.action_threshold_value).into()),
        );
    }
    if let Some(var_158) = &input.action_threshold_type {
        object.key("ActionThresholdType").string(var_158.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Definition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_159) = &input.iam_action_definition {
        #[allow(unused_mut)]
        let mut object_160 = object.key("IamActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_iam_action_definition(
            &mut object_160,
            var_159,
        )?;
        object_160.finish();
    }
    if let Some(var_161) = &input.scp_action_definition {
        #[allow(unused_mut)]
        let mut object_162 = object.key("ScpActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_scp_action_definition(
            &mut object_162,
            var_161,
        )?;
        object_162.finish();
    }
    if let Some(var_163) = &input.ssm_action_definition {
        #[allow(unused_mut)]
        let mut object_164 = object.key("SsmActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_ssm_action_definition(
            &mut object_164,
            var_163,
        )?;
        object_164.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_subscriber(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Subscriber,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_165) = &input.subscription_type {
        object.key("SubscriptionType").string(var_165.as_str());
    }
    if let Some(var_166) = &input.address {
        object.key("Address").string(var_166.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Notification,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_167) = &input.notification_type {
        object.key("NotificationType").string(var_167.as_str());
    }
    if let Some(var_168) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_168.as_str());
    }
    {
        object.key("Threshold").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.threshold).into()),
        );
    }
    if let Some(var_169) = &input.threshold_type {
        object.key("ThresholdType").string(var_169.as_str());
    }
    if let Some(var_170) = &input.notification_state {
        object.key("NotificationState").string(var_170.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_time_period(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimePeriod,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_171) = &input.start {
        object
            .key("Start")
            .date_time(var_171, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_172) = &input.end {
        object
            .key("End")
            .date_time(var_172, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spend(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Spend,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_173) = &input.amount {
        object.key("Amount").string(var_173.as_str());
    }
    if let Some(var_174) = &input.unit {
        object.key("Unit").string(var_174.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cost_types(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CostTypes,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_175) = &input.include_tax {
        object.key("IncludeTax").boolean(*var_175);
    }
    if let Some(var_176) = &input.include_subscription {
        object.key("IncludeSubscription").boolean(*var_176);
    }
    if let Some(var_177) = &input.use_blended {
        object.key("UseBlended").boolean(*var_177);
    }
    if let Some(var_178) = &input.include_refund {
        object.key("IncludeRefund").boolean(*var_178);
    }
    if let Some(var_179) = &input.include_credit {
        object.key("IncludeCredit").boolean(*var_179);
    }
    if let Some(var_180) = &input.include_upfront {
        object.key("IncludeUpfront").boolean(*var_180);
    }
    if let Some(var_181) = &input.include_recurring {
        object.key("IncludeRecurring").boolean(*var_181);
    }
    if let Some(var_182) = &input.include_other_subscription {
        object.key("IncludeOtherSubscription").boolean(*var_182);
    }
    if let Some(var_183) = &input.include_support {
        object.key("IncludeSupport").boolean(*var_183);
    }
    if let Some(var_184) = &input.include_discount {
        object.key("IncludeDiscount").boolean(*var_184);
    }
    if let Some(var_185) = &input.use_amortized {
        object.key("UseAmortized").boolean(*var_185);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_calculated_spend(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculatedSpend,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_186) = &input.actual_spend {
        #[allow(unused_mut)]
        let mut object_187 = object.key("ActualSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_187, var_186)?;
        object_187.finish();
    }
    if let Some(var_188) = &input.forecasted_spend {
        #[allow(unused_mut)]
        let mut object_189 = object.key("ForecastedSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_189, var_188)?;
        object_189.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_auto_adjust_data(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AutoAdjustData,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_190) = &input.auto_adjust_type {
        object.key("AutoAdjustType").string(var_190.as_str());
    }
    if let Some(var_191) = &input.historical_options {
        #[allow(unused_mut)]
        let mut object_192 = object.key("HistoricalOptions").start_object();
        crate::json_ser::serialize_structure_crate_model_historical_options(
            &mut object_192,
            var_191,
        )?;
        object_192.finish();
    }
    if let Some(var_193) = &input.last_auto_adjust_time {
        object
            .key("LastAutoAdjustTime")
            .date_time(var_193, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_iam_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IamActionDefinition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_194) = &input.policy_arn {
        object.key("PolicyArn").string(var_194.as_str());
    }
    if let Some(var_195) = &input.roles {
        let mut array_196 = object.key("Roles").start_array();
        for item_197 in var_195 {
            {
                array_196.value().string(item_197.as_str());
            }
        }
        array_196.finish();
    }
    if let Some(var_198) = &input.groups {
        let mut array_199 = object.key("Groups").start_array();
        for item_200 in var_198 {
            {
                array_199.value().string(item_200.as_str());
            }
        }
        array_199.finish();
    }
    if let Some(var_201) = &input.users {
        let mut array_202 = object.key("Users").start_array();
        for item_203 in var_201 {
            {
                array_202.value().string(item_203.as_str());
            }
        }
        array_202.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_scp_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScpActionDefinition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_204) = &input.policy_id {
        object.key("PolicyId").string(var_204.as_str());
    }
    if let Some(var_205) = &input.target_ids {
        let mut array_206 = object.key("TargetIds").start_array();
        for item_207 in var_205 {
            {
                array_206.value().string(item_207.as_str());
            }
        }
        array_206.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ssm_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SsmActionDefinition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_208) = &input.action_sub_type {
        object.key("ActionSubType").string(var_208.as_str());
    }
    if let Some(var_209) = &input.region {
        object.key("Region").string(var_209.as_str());
    }
    if let Some(var_210) = &input.instance_ids {
        let mut array_211 = object.key("InstanceIds").start_array();
        for item_212 in var_210 {
            {
                array_211.value().string(item_212.as_str());
            }
        }
        array_211.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_historical_options(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HistoricalOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_213) = &input.budget_adjustment_period {
        object.key("BudgetAdjustmentPeriod").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_213).into()),
        );
    }
    if let Some(var_214) = &input.look_back_available_periods {
        object.key("LookBackAvailablePeriods").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_214).into()),
        );
    }
    Ok(())
}
