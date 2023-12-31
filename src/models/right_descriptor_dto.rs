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
pub struct RightDescriptorDto {
    #[serde(rename = "rightCode")]
    pub right_code: Box<crate::models::PermissionIdentifier>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "group")]
    pub group: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[serde(rename = "dependencies")]
    pub dependencies: Vec<String>,
}

impl RightDescriptorDto {
    pub fn new(right_code: crate::models::PermissionIdentifier, name: String, group: String, description: String, target_name: String, dependencies: Vec<String>) -> RightDescriptorDto {
        RightDescriptorDto {
            right_code: Box::new(right_code),
            name,
            group,
            description,
            target_name,
            dependencies,
        }
    }
}


