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
pub struct EsDefaultProfileLoginDetails {
    #[serde(rename = "login", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub login: Option<Option<String>>,
    #[serde(rename = "firstName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    #[serde(rename = "lastName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "avatarUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<Option<String>>,
}

impl EsDefaultProfileLoginDetails {
    pub fn new(email_verified: bool) -> EsDefaultProfileLoginDetails {
        EsDefaultProfileLoginDetails {
            login: None,
            first_name: None,
            last_name: None,
            email: None,
            email_verified,
            avatar_url: None,
        }
    }
}


