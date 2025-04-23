use solana_sdk::{account::Account, clock::Slot, pubkey::Pubkey, signature::Signature, transaction::SanitizedTransaction};
use solana_transaction_status::TransactionStatusMeta;
// use solana_transaction_status::TransactionStatusMeta;

use super::geyser_plugin_interface::{ReplicaAccountInfoV3, ReplicaTransactionInfoV2};





// #[derive(Debug, Clone, PartialEq)]
// pub struct MessageAccountInfo {
//     pub pubkey: Pubkey,
//     pub lamports: u64,
//     pub owner: Pubkey,
//     pub executable: bool,
//     pub rent_epoch: u64,
//     pub data: Vec<u8>,
//     pub write_version: u64,
//     pub txn_signature: Option<Signature>,
// }
// impl MessageAccountInfo {
//     pub fn from_geyser(info: &ReplicaAccountInfoV3<'_>) -> Self {
//         Self {
//             pubkey: Pubkey::try_from(info.pubkey).expect("valid Pubkey"),
//             lamports: info.lamports,
//             owner: Pubkey::try_from(info.owner).expect("valid Pubkey"),
//             executable: info.executable,
//             rent_epoch: info.rent_epoch,
//             data: info.data.into(),
//             write_version: info.write_version,
//             txn_signature: info.txn.map(|txn| *txn.signature()),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct MessageAccount {
    // pub account: MessageAccountInfo,
    pub account: Account,
    pub pubkey: Pubkey,
    pub write_version: u64,
    pub txn_signature: Option<Signature>,
    pub slot: Slot,
    pub is_startup: bool,
    pub tx: Option<SanitizedTransaction>
    // pub created_at: Timestamp,
}

impl MessageAccount {
    pub fn from_geyser(info: &ReplicaAccountInfoV3<'_>, slot: Slot, is_startup: bool) -> Self {
        let tx = if let Some(txn) = info.txn {
            Some(txn.clone())
        } else {
            None
        };
        Self {
            account: Account { 
                lamports: info.lamports, 
                data: info.data.into(), 
                owner: Pubkey::try_from(info.owner).expect("valid Pubkey"), 
                executable: info.executable, 
                rent_epoch: info.rent_epoch 
            },
            pubkey: Pubkey::try_from(info.pubkey).expect("valid Pubkey"),
            write_version: info.write_version,
            txn_signature: info.txn.map(|txn| *txn.signature()),
            slot,
            is_startup,
            tx
            // created_at: Timestamp::from(SystemTime::now()),
        }
    }
}


// #[derive(Debug, Clone, PartialEq)]
// pub struct MessageTransaction {
//     pub transaction: MessageTransactionInfo,
//     pub slot: u64,
//     // pub created_at: Timestamp,
// }
#[derive(Debug, Clone, PartialEq)]
pub struct MessageTransactionInfo {
    pub signature: Signature,
    pub is_vote: bool,
    pub index: usize,
    pub solt: u64,
    pub tx: SanitizedTransaction,
    pub meta: TransactionStatusMeta,
    // pub account_keys: HashSet<Pubkey>,
}

impl MessageTransactionInfo {
    pub fn from_geyser(info: &ReplicaTransactionInfoV2<'_>, solt: u64) -> Self {
        Self {
            signature: *info.signature,
            is_vote: info.is_vote,
            tx: info.transaction.clone(),
            meta: info.transaction_status_meta.clone(),
            index: info.index,
            solt
        }
    }
}