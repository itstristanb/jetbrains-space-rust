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
pub enum MeetingOrigin {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "GoogleCalendar")]
    GoogleCalendar,

}

impl ToString for MeetingOrigin {
    fn to_string(&self) -> String {
        match self {
            Self::User => String::from("User"),
            Self::GoogleCalendar => String::from("GoogleCalendar"),
        }
    }
}

impl Default for MeetingOrigin {
    fn default() -> MeetingOrigin {
        Self::User
    }
}




