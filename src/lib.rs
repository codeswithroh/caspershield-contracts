#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    ApiError, EntryPoints, Key, U512,
};
use alloc::format;
use alloc::collections::BTreeSet;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    runtime::revert(ApiError::User(1000))
}

const CONTRACT_NAME: &str = "CasperShieldVault";

// Storage keys
const USER_MODE: &str = "user_mode";
const ALLOWED_CONTRACTS: &str = "allowed_contracts";
const MAX_TX_AMOUNT_SAFE: &str = "max_tx_amount_safe";
const MAX_TX_AMOUNT_BALANCED: &str = "max_tx_amount_balanced";
const ADMIN: &str = "admin";

// Default values
const DEFAULT_SAFE_LIMIT: u64 = 1000;
const DEFAULT_BALANCED_LIMIT: u64 = 10000;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SafetyMode {
    Safe = 0,
    Balanced = 1,
    Degenerate = 2,
}

#[repr(u16)]
pub enum Error {
    Unauthorized = 1,
    ContractNotAllowed = 2,
    AmountExceedsLimit = 3,
    InvalidMode = 4,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    // Initialize entry points
    let entry_points = EntryPoints::new();
    
    let (contract_package_hash, _) = storage::new_contract(
        entry_points,
        None,
        None,
        None,
        None,
    );
    runtime::put_key(CONTRACT_NAME, contract_package_hash.into());
    
    // Initialize contract storage
    initialize_contract();
}

fn initialize_contract() {
    let caller = runtime::get_caller();
    
    // Set admin as the caller
    let admin_key = storage::new_uref(caller).into();
    runtime::put_key(ADMIN, admin_key);
    
    // Initialize default transaction limits
    let safe_limit_key = storage::new_uref(U512::from(DEFAULT_SAFE_LIMIT)).into();
    runtime::put_key(MAX_TX_AMOUNT_SAFE, safe_limit_key);
    
    let balanced_limit_key = storage::new_uref(U512::from(DEFAULT_BALANCED_LIMIT)).into();
    runtime::put_key(MAX_TX_AMOUNT_BALANCED, balanced_limit_key);
    
    // Initialize empty allowed contracts set
    let allowed_contracts: BTreeSet<Key> = BTreeSet::new();
    let contracts_key = storage::new_uref(allowed_contracts).into();
    runtime::put_key(ALLOWED_CONTRACTS, contracts_key);
}

fn get_admin() -> Key {
    runtime::get_key(ADMIN).unwrap_or_revert_with(Error::Unauthorized)
}

fn is_admin() -> bool {
    match get_admin() {
        Key::Account(admin_hash) => runtime::get_caller() == admin_hash,
        _ => false,
    }
}

fn get_user_mode(user: Key) -> u8 {
    let user_mode_key = format!("{}:{}", USER_MODE, user);
    runtime::get_key(&user_mode_key)
        .map(|key| {
            let mode_uref = key.into_uref().unwrap();
            storage::read::<u8>(mode_uref).unwrap_or(None).unwrap_or(0u8) // Default to SAFE mode
        })
        .unwrap_or(0u8) // Default to SAFE mode
}

fn set_user_mode(user: Key, mode: u8) {
    let user_mode_key = format!("{}:{}", USER_MODE, user);
    let mode_key = storage::new_uref(mode).into();
    runtime::put_key(&user_mode_key, mode_key);
}

fn is_contract_allowed(contract: Key) -> bool {
    runtime::get_key(ALLOWED_CONTRACTS)
        .map(|key| {
            let contracts_uref = key.into_uref().unwrap();
            let contracts: BTreeSet<Key> = storage::read(contracts_uref).unwrap_or(None).unwrap_or_default();
            contracts.contains(&contract)
        })
        .unwrap_or(false)
}

fn get_max_tx_amount_safe() -> U512 {
    runtime::get_key(MAX_TX_AMOUNT_SAFE)
        .map(|key| {
            let amount_uref = key.into_uref().unwrap();
            storage::read::<U512>(amount_uref).unwrap_or(None).unwrap_or(U512::from(DEFAULT_SAFE_LIMIT))
        })
        .unwrap_or(U512::from(DEFAULT_SAFE_LIMIT))
}

fn get_max_tx_amount_balanced() -> U512 {
    runtime::get_key(MAX_TX_AMOUNT_BALANCED)
        .map(|key| {
            let amount_uref = key.into_uref().unwrap();
            storage::read::<U512>(amount_uref).unwrap_or(None).unwrap_or(U512::from(DEFAULT_BALANCED_LIMIT))
        })
        .unwrap_or(U512::from(DEFAULT_BALANCED_LIMIT))
}

#[no_mangle]
pub extern "C" fn set_mode() {
    let mode: u8 = 1; // Simplified for now - will add parameters later
    let caller = runtime::get_caller();
    
    if mode > 2 {
        runtime::revert(Error::InvalidMode);
    }
    
    set_user_mode(Key::Account(caller), mode);
}

#[no_mangle]
pub extern "C" fn execute_action() {
    let target_contract: Key = Key::Account(runtime::get_caller()); // Simplified for now
    let amount: U512 = U512::from(500); // Simplified for now
    
    let caller = runtime::get_caller();
    let mode = get_user_mode(Key::Account(caller));
    
    match mode {
        0 => { // Safe
            if !is_contract_allowed(target_contract) {
                runtime::revert(Error::ContractNotAllowed);
            }
            if amount > get_max_tx_amount_safe() {
                runtime::revert(Error::AmountExceedsLimit);
            }
        }
        1 => { // Balanced
            if amount > get_max_tx_amount_balanced() {
                runtime::revert(Error::AmountExceedsLimit);
            }
            if !is_contract_allowed(target_contract) {
                // Would emit warning in full implementation
            }
        }
        2 => { // Degenerate
            // Always allow
        }
        _ => {
            runtime::revert(Error::InvalidMode);
        }
    }
}

#[no_mangle]
pub extern "C" fn add_allowed_contract() {
    if !is_admin() {
        runtime::revert(Error::Unauthorized);
    }
    
    // Placeholder implementation - will add parameters later
}

#[no_mangle]
pub extern "C" fn remove_allowed_contract() {
    if !is_admin() {
        runtime::revert(Error::Unauthorized);
    }
    
    // Placeholder implementation - will add parameters later
}

#[no_mangle]
pub extern "C" fn update_limits() {
    if !is_admin() {
        runtime::revert(Error::Unauthorized);
    }
    
    // Placeholder implementation - will add parameters later
}
