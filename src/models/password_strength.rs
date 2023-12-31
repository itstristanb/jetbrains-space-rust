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
pub enum PasswordStrength {
    #[serde(rename = "NO_POLICY")]
    NoPolicy,
    #[serde(rename = "FAIR")]
    Fair,
    #[serde(rename = "GOOD")]
    Good,
    #[serde(rename = "STRONG")]
    Strong,

}

impl ToString for PasswordStrength {
    fn to_string(&self) -> String {
        match self {
            Self::NoPolicy => String::from("NO_POLICY"),
            Self::Fair => String::from("FAIR"),
            Self::Good => String::from("GOOD"),
            Self::Strong => String::from("STRONG"),
        }
    }
}

impl Default for PasswordStrength {
    fn default() -> PasswordStrength {
        Self::NoPolicy
    }
}




