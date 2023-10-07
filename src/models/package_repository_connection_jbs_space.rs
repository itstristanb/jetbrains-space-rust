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
pub struct PackageRepositoryConnectionJbsSpace {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "rememberDownloaded")]
    pub remember_downloaded: bool,
    #[serde(rename = "packageNameFilters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub package_name_filters: Option<Option<Vec<String>>>,
    #[serde(rename = "repository")]
    pub repository: Box<crate::models::ProjectPackageRepository>,
}

impl PackageRepositoryConnectionJbsSpace {
    pub fn new(id: String, enabled: bool, remember_downloaded: bool, repository: crate::models::ProjectPackageRepository) -> PackageRepositoryConnectionJbsSpace {
        PackageRepositoryConnectionJbsSpace {
            id,
            description: None,
            enabled,
            remember_downloaded,
            package_name_filters: None,
            repository: Box::new(repository),
        }
    }
}


