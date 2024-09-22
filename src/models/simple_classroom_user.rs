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

/// SimpleClassroomUser : A GitHub user simplified for Classroom.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleClassroomUser {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
}

impl SimpleClassroomUser {
    /// A GitHub user simplified for Classroom.
    pub fn new(id: i32, login: String, avatar_url: String, html_url: String) -> SimpleClassroomUser {
        SimpleClassroomUser {
            id,
            login,
            avatar_url,
            html_url,
        }
    }
}

