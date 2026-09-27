use soroban_sdk::{contracttype, Address, Env, Symbol};

/// Emitted when a bounty is created.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BountyCreated {
    pub bounty_id: u64,
    pub creator: Address,
    pub reward: i128,
}

/// Emitted when a bounty is claimed by a contributor.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BountyClaimed {
    pub bounty_id: u64,
    pub contributor: Address,
}

/// Emitted when a bounty is completed and the reward is paid out.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BountyCompleted {
    pub bounty_id: u64,
    pub contributor: Address,
    pub reward: i128,
}

/// Emitted when a claimed bounty expires without being completed.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BountyExpired {
    pub bounty_id: u64,
    pub contributor: Address,
}

/// Emitted whenever a contributor's reputation changes.
///
/// `delta` is the signed change applied to the reputation score and
/// `new_reputation` is the resulting (clamped) score so that off-chain
/// profiles can stay accurate without recomputing the penalty.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationChanged {
    pub contributor: Address,
    pub delta: i128,
    pub new_reputation: u32,
}

/// Publish a `ReputationChanged` event for the given contributor.
pub fn emit_reputation_changed(env: &Env, contributor: &Address, delta: i128, new_reputation: u32) {
    let event = ReputationChanged {
        contributor: contributor.clone(),
        delta,
        new_reputation,
    };
    env.events()
        .publish((Symbol::new(env, "reputation_changed"),), event);
}
