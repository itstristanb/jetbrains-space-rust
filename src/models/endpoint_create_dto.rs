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
pub struct EndpointCreateDto {
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "sslVerification")]
    pub ssl_verification: bool,
}

impl EndpointCreateDto {
    pub fn new(ssl_verification: bool) -> EndpointCreateDto {
        EndpointCreateDto {
            url: None,
            ssl_verification,
        }
    }
}


