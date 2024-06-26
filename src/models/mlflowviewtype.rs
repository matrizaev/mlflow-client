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
pub enum Mlflowviewtype {
    #[serde(rename = "ACTIVE_ONLY")]
    ActiveOnly,
    #[serde(rename = "DELETED_ONLY")]
    DeletedOnly,
    #[serde(rename = "ALL")]
    All,

}

impl ToString for Mlflowviewtype {
    fn to_string(&self) -> String {
        match self {
            Self::ActiveOnly => String::from("ACTIVE_ONLY"),
            Self::DeletedOnly => String::from("DELETED_ONLY"),
            Self::All => String::from("ALL"),
        }
    }
}

impl Default for Mlflowviewtype {
    fn default() -> Mlflowviewtype {
        Self::ActiveOnly
    }
}

