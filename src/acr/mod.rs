use lifec::{plugins::ThunkContext, AttributeGraph};
use lifec_registry::MirrorEvent;
use poem::{Response, http::StatusCode, Body};
use serde::Deserialize;
use tracing::{event, Level};

/// Launchpad to expirament w/ new acr features
///
#[derive(Default, Debug)]
pub struct Acr {
    /// Enables teleport compatibility look-up
    enable_teleport: bool
}

impl Acr {
    fn resolve(&self, tc: &ThunkContext) -> poem::Response {
        match self {
            Self { enable_teleport: true, .. } => {
                event!(Level::DEBUG, "acr teleport is enabled, checking for accelerated image artifacts");
               
                if let Some(artifact) =  tc.as_ref().find_binary("artifacts").and_then(|b| serde_json::from_slice::<AcceleratedImageArtifact>(&b).ok()){
                    event!(Level::DEBUG, "found accelerated image artifact, {:#?}", artifact);
                    poem::Response::builder().body(artifact.get_manifest(tc))
                } else {
                    Response::builder().status(StatusCode::SERVICE_UNAVAILABLE).finish()
                }
            }
            _ => Response::builder().status(StatusCode::SERVICE_UNAVAILABLE).finish()
        }
    }
}

impl From<AttributeGraph> for Acr {
    fn from(value: AttributeGraph) -> Self {
        Self {
            enable_teleport: value.is_enabled("enable_teleport").unwrap_or_default()
        }
    }
}

impl MirrorEvent for Acr {
    fn resolve_response(tc: &lifec::plugins::ThunkContext) -> poem::Response {
        Acr::from(tc.as_ref().clone()).resolve(tc)
    }

    fn resolve_error(err: String, _: &lifec::plugins::ThunkContext) -> poem::Response {
        event!(Level::ERROR, "acr mirror resolving error {err}");
        Response::builder().status(StatusCode::SERVICE_UNAVAILABLE).body(err)
    }
}

#[derive(Deserialize, Debug)]
pub struct AcceleratedImageArtifact {
    // TODO 
}

impl AcceleratedImageArtifact {
    /// Gets the manifest from this artifact
    fn get_manifest(&self, tc: &ThunkContext) -> impl Into<Body> {
        // tc.debug();
        todo!()
    }
}
