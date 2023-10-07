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
pub struct PackageVersionInfo {
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::PackageType>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
}

impl PackageVersionInfo {
    pub fn new(r#type: crate::models::PackageType, repository: String, name: String, version: String) -> PackageVersionInfo {
        PackageVersionInfo {
            r#type: Box::new(r#type),
            repository,
            name,
            version,
            tags: None,
        }
    }
}


