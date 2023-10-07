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
pub struct WeekDayTimeInterval {
    #[serde(rename = "day")]
    pub day: i32,
    #[serde(rename = "checked")]
    pub checked: bool,
    #[serde(rename = "interval")]
    pub interval: Box<crate::models::TimeInterval>,
}

impl WeekDayTimeInterval {
    pub fn new(day: i32, checked: bool, interval: crate::models::TimeInterval) -> WeekDayTimeInterval {
        WeekDayTimeInterval {
            day,
            checked,
            interval: Box::new(interval),
        }
    }
}


