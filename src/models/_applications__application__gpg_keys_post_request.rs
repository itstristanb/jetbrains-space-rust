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
pub struct ApplicationsApplicationGpgKeysPostRequest {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ApplicationsApplicationGpgKeysPostRequest {
    pub fn new(public_key: String) -> ApplicationsApplicationGpgKeysPostRequest {
        ApplicationsApplicationGpgKeysPostRequest {
            public_key,
            comment: None,
        }
    }
}


