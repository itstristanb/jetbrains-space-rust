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
pub struct WeekDayLocation {
    #[serde(rename = "day")]
    pub day: i32,
    #[serde(rename = "remote")]
    pub remote: bool,
}

impl WeekDayLocation {
    pub fn new(day: i32, remote: bool) -> WeekDayLocation {
        WeekDayLocation {
            day,
            remote,
        }
    }
}

