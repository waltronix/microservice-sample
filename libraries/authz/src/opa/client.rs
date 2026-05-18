use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::backend::{AuthzBackend, AuthzError};

#[derive(Clone, Debug)]
pub struct OpaClient {
    http: Client,
    base_url: String,
}

impl OpaClient {
    pub fn new(base_url: String) -> Self {
        Self { http: Client::new(), base_url }
    }

    /// OPA data path for a tuple fact.
    /// Layout: /v1/data/library/tuples/<obj_type>/<obj_id>/<relation>/<usr_type>/<usr_id>
    ///
    /// Group-member subjects ("group:uuid#member") are stored with user_type
    /// "group#member" and user_id equal to the group UUID, matching the Rego
    /// policy's expansion rules.
    ///
    /// Tildes and slashes within segments are JSON-Pointer-encoded (~0 / ~1).
    fn tuple_path(user: &str, relation: &str, object: &str) -> String {
        fn encode(s: &str) -> String {
            s.replace('~', "~0").replace('/', "~1")
        }
        let (obj_type, obj_id) = split_colon(object);
        let (usr_type, usr_id) = if let Some(group_id) = user
            .strip_prefix("group:")
            .and_then(|s| s.strip_suffix("#member"))
        {
            ("group#member", group_id)
        } else {
            split_colon(user)
        };
        format!(
            "/v1/data/library/tuples/{}/{}/{}/{}/{}",
            encode(obj_type),
            encode(obj_id),
            encode(relation),
            encode(usr_type),
            encode(usr_id),
        )
    }
}

fn split_colon(s: &str) -> (&str, &str) {
    s.split_once(':').unwrap_or(("", s))
}

#[derive(Deserialize)]
struct CheckResponse {
    result: Option<bool>,
}

#[async_trait]
impl AuthzBackend for OpaClient {
    async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError> {
        let url = format!("{}/v1/data/library/authz/allow", self.base_url);
        let resp = self
            .http
            .post(&url)
            .json(&json!({ "input": { "user": user, "relation": relation, "object": object } }))
            .send()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AuthzError::Infrastructure(format!("{status}: {text}")));
        }

        let body: CheckResponse = resp
            .json()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;
        Ok(body.result.unwrap_or(false))
    }

    async fn write_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        let url = format!("{}{}", self.base_url, Self::tuple_path(user, relation, object));
        let resp = self
            .http
            .put(&url)
            .json(&true)
            .send()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AuthzError::Infrastructure(format!("{status}: {text}")));
        }
        Ok(())
    }

    async fn delete_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        let url = format!("{}{}", self.base_url, Self::tuple_path(user, relation, object));
        let resp = self
            .http
            .delete(&url)
            .send()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AuthzError::Infrastructure(format!("{status}: {text}")));
        }
        Ok(())
    }
}
