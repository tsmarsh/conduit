use merkql::broker::{Broker, BrokerConfig};
use meshql_core::{GraphletteConfig, RestletteConfig, RootConfig, ServerConfig};
use meshql_merkql::{MerkqlRepository, MerkqlSearcher};
use std::path::PathBuf;
use std::sync::Arc;

// --- GraphQL schemas (11 event types) ---
const SYSTEM_REGISTERED_GRAPHQL: &str = include_str!("../config/graph/system_registered.graphql");
const SYSTEM_UPDATED_GRAPHQL: &str = include_str!("../config/graph/system_updated.graphql");
const SYSTEM_DECOMMISSIONED_GRAPHQL: &str =
    include_str!("../config/graph/system_decommissioned.graphql");
const ENVIRONMENT_ADDED_GRAPHQL: &str = include_str!("../config/graph/environment_added.graphql");
const ENVIRONMENT_REMOVED_GRAPHQL: &str =
    include_str!("../config/graph/environment_removed.graphql");
const DEPENDENCY_DECLARED_GRAPHQL: &str =
    include_str!("../config/graph/dependency_declared.graphql");
const DEPENDENCY_REMOVED_GRAPHQL: &str =
    include_str!("../config/graph/dependency_removed.graphql");
const ADVISORY_RAISED_GRAPHQL: &str = include_str!("../config/graph/advisory_raised.graphql");
const ADVISORY_ACKNOWLEDGED_GRAPHQL: &str =
    include_str!("../config/graph/advisory_acknowledged.graphql");
const ADVISORY_RESOLVED_GRAPHQL: &str = include_str!("../config/graph/advisory_resolved.graphql");
const ANNOTATION_ADDED_GRAPHQL: &str = include_str!("../config/graph/annotation_added.graphql");

// --- JSON schemas (11 event types) ---
const SYSTEM_REGISTERED_JSON: &str = include_str!("../config/json/system_registered.schema.json");
const SYSTEM_UPDATED_JSON: &str = include_str!("../config/json/system_updated.schema.json");
const SYSTEM_DECOMMISSIONED_JSON: &str =
    include_str!("../config/json/system_decommissioned.schema.json");
const ENVIRONMENT_ADDED_JSON: &str = include_str!("../config/json/environment_added.schema.json");
const ENVIRONMENT_REMOVED_JSON: &str =
    include_str!("../config/json/environment_removed.schema.json");
const DEPENDENCY_DECLARED_JSON: &str =
    include_str!("../config/json/dependency_declared.schema.json");
const DEPENDENCY_REMOVED_JSON: &str =
    include_str!("../config/json/dependency_removed.schema.json");
const ADVISORY_RAISED_JSON: &str = include_str!("../config/json/advisory_raised.schema.json");
const ADVISORY_ACKNOWLEDGED_JSON: &str =
    include_str!("../config/json/advisory_acknowledged.schema.json");
