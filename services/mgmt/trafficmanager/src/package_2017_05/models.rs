#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "endpointStatus", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<endpoint_properties::EndpointStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "endpointLocation", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_location: Option<String>,
    #[serde(rename = "endpointMonitorStatus", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_monitor_status: Option<endpoint_properties::EndpointMonitorStatus>,
    #[serde(rename = "minChildEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub min_child_endpoints: Option<i64>,
    #[serde(rename = "geoMapping", default, skip_serializing_if = "Vec::is_empty")]
    pub geo_mapping: Vec<String>,
}
pub mod endpoint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointStatus {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointMonitorStatus {
        CheckingEndpoint,
        Online,
        Degraded,
        Disabled,
        Inactive,
        Stopped,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckTrafficManagerRelativeDnsNameAvailabilityParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde(rename = "relativeName", default, skip_serializing_if = "Option::is_none")]
    pub relative_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorConfig {
    #[serde(rename = "profileMonitorStatus", default, skip_serializing_if = "Option::is_none")]
    pub profile_monitor_status: Option<monitor_config::ProfileMonitorStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<monitor_config::Protocol>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
    #[serde(rename = "toleratedNumberOfFailures", default, skip_serializing_if = "Option::is_none")]
    pub tolerated_number_of_failures: Option<i64>,
}
pub mod monitor_config {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProfileMonitorStatus {
        CheckingEndpoints,
        Online,
        Degraded,
        Disabled,
        Inactive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "HTTP")]
        Http,
        #[serde(rename = "HTTPS")]
        Https,
        #[serde(rename = "TCP")]
        Tcp,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileProperties {
    #[serde(rename = "profileStatus", default, skip_serializing_if = "Option::is_none")]
    pub profile_status: Option<profile_properties::ProfileStatus>,
    #[serde(rename = "trafficRoutingMethod", default, skip_serializing_if = "Option::is_none")]
    pub traffic_routing_method: Option<profile_properties::TrafficRoutingMethod>,
    #[serde(rename = "dnsConfig", default, skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    #[serde(rename = "monitorConfig", default, skip_serializing_if = "Option::is_none")]
    pub monitor_config: Option<MonitorConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,
}
pub mod profile_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProfileStatus {
        Enabled,
        Disabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrafficRoutingMethod {
        Performance,
        Priority,
        Weighted,
        Geographic,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficManagerNameAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<Region>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeographicHierarchyProperties {
    #[serde(rename = "geographicHierarchy", default, skip_serializing_if = "Option::is_none")]
    pub geographic_hierarchy: Option<Region>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrafficManagerGeographicHierarchy {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GeographicHierarchyProperties>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
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
