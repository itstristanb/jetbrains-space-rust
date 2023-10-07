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
pub struct SfuProducerParameters {
    #[serde(rename = "mediaKind")]
    pub media_kind: crate::models::SfuMediaKind,
    #[serde(rename = "rtpParameters")]
    pub rtp_parameters: String,
    #[serde(rename = "paused")]
    pub paused: bool,
    #[serde(rename = "closed")]
    pub closed: bool,
}

impl SfuProducerParameters {
    pub fn new(media_kind: crate::models::SfuMediaKind, rtp_parameters: String, paused: bool, closed: bool) -> SfuProducerParameters {
        SfuProducerParameters {
            media_kind,
            rtp_parameters,
            paused,
            closed,
        }
    }
}

