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
pub struct ExternalIssueTrackerProjectIn {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "issuePrefix")]
    pub issue_prefix: String,
    #[serde(rename = "linkReplacement")]
    pub link_replacement: String,
    #[serde(rename = "issueListUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue_list_url: Option<Option<String>>,
}

impl ExternalIssueTrackerProjectIn {
    pub fn new(name: String, issue_prefix: String, link_replacement: String) -> ExternalIssueTrackerProjectIn {
        ExternalIssueTrackerProjectIn {
            name,
            issue_prefix,
            link_replacement,
            issue_list_url: None,
        }
    }
}

