mod account_all;
mod account_self;
mod common;
mod player_info;
mod vehicle_all;
mod vehicle_self;

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use crate::account_all::AccountAll;
pub use crate::account_self::{AccountSelf, AccountSelfExtra};
pub use crate::common::Common;
pub use crate::player_info::PlayerInfo;
pub use crate::vehicle_all::VehicleAll;
pub use crate::vehicle_self::VehicleSelf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Battle {
    #[serde(rename(serialize = "arenaUniqueID"))]
    pub arena_unique_id: String,
    pub common:          serde_json::Value,
    pub player_info:     HashMap<String, serde_json::Value>,
    pub account_all:     HashMap<String, serde_json::Value>,
    pub vehicle_all:     HashMap<String, serde_json::Value>,
    pub vehicle_self:    HashMap<String, serde_json::Value>,
    pub account_self:    HashMap<String, serde_json::Value>,
}

/// TODO?????
pub trait ArenaFieldsGetter {
    type EnumType: DeserializeOwned;
    fn get_arena_fields(&self) -> HashMap<String, serde_json::Value>;

    /// Check if there is any fields that are not arena/gamemode fields
    fn validate_arena_fields(&self) -> anyhow::Result<()> {
        let arena_fields = self.get_arena_fields();
        if arena_fields.is_empty() {
            Ok(())
        } else {
            let arena_fields = serde_json::to_value(arena_fields)?;
            let _arena_fields: Self::EnumType = serde_json::from_value(arena_fields)?;

            Ok(())
        }
    }
}
