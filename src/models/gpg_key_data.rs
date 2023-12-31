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
pub struct GpgKeyData {
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<crate::models::GpgKeyDataJbsKeyUserId>,
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "added")]
    pub added: String,
    #[serde(rename = "expires", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    #[serde(rename = "revoked")]
    pub revoked: bool,
    #[serde(rename = "revokedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<Option<String>>,
    #[serde(rename = "revokeComment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revoke_comment: Option<Option<String>>,
}

impl GpgKeyData {
    pub fn new(fingerprint: String, public_key: String, user_ids: Vec<crate::models::GpgKeyDataJbsKeyUserId>, comment: String, created: String, added: String, revoked: bool) -> GpgKeyData {
        GpgKeyData {
            fingerprint,
            public_key,
            user_ids,
            comment,
            created,
            added,
            expires: None,
            revoked,
            revoked_at: None,
            revoke_comment: None,
        }
    }
}


