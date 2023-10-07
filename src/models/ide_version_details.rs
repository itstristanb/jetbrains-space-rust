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
pub struct IdeVersionDetails {
    #[serde(rename = "ideName")]
    pub ide_name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "versionDisplayName")]
    pub version_display_name: String,
    #[serde(rename = "build")]
    pub build: String,
    #[serde(rename = "releaseDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<Option<String>>,
    #[serde(rename = "quality")]
    pub quality: Box<crate::models::IdeVersionQuality>,
}

impl IdeVersionDetails {
    pub fn new(ide_name: String, version: String, version_display_name: String, build: String, quality: crate::models::IdeVersionQuality) -> IdeVersionDetails {
        IdeVersionDetails {
            ide_name,
            version,
            version_display_name,
            build,
            release_date: None,
            quality: Box::new(quality),
        }
    }
}


