#![cfg(test)]
extern crate std;

use crate::contract::{ClaimableBalanceContract, ClaimableBalanceContractClient, Claimant};
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{
    token::Client as TokenClient, token::StellarAssetClient as TokenAdminClient, vec, Address, Env,
    Vec,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(e, &sac.address()),
        TokenAdminClient::new(e, &sac.address()),
    )
}

#[test]
fn test_deposit_and_claim_multiple() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 12345);

    let admin = Address::generate(&env);
    let claimants: Vec<Claimant> = vec![
        &env,
        Claimant {
            claimant: Address::generate(&env),
            amount: 10,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 20,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 30,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 40,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 50,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 60,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 70,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 80,
        },
    ];

    let (token, token_admin_client) = create_token_contract(&env, &admin);
    token_admin_client.mint(&admin, &360);

    let contract = ClaimableBalanceContractClient::new(
        &env,
        &env.register(ClaimableBalanceContract, (&admin,)),
    );

    contract.deposit(&admin, &token.address, &claimants.clone());

    for c in &claimants {
        assert_eq!(token.balance(&c.claimant), 0);
    }

    for c in &claimants {
        contract.claim(&c.claimant, &token.address);
    }

    for c in &claimants {
        assert_eq!(token.balance(&c.claimant), c.amount);
    }

    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
fn test_deposit_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 12345);

    let admin = Address::generate(&env);
    let claimants = std::vec![
        Claimant {
            claimant: Address::generate(&env),
            amount: 10,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 20,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 30,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 40,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 50,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 60,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 70,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 80,
        },
    ];

    let (token, token_admin_client) = create_token_contract(&env, &admin);
    token_admin_client.mint(&admin, &360);

    let contract = ClaimableBalanceContractClient::new(
        &env,
        &env.register(ClaimableBalanceContract, (&admin,)),
    );

    let part1 = vec![
        &env,
        claimants[0].clone(),
        claimants[1].clone(),
        claimants[2].clone(),
    ];

    let part2 = vec![
        &env,
        claimants[3].clone(),
        claimants[4].clone(),
        claimants[5].clone(),
    ];

    let part3 = vec![&env, claimants[6].clone(), claimants[7].clone()];

    contract.deposit(&admin, &token.address, &part1);
    contract.deposit(&admin, &token.address, &part2);
    contract.deposit(&admin, &token.address, &part3);

    for c in &claimants {
        assert_eq!(token.balance(&c.claimant), 0);
    }

    for c in &claimants {
        contract.claim(&c.claimant, &token.address);
    }

    for c in &claimants {
        assert_eq!(token.balance(&c.claimant), c.amount);
    }

    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
#[should_panic(expected = "no matching claimant found")]
fn test_duplicate_claim_should_fail() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let claimer = Address::generate(&env);

    let claimant_list = vec![
        &env,
        Claimant {
            claimant: claimer.clone(),
            amount: 50,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 50,
        },
    ];

    let (token, token_admin_client) = create_token_contract(&env, &admin);
    token_admin_client.mint(&admin, &100);

    let contract = ClaimableBalanceContractClient::new(
        &env,
        &env.register(ClaimableBalanceContract, (&admin,)),
    );

    contract.deposit(&admin, &token.address, &claimant_list);

    // claiming twice fails
    contract.claim(&claimer, &token.address);
    contract.claim(&claimer, &token.address); // should panic
}

#[test]
#[should_panic(expected = "Only admin can create new claimable balance!")]
fn test_non_admin_cannot_deposit() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let not_admin = Address::generate(&env);
    let claimant = Address::generate(&env);

    let (token, token_admin_client) = create_token_contract(&env, &admin);
    token_admin_client.mint(&not_admin, &100);

    let contract = ClaimableBalanceContractClient::new(
        &env,
        &env.register(ClaimableBalanceContract, (&admin,)),
    );

    contract.deposit(
        &not_admin,
        &token.address,
        &vec![
            &env,
            Claimant {
                claimant,
                amount: 100,
            },
        ],
    );
}

#[test]
#[should_panic(expected = "too many claimants")]
fn test_too_many_claimants_should_fail() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token, token_admin_client) = create_token_contract(&env, &admin);
    token_admin_client.mint(&admin, &1000);

    let contract = ClaimableBalanceContractClient::new(
        &env,
        &env.register(ClaimableBalanceContract, (&admin,)),
    );

    let claimants: Vec<Claimant> = vec![
        &env,
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
        Claimant {
            claimant: Address::generate(&env),
            amount: 1,
        },
    ];

    contract.deposit(&admin, &token.address, &claimants);
}
