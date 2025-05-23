/*
 * Private Proof Delegation-CCL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@taceo.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleJobRequest {
    #[serde(rename = "blueprint_id")]
    pub blueprint_id: uuid::Uuid,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "job_type")]
    pub job_type: models::JobType,
}

impl ScheduleJobRequest {
    pub fn new(
        blueprint_id: uuid::Uuid,
        code: String,
        job_type: models::JobType,
    ) -> ScheduleJobRequest {
        ScheduleJobRequest {
            blueprint_id,
            code,
            job_type,
        }
    }
}
