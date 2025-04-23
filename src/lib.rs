use std::{any::Any, cell::RefCell, time::Instant};

use geyser::Geyser;
use geyser_plugin_interface::GeyserPlugin;
use libloading::{Library, Symbol};
use once_cell::sync::{Lazy, OnceCell};
// use rpc_plugin::{Rpc, RpcPlugin};
// use solana_rpc::rpc::JsonRpcRequestProcessor;
use solana_sdk::{account::Account, bs58, pubkey::{self, Pubkey}};
// use rpc_plugin::{Rpc, RpcPlugin};



pub mod geyser;
pub mod geyser_plugin_interface;
pub mod message;
pub mod rpc_plugin;
pub mod ktest;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Geyser::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}


// #[derive(PartialEq, Eq, Clone, Default, Debug)]
// #[repr(C)]
// pub struct AccountTest{
//     /// lamports in the account
//     lamports: u64,
//     /// data held in this account
//     data: Vec<u8>,
//     /// the program that owns this account. If executable, the program that loads this account.
//     owner: Pubkey,
//     /// this account's data contains a loaded program (and is now read-only)
//     executable: bool,
//     /// the epoch at which this account will next owe rent
//     rent_epoch: u64,
// }
// #[repr(C)]
// pub struct ReqTest {
//     pub pk: Pubkey
// }
pub static PKS: Lazy<[Pubkey; 44]> = Lazy::new(|| {
    [
        solana_sdk::pubkey!("G7vkhSdtKTYCduKDMEhd9db8Z3ZhnQPi5TngMMKkhuV4"),
        solana_sdk::pubkey!("G832qDGZFuW8wBTST6CSdgzSsurPk4qDPFw5JDaNddYN"),
        solana_sdk::pubkey!("G7vjNb6bXZUivDt9XujX8Dag8nCzqizJL7beoyF2Kdgs"),
        solana_sdk::pubkey!("FmsiuDjJtcFP8UHtpcKnKqvWpGB6ZAxdVPw7ocKAcue7"),
        solana_sdk::pubkey!("G7voSf5c5RsRjkjwS248JcxHweu9DDmfFr5MTYaGpump"),
        solana_sdk::pubkey!("G8Cmg5TPUhZtDKQWPxgMkzU54fBh9t3VQbdthC668Gkc"),
        solana_sdk::pubkey!("G834R3oXLBtkeTYEzsFtKuU28BnTHtqsdGakmZDWpump"),
        solana_sdk::pubkey!("G836ahuGcSUErwLw2r6F4R6rFAgNMeo3Jrcnc7Hq6jPF"),
        solana_sdk::pubkey!("G7vpbBx4NJENkuj9G8WGD2sbsjBjt8WBBFPwVh5jQjjR"),
        solana_sdk::pubkey!("EcdZgna88tiorK3DenV7E8R4jXGANwD9yP4nk22FTTc7"),
        solana_sdk::pubkey!("G7yrwW7kuLvwq8WQznnmBjuaEwv3qDXzoZ5VG78ALW6w"),
        solana_sdk::pubkey!("G7vkcpufu87E4cuok2DGiGbMTrttJX6rUG8dU7aL3e6t"),
        solana_sdk::pubkey!("G8Cg6oDdM7vPX1agQpy9NE4hFhN3cS4XnYfKEQmx4ZCV"),
        solana_sdk::pubkey!("CYmvegZxr2vjmNKKgAWqfPLBNk3rff8dgBhdFEnVpasC"),
        solana_sdk::pubkey!("G8CjE5itbjBofBCPKhtwjugvhzCAZ5qSP8ZiRCnrp943"),
        solana_sdk::pubkey!("G834YjDY8MtCaJ4vhuTRfkQqAhRCxf4XG32aBqr5QD2D"),
        solana_sdk::pubkey!("G7vpx1FsHaDqekAdtbzqYKjLgQNjx8NJMBSrQGFyXgmb"),
        solana_sdk::pubkey!("G7yudbfkHUEg5wx3AaNDNd26i9osHQKAzUaYzawWha6n"),
        solana_sdk::pubkey!("G7vmktA2ETGT3qPXccmehadW1KPBZ1Mny18MqQv5EoFk"),
        solana_sdk::pubkey!("G8Cg73PmewDdqesS9FHijftFrBvRJt61GdWgt5NAw8sT"),
        solana_sdk::pubkey!("G7yvHn4SfaKSxQSacXM7wEwtRnsrNpbtnBf2Qwe2SMBx"),
        solana_sdk::pubkey!("G7vi4sbTvgxtB7PjD6gjiCmbXmW65MoTTurF4FP8oJH8"),
        solana_sdk::pubkey!("G8Cn1E4R5ubt1NVXW2BZtNJ255wG8v39ZYYkthaTfEgK"),
        solana_sdk::pubkey!("G7yq52n8uAof2kUDtvmqMZuRKdVx9KeTQN8cRukWbq6j"),
        solana_sdk::pubkey!("G8CmnrJaK8zbYgDPJ93aBBGQemXDqqP339xwWmBAiTC8"),
        solana_sdk::pubkey!("G8CgXJuTJBsZDoqWxYaWeJhjVS4C79ueCzgFtRg3daAu"),
        solana_sdk::pubkey!("G832LPUDV2JMXCeYRmVaq7b9bJDaoYDfpF2R6WgJXwGf"),
        solana_sdk::pubkey!("G8Cki6X3CTf4AUW8rxJCwCdT14RLn6Qmhs6BXHc7grsJ"),
        solana_sdk::pubkey!("G8CijTKZNVqbqVf8Kp474oDiZzkPMAwBbMysSWBpfUxz"),
        solana_sdk::pubkey!("G7vmBiQZkTgaQtgQGUqkVe58JFZpoZGjTT8Lv1RbKkGt"),
        solana_sdk::pubkey!("G7voqK7jB4D4gTXEWiQBae9GqfXy2Y4rJSpTf9YPAuYu"),
        solana_sdk::pubkey!("G7vqDMn11ASQvqK6cwgD9oYko5D8gCNN32r3cfhp8m4G"),
        solana_sdk::pubkey!("G8CjCCwz34AHuBWDkKCw56gbx8b8kWXSMRCVs961PwmY"),
        solana_sdk::pubkey!("G8CgSKPc9c1xFeUnATXVnxUm9ZdkNLdRC2WuBiF4t3hC"),
        solana_sdk::pubkey!("G8ChuSDSfL6TXHazf2BoQncVzAy1t4jKSv8XQ5Y2pump"),
        solana_sdk::pubkey!("G7yvGWB5nc17Gsj3EayYoqGohCW1bwqZhmVu89smNGF6"),
        solana_sdk::pubkey!("G8CiDyxE5MovyFneyuo1UQusuVC2V5r4ADR8s751tRyZ"),
        solana_sdk::pubkey!("G7vqK6rdjzrbukDj36Pi2HwnmdmnSEBei1XiSbPvg4RD"),
        solana_sdk::pubkey!("G834ftF74zJSPudPtWiFsQpKWPmiS3aiWs9uBb5yoaQT"),
        solana_sdk::pubkey!("G7vnGsbbAUXPZgDDqaKQbkSiMiqTzomscFRv1NVkz5LF"),
        solana_sdk::pubkey!("G7vjLJJ25M4VBzFeWir6uDf3KrWG7ct66UfkQnv4tdBt"),
        solana_sdk::pubkey!("G7viZLNc3PanHfVBLMic13C5N2X3tzSUKTh2BxqNKv1C"),
        solana_sdk::pubkey!("G835G6xmgFtmHnynqhbEdHrKJUhTiXZmtnUqwqXMYS6q"),
        solana_sdk::pubkey!("GXATMRpvhPbMuqk5NtuDAYEqczrsurBVS5M97MdhWpC5")
    ]
});

