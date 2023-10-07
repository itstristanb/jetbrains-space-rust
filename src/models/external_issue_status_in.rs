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
pub struct ExternalIssueStatusIn {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "oldName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub old_name: Option<Option<String>>,
}

impl ExternalIssueStatusIn {
    pub fn new(name: String) -> ExternalIssueStatusIn {
        ExternalIssueStatusIn {
            name,
            old_name: None,
        }
    }
}


