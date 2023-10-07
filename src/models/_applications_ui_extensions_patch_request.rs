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
pub struct ApplicationsUiExtensionsPatchRequest {
    #[serde(rename = "contextIdentifier")]
    pub context_identifier: Box<crate::models::PermissionContextIdentifier>,
    #[serde(rename = "extensions")]
    pub extensions: Vec<crate::models::AppUiExtensionIn>,
}

impl ApplicationsUiExtensionsPatchRequest {
    pub fn new(context_identifier: crate::models::PermissionContextIdentifier, extensions: Vec<crate::models::AppUiExtensionIn>) -> ApplicationsUiExtensionsPatchRequest {
        ApplicationsUiExtensionsPatchRequest {
            context_identifier: Box::new(context_identifier),
            extensions,
        }
    }
}

