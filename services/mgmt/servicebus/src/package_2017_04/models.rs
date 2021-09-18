#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbAuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbAuthorizationRule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbAuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<sb_authorization_rule::Properties>,
}
pub mod sb_authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        pub rights: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessKeys {
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "aliasPrimaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_primary_connection_string: Option<String>,
    #[serde(rename = "aliasSecondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmDisasterRecovery {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<arm_disaster_recovery::Properties>,
}
pub mod arm_disaster_recovery {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(rename = "pendingReplicationOperationsCount", default, skip_serializing_if = "Option::is_none")]
        pub pending_replication_operations_count: Option<i64>,
        #[serde(rename = "partnerNamespace", default, skip_serializing_if = "Option::is_none")]
        pub partner_namespace: Option<String>,
        #[serde(rename = "alternateName", default, skip_serializing_if = "Option::is_none")]
        pub alternate_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role: Option<properties::Role>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Succeeded,
            Failed,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Role {
            Primary,
            PrimaryNotReplicating,
            Secondary,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmDisasterRecoveryListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArmDisasterRecovery>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eventhub {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eventhub::Properties>,
}
pub mod eventhub {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "partitionIds", default, skip_serializing_if = "Vec::is_empty")]
        pub partition_ids: Vec<String>,
        #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[serde(rename = "messageRetentionInDays", default, skip_serializing_if = "Option::is_none")]
        pub message_retention_in_days: Option<i64>,
        #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
        pub partition_count: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<EntityStatus>,
        #[serde(rename = "captureDescription", default, skip_serializing_if = "Option::is_none")]
        pub capture_description: Option<CaptureDescription>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Eventhub>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptureDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<capture_description::Encoding>,
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "sizeLimitInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_limit_in_bytes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
}
pub mod capture_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<destination::Properties>,
}
pub mod destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_resource_id: Option<String>,
        #[serde(rename = "blobContainer", default, skip_serializing_if = "Option::is_none")]
        pub blob_container: Option<String>,
        #[serde(rename = "archiveNameFormat", default, skip_serializing_if = "Option::is_none")]
        pub archive_name_format: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespaceMigrate {
    #[serde(rename = "targetNamespaceType")]
    pub target_namespace_type: sb_namespace_migrate::TargetNamespaceType,
}
pub mod sb_namespace_migrate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TargetNamespaceType {
        Messaging,
        NotificationHub,
        Mixed,
        EventHub,
        Relay,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationConfigProperties {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<migration_config_properties::Properties>,
}
pub mod migration_config_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[serde(rename = "pendingReplicationOperationsCount", default, skip_serializing_if = "Option::is_none")]
        pub pending_replication_operations_count: Option<i64>,
        #[serde(rename = "targetNamespace")]
        pub target_namespace: String,
        #[serde(rename = "postMigrationName")]
        pub post_migration_name: String,
        #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
        pub migration_state: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationConfigListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MigrationConfigProperties>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNamespacePatch {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbNamespace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SbSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbNamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespaceUpdateParameters {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SbSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbNamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbNamespaceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbSku {
    pub name: sb_sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sb_sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod sb_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NwRuleSetIpRules {
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<nw_rule_set_ip_rules::Action>,
}
pub mod nw_rule_set_ip_rules {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NwRuleSetVirtualNetworkRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_rule_set::Properties>,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
        pub default_action: Option<properties::DefaultAction>,
        #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
        pub virtual_network_rules: Vec<NwRuleSetVirtualNetworkRules>,
        #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_rules: Vec<NwRuleSetIpRules>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum DefaultAction {
            Allow,
            Deny,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSetListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkRuleSet>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PremiumMessagingRegions {
    #[serde(flatten)]
    pub resource_namespace_patch: ResourceNamespacePatch,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<premium_messaging_regions::Properties>,
}
pub mod premium_messaging_regions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
        pub full_name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PremiumMessagingRegionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PremiumMessagingRegions>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbQueueListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbQueue>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbQueue {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbQueueProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbQueueProperties {
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "accessedAt", default, skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<String>,
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "messageCount", default, skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
    #[serde(rename = "lockDuration", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[serde(rename = "maxSizeInMegabytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i32>,
    #[serde(rename = "requiresDuplicateDetection", default, skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[serde(rename = "requiresSession", default, skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(rename = "deadLetteringOnMessageExpiration", default, skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "enablePartitioning", default, skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[serde(rename = "enableExpress", default, skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
    #[serde(rename = "forwardTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<String>,
    #[serde(rename = "forwardDeadLetteredMessagesTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_dead_lettered_messages_to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Ruleproperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ruleproperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "filterType", default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<FilterType>,
    #[serde(rename = "sqlFilter", default, skip_serializing_if = "Option::is_none")]
    pub sql_filter: Option<SqlFilter>,
    #[serde(rename = "correlationFilter", default, skip_serializing_if = "Option::is_none")]
    pub correlation_filter: Option<CorrelationFilter>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Rule>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FilterType {
    SqlFilter,
    CorrelationFilter,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlFilter {
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorrelationFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "replyToSessionId", default, skip_serializing_if = "Option::is_none")]
    pub reply_to_session_id: Option<String>,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlRuleAction {
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "sqlExpression", default, skip_serializing_if = "Option::is_none")]
    pub sql_expression: Option<String>,
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[serde(rename = "requiresPreprocessing", default, skip_serializing_if = "Option::is_none")]
    pub requires_preprocessing: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbSubscriptionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbSubscription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbSubscriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbSubscriptionProperties {
    #[serde(rename = "messageCount", default, skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "accessedAt", default, skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "lockDuration", default, skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[serde(rename = "requiresSession", default, skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(
        rename = "deadLetteringOnFilterEvaluationExceptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dead_lettering_on_filter_evaluation_exceptions: Option<bool>,
    #[serde(rename = "deadLetteringOnMessageExpiration", default, skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[serde(rename = "maxDeliveryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "forwardTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<String>,
    #[serde(rename = "forwardDeadLetteredMessagesTo", default, skip_serializing_if = "Option::is_none")]
    pub forward_dead_lettered_messages_to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbTopicListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SbTopic>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbTopic {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SbTopicProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SbTopicProperties {
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "accessedAt", default, skip_serializing_if = "Option::is_none")]
    pub accessed_at: Option<String>,
    #[serde(rename = "subscriptionCount", default, skip_serializing_if = "Option::is_none")]
    pub subscription_count: Option<i32>,
    #[serde(rename = "countDetails", default, skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "defaultMessageTimeToLive", default, skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(rename = "maxSizeInMegabytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i32>,
    #[serde(rename = "requiresDuplicateDetection", default, skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[serde(rename = "enableBatchedOperations", default, skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "supportOrdering", default, skip_serializing_if = "Option::is_none")]
    pub support_ordering: Option<bool>,
    #[serde(rename = "autoDeleteOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "enablePartitioning", default, skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[serde(rename = "enableExpress", default, skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorResponse>,
        #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_info: Vec<ErrorAdditionalInfo>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
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
pub enum EntityStatus {
    Active,
    Disabled,
    Restoring,
    SendDisabled,
    ReceiveDisabled,
    Creating,
    Deleting,
    Renaming,
    Unknown,
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
pub struct MessageCountDetails {
    #[serde(rename = "activeMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub active_message_count: Option<i64>,
    #[serde(rename = "deadLetterMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_message_count: Option<i64>,
    #[serde(rename = "scheduledMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_message_count: Option<i64>,
    #[serde(rename = "transferMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_message_count: Option<i64>,
    #[serde(rename = "transferDeadLetterMessageCount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_dead_letter_message_count: Option<i64>,
}
