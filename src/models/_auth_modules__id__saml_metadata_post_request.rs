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
pub struct AuthModulesIdSamlMetadataPostRequest {
    #[serde(rename = "idpUrl")]
    pub idp_url: String,
    #[serde(rename = "idpEntityId")]
    pub idp_entity_id: String,
    #[serde(rename = "idpCertificateSHA256")]
    pub idp_certificate_sha256: String,
    #[serde(rename = "spEntityId")]
    pub sp_entity_id: String,
    #[serde(rename = "sslKeystore", skip_serializing_if = "Option::is_none")]
    pub ssl_keystore: Option<Box<crate::models::SslKeystore>>,
    #[serde(rename = "contactProfileId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contact_profile_id: Option<Option<String>>,
}

impl AuthModulesIdSamlMetadataPostRequest {
    pub fn new(idp_url: String, idp_entity_id: String, idp_certificate_sha256: String, sp_entity_id: String) -> AuthModulesIdSamlMetadataPostRequest {
        AuthModulesIdSamlMetadataPostRequest {
            idp_url,
            idp_entity_id,
            idp_certificate_sha256,
            sp_entity_id,
            ssl_keystore: None,
            contact_profile_id: None,
        }
    }
}


