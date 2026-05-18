use cucumber::given;

use crate::stack::{ensure_stack, fga_write_tuple, opa_write_tuple, AuthzSetup};
use crate::world::E2eWorld;

#[given("no preconditions")]
pub async fn no_preconditions(_world: &mut E2eWorld) {}

/// Writes the system-admin tuple directly via the backend CLI/API — there is
/// no REST endpoint to bootstrap the first admin without an existing admin.
#[given(expr = "user {string} is a system admin")]
pub async fn make_system_admin(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let user_tuple = format!("user:{}", E2eWorld::user_id(&user));
    match &stack.authz {
        AuthzSetup::OpenFga { url, store_id, model_id } => {
            fga_write_tuple(url, store_id, model_id, &user_tuple, "admin", "system:library");
        }
        AuthzSetup::Opa { url } => {
            opa_write_tuple(&world.http, url, &user_tuple, "admin", "system:library").await;
        }
    }
}
