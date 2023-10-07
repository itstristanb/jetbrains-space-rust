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
pub struct MavenPackageVersionDetails {
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::PackageType>,
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(rename = "created")]
    pub created: i64,
    #[serde(rename = "accessed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accessed: Option<Option<i64>>,
    #[serde(rename = "downloads")]
    pub downloads: i64,
    #[serde(rename = "pinned")]
    pub pinned: bool,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    #[serde(rename = "diskSize")]
    pub disk_size: i64,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<crate::models::CPrincipal>>,
    #[serde(rename = "authors", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Option<Vec<crate::models::CPrincipal>>>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Box<crate::models::PackageOrigin>>,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<::std::collections::HashMap<String, String>>>,
    #[serde(rename = "packaging", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub packaging: Option<Option<String>>,
    #[serde(rename = "packageName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub package_name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "licenses")]
    pub licenses: Vec<String>,
    #[serde(rename = "scmUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scm_url: Option<Option<String>>,
    #[serde(rename = "dependencies")]
    pub dependencies: Vec<crate::models::MavenPackageDependency>,
    #[serde(rename = "kotlinPlatforms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub kotlin_platforms: Option<Option<Vec<crate::models::KotlinPlatform>>>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<crate::models::MavenPackageParent>>,
    #[serde(rename = "pathPrefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<Option<String>>,
    #[serde(rename = "files")]
    pub files: Vec<crate::models::MavenPackageFile>,
}

impl MavenPackageVersionDetails {
    pub fn new(r#type: crate::models::PackageType, repository: String, name: String, version: String, created: i64, downloads: i64, pinned: bool, disk_size: i64, licenses: Vec<String>, dependencies: Vec<crate::models::MavenPackageDependency>, files: Vec<crate::models::MavenPackageFile>) -> MavenPackageVersionDetails {
        MavenPackageVersionDetails {
            r#type: Box::new(r#type),
            repository,
            name,
            version,
            tags: None,
            created,
            accessed: None,
            downloads,
            pinned,
            comment: None,
            disk_size,
            author: None,
            authors: None,
            origin: None,
            metadata: None,
            packaging: None,
            package_name: None,
            description: None,
            url: None,
            licenses,
            scm_url: None,
            dependencies,
            kotlin_platforms: None,
            parent: None,
            path_prefix: None,
            files,
        }
    }
}

