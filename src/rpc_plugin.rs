use std::any::Any;

use solana_sdk::{account::Account, pubkey::Pubkey};



// #[repr(C)]
// #[derive(PartialEq, Eq, Clone, Default, Debug)]
// pub struct AccountTest2 {
//     // 和 a 项目的 Account 完全一样（字段对齐）
//     pub lamports: u64,
//     pub data_len: u64,
//     pub owner: [u8; 32],
//     pub executable: bool,
//     pub rent_epoch: u64,
// }

// pub static JRP: once_cell::sync::OnceCell<AccountTest2> = once_cell::sync::OnceCell::new();

// #[no_mangle]
// pub extern "C" fn init_jrp(ptr: *mut AccountTest2) {
//     let jrp = unsafe { Box::from_raw(ptr) };
//     assert!(JRP.set(*jrp).is_ok());
// }
// #[no_mangle]
// pub extern "C" fn jrp_get_account(pubkey: *const Pubkey) -> *mut AccountTest2 {
//     if let Some(jrp) = JRP.get() {
//         Box::into_raw(Box::new(jrp.clone()))
//     } else {
//         std::ptr::null_mut()  
//     }
// }
// pub type RpcPluginCreate = unsafe fn() -> *mut dyn RpcPlugin;





// #[derive(Clone, Default)]
// pub struct Rpc {
// }
// impl RpcPlugin for Rpc {
//     fn get_account(&self, pk: Pubkey) -> Option<Account> {
//         if let Some(a) = JRP.get() {
//             // Some(a.clone())
//             None
//         } else {
//             None
//         }
//     }
// }

