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
pub struct GitFileChange {
    #[serde(rename = "changeType")]
    pub change_type: crate::models::GitCommitChangeType,
    #[serde(rename = "old", skip_serializing_if = "Option::is_none")]
    pub old: Option<Box<crate::models::GitFile>>,
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<Box<crate::models::GitFile>>,
}

impl GitFileChange {
    pub fn new(change_type: crate::models::GitCommitChangeType) -> GitFileChange {
        GitFileChange {
            change_type,
            old: None,
            new: None,
        }
    }
}


