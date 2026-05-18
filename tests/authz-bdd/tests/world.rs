use cucumber::World;
use uuid::Uuid;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct E2eWorld {
    pub http: reqwest::Client,
    pub last_status: u16,
    pub group_id: Option<Uuid>,
    pub book_id: Option<Uuid>,
    pub review_id: Option<Uuid>,
}

impl E2eWorld {
    async fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            last_status: 0,
            group_id: None,
            book_id: None,
            review_id: None,
        }
    }

    /// Stable UUID for a test user, derived from their name.
    pub fn user_id(name: &str) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes())
    }

    pub async fn assert_status(&self, method: reqwest::Method, url: String, user: &str, expected: u16) {
        let status = self
            .http
            .request(method, &url)
            .header("X-User-Id", Self::user_id(user).to_string())
            .send()
            .await
            .expect("HTTP request")
            .status()
            .as_u16();
        assert_eq!(status, expected, "expected {expected} from {url}, got {status}");
    }
}
