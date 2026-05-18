use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::backend::{AuthzBackend, AuthzError};

#[derive(Clone, Debug)]
pub struct OpenFgaClient {
    http: Client,
    base_url: String,
    store_id: String,
    model_id: String,
}

#[derive(Serialize)]
struct CheckBody<'a> {
    tuple_key: TupleKey<'a>,
    authorization_model_id: &'a str,
}

#[derive(Serialize)]
struct WriteBody<'a> {
    writes: Tuples<'a>,
    authorization_model_id: &'a str,
}

#[derive(Serialize)]
struct DeleteBody<'a> {
    deletes: Tuples<'a>,
    authorization_model_id: &'a str,
}

#[derive(Serialize)]
struct Tuples<'a> {
    tuple_keys: Vec<TupleKey<'a>>,
}

#[derive(Serialize)]
struct TupleKey<'a> {
    user: &'a str,
    relation: &'a str,
    object: &'a str,
}

#[derive(Deserialize)]
struct CheckResponse {
    allowed: Option<bool>,
}

impl OpenFgaClient {
    pub fn new(base_url: String, store_id: String, model_id: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
            store_id,
            model_id,
        }
    }

    pub async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError> {
        let url = format!("{}/stores/{}/check", self.base_url, self.store_id);
        let body = CheckBody {
            tuple_key: TupleKey { user, relation, object },
            authorization_model_id: &self.model_id,
        };
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AuthzError::Infrastructure(format!("{status}: {text}")));
        }

        let check: CheckResponse = resp
            .json()
            .await
            .map_err(|e| AuthzError::Infrastructure(e.to_string()))?;
        Ok(check.allowed.unwrap_or(false))
    }

    pub async fn write_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        let url = format!("{}/stores/{}/write", self.base_url, self.store_id);
        let body = WriteBody {
            writes: Tuples {
                tuple_keys: vec![TupleKey { user, relation, object }],
            },
            authorization_model_id: &self.model_id,
        };
        let resp = self
            .http
            .post(&url)
            .json(&body)
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

    pub async fn delete_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        let url = format!("{}/stores/{}/write", self.base_url, self.store_id);
        let body = DeleteBody {
            deletes: Tuples {
                tuple_keys: vec![TupleKey { user, relation, object }],
            },
            authorization_model_id: &self.model_id,
        };
        let resp = self
            .http
            .post(&url)
            .json(&body)
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

    /// Enforce a permission check, returning Forbidden if not allowed.
    pub async fn require(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        if self.check(user, relation, object).await? {
            Ok(())
        } else {
            Err(AuthzError::Forbidden)
        }
    }
}

#[async_trait]
impl AuthzBackend for OpenFgaClient {
    async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError> {
        self.check(user, relation, object).await
    }

    async fn write_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        self.write_tuple(user, relation, object).await
    }

    async fn delete_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        self.delete_tuple(user, relation, object).await
    }
}
