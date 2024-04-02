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
pub struct MlflowserviceupdaterunRequest {
    /// ID of the run to update. Must be provided.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// [Deprecated, use run_id instead] ID of the run to update.. This field will be removed in a future MLflow version.
    #[serde(rename = "run_uuid", skip_serializing_if = "Option::is_none")]
    pub run_uuid: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::Mlflowrunstatus>,
    /// Unix timestamp in milliseconds of when the run ended.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Updated name of the run.
    #[serde(rename = "run_name", skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
}

impl MlflowserviceupdaterunRequest {
    pub fn new() -> MlflowserviceupdaterunRequest {
        MlflowserviceupdaterunRequest {
            run_id: None,
            run_uuid: None,
            status: None,
            end_time: None,
            run_name: None,
        }
    }
}
