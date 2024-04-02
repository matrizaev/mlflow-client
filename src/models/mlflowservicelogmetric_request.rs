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
pub struct MlflowservicelogmetricRequest {
    /// ID of the run under which to log the metric. Must be provided.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// [Deprecated, use run_id instead] ID of the run under which to log the metric. This field will be removed in a future MLflow version.
    #[serde(rename = "run_uuid", skip_serializing_if = "Option::is_none")]
    pub run_uuid: Option<String>,
    /// Name of the metric. This field is required.
    #[serde(rename = "key")]
    pub key: String,
    /// Double value of the metric being logged. This field is required.
    #[serde(rename = "value")]
    pub value: f64,
    /// Unix timestamp in milliseconds at the time metric was logged. This field is required.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Step at which to log the metric
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
}

impl MlflowservicelogmetricRequest {
    pub fn new(key: String, value: f64, timestamp: i64) -> MlflowservicelogmetricRequest {
        MlflowservicelogmetricRequest {
            run_id: None,
            run_uuid: None,
            key,
            value,
            timestamp,
            step: None,
        }
    }
}
