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
pub struct BranchInfo {
    #[serde(rename = "head")]
    pub head: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
}

impl BranchInfo {
    pub fn new(head: String, r#ref: String) -> BranchInfo {
        BranchInfo {
            head,
            r#ref,
        }
    }
}

