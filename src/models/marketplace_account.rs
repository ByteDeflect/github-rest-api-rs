/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceAccount {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(rename = "organization_billing_email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_billing_email: Option<Option<String>>,
}

impl MarketplaceAccount {
    pub fn new(url: String, id: i32, r#type: String, login: String) -> MarketplaceAccount {
        MarketplaceAccount {
            url,
            id,
            r#type,
            node_id: None,
            login,
            email: None,
            organization_billing_email: None,
        }
    }
}

