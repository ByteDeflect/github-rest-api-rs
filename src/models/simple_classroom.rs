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

/// SimpleClassroom : A GitHub Classroom classroom
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleClassroom {
    /// Unique identifier of the classroom.
    #[serde(rename = "id")]
    pub id: i32,
    /// The name of the classroom.
    #[serde(rename = "name")]
    pub name: String,
    /// Returns whether classroom is archived or not.
    #[serde(rename = "archived")]
    pub archived: bool,
    /// The url of the classroom on GitHub Classroom.
    #[serde(rename = "url")]
    pub url: String,
}

impl SimpleClassroom {
    /// A GitHub Classroom classroom
    pub fn new(id: i32, name: String, archived: bool, url: String) -> SimpleClassroom {
        SimpleClassroom {
            id,
            name,
            archived,
            url,
        }
    }
}

