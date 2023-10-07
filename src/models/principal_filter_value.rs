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
pub struct PrincipalFilterValue {
    #[serde(rename = "principalIn", skip_serializing_if = "Option::is_none")]
    pub principal_in: Option<Box<crate::models::PrincipalIn>>,
}

impl PrincipalFilterValue {
    pub fn new() -> PrincipalFilterValue {
        PrincipalFilterValue {
            principal_in: None,
        }
    }
}

