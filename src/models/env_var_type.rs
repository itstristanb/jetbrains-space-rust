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
pub enum EnvVarType {
    #[serde(rename = "PLAIN_TEXT")]
    PlainText,
    #[serde(rename = "PROJECT_PARAMETER")]
    ProjectParameter,
    #[serde(rename = "PROJECT_SECRET")]
    ProjectSecret,

}

impl ToString for EnvVarType {
    fn to_string(&self) -> String {
        match self {
            Self::PlainText => String::from("PLAIN_TEXT"),
            Self::ProjectParameter => String::from("PROJECT_PARAMETER"),
            Self::ProjectSecret => String::from("PROJECT_SECRET"),
        }
    }
}

impl Default for EnvVarType {
    fn default() -> EnvVarType {
        Self::PlainText
    }
}




