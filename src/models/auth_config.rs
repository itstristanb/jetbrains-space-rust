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
pub struct AuthConfig {
    #[serde(rename = "dontRememberMeTtl")]
    pub dont_remember_me_ttl: i32,
    #[serde(rename = "adminRememberMeTtl")]
    pub admin_remember_me_ttl: i32,
    #[serde(rename = "userRememberMeTtl")]
    pub user_remember_me_ttl: i32,
}

impl AuthConfig {
    pub fn new(dont_remember_me_ttl: i32, admin_remember_me_ttl: i32, user_remember_me_ttl: i32) -> AuthConfig {
        AuthConfig {
            dont_remember_me_ttl,
            admin_remember_me_ttl,
            user_remember_me_ttl,
        }
    }
}

