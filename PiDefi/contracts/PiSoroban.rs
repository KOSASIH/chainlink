// SPDX-License-Identifier: MIT
// PiSoroban.rs: Soroban contracts for Pi Coin (a stablecoin) on Stellar.
// - Name: Pi Coin
// - Symbol: PI
// - Target Value: $314,159 (initial price in PriceOracle; pegged via stabilization logic).
// Deploy each contract separately in Soroban. This file combines them for illustration.

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map, log, panic_with_error, I128Val, i128};

// Define symbols for events
const TRANSFER: Symbol = Symbol::short("Transfer");
const APPROVAL: Symbol = Symbol::short("Approval");
const MINT: Symbol = Symbol::short("Mint");
const BURN: Symbol = Symbol::short("Burn");
const LIQUIDITY_ADDED: Symbol = Symbol::short("LiqAdded");
const LIQUIDITY_REMOVED: Symbol = Symbol::short("LiqRem");
const PRICE_UPDATED: Symbol = Symbol::short("PriceUpd");
const ADMIN_ADDED: Symbol = Symbol::short("AdminAdd");
const ADMIN_REMOVED: Symbol = Symbol::short("AdminRem");
const STABILIZED: Symbol = Symbol::short("Stabilized");

// Error codes
const INSUFFICIENT_BALANCE: u32 = 1;
const ALLOWANCE_EXCEEDED: u32 = 2;
const NOT_OWNER: u32 = 3;
const AMOUNT_ZERO: u32 = 4;
const INSUFFICIENT_LIQUIDITY: u32 = 5;
const ADMIN_EXISTS: u32 = 6;
const ADMIN_NOT_EXISTS: u32 = 7;
const OVERFLOW: u32 = 8;
const PRICE_TOO_LOW: u32 = 9;
const PRICE_TOO_HIGH: u32 = 10;

// StableCoin Contract (Core stablecoin with pegging)
#[contract]
pub struct StableCoin;

#[contractimpl]
impl StableCoin {
    pub fn __constructor(env: Env, oracle_contract: Address) {
        let owner = env.invoker();
        env.storage().set(&Symbol::short("name"), &"Pi Coin");
        env.storage().set(&Symbol::short("symbol"), &"PI");
        env.storage().set(&Symbol::short("decimals"), &18u32);
        let total_supply = i128!(100_000_000_000) * i128!(10).pow(18); // 100 billion with 18 decimals
        env.storage().set(&Symbol::short("total_supply"), &total_supply);
        env.storage().set(&owner, &total_supply); // balanceOf[owner]
        env.storage().set(&Symbol::short("owner"), &owner);
        env.storage().set(&Symbol::short("oracle"), &oracle_contract); // Link to PriceOracle
        env.storage().set(&Symbol::short("target_price"), &i128!(314159)); // Target $314,159
    }

    pub fn name(env: Env) -> String {
        env.storage().get(&Symbol::short("name")).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().get(&Symbol::short("symbol")).unwrap()
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().get(&Symbol::short("decimals")).unwrap()
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().get(&Symbol::short("total_supply")).unwrap()
    }

    pub fn balance_of(env: Env, account: Address) -> i128 {
        env.storage().get(&account).unwrap_or(i128!(0))
    }

    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let key = (owner, spender);
        env.storage().get(&key).unwrap_or(i128!(0))
    }

    pub fn owner(env: Env) -> Address {
        env.storage().get(&Symbol::short("owner")).unwrap()
    }

    pub fn oracle(env: Env) -> Address {
        env.storage().get(&Symbol::short("oracle")).unwrap()
    }

    pub fn target_price(env: Env) -> i128 {
        env.storage().get(&Symbol::short("target_price")).unwrap()
    }

    pub fn transfer(env: Env, to: Address, value: i128) -> bool {
        let from = env.invoker();
        let from_balance = Self::balance_of(env.clone(), from.clone());
        if from_balance < value {
            panic_with_error!(env, INSUFFICIENT_BALANCE);
        }
        env.storage().set(&from, &(from_balance - value));
        let to_balance = Self::balance_of(env.clone(), to.clone());
        env.storage().set(&to, &(to_balance + value));
        log!(env, TRANSFER, from, to, value);
        true
    }

    pub fn approve(env: Env, spender: Address, value: i128) -> bool {
        let owner = env.invoker();
        let key = (owner.clone(), spender.clone());
        env.storage().set(&key, &value);
        log!(env, APPROVAL, owner, spender, value);
        true
    }

    pub fn transfer_from(env: Env, from: Address, to: Address, value: i128) -> bool {
        let spender = env.invoker();
        let from_balance = Self::balance_of(env.clone(), from.clone());
        if from_balance < value {
            panic_with_error!(env, INSUFFICIENT_BALANCE);
        }
        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < value {
            panic_with_error!(env, ALLOWANCE_EXCEEDED);
        }
        env.storage().set(&from, &(from_balance - value));
        let to_balance = Self::balance_of(env.clone(), to.clone());
        env.storage().set(&to, &(to_balance + value));
        env.storage().set(&(from, spender), &(allowance - value));
        log!(env, TRANSFER, from, to, value);
        true
    }

    pub fn mint(env: Env, to: Address, value: i128) {
        let owner = Self::owner(env.clone());
        if env.invoker() != owner {
            panic_with_error!(env, NOT_OWNER);
        }
        let total_supply = Self::total_supply(env.clone()) + value;
        env.storage().set(&Symbol::short("total_supply"), &total_supply);
        let to_balance = Self::balance_of(env.clone(), to.clone()) + value;
        env.storage().set(&to, &to_balance);
        log!(env, MINT, to, value);
    }

    pub fn burn(env: Env, value: i128) {
        let from = env.invoker();
        let from_balance = Self::balance_of(env.clone(), from.clone());
        if from_balance < value {
            panic_with_error!(env, INSUFFICIENT_BALANCE);
        }
        env.storage().set(&from, &(from_balance - value));
        let total_supply = Self::total_supply(env.clone()) - value;
        env.storage().set(&Symbol::short("total_supply"), &total_supply);
        log!(env, BURN, from, value);
    }

    // New: Stabilization function for pegging to $314,159
    pub fn stabilize(env: Env) {
        let oracle = Self::oracle(env.clone());
        let current_price = env.invoke_contract(&oracle, &Symbol::short("get_price"), Vec::new(&env));
        let target = Self::target_price(env.clone());
        let total_supply = Self::total_supply(env.clone());
        let adjustment_factor = i128!(1000); // 10% adjustment per call (tunable)

        if current_price < target {
            // Price too low: Burn to increase value
            let burn_amount = total_supply / adjustment_factor;
            Self::burn(env.clone(), burn_amount);
            log!(env, STABILIZED, "burn", burn_amount);
        } else if current_price > target {
            // Price too high: Mint to decrease value
            let mint_amount = total_supply / adjustment_factor;
            Self::mint(env.clone(), env.current_contract_address(), mint_amount); // Mint to contract
            log!(env, STABILIZED, "mint", mint_amount);
        }
        // If equal, do nothing
    }
}

