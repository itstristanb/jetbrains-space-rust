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
pub struct ChecklistsChecklistItemsPostRequest {
    #[serde(rename = "parentItem", skip_serializing_if = "Option::is_none")]
    pub parent_item: Option<Box<crate::models::PlanItemIdentifierJbsId>>,
    #[serde(rename = "itemText")]
    pub item_text: String,
}

impl ChecklistsChecklistItemsPostRequest {
    pub fn new(item_text: String) -> ChecklistsChecklistItemsPostRequest {
        ChecklistsChecklistItemsPostRequest {
            parent_item: None,
            item_text,
        }
    }
}


