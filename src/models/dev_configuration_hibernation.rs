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
pub struct DevConfigurationHibernation {
    #[serde(rename = "hibernateAfter")]
    pub hibernate_after: String,
}

impl DevConfigurationHibernation {
    pub fn new(hibernate_after: String) -> DevConfigurationHibernation {
        DevConfigurationHibernation {
            hibernate_after,
        }
    }
}

