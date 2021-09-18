#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Succeeded,
    Failed,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    pub capacity: i32,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        S1,
        S2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrackedResourceProperties {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub sku: Sku,
    pub properties: EnvironmentCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreationProperties {
    #[serde(rename = "dataRetentionTime")]
    pub data_retention_time: String,
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<environment_creation_properties::StorageLimitExceededBehavior>,
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<PartitionKeyProperty>,
}
pub mod environment_creation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResourceProperties {
    #[serde(flatten)]
    pub environment_creation_properties: EnvironmentCreationProperties,
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "dataAccessId", default, skip_serializing_if = "Option::is_none")]
    pub data_access_id: Option<String>,
    #[serde(rename = "dataAccessFqdn", default, skip_serializing_if = "Option::is_none")]
    pub data_access_fqdn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentMutableProperties {
    #[serde(rename = "dataRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub data_retention_time: Option<String>,
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<environment_mutable_properties::StorageLimitExceededBehavior>,
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<PartitionKeyProperty>,
}
pub mod environment_mutable_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartitionKeyProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<partition_key_property::Type>,
}
pub mod partition_key_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<IngressEnvironmentStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngressEnvironmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ingress_environment_status::State>,
    #[serde(rename = "stateDetails", default, skip_serializing_if = "Option::is_none")]
    pub state_details: Option<EnvironmentStateDetails>,
}
pub mod ingress_environment_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Ready,
        Running,
        Paused,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentStateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub kind: event_source_create_or_update_parameters::Kind,
}
pub mod event_source_create_or_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    pub properties: EventHubEventSourceCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    pub properties: IoTHubEventSourceCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSourceMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubEventSourceMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSourceResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub kind: event_source_resource::Kind,
}
pub mod event_source_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    pub properties: EventHubEventSourceResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    pub properties: IoTHubEventSourceResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceCommonProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureEventSourceProperties {
    #[serde(flatten)]
    pub event_source_common_properties: EventSourceCommonProperties,
    #[serde(rename = "eventSourceResourceId")]
    pub event_source_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[serde(rename = "serviceBusNamespace")]
    pub service_bus_namespace: String,
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[serde(rename = "iotHubName")]
    pub iot_hub_name: String,
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalTimestamp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<local_timestamp::Format>,
    #[serde(rename = "timeZoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_offset: Option<local_timestamp::TimeZoneOffset>,
}
pub mod local_timestamp {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Embedded,
        Iana,
        TimeSpan,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TimeZoneOffset {
        #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
        pub property_name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceMutableProperties {
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub properties: ReferenceDataSetCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReferenceDataSetResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReferenceDataSetResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreationProperties {
    #[serde(rename = "keyProperties")]
    pub key_properties: Vec<ReferenceDataSetKeyProperty>,
    #[serde(rename = "dataStringComparisonBehavior", default, skip_serializing_if = "Option::is_none")]
    pub data_string_comparison_behavior: Option<reference_data_set_creation_properties::DataStringComparisonBehavior>,
}
pub mod reference_data_set_creation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataStringComparisonBehavior {
        Ordinal,
        OrdinalIgnoreCase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResourceProperties {
    #[serde(flatten)]
    pub reference_data_set_creation_properties: ReferenceDataSetCreationProperties,
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetKeyProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<reference_data_set_key_property::Type>,
}
pub mod reference_data_set_key_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        Double,
        Bool,
        DateTime,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyCreateOrUpdateParameters {
    pub properties: AccessPolicyResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessPolicyResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyResourceProperties {
    #[serde(rename = "principalObjectId", default, skip_serializing_if = "Option::is_none")]
    pub principal_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyMutableProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
