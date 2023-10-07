/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LdapModuleType {
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "OPEN_LDAP")]
    OpenLdap,

}

impl ToString for LdapModuleType {
    fn to_string(&self) -> String {
        match self {
            Self::Ad => String::from("AD"),
            Self::OpenLdap => String::from("OPEN_LDAP"),
        }
    }
}

impl Default for LdapModuleType {
    fn default() -> LdapModuleType {
        Self::Ad
    }
}




