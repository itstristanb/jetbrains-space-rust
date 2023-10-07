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
pub struct CommitMessageUnfurlsRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "unfurls")]
    pub unfurls: Vec<crate::models::Unfurl>,
}

impl CommitMessageUnfurlsRecord {
    pub fn new(id: String, unfurls: Vec<crate::models::Unfurl>) -> CommitMessageUnfurlsRecord {
        CommitMessageUnfurlsRecord {
            id,
            unfurls,
        }
    }
}

