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
pub struct RecurrenceRuleFreqJbsMonthlyOnLastWeekday {
    #[serde(rename = "weekday")]
    pub weekday: crate::models::Weekday,
    #[serde(rename = "interval")]
    pub interval: i32,
}

impl RecurrenceRuleFreqJbsMonthlyOnLastWeekday {
    pub fn new(weekday: crate::models::Weekday, interval: i32) -> RecurrenceRuleFreqJbsMonthlyOnLastWeekday {
        RecurrenceRuleFreqJbsMonthlyOnLastWeekday {
            weekday,
            interval,
        }
    }
}


