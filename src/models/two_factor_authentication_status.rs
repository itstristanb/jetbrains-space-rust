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
pub enum TwoFactorAuthenticationStatus {
    #[serde(rename = "NOT_SETUP")]
    NotSetup,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "ACTIVE")]
    Active,

}

impl ToString for TwoFactorAuthenticationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NotSetup => String::from("NOT_SETUP"),
            Self::Inactive => String::from("INACTIVE"),
            Self::Active => String::from("ACTIVE"),
        }
    }
}

impl Default for TwoFactorAuthenticationStatus {
    fn default() -> TwoFactorAuthenticationStatus {
        Self::NotSetup
    }
}



