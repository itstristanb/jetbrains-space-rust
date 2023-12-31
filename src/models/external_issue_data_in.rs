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
pub struct ExternalIssueDataIn {
    #[serde(rename = "issueId")]
    pub issue_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    #[serde(rename = "summary", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub summary: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<crate::models::ExternalIssueField>>>,
    #[serde(rename = "createRequestId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub create_request_id: Option<Option<String>>,
}

impl ExternalIssueDataIn {
    pub fn new(issue_id: String, url: String) -> ExternalIssueDataIn {
        ExternalIssueDataIn {
            issue_id,
            url,
            status: None,
            summary: None,
            fields: None,
            create_request_id: None,
        }
    }
}


