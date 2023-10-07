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
pub struct IssueFieldOrderInJbsCustom {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "scope")]
    pub scope: Box<crate::models::ExtendedTypeScope>,
}

impl IssueFieldOrderInJbsCustom {
    pub fn new(id: String, scope: crate::models::ExtendedTypeScope) -> IssueFieldOrderInJbsCustom {
        IssueFieldOrderInJbsCustom {
            id,
            scope: Box::new(scope),
        }
    }
}


