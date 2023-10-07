/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreateExternalIssueProjectError {
    #[serde(rename = "DUPLICATE_ISSUE_PREFIX")]
    DuplicateIssuePrefix,

}

impl ToString for CreateExternalIssueProjectError {
    fn to_string(&self) -> String {
        match self {
            Self::DuplicateIssuePrefix => String::from("DUPLICATE_ISSUE_PREFIX"),
        }
    }
}

impl Default for CreateExternalIssueProjectError {
    fn default() -> CreateExternalIssueProjectError {
        Self::DuplicateIssuePrefix
    }
}