const ADVISORY_RESOLVED_JSON: &str = include_str!("../config/json/advisory_resolved.schema.json");
const ANNOTATION_ADDED_JSON: &str = include_str!("../config/json/annotation_added.schema.json");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("FUNCTIONS_CUSTOMHANDLER_PORT must be a valid port number");

    let data_path = std::env::var("MERKQL_DATA_PATH").unwrap_or_else(|_| "/mnt/merkql".to_string());

    let broker = Broker::open(BrokerConfig::new(PathBuf::from(&data_path)))?;

    // ===== REPOSITORIES (11) =====
    let system_registered_repo = Arc::new(MerkqlRepository::new(broker.clone(), "system_registered"));
    let system_updated_repo = Arc::new(MerkqlRepository::new(broker.clone(), "system_updated"));
    let system_decommissioned_repo = Arc::new(MerkqlRepository::new(broker.clone(), "system_decommissioned"));
    let environment_added_repo = Arc::new(MerkqlRepository::new(broker.clone(), "environment_added"));
    let environment_removed_repo = Arc::new(MerkqlRepository::new(broker.clone(), "environment_removed"));
    let dependency_declared_repo = Arc::new(MerkqlRepository::new(broker.clone(), "dependency_declared"));
    let dependency_removed_repo = Arc::new(MerkqlRepository::new(broker.clone(), "dependency_removed"));
    let advisory_raised_repo = Arc::new(MerkqlRepository::new(broker.clone(), "advisory_raised"));
    let advisory_acknowledged_repo = Arc::new(MerkqlRepository::new(broker.clone(), "advisory_acknowledged"));
    let advisory_resolved_repo = Arc::new(MerkqlRepository::new(broker.clone(), "advisory_resolved"));
    let annotation_added_repo = Arc::new(MerkqlRepository::new(broker.clone(), "annotation_added"));

    // ===== SEARCHERS (11) =====
    let system_registered_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "system_registered"));
    let system_updated_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "system_updated"));
    let system_decommissioned_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "system_decommissioned"));
    let environment_added_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "environment_added"));
    let environment_removed_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "environment_removed"));
    let dependency_declared_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "dependency_declared"));
    let dependency_removed_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "dependency_removed"));
    let advisory_raised_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "advisory_raised"));
    let advisory_acknowledged_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "advisory_acknowledged"));
    let advisory_resolved_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "advisory_resolved"));
    let annotation_added_searcher: Arc<dyn meshql_core::Searcher> =
        Arc::new(MerkqlSearcher::new(broker.clone(), "annotation_added"));

    // ===== ROOT CONFIGS (11) =====
    // Events are standalone — no cross-entity resolvers needed in Phase 1.
    // Future phases will add aggregate queries backed by DuckDB.

    let system_registered_config = RootConfig::builder()
        .singleton("getSystemRegistered", r#"{"id": "{{id}}"}"#)
        .vector("getSystemRegisteredEvents", "{}")
        .vector("getSystemRegisteredByName", r#"{"payload.name": "{{name}}"}"#)
        .vector("getSystemRegisteredByOwner", r#"{"payload.owner": "{{owner}}"}"#)
        .build();

    let system_updated_config = RootConfig::builder()
        .singleton("getSystemUpdated", r#"{"id": "{{id}}"}"#)
        .vector("getSystemUpdatedEvents", "{}")
        .vector("getSystemUpdatedBySystemId", r#"{"payload.systemId": "{{systemId}}"}"#)
        .build();

    let system_decommissioned_config = RootConfig::builder()
        .singleton("getSystemDecommissioned", r#"{"id": "{{id}}"}"#)
        .vector("getSystemDecommissionedEvents", "{}")
        .vector("getSystemDecommissionedBySystemId", r#"{"payload.systemId": "{{systemId}}"}"#)
        .build();

    let environment_added_config = RootConfig::builder()
        .singleton("getEnvironmentAdded", r#"{"id": "{{id}}"}"#)
        .vector("getEnvironmentAddedEvents", "{}")
        .vector("getEnvironmentAddedBySystemId", r#"{"payload.systemId": "{{systemId}}"}"#)
        .build();

    let environment_removed_config = RootConfig::builder()
        .singleton("getEnvironmentRemoved", r#"{"id": "{{id}}"}"#)
        .vector("getEnvironmentRemovedEvents", "{}")
        .vector("getEnvironmentRemovedByEnvironmentId", r#"{"payload.environmentId": "{{environmentId}}"}"#)
        .build();

    let dependency_declared_config = RootConfig::builder()
        .singleton("getDependencyDeclared", r#"{"id": "{{id}}"}"#)
        .vector("getDependencyDeclaredEvents", "{}")
        .vector("getDependencyDeclaredByFrom", r#"{"payload.fromEnvironmentId": "{{fromEnvironmentId}}"}"#)
        .vector("getDependencyDeclaredByTo", r#"{"payload.toEnvironmentId": "{{toEnvironmentId}}"}"#)
        .build();

    let dependency_removed_config = RootConfig::builder()
        .singleton("getDependencyRemoved", r#"{"id": "{{id}}"}"#)
        .vector("getDependencyRemovedEvents", "{}")
        .vector("getDependencyRemovedByDependencyId", r#"{"payload.dependencyId": "{{dependencyId}}"}"#)
        .build();

    let advisory_raised_config = RootConfig::builder()
        .singleton("getAdvisoryRaised", r#"{"id": "{{id}}"}"#)
        .vector("getAdvisoryRaisedEvents", "{}")
        .vector("getAdvisoryRaisedByTarget", r#"{"payload.targetId": "{{targetId}}"}"#)
        .vector("getAdvisoryRaisedBySeverity", r#"{"payload.severity": "{{severity}}"}"#)
        .build();

    let advisory_acknowledged_config = RootConfig::builder()
        .singleton("getAdvisoryAcknowledged", r#"{"id": "{{id}}"}"#)
        .vector("getAdvisoryAcknowledgedEvents", "{}")
        .vector("getAdvisoryAcknowledgedByAdvisoryId", r#"{"payload.advisoryId": "{{advisoryId}}"}"#)
        .build();

    let advisory_resolved_config = RootConfig::builder()
        .singleton("getAdvisoryResolved", r#"{"id": "{{id}}"}"#)
        .vector("getAdvisoryResolvedEvents", "{}")
        .vector("getAdvisoryResolvedByAdvisoryId", r#"{"payload.advisoryId": "{{advisoryId}}"}"#)
        .build();

    let annotation_added_config = RootConfig::builder()
        .singleton("getAnnotationAdded", r#"{"id": "{{id}}"}"#)
        .vector("getAnnotationAddedEvents", "{}")
        .vector("getAnnotationAddedByTarget", r#"{"payload.targetId": "{{targetId}}"}"#)
        .build();

    // ===== SERVER CONFIG =====
    let config = ServerConfig {
        port,
        graphlettes: vec![
            GraphletteConfig {
                path: "/system_registered/graph".to_string(),
                schema_text: SYSTEM_REGISTERED_GRAPHQL.to_string(),
                root_config: system_registered_config,
                searcher: system_registered_searcher,
            },
            GraphletteConfig {
                path: "/system_updated/graph".to_string(),
                schema_text: SYSTEM_UPDATED_GRAPHQL.to_string(),
                root_config: system_updated_config,
                searcher: system_updated_searcher,
            },
            GraphletteConfig {
                path: "/system_decommissioned/graph".to_string(),
                schema_text: SYSTEM_DECOMMISSIONED_GRAPHQL.to_string(),
                root_config: system_decommissioned_config,
                searcher: system_decommissioned_searcher,
            },
            GraphletteConfig {
                path: "/environment_added/graph".to_string(),
                schema_text: ENVIRONMENT_ADDED_GRAPHQL.to_string(),
                root_config: environment_added_config,
                searcher: environment_added_searcher,
            },
            GraphletteConfig {
                path: "/environment_removed/graph".to_string(),
                schema_text: ENVIRONMENT_REMOVED_GRAPHQL.to_string(),
                root_config: environment_removed_config,
                searcher: environment_removed_searcher,
            },
            GraphletteConfig {
                path: "/dependency_declared/graph".to_string(),
                schema_text: DEPENDENCY_DECLARED_GRAPHQL.to_string(),
                root_config: dependency_declared_config,
                searcher: dependency_declared_searcher,
            },
            GraphletteConfig {
                path: "/dependency_removed/graph".to_string(),
                schema_text: DEPENDENCY_REMOVED_GRAPHQL.to_string(),
                root_config: dependency_removed_config,
                searcher: dependency_removed_searcher,
            },
            GraphletteConfig {
                path: "/advisory_raised/graph".to_string(),
                schema_text: ADVISORY_RAISED_GRAPHQL.to_string(),
                root_config: advisory_raised_config,
                searcher: advisory_raised_searcher,
            },
            GraphletteConfig {
                path: "/advisory_acknowledged/graph".to_string(),
                schema_text: ADVISORY_ACKNOWLEDGED_GRAPHQL.to_string(),
                root_config: advisory_acknowledged_config,
                searcher: advisory_acknowledged_searcher,
            },
            GraphletteConfig {
                path: "/advisory_resolved/graph".to_string(),
                schema_text: ADVISORY_RESOLVED_GRAPHQL.to_string(),
                root_config: advisory_resolved_config,
                searcher: advisory_resolved_searcher,
            },
            GraphletteConfig {
                path: "/annotation_added/graph".to_string(),
                schema_text: ANNOTATION_ADDED_GRAPHQL.to_string(),
                root_config: annotation_added_config,
                searcher: annotation_added_searcher,
            },
        ],
        restlettes: vec![
            RestletteConfig {
                path: "/system_registered/api".to_string(),
                schema_json: serde_json::from_str(SYSTEM_REGISTERED_JSON)
                    .expect("invalid system_registered JSON schema"),
                repository: system_registered_repo,
            },
            RestletteConfig {
                path: "/system_updated/api".to_string(),
                schema_json: serde_json::from_str(SYSTEM_UPDATED_JSON)
                    .expect("invalid system_updated JSON schema"),
                repository: system_updated_repo,
            },
            RestletteConfig {
                path: "/system_decommissioned/api".to_string(),
                schema_json: serde_json::from_str(SYSTEM_DECOMMISSIONED_JSON)
                    .expect("invalid system_decommissioned JSON schema"),
                repository: system_decommissioned_repo,
            },
            RestletteConfig {
                path: "/environment_added/api".to_string(),
                schema_json: serde_json::from_str(ENVIRONMENT_ADDED_JSON)
                    .expect("invalid environment_added JSON schema"),
                repository: environment_added_repo,
            },
            RestletteConfig {
                path: "/environment_removed/api".to_string(),
                schema_json: serde_json::from_str(ENVIRONMENT_REMOVED_JSON)
                    .expect("invalid environment_removed JSON schema"),
                repository: environment_removed_repo,
            },
            RestletteConfig {
                path: "/dependency_declared/api".to_string(),
                schema_json: serde_json::from_str(DEPENDENCY_DECLARED_JSON)
                    .expect("invalid dependency_declared JSON schema"),
                repository: dependency_declared_repo,
            },
            RestletteConfig {
                path: "/dependency_removed/api".to_string(),
                schema_json: serde_json::from_str(DEPENDENCY_REMOVED_JSON)
                    .expect("invalid dependency_removed JSON schema"),
                repository: dependency_removed_repo,
            },
            RestletteConfig {
                path: "/advisory_raised/api".to_string(),
                schema_json: serde_json::from_str(ADVISORY_RAISED_JSON)
                    .expect("invalid advisory_raised JSON schema"),
                repository: advisory_raised_repo,
            },
            RestletteConfig {
                path: "/advisory_acknowledged/api".to_string(),
                schema_json: serde_json::from_str(ADVISORY_ACKNOWLEDGED_JSON)
                    .expect("invalid advisory_acknowledged JSON schema"),
                repository: advisory_acknowledged_repo,
            },
            RestletteConfig {
                path: "/advisory_resolved/api".to_string(),
                schema_json: serde_json::from_str(ADVISORY_RESOLVED_JSON)
                    .expect("invalid advisory_resolved JSON schema"),
                repository: advisory_resolved_repo,
            },
            RestletteConfig {
                path: "/annotation_added/api".to_string(),
                schema_json: serde_json::from_str(ANNOTATION_ADDED_JSON)
                    .expect("invalid annotation_added JSON schema"),
                repository: annotation_added_repo,
            },
        ],
    };

    meshql_server::run(config).await
}
