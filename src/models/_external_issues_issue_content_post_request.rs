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
pub struct ExternalIssuesIssueContentPostRequest {
    #[serde(rename = "issuePrefix")]
    pub issue_prefix: String,
    #[serde(rename = "issues")]
    pub issues: Vec<crate::models::ExternalIssueDataIn>,
}

impl ExternalIssuesIssueContentPostRequest {
    pub fn new(issue_prefix: String, issues: Vec<crate::models::ExternalIssueDataIn>) -> ExternalIssuesIssueContentPostRequest {
        ExternalIssuesIssueContentPostRequest {
            issue_prefix,
            issues,
        }
    }
}