#[allow(improper_ctypes_definitions)]
pub type GetAccount = extern "C" fn(Pubkey) -> Option<Account>;

#[allow(improper_ctypes_definitions)]
pub type GetMultipleAccount = extern "C" fn(Vec<Pubkey>) -> Vec<Option<Account>>;

pub static GET_ACCOUNT: OnceCell<GetAccount> = OnceCell::new();
pub static GET_MULTIPLE_ACCOUNT: OnceCell<GetMultipleAccount> = OnceCell::new();


#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn register_get_multiple_account(f: GetMultipleAccount) {
    assert!(GET_MULTIPLE_ACCOUNT.set(f).is_ok());
    println!("register_get_multiple_account");
    std::thread::spawn(|| {

        let mut i = 0;
        let step = 10;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if i >= PKS.len()-step {
                i = 0
            }
            // let pk: [u8; 32] = pks[i].clone().try_into().expect("Invalid public key length");
            let start = Instant::now();
            if let Some(f) = GET_MULTIPLE_ACCOUNT.get() {
                let mut pks = Vec::with_capacity(step);
                for n in 0..step {
                    pks.push(PKS[i+n]);
                }
                let r = f(pks);
                println!("time: {:?}", start.elapsed());
                println!("len: {}", r.len());
                println!("{:?}", r);
            }
            i += step;
        }
    });
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn register_get_account(f: GetAccount) {
    assert!(GET_ACCOUNT.set(f).is_ok());
    println!("register_get_account");
    // std::thread::spawn(|| {

    //     let mut i = 0;
    //     loop {
    //         std::thread::sleep(std::time::Duration::from_secs(1));
    //         if i >= PKS.len() {
    //             i = 0
    //         }
    //         // let pk: [u8; 32] = pks[i].clone().try_into().expect("Invalid public key length");
    //         let start = Instant::now();
    //         if let Some(f) = GET_ACCOUNT.get() {
    //             let r = f(ReqTest { pk: PKS[i] });
    //             println!("time: {:?}", start.elapsed());
    //             println!("{:?}", r);
    //         }
        //         i += 1;
    //     }
    // });
}




