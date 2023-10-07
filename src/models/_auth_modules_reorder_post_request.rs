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
pub struct AuthModulesReorderPostRequest {
    #[serde(rename = "order")]
    pub order: Vec<String>,
}

impl AuthModulesReorderPostRequest {
    pub fn new(order: Vec<String>) -> AuthModulesReorderPostRequest {
        AuthModulesReorderPostRequest {
            order,
        }
    }
}


