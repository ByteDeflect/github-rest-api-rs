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
pub struct MarketplacePurchaseMarketplacePurchase {
    #[serde(rename = "billing_cycle", skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(rename = "next_billing_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<Option<String>>,
    #[serde(rename = "is_installed", skip_serializing_if = "Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(rename = "unit_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<Option<i32>>,
    #[serde(rename = "on_free_trial", skip_serializing_if = "Option::is_none")]
    pub on_free_trial: Option<bool>,
    #[serde(rename = "free_trial_ends_on", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub free_trial_ends_on: Option<Option<String>>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<models::MarketplaceListingPlan>>,
}

impl MarketplacePurchaseMarketplacePurchase {
    pub fn new() -> MarketplacePurchaseMarketplacePurchase {
        MarketplacePurchaseMarketplacePurchase {
            billing_cycle: None,
            next_billing_date: None,
            is_installed: None,
            unit_count: None,
            on_free_trial: None,
            free_trial_ends_on: None,
            updated_at: None,
            plan: None,
        }
    }
}

