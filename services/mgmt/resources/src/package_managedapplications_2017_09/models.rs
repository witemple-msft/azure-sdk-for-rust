#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    pub kind: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationPropertiesPatchable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationDefinitionProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "managedResourceGroupId")]
    pub managed_resource_group_id: String,
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPropertiesPatchable {
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionProperties {
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplicationLockLevel,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<String>,
    pub authorizations: Vec<ApplicationProviderAuthorization>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageFileUri", default, skip_serializing_if = "Option::is_none")]
    pub package_file_uri: Option<String>,
    #[serde(rename = "mainTemplate", default, skip_serializing_if = "Option::is_none")]
    pub main_template: Option<serde_json::Value>,
    #[serde(rename = "createUiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub create_ui_definition: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    pub version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactType {
    Template,
    Custom,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationArtifact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApplicationArtifactType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProviderAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "httpStatus", default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
