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
pub struct IssueChecklistsChangedDetails {
    #[serde(rename = "addedChecklists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub added_checklists: Option<Option<Vec<crate::models::Checklist>>>,
    #[serde(rename = "removedChecklists", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub removed_checklists: Option<Option<Vec<crate::models::Checklist>>>,
}

impl IssueChecklistsChangedDetails {
    pub fn new() -> IssueChecklistsChangedDetails {
        IssueChecklistsChangedDetails {
            added_checklists: None,
            removed_checklists: None,
        }
    }
}


