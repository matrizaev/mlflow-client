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
pub struct Modelregistryservicesearchregisteredmodels200Response {
    /// Registered Models that match the search criteria.
    #[serde(rename = "registered_models", skip_serializing_if = "Option::is_none")]
    pub registered_models: Option<Vec<models::Mlflowregisteredmodel>>,
    /// Pagination token to request the next page of models.
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Modelregistryservicesearchregisteredmodels200Response {
    pub fn new() -> Modelregistryservicesearchregisteredmodels200Response {
        Modelregistryservicesearchregisteredmodels200Response {
            registered_models: None,
            next_page_token: None,
        }
    }
}
