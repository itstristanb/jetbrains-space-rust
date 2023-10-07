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
pub struct ExternalIssuesMarkIssuesAsDeletedPostRequest {
    #[serde(rename = "issuePrefix")]
    pub issue_prefix: String,
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<String>,
}

impl ExternalIssuesMarkIssuesAsDeletedPostRequest {
    pub fn new(issue_prefix: String, issue_ids: Vec<String>) -> ExternalIssuesMarkIssuesAsDeletedPostRequest {
        ExternalIssuesMarkIssuesAsDeletedPostRequest {
            issue_prefix,
            issue_ids,
        }
    }
}

