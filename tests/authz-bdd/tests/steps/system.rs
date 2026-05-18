use cucumber::given;

use crate::stack::{ensure_stack, fga_write_tuple};
use crate::world::E2eWorld;

#[given("no preconditions")]
pub async fn no_preconditions(_world: &mut E2eWorld) {}

/// Writes the system-admin tuple directly via `fga` — there is no REST
/// endpoint to bootstrap the first admin without an existing admin.
#[given(expr = "user {string} is a system admin")]
pub async fn make_system_admin(_world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    fga_write_tuple(
        &stack.openfga_url,
        &stack.store_id,
        &stack.model_id,
        &format!("user:{}", E2eWorld::user_id(&user)),
        "admin",
        "system:library",
    );
}
