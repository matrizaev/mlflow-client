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
pub struct Model20MlflowMetricsGetHistoryGet200Response {
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<models::Mlflowmetric>>,
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Model20MlflowMetricsGetHistoryGet200Response {
    pub fn new() -> Model20MlflowMetricsGetHistoryGet200Response {
        Model20MlflowMetricsGetHistoryGet200Response {
            metrics: None,
            next_page_token: None,
        }
    }
}

