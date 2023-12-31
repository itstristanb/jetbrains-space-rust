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
pub struct UnfurlDetailsFolder {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: bool,
    #[serde(rename = "folder", skip_serializing_if = "Option::is_none")]
    pub folder: Option<Box<crate::models::DocumentFolder>>,
}

impl UnfurlDetailsFolder {
    pub fn new(name: String, root: bool) -> UnfurlDetailsFolder {
        UnfurlDetailsFolder {
            name,
            root,
            folder: None,
        }
    }
}


