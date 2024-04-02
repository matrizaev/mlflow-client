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
pub struct MlflowservicelogbatchRequest {
    /// ID of the run to log under
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Metrics to log. A single request can contain up to 1000 metrics, and up to 1000 metrics, params, and tags in total.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<models::Mlflowmetric>>,
    /// Params to log. A single request can contain up to 100 params, and up to 1000 metrics, params, and tags in total.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<models::Mlflowparam>>,
    /// Tags to log. A single request can contain up to 100 tags, and up to 1000 metrics, params, and tags in total.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<models::Mlflowruntag>>,
}

impl MlflowservicelogbatchRequest {
    pub fn new() -> MlflowservicelogbatchRequest {
        MlflowservicelogbatchRequest {
            run_id: None,
            metrics: None,
            params: None,
            tags: None,
        }
    }
}
