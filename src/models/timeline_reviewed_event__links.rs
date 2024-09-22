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
pub struct TimelineReviewedEventLinks {
    #[serde(rename = "html")]
    pub html: Box<models::TimelineReviewedEventLinksHtml>,
    #[serde(rename = "pull_request")]
    pub pull_request: Box<models::TimelineReviewedEventLinksHtml>,
}

impl TimelineReviewedEventLinks {
    pub fn new(html: models::TimelineReviewedEventLinksHtml, pull_request: models::TimelineReviewedEventLinksHtml) -> TimelineReviewedEventLinks {
        TimelineReviewedEventLinks {
            html: Box::new(html),
            pull_request: Box::new(pull_request),
        }
    }
}

