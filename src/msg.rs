use cosmwasm_schema::cw_serde;
use nois::NoisCallback;

#[cw_serde]
pub struct InstantiateMsg {
    pub participant_count: u32,
    pub nois_proxy_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestRandomness {},
    NoisReceive { callback: NoisCallback },
    TestRandomizer { randomness: String },
}

#[cw_serde]
pub enum QueryMsg {
    ParticipantCount {},
    Winners {},
    Admin {},
    NoisProxy {},
}
