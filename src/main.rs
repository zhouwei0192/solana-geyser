use core::prelude;
use std::{str::FromStr, sync::{Arc, RwLock}, thread, time::{Duration, Instant}};

use libloading::{Library, Symbol};
use solana_geyser::{ geyser_plugin_interface::{GeyserPlugin, ReplicaAccountInfoV3, ReplicaAccountInfoVersions},};
use solana_sdk::{account::Account, hash::Hash, message::{v0::{self, LoadedAddresses}, MessageHeader, VersionedMessage}, pubkey::{self, Pubkey}, transaction::{SanitizedTransaction, SanitizedVersionedTransaction, VersionedTransaction}};





fn main() {
    geyser();
    // let t = RwLock::new(Arc::new(Account::default()));
    // let start = std::time::Instant::now();

    // for i in 0..100 {
    //     let t= t.read().unwrap().clone();
    // }
    // println!("time: {:?}", start.elapsed());

}

#[derive(Debug)]
pub struct LoadedGeyserPlugin {
    plugin: Box<dyn GeyserPlugin>,
    #[allow(dead_code)]
    library: Library,
}
fn geyser() {

    let library = unsafe {
        Library::new("target/release/libsolana_geyser.so").unwrap()
    };
    let plugin_symbol: Symbol<unsafe extern "C" fn() -> *mut dyn GeyserPlugin> = unsafe { 
        library.get(b"_create_plugin").unwrap()
    };
    let plugin = unsafe { Box::from_raw(plugin_symbol()) };
    let mut plugin = LoadedGeyserPlugin {
        plugin,
        library
    };

    let pubkey = Pubkey::default();
    let owner = Pubkey::default();
    // let mut data = Vec::with_capacity(1024*1024);
    let mut data = Vec::with_capacity(1024*5);
    for i in 0..data.capacity() {
        data.push((i % 255) as u8);
    }
    let binding = ReplicaAccountInfoV3 {
        pubkey: pubkey.as_ref(),
        lamports: 1111,
        owner: owner.as_ref(),
        executable: true,
        rent_epoch: 123,
        data: data.as_ref(),
        write_version: 12233,
        txn: None,
    };
    plugin.plugin.on_load("config_file", true).unwrap();


    let start = Instant::now(); // 记录开始时间

    // for i in 0..600 {
    //     let mut accounts = Vec::new();
    //     for i in 0..25 {
    //         let account = ReplicaAccountInfoVersions::V0_0_3(&binding);
    //         accounts.push(account);
    //     }
    //     plugin.plugin.batch_update_account(accounts, i, false).unwrap();
    // }
    for i in 0..1500 {
        for i in 0..10 {
            let account = ReplicaAccountInfoVersions::V0_0_3(&binding);
            plugin.plugin.update_account(account, i, false).unwrap();
        }
    }
    // println!("{}", plugin.plugin.get_test());
    let t = start.elapsed();
    thread::sleep(Duration::from_secs(1)); // 休眠 1 秒
    println!("代码执行时间: {:?}", t);
}