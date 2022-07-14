use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use poem_openapi::Object;
use crate::models::get_db_objects;

/// Get the current registry map in the store
pub fn get_registry_map_store(path: &str) -> Option<Vec<RegistryMapStore>> {
    get_db_objects(path, COLUMN_NAME_REGISTRY_MAP_STORE)
}

/// Get the current registry config from the store
pub fn get_registry_config_store_config(path: &str) -> Option<Vec<Config>> {
    get_db_objects(path, COLUMN_NAME_ON_PREM_CONFIG_STORE_CONFIG)
}

/// Get the current users in the store
pub fn get_registry_config_store_users(path: &str) -> Option<Vec<BTreeMap<String, User>>> {
    get_db_objects(path, COLUMN_NAME_ON_PREM_CONFIG_STORE_USERS)
}

const COLUMN_NAME_REGISTRY_MAP_STORE: &str = "RegistryMapStore";
const COLUMN_NAME_ON_PREM_CONFIG_STORE_CONFIG: &str = "onpremconfigstore-config";
const COLUMN_NAME_ON_PREM_CONFIG_STORE_USERS: &str = "onpremconfigstore-users";

#[allow(dead_code)]
const COLUMN_NAME_ON_PREM_CONFIG_STORE_REPORTED_CONFIG: &str = "onpremconfigstore-reported-config";

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct RegistryMapStore {
    #[serde(rename = "Children")]
    children: Vec<String>, 
    #[serde(rename = "ConnectedRegistryId")]
    connected_registry_id: String,
    #[serde(rename = "ParentRegistryId")]
    parent_registry_id: String, 
    #[serde(rename = "ConnectedRegistryName")]
    connected_registry_name: String,
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Config {
        #[serde(rename = "authModes")]
        authorization_modes: Vec<String>,
        #[serde(rename = "authSvr")]
        authorization_server: Option<String>,
        #[serde(rename = "configRev")]
        config_revision_id: String,
        #[serde(rename = "connRegId")]
        connected_registry_id: String,
        #[serde(rename = "connRegName")]
        connected_registry_name: String,
        #[serde(rename = "connRegPath")]
        connected_registry_path: String,
        #[serde(rename = "loginSvr")]
        login_server: String,
        #[serde(rename = "loginSvrProps")]
        login_server_properties: Option<String>,
        #[serde(rename = "ltRepMsgSeqNum")]
        latest_reported_message_seq_number: Option<String>,
        mode: String,
        #[serde(rename = "notfLst")]
        notifications_list: Option<String>,
        #[serde(rename = "parentRegId")]
        parent_registry_id: String,
        #[serde(rename = "rtVer")]
        runtime_version: Option<String>,
        #[serde(rename = "act")]
        activation: Activation,
        logging: Log,
        parent: Parent,
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct LoginServerProperties {
        host: String,
        tls: TlsProperties
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct TlsProperties {
        #[serde(rename = "stat")]
        status: String,
        #[serde(rename = "cert")]
        certificate: TlsCertificateProperties
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct TlsCertificateProperties {
        #[serde(rename = "type")]
        certificate_type: String,
        #[serde(rename = "loc")]
        location: String,
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Parent {
        id: Option<String>,
        #[serde(rename = "syncProps")]
        sync_properties: Sync
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct User {
        #[serde(rename = "connRegId")]
        connected_registry_id: String,
        #[serde(rename = "parentRegId")]
        parent_registry_id: String,
        #[serde(rename = "permJsons")]
        permission_jsons: Vec<String>,
        #[serde(rename = "pwdJson")]
        password_json: String,
        #[serde(rename = "userId")]
        user_id: String,
        #[serde(rename = "userName")]
        user_name: String
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Log {
        #[serde(rename = "auditLogFlag")]
        audit_log_enabled: bool,
        #[serde(rename = "logLevel")]
        log_level: String
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Activation {
        #[serde(rename = "actId")]
        activation_id: Option<String>,
        status: String,
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Sync {
        #[serde(rename = "gwEp")]
        gateway_endpoint: Option<String>,
        #[serde(rename = "lastSyncTime")]
        last_sync_time: Option<String>,
        #[serde(rename = "msgTtl")]
        message_ttl: String,
        #[serde(rename = "nextSyncTime")]
        next_sync_time: Option<String>,
        #[serde(rename = "sch")]
        schedule: String,
        #[serde(rename = "syncWin")]
        sync_window: String,
        #[serde(rename = "tokenConfig")]
        token_config: User
    }

