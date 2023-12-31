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
pub struct EsLdapAttributeNames {
    #[serde(rename = "fullNameAttributeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub full_name_attribute_name: Option<Option<String>>,
    #[serde(rename = "usernameAttributeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username_attribute_name: Option<Option<String>>,
    #[serde(rename = "emailAttributeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email_attribute_name: Option<Option<String>>,
    #[serde(rename = "groupsAttributeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub groups_attribute_name: Option<Option<String>>,
}

impl EsLdapAttributeNames {
    pub fn new() -> EsLdapAttributeNames {
        EsLdapAttributeNames {
            full_name_attribute_name: None,
            username_attribute_name: None,
            email_attribute_name: None,
            groups_attribute_name: None,
        }
    }
}


