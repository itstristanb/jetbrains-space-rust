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
pub struct PackageRepository {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::PackageType>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "public")]
    pub public: bool,
    #[serde(rename = "cleanupEnabled")]
    pub cleanup_enabled: bool,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::EsPackageRepositorySettings>>,
    #[serde(rename = "mode")]
    pub mode: crate::models::PackageRepositoryMode,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl PackageRepository {
    pub fn new(id: String, r#type: crate::models::PackageType, public: bool, cleanup_enabled: bool, mode: crate::models::PackageRepositoryMode, archived: bool) -> PackageRepository {
        PackageRepository {
            id,
            r#type: Box::new(r#type),
            name: None,
            description: None,
            public,
            cleanup_enabled,
            settings: None,
            mode,
            archived,
        }
    }
}

