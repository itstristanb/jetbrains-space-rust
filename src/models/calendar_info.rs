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
pub struct CalendarInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "defaultColor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_color: Option<Option<String>>,
    #[serde(rename = "freeBusyOnly")]
    pub free_busy_only: bool,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(rename = "exposeToCalDav")]
    pub expose_to_cal_dav: bool,
    #[serde(rename = "exposeToGoogle")]
    pub expose_to_google: bool,
    #[serde(rename = "sourceUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Option<String>>,
    #[serde(rename = "syncAttemptCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sync_attempt_count: Option<Option<i32>>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl CalendarInfo {
    pub fn new(id: String, free_busy_only: bool, read_only: bool, expose_to_cal_dav: bool, expose_to_google: bool, archived: bool) -> CalendarInfo {
        CalendarInfo {
            id,
            name: None,
            default_color: None,
            free_busy_only,
            read_only,
            expose_to_cal_dav,
            expose_to_google,
            source_url: None,
            sync_attempt_count: None,
            archived,
        }
    }
}


