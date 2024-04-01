/*
 * Mlflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v2.11.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mlflowrunstatus {
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "FINISHED")]
    Finished,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "KILLED")]
    Killed,

}

impl ToString for Mlflowrunstatus {
    fn to_string(&self) -> String {
        match self {
            Self::Running => String::from("RUNNING"),
            Self::Scheduled => String::from("SCHEDULED"),
            Self::Finished => String::from("FINISHED"),
            Self::Failed => String::from("FAILED"),
            Self::Killed => String::from("KILLED"),
        }
    }
}

impl Default for Mlflowrunstatus {
    fn default() -> Mlflowrunstatus {
        Self::Running
    }
}

