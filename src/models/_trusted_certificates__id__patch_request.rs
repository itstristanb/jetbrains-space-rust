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
pub struct TrustedCertificatesIdPatchRequest {
    #[serde(rename = "alias", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alias: Option<Option<String>>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<String>>,
    #[serde(rename = "archived", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub archived: Option<Option<bool>>,
}

impl TrustedCertificatesIdPatchRequest {
    pub fn new() -> TrustedCertificatesIdPatchRequest {
        TrustedCertificatesIdPatchRequest {
            alias: None,
            data: None,
            archived: None,
        }
    }
}


