/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMenuItemUiExtensionApi {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "menuItemUniqueCode")]
    pub menu_item_unique_code: String,
    #[serde(rename = "visibilityFilters")]
    pub visibility_filters: Vec<serde_json::Value>,
}

impl IssueMenuItemUiExtensionApi {
    pub fn new(display_name: String, menu_item_unique_code: String, visibility_filters: Vec<serde_json::Value>) -> IssueMenuItemUiExtensionApi {
        IssueMenuItemUiExtensionApi {
            display_name,
            description: None,
            menu_item_unique_code,
            visibility_filters,
        }
    }
}


