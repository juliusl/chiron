use lifec::{plugins::ThunkContext, AttributeGraph, Component, DefaultVecStorage};
use lifec_registry::MirrorEvent;
use poem::{http::StatusCode, Response};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{event, Level};

/// Launchpad to experiment w/ new acr features
///
#[derive(Default, Debug)]
pub struct Acr {
    /// Enables teleport compatibility look-up
    enable_teleport: bool,
    /// Returns the manifest returned by the resolve plugin,
    ///
    /// **Note** For the ACR case this is useful because creds on the machine will be resolved either by
    /// service principal or dev azure cli credentials, so using this resolver would mean additional docker login is not
    /// required
    enable_resolver: bool,
}

/// Registry descriptor data schema
///
/// A descriptor is a common specification registries use to reference content,
///
/// Caveat: The content of a descriptor matters, once a client pushes a descriptor to a registry,
/// **no** fields may change, this will change the effective content digest.
///
#[derive(Default, Component, Deserialize, Serialize, Debug)]
#[storage(DefaultVecStorage)]
pub struct Descriptor {
    #[serde(rename = "mediaType")]
    media_type: String,
    #[serde(rename = "artifactType")]
    artifact_type: Option<String>,
    #[serde(rename = "digest")]
    digest: String,
    #[serde(rename = "size")]
    size: u64,
    #[serde(rename = "annotations")]
    annotations: Option<BTreeMap<String, String>>,
    #[serde(rename = "urls")]
    urls: Option<Vec<String>>,
    #[serde(rename = "data")]
    data: Option<String>,
    platform: Option<Platform>,
}

/// Manifest struct for stored artifacts related to an image
///
/// Artifacts are data related to the image, but that are not directly part of any of the
/// image layers that make up the container's filesystem.
///
/// An artifact can be literally anything, but example usages include sbom's, signatures, etc.
/// In ACR's case, it is used to extend images
///
#[derive(Default, Component, Deserialize, Serialize, Debug)]
#[storage(DefaultVecStorage)]
pub struct ArtifactManifest {
    #[serde(rename = "mediaType")]
    media_type: String,
    #[serde(rename = "artifactType")]
    artifact_type: String,
    #[serde(rename = "blobs")]
    blobs: Vec<Descriptor>,
    #[serde(rename = "subject")]
    subject: Descriptor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    architecture: String,
    os: String,
    variant: Option<String>
}

/// Format of the response from the "referrers" api
///
#[derive(Component, Default, Debug, Deserialize, Serialize)]
#[storage(DefaultVecStorage)]
pub struct ReferrersResponse {
    referrers: Vec<Descriptor>,
}

impl Acr {
    // Handles conditions for
    fn resolve(&self, tc: &ThunkContext) -> poem::Response {
        event!(
            Level::DEBUG,
            "resolving mirror response"
        );
        match self {
            Self {
                enable_teleport: true,
                ..
            } => {
                event!(
                    Level::DEBUG,
                    "acr teleport is enabled, checking for accelerated image artifacts"
                );

                if let Some(referrers) = tc
                    .as_ref()
                    .find_binary("referrers")
                    .and_then(|b| serde_json::from_slice::<ReferrersResponse>(&b).ok())
                {
                    event!(Level::DEBUG, "found referrers, {:#?}", referrers);

                    todo!()
                } else {
                    event!(Level::DEBUG, "no referrers attribute was found");
                    // Fall-back response
                    Response::builder()
                        .status(StatusCode::SERVICE_UNAVAILABLE)
                        .finish()
                }
            }

            Self {
                enable_resolver: true,
                ..
            } => {
                event!(Level::DEBUG, "no referrers attribute was found");
                // Fall-back response
                Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .finish()
            }
            // Fall-back response
            _ => {
                event!(
                    Level::DEBUG,
                    "no extended features were enabled, falling back"
                );
                Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .finish()
            }
        }
    }
}

impl From<AttributeGraph> for Acr {
    fn from(value: AttributeGraph) -> Self {
        Self {
            enable_teleport: value.is_enabled("enable_teleport").unwrap_or_default(),
            enable_resolver: value.is_enabled("enable_resolver").unwrap_or_default(),
        }
    }
}

impl MirrorEvent for Acr {
    fn resolve_response(tc: &lifec::plugins::ThunkContext) -> poem::Response {
        Acr::from(tc.as_ref().clone()).resolve(tc)
    }

    fn resolve_error(err: String, _: &lifec::plugins::ThunkContext) -> poem::Response {
        event!(Level::ERROR, "acr mirror resolving error {err}");
        Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(err)
    }
}
