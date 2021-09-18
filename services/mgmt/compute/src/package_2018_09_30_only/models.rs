#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskList {
    pub value: Vec<Disk>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<disk_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
pub mod disk_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(rename = "UltraSSD_LRS")]
        UltraSsdLrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<snapshot_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
pub mod snapshot_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_properties::OsType>,
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_properties::HyperVGeneration>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i32>,
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<disk_properties::DiskState>,
}
pub mod disk_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DiskState {
        Unattached,
        Attached,
        Reserved,
        #[serde(rename = "ActiveSAS")]
        ActiveSas,
        ReadyToUpload,
        ActiveUpload,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_properties::OsType>,
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<snapshot_properties::HyperVGeneration>,
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
pub mod snapshot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HyperVGeneration {
        V1,
        V2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsCollection {
    pub enabled: bool,
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<EncryptionSettingsElement>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsElement {
    #[serde(rename = "diskEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndSecretReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndKeyReference {
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceVault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskUpdateProperties {
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i32>,
}
pub mod disk_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotUpdateProperties {
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_update_properties::OsType>,
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
}
pub mod snapshot_update_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationData {
    #[serde(rename = "createOption")]
    pub create_option: creation_data::CreateOption,
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
pub mod creation_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Empty,
        Attach,
        FromImage,
        Import,
        Copy,
        Restore,
        Upload,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDiskReference {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantAccessData {
    pub access: grant_access_data::Access,
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i32,
}
pub mod grant_access_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        None,
        Read,
        Write,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessUri {
    #[serde(rename = "accessSAS", default, skip_serializing_if = "Option::is_none")]
    pub access_sas: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotUpdateProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotList {
    pub value: Vec<Snapshot>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
