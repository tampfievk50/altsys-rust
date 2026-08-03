use serde::Serialize;
use utoipa::OpenApi;
use utoipa::openapi::path::PathItem;
use utoipa::ToSchema;

use sso_domain::dto::PermissionResponse::PermissionResponse;

use crate::openapi::ApiDoc;

/// A route the SSO service currently exposes, expressed the same way a
/// Permission is: `resource` is the literal OpenAPI path template (e.g.
/// `/api/v1/tenants/{id}`) and `action` is the HTTP method. The Casbin
/// matcher resolves these against real request paths via `keyMatch3`, which
/// understands the same `{param}` placeholder syntax utoipa already uses —
/// so no translation step is needed between "what utoipa documents" and
/// "what Casbin enforces".
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PermissionCatalogEntry {
    pub resource: String,
    pub action: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PermissionCatalogDiff {
    /// Routes with no matching Permission row yet.
    pub missing: Vec<PermissionCatalogEntry>,
    /// Permission rows that no longer correspond to any current route.
    /// Not auto-deleted — a role may still reference one, or it may be
    /// intentionally custom — surfaced for a human to review.
    pub orphaned: Vec<PermissionResponse>,
}

/// Auth routes (login/refresh/logout) are intentionally public and never
/// permission-gated, so they're excluded from the catalog.
const EXCLUDED_TAGS: &[&str] = &["Auth"];

/// Walks the service's own OpenAPI spec (the same one Swagger UI serves) to
/// build the canonical list of protected features. Reusing it instead of a
/// hand-maintained list means this can never drift from the real route
/// surface — every `#[utoipa::path(...)]` on a controller is automatically
/// a candidate permission.
pub fn current_feature_permissions() -> Vec<PermissionCatalogEntry> {
    let openapi = ApiDoc::openapi();
    let mut entries = Vec::new();

    for (path, item) in &openapi.paths.paths {
        entries.extend(operations(item).into_iter().filter_map(|(action, operation)| {
            let tags_excluded = operation
                .tags
                .as_ref()
                .is_some_and(|tags| tags.iter().any(|t| EXCLUDED_TAGS.contains(&t.as_str())));

            if tags_excluded {
                return None;
            }

            let name = operation
                .operation_id
                .clone()
                .unwrap_or_else(|| format!("{}_{}", action.to_lowercase(), path.replace(['/', '{', '}'], "_")));
            let description = operation.description.clone().or_else(|| operation.summary.clone());

            Some(PermissionCatalogEntry {
                resource: path.clone(),
                action: action.to_string(),
                name,
                description,
            })
        }));
    }

    entries
}

fn operations(item: &PathItem) -> Vec<(&'static str, &utoipa::openapi::path::Operation)> {
    [
        ("GET", &item.get),
        ("POST", &item.post),
        ("PUT", &item.put),
        ("DELETE", &item.delete),
        ("PATCH", &item.patch),
    ]
    .into_iter()
    .filter_map(|(method, op)| op.as_ref().map(|op| (method, op)))
    .collect()
}

/// Diffs the current route surface against the given Permission rows.
pub fn diff(existing: &[PermissionResponse]) -> PermissionCatalogDiff {
    let features = current_feature_permissions();

    let missing = features
        .iter()
        .filter(|f| !existing.iter().any(|p| p.resource == f.resource && p.action == f.action))
        .cloned()
        .collect();

    let orphaned = existing
        .iter()
        .filter(|p| !features.iter().any(|f| f.resource == p.resource && f.action == p.action))
        .cloned()
        .collect();

    PermissionCatalogDiff { missing, orphaned }
}
