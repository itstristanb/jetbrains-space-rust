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
pub struct CalendarsMeetingsIdConferenceRoomsPostRequest {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "dateTime")]
    pub date_time: String,
}

impl CalendarsMeetingsIdConferenceRoomsPostRequest {
    pub fn new(room_id: String, date_time: String) -> CalendarsMeetingsIdConferenceRoomsPostRequest {
        CalendarsMeetingsIdConferenceRoomsPostRequest {
            room_id,
            date_time,
        }
    }
}


