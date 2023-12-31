/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndpointAppLevelAuthUpdateType {
    #[serde(rename = "PublicKeySignature")]
    PublicKeySignature,
    #[serde(rename = "SigningKey")]
    SigningKey,
    #[serde(rename = "Bearer")]
    Bearer,
    #[serde(rename = "Basic")]
    Basic,

}

impl ToString for EndpointAppLevelAuthUpdateType {
    fn to_string(&self) -> String {
        match self {
            Self::PublicKeySignature => String::from("PublicKeySignature"),
            Self::SigningKey => String::from("SigningKey"),
            Self::Bearer => String::from("Bearer"),
            Self::Basic => String::from("Basic"),
        }
    }
}

impl Default for EndpointAppLevelAuthUpdateType {
    fn default() -> EndpointAppLevelAuthUpdateType {
        Self::PublicKeySignature
    }
}




