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
pub struct RightUpdateDto {
    #[serde(rename = "rightCode")]
    pub right_code: Box<crate::models::PermissionIdentifier>,
    #[serde(rename = "status")]
    pub status: crate::models::RightStatus,
}

impl RightUpdateDto {
    pub fn new(right_code: crate::models::PermissionIdentifier, status: crate::models::RightStatus) -> RightUpdateDto {
        RightUpdateDto {
            right_code: Box::new(right_code),
            status,
        }
    }
}


