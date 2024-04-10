use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const PARTICIPANT_COUNT: Item<u32> = Item::new("participant_count");
pub const NOIS_PROXY: Item<Addr> = Item::new("nois_proxy");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const WINNERS: Item<Vec<u32>> = Item::new("winners");
