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
pub struct Model20MlflowRunsLogMetricPostRequest {
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "run_uuid", skip_serializing_if = "Option::is_none")]
    pub run_uuid: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
}

impl Model20MlflowRunsLogMetricPostRequest {
    pub fn new() -> Model20MlflowRunsLogMetricPostRequest {
        Model20MlflowRunsLogMetricPostRequest {
            run_id: None,
            run_uuid: None,
            key: None,
            value: None,
            timestamp: None,
            step: None,
        }
    }
}

