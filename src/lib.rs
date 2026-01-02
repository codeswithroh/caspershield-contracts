#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    ApiError, EntryPoints, Key, U512,
};
use alloc::format;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    runtime::revert(ApiError::User(1000))
}

const CONTRACT_NAME: &str = "CasperShieldVault";

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

const USER_MODE: &str = "user_mode";
const ADMIN: &str = "admin";

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::new_contract(
        EntryPoints::new(),
        None,
        None,
        None,
        None,
    );
    runtime::put_key(CONTRACT_NAME, contract_package_hash.into());
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

fn get_user_mode(_user: Key) -> u8 {
    // Simplified implementation - always return Balanced mode
    1u8
}

fn set_user_mode(user: Key, mode: u8) {
    let user_mode_key = format!("{}:{}", USER_MODE, user);
    let mode_key = storage::new_uref(mode).into();
    runtime::put_key(&user_mode_key, mode_key);
}

fn is_contract_allowed(_contract: Key) -> bool {
    // Simplified implementation - always allow for demo
    true
}

fn get_max_tx_amount_safe() -> U512 {
    // Fixed default limit
    U512::from(1000)
}

fn get_max_tx_amount_balanced() -> U512 {
    // Fixed default limit
    U512::from(10000)
}

#[no_mangle]
pub extern "C" fn set_mode() {
    let mode: u8 = 1; // Default to Balanced for demo
    let caller = runtime::get_caller();
    
    if mode > 2 {
        runtime::revert(Error::InvalidMode);
    }
    
    set_user_mode(Key::Account(caller), mode);
}

#[no_mangle]
pub extern "C" fn execute_action() {
    let target_contract: Key = Key::Account(runtime::get_caller()); // Simplified
    let amount: U512 = U512::from(500); // Simplified
    
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
    
    // Placeholder implementation
}

#[no_mangle]
pub extern "C" fn remove_allowed_contract() {
    if !is_admin() {
        runtime::revert(Error::Unauthorized);
    }
    
    // Placeholder implementation
}

#[no_mangle]
pub extern "C" fn update_limits() {
    if !is_admin() {
        runtime::revert(Error::Unauthorized);
    }
    
    // Placeholder implementation
}
