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
pub struct ChangeInReview {
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "change")]
    pub change: Box<crate::models::GitCommitChange>,
    #[serde(rename = "read")]
    pub read: bool,
}

impl ChangeInReview {
    pub fn new(repository: String, change: crate::models::GitCommitChange, read: bool) -> ChangeInReview {
        ChangeInReview {
            repository,
            change: Box::new(change),
            read,
        }
    }
}


