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
pub struct AdministrationUserAgreementPatchRequest {
    #[serde(rename = "newContent")]
    pub new_content: String,
    #[serde(rename = "invalidate")]
    pub invalidate: bool,
}

impl AdministrationUserAgreementPatchRequest {
    pub fn new(new_content: String, invalidate: bool) -> AdministrationUserAgreementPatchRequest {
        AdministrationUserAgreementPatchRequest {
            new_content,
            invalidate,
        }
    }
}

