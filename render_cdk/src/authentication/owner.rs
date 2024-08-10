#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused)]
// [render_cdk] modules.
use crate::{environment_management::prelude::*, state_management::state::Owner};

#[derive(Debug, Clone)]
pub struct Info {
    pub OWNER_ID: String,
}

impl Info {
    pub async fn get_owner_id() -> String {
        let owner_credentials = EnvironmentManager::retrieve_env_config().OWNER_CREDENTIALS;
        let authorized_users = Owner::list_authorized_users(&owner_credentials, "100")
            .await
            .unwrap();

        let owner_id = authorized_users
            .first()
            .map(|owner_response| owner_response.owner.id.clone())
            .expect("No authorized users found.");

        owner_id
    }
}
