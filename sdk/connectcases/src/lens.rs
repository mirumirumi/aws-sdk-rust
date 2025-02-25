// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_case_output_next_token(
    input: &crate::output::GetCaseOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_cases_for_contact_output_next_token(
    input: &crate::output::ListCasesForContactOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_domains_output_next_token(
    input: &crate::output::ListDomainsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_field_options_output_next_token(
    input: &crate::output::ListFieldOptionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_fields_output_next_token(
    input: &crate::output::ListFieldsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_layouts_output_next_token(
    input: &crate::output::ListLayoutsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_templates_output_next_token(
    input: &crate::output::ListTemplatesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_search_cases_output_next_token(
    input: &crate::output::SearchCasesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_search_related_items_output_next_token(
    input: &crate::output::SearchRelatedItemsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_search_cases_output_cases(
    input: crate::output::SearchCasesOutput,
) -> std::option::Option<std::vec::Vec<std::option::Option<crate::model::SearchCasesResponseItem>>>
{
    let input = match input.cases {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_search_related_items_output_related_items(
    input: crate::output::SearchRelatedItemsOutput,
) -> std::option::Option<
    std::vec::Vec<std::option::Option<crate::model::SearchRelatedItemsResponseItem>>,
> {
    let input = match input.related_items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
