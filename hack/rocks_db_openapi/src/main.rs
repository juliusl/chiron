use poem::{
    endpoint::StaticFilesEndpoint,
    http::StatusCode,
    listener::TcpListener,
    Route, Server,
};
use poem_openapi::{
    param::Query,
    payload::{Json, Response},
    OpenApi, OpenApiService,
};
use std::collections::BTreeMap;

use lifec::plugins::ThunkContext;
use lifec_poem::WebApp;

mod models;
use models::config::{
    get_registry_config_store_config, get_registry_config_store_users, get_registry_map_store,
    Config, RegistryMapStore, User,
};

use models::metadata::{
    list_repositories, list_repository_manifests, list_repository_tags, ManifestMetadata,
    Repository, TagMetadata,
};

const CONFIG_STORE_ROOT_PATH: &str = "/var/acr/data/rocksdb/config";
const METADATA_STORE_ROOT_PATH: &str = "/var/acr/data/rocksdb/metadata";

struct Api(
    /// config store path
    String,
    /// metadata store path
    String, 
);

#[OpenApi]
impl Api {
    #[oai(path = "/config", method = "get")]
    async fn config(&self) -> Response<Json<Vec<Config>>> {
        let Self(config_store_path, _) = self;

        match get_registry_config_store_config(config_store_path) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }

    #[oai(path = "/config/registrymap", method = "get")]
    async fn config_registry_map(&self) -> Response<Json<Vec<RegistryMapStore>>> {
        match get_registry_map_store(CONFIG_STORE_ROOT_PATH) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }
    #[oai(path = "/config/users", method = "get")]
    async fn config_users(&self) -> Response<Json<Vec<BTreeMap<String, User>>>> {
        match get_registry_config_store_users(CONFIG_STORE_ROOT_PATH) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }

    #[oai(path = "/metadata/repositories", method = "get")]
    async fn metadata_repositories(
        &self,
        registry_id: Query<String>,
    ) -> Response<Json<Vec<Repository>>> {
        match list_repositories(METADATA_STORE_ROOT_PATH, registry_id.as_str()) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }

    #[oai(path = "/metadata/manifests", method = "get")]
    async fn metadata_manifests(
        &self,
        registry_id: Query<String>,
    ) -> Response<Json<Vec<ManifestMetadata>>> {
        match list_repository_manifests(METADATA_STORE_ROOT_PATH, registry_id.as_str()) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }

    #[oai(path = "/metadata/tags", method = "get")]
    async fn metadata_tags(&self, registry_id: Query<String>) -> Response<Json<Vec<TagMetadata>>> {
        match list_repository_tags(METADATA_STORE_ROOT_PATH, registry_id.as_str()) {
            Some(map) => Response::new(Json(map)),
            _ => Response::new(Json(vec![])).status(StatusCode::NOT_FOUND),
        }
    }
}

impl WebApp for Api {
    fn create(_: &mut ThunkContext) -> Self {
        todo!()
    }

    fn routes(&mut self) -> Route {
        let api = Api(CONFIG_STORE_ROOT_PATH.to_string(), METADATA_STORE_ROOT_PATH.to_string());
        let api_service = OpenApiService::new(api, "OnPrem Connected Registry", "0.1")
            .server("http://localhost:8000/api");

        let swagger = api_service.swagger_ui();

        Route::new()
            .nest("/api", api_service)
            .nest("/swagger", swagger)
            .at(
                "/",
                StaticFilesEndpoint::new("/root/dash").index_file("index.html"),
            )
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(
            Api(
                CONFIG_STORE_ROOT_PATH.to_string(), 
                METADATA_STORE_ROOT_PATH.to_string()
            ), 
    "OnPrem Connected Registry", 
        "0.1"
        ).server("http://localhost:8000/api");

    let swagger = api_service.swagger_ui();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/swagger", swagger)
        .at(
            "/",
            StaticFilesEndpoint::new("/root/dash").index_file("index.html"),
        );

    // Enable TLS
    // let key = fs::read_to_string("/root/certs/tls.key").unwrap();
    // let cert = fs::read_to_string("/root/certs/tls.crt").unwrap();

    // Server::new(TcpListener::bind("0.0.0.0:8443")
    //     .rustls(RustlsConfig::new().key(key).cert(cert)))
    //     .run(app)
    //     .await

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
}
