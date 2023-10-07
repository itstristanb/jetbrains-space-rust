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
pub struct PublicHolidaysCalendarsImportPostRequest {
    #[serde(rename = "calendar")]
    pub calendar: String,
    #[serde(rename = "attachmentId")]
    pub attachment_id: String,
}

impl PublicHolidaysCalendarsImportPostRequest {
    pub fn new(calendar: String, attachment_id: String) -> PublicHolidaysCalendarsImportPostRequest {
        PublicHolidaysCalendarsImportPostRequest {
            calendar,
            attachment_id,
        }
    }
}

