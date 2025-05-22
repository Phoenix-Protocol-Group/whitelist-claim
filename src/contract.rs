//! Based on: https://developers.stellar.org/docs/glossary/claimable-balance)
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Init,
    Balance(Address),
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub total_amount: i128,
    pub claimants: Vec<Claimant>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Claimant {
    pub amount: i128,
    pub claimant: Address,
}

// Constants for storage bump amounts
pub const DAY_IN_LEDGERS: u32 = 17280;

// target TTL for the contract instance and its code.
// When a TTL extension is triggered the instance's TTL is reset to this value (7 days of ledger units).
pub const INSTANCE_TARGET_TTL: u32 = 7 * DAY_IN_LEDGERS;
// if the current instance TTL falls below this threshold (i.e., less than 6 days of ledger units), the TTL extension mechanism will refresh it to INSTANCE_TARGET_TTL.
pub const INSTANCE_RENEWAL_THRESHOLD: u32 = INSTANCE_TARGET_TTL - DAY_IN_LEDGERS;

#[contract]
pub struct ClaimableBalanceContract;

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Init, &());
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn deposit(env: Env, from: Address, token: Address, claimants: Vec<Claimant>) {
        if from != env.storage().instance().get(&DataKey::Admin).unwrap() {
            panic!("Only admin can create new claimable balance!");
        }
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        from.require_auth();

        let amount = claimants.iter().map(|c| c.amount).sum();

        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);

        if let Some(old_balance) = env
            .storage()
            .instance()
            .get::<DataKey, ClaimableBalance>(&DataKey::Balance(token.clone()))
        {
            env.storage()
                .instance()
                .extend_ttl(INSTANCE_RENEWAL_THRESHOLD, INSTANCE_TARGET_TTL);

            // because soroban_sdk::Vec...
            let mut claimants = claimants.clone();
            claimants.append(&old_balance.claimants);

            env.storage().instance().set(
                &DataKey::Balance(token.clone()),
                &ClaimableBalance {
                    token,
                    total_amount: amount + old_balance.total_amount,
                    claimants,
                },
            );
        } else {
            env.storage().instance().set(
                &DataKey::Balance(token.clone()),
                &ClaimableBalance {
                    token,
                    total_amount: amount,
                    claimants,
                },
            );
        }
    }

    pub fn claim(env: Env, sender: Address, token: Address) {
        sender.require_auth();
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_RENEWAL_THRESHOLD, INSTANCE_TARGET_TTL);
        let mut claimable_balance: ClaimableBalance = env
            .storage()
            .instance()
            .get(&DataKey::Balance(token.clone()))
            .unwrap();

        let (id, amount) = {
            let mut valid = None;
            for (idx, c) in claimable_balance.claimants.iter().enumerate() {
                if sender == c.claimant {
                    valid = Some((idx, c.amount));
                    break;
                }
            }
            valid.expect("no matching claimant found")
        };
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &sender,
            &amount,
        );

        // now remove the entry
        let mut claimants = claimable_balance.claimants.clone();
        claimants.remove(id as u32).unwrap();

        claimable_balance.claimants = claimants;

        env.storage()
            .instance()
            .set(&DataKey::Balance(token), &claimable_balance);
    }
}

// mod test;
