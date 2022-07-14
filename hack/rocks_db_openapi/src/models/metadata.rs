use serde::{Serialize, Deserialize};
use poem_openapi::Object;
use crate::models::get_db_objects;

/// Get the current registry map in the store
pub fn list_repositories(root: &str, registry_id: &str) -> Option<Vec<Repository>> {
    get_db_objects(format!("{}/{}", root, registry_id).as_str(), COLUMN_NAME_REPOSITORY)
}

/// Get the current registry config from the store
pub fn list_repository_manifests(root: &str, registry_id: &str) -> Option<Vec<ManifestMetadata>> {
    get_db_objects(format!("{}/{}", root, registry_id).as_str(), COLUMN_NAME_REPOSITORY_MANIFEST_METADATA)
}

/// Get the current users in the store
pub fn list_repository_tags(root: &str, registry_id: &str) -> Option<Vec<TagMetadata>> {
    get_db_objects(format!("{}/{}", root, registry_id).as_str(), COLUMN_NAME_REPOSITORY_TAG_METADATA)
}

const COLUMN_NAME_REPOSITORY: &str = "Repository";
const COLUMN_NAME_REPOSITORY_MANIFEST_METADATA: &str = "RepositoryManifestMetadata";
const COLUMN_NAME_REPOSITORY_TAG_METADATA: &str = "RepositoryTagMetadata";
// Repository
// RepositoryManifestMetadata
// RepositoryTagMetadata

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct Repository {
    #[serde(rename = "NoDelete")]
    no_delete: bool,
    #[serde(rename = "NoChange")]
    no_change: bool,
    #[serde(rename = "NoList")]
    no_list: bool,
    #[serde(rename = "NoRead")]
    no_read: bool,
    #[serde(rename = "MetadataUpdateTime")]
    metadata_update_time: String,
    #[serde(rename = "LastUpdateTime")]
    last_update_time: String,
    #[serde(rename = "CreatedTime")]
    created_time: String,
    #[serde(rename = "IsNewRepository")]
    is_new_repository: bool,
    #[serde(rename = "RepositoryName")]
    repository_name: String,
    #[serde(rename = "Locked")]
    locked: bool,
    #[serde(rename = "LockedTimestamp")]
    locked_timestamp: String,
    #[serde(rename = "RepositoryID")]
    repository_id: String
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct TagMetadata {
    #[serde(rename = "NoDelete")]
    no_delete: bool,
    #[serde(rename = "NoChange")]
    no_change: bool,
    #[serde(rename = "NoList")]
    no_list: bool,
    #[serde(rename = "NoRead")]
    no_read: bool,
    #[serde(rename = "MetadataUpdateTime")]
    metadata_update_time: String,
    #[serde(rename = "LastUpdateTime")]
    last_update_time: String,
    #[serde(rename = "CreatedTime")]
    created_time: String,
    #[serde(rename = "RepositoryId")]
    repository_id: String,
    #[serde(rename = "Tag")]
    tag: String,
    #[serde(rename = "Digest")]
    digest: String,
    #[serde(rename = "SignatureRecord")]
    signature_record: Option<String>, 
    #[serde(rename = "QuarantineState")]
    quarantine_state: Option<String>
}

#[derive(Default, Object, Serialize, Deserialize, Debug)]
pub struct ManifestMetadata {
    #[serde(rename = "NoDelete")]
    no_delete: bool,
    #[serde(rename = "NoChange")]
    no_change: bool,
    #[serde(rename = "NoList")]
    no_list: bool,
    #[serde(rename = "NoRead")]
    no_read: bool,
    #[serde(rename = "MetadataUpdateTime")]
    metadata_update_time: String,
    #[serde(rename = "LastUpdateTime")]
    last_update_time: String,
    #[serde(rename = "CreatedTime")]
    created_time: String,
    #[serde(rename = "RepositoryId")]
    repository_id: String,
    #[serde(rename = "Digest")]
    digest: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "ImageSize")]
    image_size: i64,
    #[serde(rename = "OS")]
    os: Option<String>,
    #[serde(rename = "MediaType")]
    media_type: String,
    #[serde(rename = "ConfigMediaType")]
    config_media_type: Option<String>,
    #[serde(rename = "Architecture")]
    architecture: Option<String>,
    #[serde(rename = "References")]
    references: Option<String>,
    #[serde(rename = "QuarantineTag")]
    quarantine_tag: Option<String>,
    #[serde(rename = "QuarantineDetails")]
    quarantine_details: Option<String>,
    #[serde(rename = "QuarantineState")]
    quarantine_state: Option<String> 
}