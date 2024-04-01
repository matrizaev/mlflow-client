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
pub struct Model20MlflowExperimentsSearchPostRequest {
    #[serde(rename = "max_results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "page_token", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "order_by", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    #[serde(rename = "view_type", skip_serializing_if = "Option::is_none")]
    pub view_type: Option<models::Mlflowviewtype>,
}

impl Model20MlflowExperimentsSearchPostRequest {
    pub fn new() -> Model20MlflowExperimentsSearchPostRequest {
        Model20MlflowExperimentsSearchPostRequest {
            max_results: None,
            page_token: None,
            filter: None,
            order_by: None,
            view_type: None,
        }
    }
}

