


use std::time::{SystemTime, UNIX_EPOCH};

use solana_sdk::{bs58, clock::Slot};

use tokio::sync::mpsc;  
use tokio::runtime::Builder;


use crate::{geyser_plugin_interface::{GeyserPlugin, ReplicaAccountInfoVersions, ReplicaTransactionInfoVersions, Result}, message::{MessageAccount, MessageTransactionInfo}};


#[derive(Debug, Clone)]
pub enum Message {
    // Slot(MessageSlot),
    Account(MessageAccount),
    Transaction(MessageTransactionInfo),
    // Entry(Arc<MessageEntry>),
    // BlockMeta(Arc<MessageBlockMeta>),
    // Block(Arc<MessageBlock>),
}
#[derive(Debug, Default, Clone)]
pub struct Geyser {
    pub grpc_channel: Option<mpsc::UnboundedSender<Message>>
}


impl GeyserPlugin for Geyser {
    fn name(&self) -> &'static str {
        return "";
    }
    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        // tokio::runtime::Builder::new_current_thread()
        //     .build()
        //     .unwrap()
        //     .block_on(async {
        //         tokio::task::spawn_blocking(move || {
        //             tokio::runtime::Builder::new_current_thread()
        //                 .build()
        //                 .unwrap()
        //                 .block_on(async {
        //                     while let Some(message) = rx.recv().await {
        //                         super::data::handle(message).await;
        //                     }
        //             })
        //         });
        // });

        std::thread::spawn(move || {
            Builder::new_multi_thread()
                .worker_threads(8)
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    while let Some(message) = rx.recv().await {
                        tokio::spawn(async move {
                            let start = SystemTime::now();
                            let since_epoch = start.duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let millis = since_epoch.as_millis();
                            println!("Current timestamp in milliseconds: {}", millis);
                            match message {
                                Message::Account(account) => {
                                    println!("txn_signature: {}", account.txn_signature.unwrap());
                                    println!("Account: {}", bs58::encode(account.pubkey.to_bytes()).into_string());
                                }
                                Message::Transaction(transaction) => {
                                    println!("Transaction: {:?}", transaction.signature);
                                }                 
                            }
                        });
                    }
                });
        });
        self.grpc_channel = Some(tx);
        Ok(())
    }
    fn on_unload(&mut self) {
        drop(self.grpc_channel.clone());
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> Result<()> {
        // if is_startup {
        //     return Ok(()) ;
        // }
        let account = match account {
            ReplicaAccountInfoVersions::V0_0_1(_info) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_1 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_2(_info) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_2 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_3(info) => info,
        };
        let success = self
            .grpc_channel
            .clone()
            .unwrap()
            .send(Message::Account(MessageAccount::from_geyser(account, slot, is_startup)))
            .is_ok();
        if !success {
            println!("update_account send fail")
        };
        Ok(())
    }
    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> Result<()> {
        // let trasaction = match transaction {
        //     ReplicaTransactionInfoVersions::V0_0_1(_info) => {
        //         unreachable!("ReplicaAccountInfoVersions::V0_0_1 is not supported")
        //     }
        //     ReplicaTransactionInfoVersions::V0_0_2(info) => info,
        // };
        // let success = self
        //     .grpc_channel
        //     .clone()
        //     .unwrap()
        //     .send(Message::Transaction(MessageTransactionInfo::from_geyser(trasaction, slot)))
        //     .is_ok();
        // if !success {
        //     println!("notify_transaction send fail")
        // };
        Ok(())
    }

}


#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the Plugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Geyser::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}