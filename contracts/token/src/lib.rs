#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address), // (owner, spender)
    TotalSupply,
    Admin,
    Locked, // reentrancy guard
}

#[contract]
pub struct TokenContract;

// ── Reentrancy guard ─────────────────────────────────────────────────────────

fn acquire_lock(env: &Env) {
    let locked: bool = env
        .storage()
        .instance()
        .get(&DataKey::Locked)
        .unwrap_or(false);
    assert!(!locked, "reentrant call");
    env.storage().instance().set(&DataKey::Locked, &true);
}

fn release_lock(env: &Env) {
    env.storage().instance().set(&DataKey::Locked, &false);
}

// ── Internal helpers ─────────────────────────────────────────────────────────

fn get_balance(env: &Env, addr: &Address) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::Balance(addr.clone()))
        .unwrap_or(0)
}

fn set_balance(env: &Env, addr: &Address, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::Balance(addr.clone()), &amount);
}

fn get_total_supply(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0)
}

fn set_total_supply(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::TotalSupply, &amount);
}

fn get_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::Allowance(owner.clone(), spender.clone()))
        .unwrap_or(0)
}

fn set_allowance(env: &Env, owner: &Address, spender: &Address, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::Allowance(owner.clone(), spender.clone()), &amount);
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        set_total_supply(&env, 0);
    }

    /// Mint reward tokens to a student upon course completion
    pub fn mint_reward(env: Env, caller: Address, recipient: Address, amount: i128) {
        acquire_lock(&env);

        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(caller == admin, "Only admin can mint");
        assert!(amount > 0, "Amount must be positive");

        set_balance(&env, &recipient, get_balance(&env, &recipient) + amount);
        set_total_supply(&env, get_total_supply(&env) + amount);

        env.events().publish(
            (symbol_short!("token"), symbol_short!("mint")),
            (recipient, amount),
        );

        release_lock(&env);
    }

    /// Approve a spender to transfer tokens on behalf of the caller
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();
        assert!(amount >= 0, "Allowance must be non-negative");
        set_allowance(&env, &owner, &spender, amount);
    }

    /// Burn tokens from the caller's own balance (deflationary / governance)
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "Amount must be positive");

        let bal = get_balance(&env, &from);
        assert!(bal >= amount, "Insufficient balance");

        set_balance(&env, &from, bal - amount);
        set_total_supply(&env, get_total_supply(&env) - amount);

        env.events().publish(
            (symbol_short!("token"), symbol_short!("burn")),
            (from, amount),
        );
    }

    /// Burn tokens from `from` using an allowance granted to `spender`
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        assert!(amount > 0, "Amount must be positive");

        let allowance = get_allowance(&env, &from, &spender);
        assert!(allowance >= amount, "Insufficient allowance");

        let bal = get_balance(&env, &from);
        assert!(bal >= amount, "Insufficient balance");

        set_allowance(&env, &from, &spender, allowance - amount);
        set_balance(&env, &from, bal - amount);
        set_total_supply(&env, get_total_supply(&env) - amount);

        env.events().publish(
            (symbol_short!("token"), symbol_short!("burn")),
            (from, amount),
        );
    }

    pub fn balance(env: Env, addr: Address) -> i128 {
        get_balance(&env, &addr)
    }

    pub fn total_supply(env: Env) -> i128 {
        get_total_supply(&env)
    }

    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        get_allowance(&env, &owner, &spender)
    }
}

mod tests;