// pub trait RpcPlugin: Any + Send + Sync {
//     fn get_account(&self, pk: Pubkey) -> Option<Account>;
// }
// #[no_mangle]
// #[allow(improper_ctypes_definitions)]
// pub unsafe extern "C" fn _create_rpc_plugin() {
//     // 每个dylib都是一个独立进程
//     let library = unsafe {
//         Library::new("/Users/zhouwei/Desktop/agavex/target/debug/libsolana_rpc.dylib").unwrap()
//     };
//     let plugin_symbol: Symbol<unsafe extern "C" fn() -> *mut dyn RpcPlugin> = unsafe { 
//         library.get(b"_create_rpc_plugin").unwrap()
//     };
//     let plugin = unsafe { Box::from_raw(plugin_symbol()) };
//     std::thread::spawn(move || {
//         let pks = vec![
//             bs58::decode("G7vkhSdtKTYCduKDMEhd9db8Z3ZhnQPi5TngMMKkhuV4").into_vec().unwrap(),
//             bs58::decode("G832qDGZFuW8wBTST6CSdgzSsurPk4qDPFw5JDaNddYN").into_vec().unwrap(),
//             bs58::decode("G7vjNb6bXZUivDt9XujX8Dag8nCzqizJL7beoyF2Kdgs").into_vec().unwrap(),
//             bs58::decode("FmsiuDjJtcFP8UHtpcKnKqvWpGB6ZAxdVPw7ocKAcue7").into_vec().unwrap(),
//             bs58::decode("G7voSf5c5RsRjkjwS248JcxHweu9DDmfFr5MTYaGpump").into_vec().unwrap(),
//             bs58::decode("G8Cmg5TPUhZtDKQWPxgMkzU54fBh9t3VQbdthC668Gkc").into_vec().unwrap(),
//             bs58::decode("G834R3oXLBtkeTYEzsFtKuU28BnTHtqsdGakmZDWpump").into_vec().unwrap(),
//             bs58::decode("G836ahuGcSUErwLw2r6F4R6rFAgNMeo3Jrcnc7Hq6jPF").into_vec().unwrap(),
//             bs58::decode("G7vpbBx4NJENkuj9G8WGD2sbsjBjt8WBBFPwVh5jQjjR").into_vec().unwrap(),
//             bs58::decode("EcdZgna88tiorK3DenV7E8R4jXGANwD9yP4nk22FTTc7").into_vec().unwrap(),
//             bs58::decode("G7yuptSezy4mZGYe6mo2A8HNKY2RUsmaFPWrMt17Y1Ve").into_vec().unwrap(),
//             bs58::decode("G7yrwW7kuLvwq8WQznnmBjuaEwv3qDXzoZ5VG78ALW6w").into_vec().unwrap(),
//             bs58::decode("G7vkcpufu87E4cuok2DGiGbMTrttJX6rUG8dU7aL3e6t").into_vec().unwrap(),
//             bs58::decode("G8Cg6oDdM7vPX1agQpy9NE4hFhN3cS4XnYfKEQmx4ZCV").into_vec().unwrap(),
//             bs58::decode("CYmvegZxr2vjmNKKgAWqfPLBNk3rff8dgBhdFEnVpasC").into_vec().unwrap(),
//             bs58::decode("G8CjE5itbjBofBCPKhtwjugvhzCAZ5qSP8ZiRCnrp943").into_vec().unwrap(),
//             bs58::decode("G834YjDY8MtCaJ4vhuTRfkQqAhRCxf4XG32aBqr5QD2D").into_vec().unwrap(),
//             bs58::decode("G7vpx1FsHaDqekAdtbzqYKjLgQNjx8NJMBSrQGFyXgmb").into_vec().unwrap(),
//             bs58::decode("G7yudbfkHUEg5wx3AaNDNd26i9osHQKAzUaYzawWha6n").into_vec().unwrap(),
//             bs58::decode("G7vmktA2ETGT3qPXccmehadW1KPBZ1Mny18MqQv5EoFk").into_vec().unwrap(),
//             bs58::decode("G8Cg73PmewDdqesS9FHijftFrBvRJt61GdWgt5NAw8sT").into_vec().unwrap(),
//             bs58::decode("G7yvHn4SfaKSxQSacXM7wEwtRnsrNpbtnBf2Qwe2SMBx").into_vec().unwrap(),
//             bs58::decode("G7vi4sbTvgxtB7PjD6gjiCmbXmW65MoTTurF4FP8oJH8").into_vec().unwrap(),
//             bs58::decode("G8Cn1E4R5ubt1NVXW2BZtNJ255wG8v39ZYYkthaTfEgK").into_vec().unwrap(),
//             bs58::decode("G7yq52n8uAof2kUDtvmqMZuRKdVx9KeTQN8cRukWbq6j").into_vec().unwrap(),
//             bs58::decode("G8CmnrJaK8zbYgDPJ93aBBGQemXDqqP339xwWmBAiTC8").into_vec().unwrap(),
//             bs58::decode("G8CgXJuTJBsZDoqWxYaWeJhjVS4C79ueCzgFtRg3daAu").into_vec().unwrap(),
//             bs58::decode("G832LPUDV2JMXCeYRmVaq7b9bJDaoYDfpF2R6WgJXwGf").into_vec().unwrap(),
//             bs58::decode("G8Cki6X3CTf4AUW8rxJCwCdT14RLn6Qmhs6BXHc7grsJ").into_vec().unwrap(),
//             bs58::decode("G8CijTKZNVqbqVf8Kp474oDiZzkPMAwBbMysSWBpfUxz").into_vec().unwrap(),
//             bs58::decode("G7vmBiQZkTgaQtgQGUqkVe58JFZpoZGjTT8Lv1RbKkGt").into_vec().unwrap(),
//             bs58::decode("G7voqK7jB4D4gTXEWiQBae9GqfXy2Y4rJSpTf9YPAuYu").into_vec().unwrap(),
//             bs58::decode("G7vqDMn11ASQvqK6cwgD9oYko5D8gCNN32r3cfhp8m4G").into_vec().unwrap(),
//             bs58::decode("G8CjCCwz34AHuBWDkKCw56gbx8b8kWXSMRCVs961PwmY").into_vec().unwrap(),
//             bs58::decode("G8CgSKPc9c1xFeUnATXVnxUm9ZdkNLdRC2WuBiF4t3hC").into_vec().unwrap(),
//             bs58::decode("G8ChuSDSfL6TXHazf2BoQncVzAy1t4jKSv8XQ5Y2pump").into_vec().unwrap(),
//             bs58::decode("G7yvGWB5nc17Gsj3EayYoqGohCW1bwqZhmVu89smNGF6").into_vec().unwrap(),
//             bs58::decode("G8CiDyxE5MovyFneyuo1UQusuVC2V5r4ADR8s751tRyZ").into_vec().unwrap(),
//             bs58::decode("G7vqK6rdjzrbukDj36Pi2HwnmdmnSEBei1XiSbPvg4RD").into_vec().unwrap(),
//             bs58::decode("G834ftF74zJSPudPtWiFsQpKWPmiS3aiWs9uBb5yoaQT").into_vec().unwrap(),
//             bs58::decode("G7vnGsbbAUXPZgDDqaKQbkSiMiqTzomscFRv1NVkz5LF").into_vec().unwrap(),
//             bs58::decode("G7vjLJJ25M4VBzFeWir6uDf3KrWG7ct66UfkQnv4tdBt").into_vec().unwrap(),
//             bs58::decode("G7viZLNc3PanHfVBLMic13C5N2X3tzSUKTh2BxqNKv1C").into_vec().unwrap(),
//             bs58::decode("G835G6xmgFtmHnynqhbEdHrKJUhTiXZmtnUqwqXMYS6q").into_vec().unwrap(),
//             bs58::decode("GXATMRpvhPbMuqk5NtuDAYEqczrsurBVS5M97MdhWpC5").into_vec().unwrap()
//        ];
//        let mut i = 0;
//        std::thread::sleep(std::time::Duration::from_secs(10));
    
//        loop {
//            std::thread::sleep(std::time::Duration::from_secs(1));
//            if i >= pks.len() {
//                i = 0
//            }
//            let pk: [u8; 32] = pks[i].clone().try_into().expect("Invalid public key length");
//            i += 1;
//            let start = Instant::now();
//            let r = plugin.get_account(Pubkey::new_from_array(pk));
//            println!("time: {:?}", start.elapsed());
//            println!("{:?}", r);
//        }
//     });

// }