// LiquidityPool Contract (Unchanged, but can integrate with stabilization)
#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    pub fn liquidity(env: Env, provider: Address) -> i128 {
        env.storage().get(&provider).unwrap_or(i128!(0))
    }

    pub fn add_liquidity(env: Env, amount: i128) {
        if amount <= i128!(0) {
            panic_with_error!(env, AMOUNT_ZERO);
        }
        let provider = env.invoker();
        let current = Self::liquidity(env.clone(), provider.clone());
        env.storage().set(&provider, &(current + amount));
        log!(env, LIQUIDITY_ADDED, provider, amount);
    }

    pub fn remove_liquidity(env: Env, amount: i128) {
        let provider = env.invoker();
        let current = Self::liquidity(env.clone(), provider.clone());
        if current < amount {
            panic_with_error!(env, INSUFFICIENT_LIQUIDITY);
        }
        env.storage().set(&provider, &(current - amount));
        log!(env, LIQUIDITY_REMOVED, provider, amount);
    }

    pub fn auto_liquidity_management(env: Env) {
        // Placeholder: Implement logic (e.g., rebalance based on price oracle)
    }
}

// PriceOracle Contract (Updated for i128 price)
#[contract]
pub struct PriceOracle;

#[contractimpl]
impl PriceOracle {
    pub fn __constructor(env: Env) {
        let owner = env.invoker();
        env.storage().set(&Symbol::short("owner"), &owner);
        env.storage().set(&Symbol::short("current_price"), &i128!(314159)); // $314,159
    }

    pub fn current_price(env: Env) -> i128 {
        env.storage().get(&Symbol::short("current_price")).unwrap()
    }

    pub fn owner(env: Env) -> Address {
        env.storage().get(&Symbol::short("owner")).unwrap()
    }

    pub fn update_price(env: Env, new_price: i128) {
        let owner = Self::owner(env.clone());
        if env.invoker() != owner {
            panic_with_error!(env, NOT_OWNER);
        }
        env.storage().set(&Symbol::short("current_price"), &new_price);
        log!(env, PRICE_UPDATED, new_price);
    }

    pub fn get_price(env: Env) -> i128 {
        Self::current_price(env)
    }
}

// Governance Contract (Unchanged)
#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    pub fn __constructor(env: Env) {
        let owner = env.invoker();
        env.storage().set(&Symbol::short("owner"), &owner);
    }

    pub fn owner(env: Env) -> Address {
        env.storage().get(&Symbol::short("owner")).unwrap()
    }

    pub fn is_admin(env: Env, admin: Address) -> bool {
        env.storage().get(&admin).unwrap_or(false)
    }

    pub fn add_admin(env: Env, admin: Address) {
        let owner = Self::owner(env.clone());
        if env.invoker() != owner {
            panic_with_error!(env, NOT_OWNER);
        }
        if Self::is_admin(env.clone(), admin.clone()) {
            panic_with_error!(env, ADMIN_EXISTS);
        }
        env.storage().set(&admin, &true);
        log!(env, ADMIN_ADDED, admin);
    }

    pub fn remove_admin(env: Env, admin: Address) {
        let owner = Self::owner(env.clone());
        if env.invoker() != owner {
            panic_with_error!(env, NOT_OWNER);
        }
        if !Self::is_admin(env.clone(), admin.clone()) {
            panic_with_error!(env, ADMIN_NOT_EXISTS);
        }
        env.storage().set(&admin, &false);
        log!(env, ADMIN_REMOVED, admin);
    }
}
