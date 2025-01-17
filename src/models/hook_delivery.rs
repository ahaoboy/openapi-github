/*
 * GitHub's official OpenAPI spec + Octokit extension
 *
 * OpenAPI specs from https://github.com/github/rest-api-description with the 'x-octokit' extension required by the Octokit SDKs
 *
 * The version of the OpenAPI document: 16.6.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// HookDelivery : Delivery made by a webhook.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookDelivery {
    /// Unique identifier of the delivery.
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier for the event (shared with all deliveries for all webhooks that subscribe to this event).
    #[serde(rename = "guid")]
    pub guid: String,
    /// Time when the delivery was delivered.
    #[serde(rename = "delivered_at")]
    pub delivered_at: String,
    /// Whether the delivery is a redelivery.
    #[serde(rename = "redelivery")]
    pub redelivery: bool,
    /// Time spent delivering.
    #[serde(rename = "duration")]
    pub duration: f64,
    /// Description of the status of the attempted delivery
    #[serde(rename = "status")]
    pub status: String,
    /// Status code received when delivery was made.
    #[serde(rename = "status_code")]
    pub status_code: i32,
    /// The event that triggered the delivery.
    #[serde(rename = "event")]
    pub event: String,
    /// The type of activity for the event that triggered the delivery.
    #[serde(rename = "action", deserialize_with = "Option::deserialize")]
    pub action: Option<String>,
    /// The id of the GitHub App installation associated with this event.
    #[serde(rename = "installation_id", deserialize_with = "Option::deserialize")]
    pub installation_id: Option<i32>,
    /// The id of the repository associated with this event.
    #[serde(rename = "repository_id", deserialize_with = "Option::deserialize")]
    pub repository_id: Option<i32>,
    /// The URL target of the delivery.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "request")]
    pub request: Box<models::HookDeliveryRequest>,
    #[serde(rename = "response")]
    pub response: Box<models::HookDeliveryResponse>,
}

impl HookDelivery {
    /// Delivery made by a webhook.
    pub fn new(id: i32, guid: String, delivered_at: String, redelivery: bool, duration: f64, status: String, status_code: i32, event: String, action: Option<String>, installation_id: Option<i32>, repository_id: Option<i32>, request: models::HookDeliveryRequest, response: models::HookDeliveryResponse) -> HookDelivery {
        HookDelivery {
            id,
            guid,
            delivered_at,
            redelivery,
            duration,
            status,
            status_code,
            event,
            action,
            installation_id,
            repository_id,
            url: None,
            request: Box::new(request),
            response: Box::new(response),
        }
    }
}

