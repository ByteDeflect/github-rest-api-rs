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
pub struct MarketplacePurchaseMarketplacePendingChange {
    #[serde(rename = "is_installed", skip_serializing_if = "Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(rename = "effective_date", skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "unit_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<Option<i32>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::MarketplaceListingPlan>>,
}

impl MarketplacePurchaseMarketplacePendingChange {
    pub fn new() -> MarketplacePurchaseMarketplacePendingChange {
        MarketplacePurchaseMarketplacePendingChange {
            is_installed: None,
            effective_date: None,
            unit_count: None,
            id: None,
            plan: None,
        }
    }
}

