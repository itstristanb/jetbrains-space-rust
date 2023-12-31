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
pub struct MergeRequestCodeIssueStatsForLevel {
    #[serde(rename = "level")]
    pub level: crate::models::CodeIssueLevel,
    #[serde(rename = "count")]
    pub count: i32,
}

impl MergeRequestCodeIssueStatsForLevel {
    pub fn new(level: crate::models::CodeIssueLevel, count: i32) -> MergeRequestCodeIssueStatsForLevel {
        MergeRequestCodeIssueStatsForLevel {
            level,
            count,
        }
    }
}


