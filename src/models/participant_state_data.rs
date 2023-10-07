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
pub struct ParticipantStateData {
    #[serde(rename = "audioEnabled")]
    pub audio_enabled: bool,
    #[serde(rename = "videoEnabled")]
    pub video_enabled: bool,
    #[serde(rename = "screenShared")]
    pub screen_shared: bool,
    #[serde(rename = "qualitiesCount", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub qualities_count: Option<Option<i32>>,
}

impl ParticipantStateData {
    pub fn new(audio_enabled: bool, video_enabled: bool, screen_shared: bool) -> ParticipantStateData {
        ParticipantStateData {
            audio_enabled,
            video_enabled,
            screen_shared,
            qualities_count: None,
        }
    }
}

