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
pub struct HaDefaultValueJbsMap {
    #[serde(rename = "elements")]
    pub elements: ::std::collections::HashMap<String, crate::models::HaDefaultValue>,
}

impl HaDefaultValueJbsMap {
    pub fn new(elements: ::std::collections::HashMap<String, crate::models::HaDefaultValue>) -> HaDefaultValueJbsMap {
        HaDefaultValueJbsMap {
            elements,
        }
    }
}

