use openapiv3::{OpenAPI, Operation, ReferenceOr};

/// Newtype wrapper around `OpenAPI` to provide mutable operations iterator.
pub struct OpenAPIExt<'a> {
    pub openapi: &'a mut OpenAPI,
}

impl OpenAPIExt<'_> {
    pub fn new(openapi: &mut OpenAPI) -> OpenAPIExt {
        OpenAPIExt { openapi }
    }

    /// Mutably iterates through all [Operation]s in this API.
    ///
    /// The iterated items are tuples of `(&str, &str, &mut Operation)` containing
    /// the path, method, and the operation.
    ///
    /// Path items containing `$ref`s are skipped.
    pub fn operations_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut Operation)> {
        self.openapi
            .paths
            .paths
            .iter_mut()
            .filter_map(|(path, item)| match *item {
                ReferenceOr::Item(ref mut path_item) => Some((path, path_item)),
                ReferenceOr::Reference { .. } => None,
            })
            .flat_map(|(path, path_item)| {
                vec![
                    (path.as_str(), "get", &mut path_item.get),
                    (path.as_str(), "put", &mut path_item.put),
                    (path.as_str(), "post", &mut path_item.post),
                    (path.as_str(), "delete", &mut path_item.delete),
                    (path.as_str(), "options", &mut path_item.options),
                    (path.as_str(), "head", &mut path_item.head),
                    (path.as_str(), "patch", &mut path_item.patch),
                    (path.as_str(), "trace", &mut path_item.trace),
                ]
                .into_iter()
                .filter_map(|(path, method, maybe_op)| {
                    maybe_op.as_mut().map(|op| (path, method, op))
                })
            })
    }
}

#[test]
fn test_add_operation_ids() {
    const SCHEMA: &str = r#"
{
    "openapi": "3.0.0",
    "info": {
        "title": "Test API",
        "version": "1.0.0"
    },
    "paths": {
        "/foo": {
            "get": {
                "responses": {}
            }
        }
    }
}
"#;

    let mut openapi: OpenAPI = serde_json::from_str(SCHEMA).unwrap();
    let mut oa_ext = OpenAPIExt::new(&mut openapi);
    for (path, method, op) in oa_ext.operations_mut() {
        op.operation_id = Some(format!("{}_{}", path, method));
    }

    assert_eq!(
        openapi.paths.paths["/foo"]
            .as_item()
            .unwrap()
            .get
            .as_ref()
            .unwrap()
            .operation_id,
        Some("/foo_get".to_string())
    );
}
