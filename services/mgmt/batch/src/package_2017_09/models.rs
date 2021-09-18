#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoStorageBaseProperties {
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountUpdateProperties {
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageBaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountCreateProperties {
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageBaseProperties>,
    #[serde(rename = "poolAllocationMode", default, skip_serializing_if = "Option::is_none")]
    pub pool_allocation_mode: Option<PoolAllocationMode>,
    #[serde(rename = "keyVaultReference", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference: Option<KeyVaultReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountCreateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    pub id: String,
    pub url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoStorageProperties {
    #[serde(flatten)]
    pub auto_storage_base_properties: AutoStorageBaseProperties,
    #[serde(rename = "lastKeySync")]
    pub last_key_sync: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountProperties {
    #[serde(rename = "accountEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub account_endpoint: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<batch_account_properties::ProvisioningState>,
    #[serde(rename = "poolAllocationMode", default, skip_serializing_if = "Option::is_none")]
    pub pool_allocation_mode: Option<PoolAllocationMode>,
    #[serde(rename = "keyVaultReference", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_reference: Option<KeyVaultReference>,
    #[serde(rename = "autoStorage", default, skip_serializing_if = "Option::is_none")]
    pub auto_storage: Option<AutoStorageProperties>,
    #[serde(rename = "dedicatedCoreQuota", default, skip_serializing_if = "Option::is_none")]
    pub dedicated_core_quota: Option<i32>,
    #[serde(rename = "lowPriorityCoreQuota", default, skip_serializing_if = "Option::is_none")]
    pub low_priority_core_quota: Option<i32>,
    #[serde(rename = "poolQuota", default, skip_serializing_if = "Option::is_none")]
    pub pool_quota: Option<i32>,
    #[serde(rename = "activeJobAndJobScheduleQuota", default, skip_serializing_if = "Option::is_none")]
    pub active_job_and_job_schedule_quota: Option<i32>,
}
pub mod batch_account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Invalid,
        Creating,
        Deleting,
        Succeeded,
        Failed,
        Cancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BatchAccountUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BatchAccount>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountRegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: batch_account_regenerate_key_parameters::KeyName,
}
pub mod batch_account_regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Primary,
        Secondary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchAccountKeys {
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivateApplicationPackageParameters {
    pub format: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCreateParameters {
    #[serde(rename = "allowUpdates", default, skip_serializing_if = "Option::is_none")]
    pub allow_updates: Option<bool>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<ApplicationPackage>,
    #[serde(rename = "allowUpdates", default, skip_serializing_if = "Option::is_none")]
    pub allow_updates: Option<bool>,
    #[serde(rename = "defaultVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<application_package::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "storageUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_url: Option<String>,
    #[serde(rename = "storageUrlExpiry", default, skip_serializing_if = "Option::is_none")]
    pub storage_url_expiry: Option<String>,
    #[serde(rename = "lastActivationTime", default, skip_serializing_if = "Option::is_none")]
    pub last_activation_time: Option<String>,
}
pub mod application_package {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Pending,
        Active,
        Unmapped,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListApplicationsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationUpdateParameters {
    #[serde(rename = "allowUpdates", default, skip_serializing_if = "Option::is_none")]
    pub allow_updates: Option<bool>,
    #[serde(rename = "defaultVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchLocationQuota {
    #[serde(rename = "accountQuota", default, skip_serializing_if = "Option::is_none")]
    pub account_quota: Option<i32>,
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
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PoolAllocationMode {
    BatchService,
    UserSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateBaseProperties {
    #[serde(rename = "thumbprintAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub thumbprint_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<certificate_base_properties::Format>,
}
pub mod certificate_base_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Pfx,
        Cer,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(flatten)]
    pub certificate_base_properties: CertificateBaseProperties,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<certificate_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state_transition_time: Option<String>,
    #[serde(rename = "previousProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub previous_provisioning_state: Option<certificate_properties::PreviousProvisioningState>,
    #[serde(
        rename = "previousProvisioningStateTransitionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_provisioning_state_transition_time: Option<String>,
    #[serde(rename = "publicData", default, skip_serializing_if = "Option::is_none")]
    pub public_data: Option<String>,
    #[serde(rename = "deleteCertificateError", default, skip_serializing_if = "Option::is_none")]
    pub delete_certificate_error: Option<DeleteCertificateError>,
}
pub mod certificate_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreviousProvisioningState {
        Succeeded,
        Deleting,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateProperties {
    #[serde(flatten)]
    pub certificate_base_properties: CertificateBaseProperties,
    pub data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateCreateOrUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCertificatesResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Certificate>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteCertificateError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<DeleteCertificateError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<pool_properties::ProvisioningState>,
    #[serde(rename = "provisioningStateTransitionTime", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state_transition_time: Option<String>,
    #[serde(rename = "allocationState", default, skip_serializing_if = "Option::is_none")]
    pub allocation_state: Option<pool_properties::AllocationState>,
    #[serde(rename = "allocationStateTransitionTime", default, skip_serializing_if = "Option::is_none")]
    pub allocation_state_transition_time: Option<String>,
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(rename = "deploymentConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    #[serde(rename = "currentDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_dedicated_nodes: Option<i32>,
    #[serde(rename = "currentLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub current_low_priority_nodes: Option<i32>,
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<ScaleSettings>,
    #[serde(rename = "autoScaleRun", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale_run: Option<AutoScaleRun>,
    #[serde(rename = "interNodeCommunication", default, skip_serializing_if = "Option::is_none")]
    pub inter_node_communication: Option<pool_properties::InterNodeCommunication>,
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "maxTasksPerNode", default, skip_serializing_if = "Option::is_none")]
    pub max_tasks_per_node: Option<i32>,
    #[serde(rename = "taskSchedulingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub task_scheduling_policy: Option<TaskSchedulingPolicy>,
    #[serde(rename = "userAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub user_accounts: Vec<UserAccount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataItem>,
    #[serde(rename = "startTask", default, skip_serializing_if = "Option::is_none")]
    pub start_task: Option<StartTask>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<CertificateReference>,
    #[serde(rename = "applicationPackages", default, skip_serializing_if = "Vec::is_empty")]
    pub application_packages: Vec<ApplicationPackageReference>,
    #[serde(rename = "applicationLicenses", default, skip_serializing_if = "Vec::is_empty")]
    pub application_licenses: Vec<String>,
    #[serde(rename = "resizeOperationStatus", default, skip_serializing_if = "Option::is_none")]
    pub resize_operation_status: Option<ResizeOperationStatus>,
}
pub mod pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AllocationState {
        Steady,
        Resizing,
        Stopping,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum InterNodeCommunication {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    #[serde(rename = "cloudServiceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_configuration: Option<CloudServiceConfiguration>,
    #[serde(rename = "virtualMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_configuration: Option<VirtualMachineConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSettings {
    #[serde(rename = "fixedScale", default, skip_serializing_if = "Option::is_none")]
    pub fixed_scale: Option<FixedScaleSettings>,
    #[serde(rename = "autoScale", default, skip_serializing_if = "Option::is_none")]
    pub auto_scale: Option<AutoScaleSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleSettings {
    pub formula: String,
    #[serde(rename = "evaluationInterval", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_interval: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedScaleSettings {
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[serde(rename = "nodeDeallocationOption", default, skip_serializing_if = "Option::is_none")]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputeNodeDeallocationOption {
    Requeue,
    Terminate,
    TaskCompletion,
    RetainedData,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateReference {
    pub id: String,
    #[serde(rename = "storeLocation", default, skip_serializing_if = "Option::is_none")]
    pub store_location: Option<certificate_reference::StoreLocation>,
    #[serde(rename = "storeName", default, skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visibility: Vec<String>,
}
pub mod certificate_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StoreLocation {
        CurrentUser,
        LocalMachine,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageReference {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ResizeError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleRunError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<AutoScaleRunError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScaleRun {
    #[serde(rename = "evaluationTime")]
    pub evaluation_time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AutoScaleRunError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineConfiguration {
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[serde(rename = "nodeAgentSkuId")]
    pub node_agent_sku_id: String,
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowsConfiguration {
    #[serde(rename = "enableAutomaticUpdates", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_updates: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisk {
    pub lun: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<CachingType>,
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i32,
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSchedulingPolicy {
    #[serde(rename = "nodeFillType")]
    pub node_fill_type: task_scheduling_policy::NodeFillType,
}
pub mod task_scheduling_policy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NodeFillType {
        Spread,
        Pack,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxUserConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[serde(rename = "sshPrivateKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_private_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub password: String,
    #[serde(rename = "elevationLevel", default, skip_serializing_if = "Option::is_none")]
    pub elevation_level: Option<ElevationLevel>,
    #[serde(rename = "linuxUserConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_user_configuration: Option<LinuxUserConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartTask {
    #[serde(rename = "commandLine", default, skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
    #[serde(rename = "resourceFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_files: Vec<ResourceFile>,
    #[serde(rename = "environmentSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_settings: Vec<EnvironmentSetting>,
    #[serde(rename = "userIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
    #[serde(rename = "maxTaskRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub max_task_retry_count: Option<i32>,
    #[serde(rename = "waitForSuccess", default, skip_serializing_if = "Option::is_none")]
    pub wait_for_success: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceFile {
    #[serde(rename = "blobSource")]
    pub blob_source: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "fileMode", default, skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSetting {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "autoUser", default, skip_serializing_if = "Option::is_none")]
    pub auto_user: Option<AutoUserSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoUserSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<auto_user_specification::Scope>,
    #[serde(rename = "elevationLevel", default, skip_serializing_if = "Option::is_none")]
    pub elevation_level: Option<ElevationLevel>,
}
pub mod auto_user_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        Task,
        Pool,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ElevationLevel {
    NonAdmin,
    Admin,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StorageAccountType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CachingType {
    None,
    ReadOnly,
    ReadWrite,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<CachingType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "endpointConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<PoolEndpointConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServiceConfiguration {
    #[serde(rename = "osFamily")]
    pub os_family: String,
    #[serde(rename = "targetOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_os_version: Option<String>,
    #[serde(rename = "currentOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_os_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataItem {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeOperationStatus {
    #[serde(rename = "targetDedicatedNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_dedicated_nodes: Option<i32>,
    #[serde(rename = "targetLowPriorityNodes", default, skip_serializing_if = "Option::is_none")]
    pub target_low_priority_nodes: Option<i32>,
    #[serde(rename = "resizeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub resize_timeout: Option<String>,
    #[serde(rename = "nodeDeallocationOption", default, skip_serializing_if = "Option::is_none")]
    pub node_deallocation_option: Option<ComputeNodeDeallocationOption>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ResizeError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolEndpointConfiguration {
    #[serde(rename = "inboundNatPools")]
    pub inbound_nat_pools: Vec<InboundNatPool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundNatPool {
    pub name: String,
    pub protocol: inbound_nat_pool::Protocol,
    #[serde(rename = "backendPort")]
    pub backend_port: i32,
    #[serde(rename = "frontendPortRangeStart")]
    pub frontend_port_range_start: i32,
    #[serde(rename = "frontendPortRangeEnd")]
    pub frontend_port_range_end: i32,
    #[serde(rename = "networkSecurityGroupRules", default, skip_serializing_if = "Vec::is_empty")]
    pub network_security_group_rules: Vec<NetworkSecurityGroupRule>,
}
pub mod inbound_nat_pool {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupRule {
    pub priority: i32,
    pub access: network_security_group_rule::Access,
    #[serde(rename = "sourceAddressPrefix")]
    pub source_address_prefix: String,
}
pub mod network_security_group_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPoolsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Pool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
pub mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Batch/batchAccounts")]
        MicrosoftBatchBatchAccounts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
