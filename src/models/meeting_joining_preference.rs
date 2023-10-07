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
pub enum MeetingJoiningPreference {
    #[serde(rename = "NOBODY")]
    Nobody,
    #[serde(rename = "EVERYONE")]
    Everyone,

}

impl ToString for MeetingJoiningPreference {
    fn to_string(&self) -> String {
        match self {
            Self::Nobody => String::from("NOBODY"),
            Self::Everyone => String::from("EVERYONE"),
        }
    }
}

impl Default for MeetingJoiningPreference {
    fn default() -> MeetingJoiningPreference {
        Self::Nobody
    }
}




