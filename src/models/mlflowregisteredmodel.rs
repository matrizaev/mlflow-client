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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mlflowregisteredmodel {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "creation_timestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<i64>,
    #[serde(rename = "last_updated_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "latest_versions", skip_serializing_if = "Option::is_none")]
    pub latest_versions: Option<Vec<models::Mlflowmodelversion>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<models::Mlflowregisteredmodeltag>>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<models::Mlflowregisteredmodelalias>>,
}

impl Mlflowregisteredmodel {
    pub fn new() -> Mlflowregisteredmodel {
        Mlflowregisteredmodel {
            name: None,
            creation_timestamp: None,
            last_updated_timestamp: None,
            user_id: None,
            description: None,
            latest_versions: None,
            tags: None,
            aliases: None,
        }
    }
}

