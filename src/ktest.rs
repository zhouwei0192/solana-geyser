use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

use serde::{Deserialize, Serialize};


#[cfg(test)]
mod test {
    use std::{fs::OpenOptions, io::{BufWriter, Read, Seek, SeekFrom, Write}, path::Path};

    use base64::Engine;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_sdk::{account::Account, pubkey};
    use super::{AccountSharedData, Reserve};

    #[test]
    fn test() {

        let s = "K/LMyhr3O38BAAAAAAAAADNpsBMAAAAAAT8AAAAAAADvOp8haFVBiPj9PYR+2RYQjXxkRGs9Q8Hc7I7KfTlSVZ6pZdHHQTV6d5Xf4oVUo90lbr4w0MqScPZR8XlV/idcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAGm4hX/quBhPtof2NGGMA12sQ53BrrO1WYoPAAAAAAAQ6WPOMvYVl4Y3F6lxYmzoq280GwZOEYNfyz+o7fIiZ9xxPe7+fi6a+KnyAVhZX/bjdA2j99IjHeui9UAJKHYyTJSPG5/Y4AADpM6+oMBZAI6htWcHksAACMzp5wG7K25wcAAAAAAAAATjrqZwAAAAAJAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABuETVrwS2LEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADdkm0lTN8AajCxgsQEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMCj44pNaezzGtYo+RZa3BKaceNKqouginN21hF5dzU/ujqJXsI/AwB0gSqzhrzEnlA0hR3U+T5byeHy/y+pjgk1VapEx67KvQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoyAAD6AOgDYwCAOgkAAAAAACAcAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAAABYGwAAjgEAAEwdAAARAgAAKCMAACgCAABUJAAACgMAABwlAADoAwAA5CUAAMQJAAAQJwAAiBMAABAnAACIEwAAECcAAIgTAAAQJwAAiBMAAGQAAAAAAAAAAIBT7nuoCgAAwOS8opcJAFNPTAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA6AMAAAAAAAB4AAAAAAAAAPAAAAAAAAAAIyxpA+CpBOOCbIHzfgChwaJoKCOEXCRj5547dpOIXxUAAP///////zQA////////AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAxqR+jQMAeThey7rh//+3kOlnAAAAAIBRAQAAAAAAAEBMlIsyAwCqKcALEQAAAGjM6WcAAAAAgFEBAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAABaAAD//////////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAApEM6sIwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        let v = base64::prelude::BASE64_STANDARD.decode(s).unwrap();
        let mut slice: &[u8] = &v[8..];
        let mut r = Reserve::deserialize(&mut slice).unwrap();
        // println!("{:?}", r);
        r.last_update.slot = 0;

        let mut data = Vec::with_capacity(v.len());
        data.extend_from_slice(&v[0..8].to_vec());
        r.serialize(&mut data).unwrap();

        write_file(vec![
            (
                AccountSharedData { 
                    write_version: 0, 
                    data_len: data.len() as u64, 
                    pubkey: pubkey!("6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p"), 
                    lamports: 60913920, 
                    rent_epoch: 0,
                    owner: pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"), 
                    executable: false, 
                    hash: [0u8; 32] 
                },
                data
            )
        ]);
    }

    fn write_file(w: Vec<(AccountSharedData, Vec<u8>)>) {
        let file_path = Path::new("account").to_path_buf();

        let mut file_size = 4;
        let file = OpenOptions::new()
            .write(true)   // 允许写入
            .create(true)  // 如果文件不存在则创建
            .truncate(true)     // 如果文件已存在则清空内容
            .open(file_path).unwrap();

        for (_, d) in w.iter() {
            file_size += 129 + d.len()
        }
        let mut writer = BufWriter::with_capacity(file_size, file);
        writer.write_all(&[0,0,0,0]).unwrap();
        
        for (a, d) in w {
            writer.write_all(&bincode::serialize(&a).unwrap()).unwrap();
            writer.write_all(d.as_ref()).unwrap();
        }

        let data_len = writer.buffer().len();

        // 在flush前将文件指针移回到文件开始
        let mut file_ref = writer.into_inner().unwrap();
        file_ref.seek(SeekFrom::Start(0)).unwrap();

        // 写入前四个字节，表示文件大小
        file_ref.write_all(&(data_len as u32).to_le_bytes()).unwrap();

        file_ref.flush().unwrap(); // 刷新缓冲区，确保数据写入磁盘
    }

    #[test]
    fn read_file() {

        let file_path = Path::new("account").to_path_buf();
        let mut data = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&file_path).unwrap();

        let file_size = std::fs::metadata(&file_path).unwrap().len() as usize;
        let mut buf = Vec::with_capacity(file_size);
        data.read_to_end(&mut buf).unwrap();



        let data_len = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;

        let mut offset = 4usize;
        loop {
            if offset >= data_len {
                break;
            }

            let next = offset + 129;
            let a = bincode::deserialize::<AccountSharedData>(&buf[offset..next]).unwrap();
            let data_len = a.data_len as usize;
            offset = next + data_len;

            let data = buf[next..offset].to_vec();
            let mut slice: &[u8] = &data[8..];
            let r = Reserve::deserialize(&mut slice).unwrap();
            println!("{:?}", r);



        }

    }
}
#[derive(PartialEq, Serialize, Deserialize, Eq, Clone, Default, Debug)]
pub struct AccountSharedData {
    pub write_version: u64,
    /// key for the account
    pub data_len: u64,
    pub pubkey: Pubkey,
    /// lamports in the account
    pub lamports: u64,
    /// the epoch at which this account will next owe rent
    pub rent_epoch: u64,
    /// the program that owns this account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    pub hash: [u8; 32],
    // pub data: Vec<u8>
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[derivative(Debug)]
// #[account(zero_copy)]
#[repr(C)]
pub struct Reserve {
    pub version: u64,

    pub last_update: LastUpdate,

    pub lending_market: Pubkey,

    pub farm_collateral: Pubkey,
    pub farm_debt: Pubkey,

    pub liquidity: ReserveLiquidity,

    // #[derivative(Debug = "ignore")]
    pub reserve_liquidity_padding: [u64; 150],

    pub collateral: ReserveCollateral,

    // #[derivative(Debug = "ignore")]
    pub reserve_collateral_padding: [u64; 150],

    pub config: ReserveConfig,

    // #[derivative(Debug = "ignore")]
    pub config_padding: [u64; 116],

    pub borrowed_amount_outside_elevation_group: u64,

    pub borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32],

    // #[derivative(Debug = "ignore")]
    pub padding: [u64; 207],
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[zero_copy]
#[repr(C)]
pub struct LastUpdate {
    slot: u64,
    stale: u8,
    price_status: u8,

    placeholder: [u8; 6],
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[zero_copy]
#[repr(C)]
pub struct ReserveLiquidity {
    pub mint_pubkey: Pubkey,
    pub supply_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub available_amount: u64,
    pub borrowed_amount_sf: u128,
    pub market_price_sf: u128,
    pub market_price_last_updated_ts: u64,
    pub mint_decimals: u64,

    pub deposit_limit_crossed_timestamp: u64,
    pub borrow_limit_crossed_timestamp: u64,

    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub accumulated_protocol_fees_sf: u128,
    pub accumulated_referrer_fees_sf: u128,
    pub pending_referrer_fees_sf: u128,
    pub absolute_referral_rate_sf: u128,
    pub token_program: Pubkey,

    pub padding2: [u64; 51],
    pub padding3: [u128; 32],
}

#[derive(BorshDeserialize, BorshSerialize, Default, Debug, PartialEq, Eq)]
// #[zero_copy]
#[repr(C)]
pub struct BigFractionBytes {
    pub value: [u64; 4],
    pub padding: [u64; 2],
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Default, PartialEq, Eq)]
// #[zero_copy]
#[repr(C)]
pub struct ReserveCollateral {
    pub mint_pubkey: Pubkey,
    pub mint_total_supply: u64,
    pub supply_vault: Pubkey,
    pub padding1: [u128; 32],
    pub padding2: [u128; 32],
}
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, Debug)]
// #[derivative(Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct ReserveConfig {
    pub status: u8,
    pub asset_tier: u8,
    pub host_fixed_interest_rate_bps: u16,
    // #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    // #[derivative(Debug = "ignore")]
    pub reserved_2: [u8; 2],
    // #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    // #[derivative(Debug = "ignore")]
    pub reserved_3: [u8; 8],
    pub protocol_take_rate_pct: u8,
    pub protocol_liquidation_fee_pct: u8,
    pub loan_to_value_pct: u8,
    pub liquidation_threshold_pct: u8,
    pub min_liquidation_bonus_bps: u16,
    pub max_liquidation_bonus_bps: u16,
    pub bad_debt_liquidation_bonus_bps: u16,
    pub deleveraging_margin_call_period_secs: u64,
    pub deleveraging_threshold_decrease_bps_per_day: u64,
    pub fees: ReserveFees,
    pub borrow_rate_curve: BorrowRateCurve,
    pub borrow_factor_pct: u64,

    pub deposit_limit: u64,
    pub borrow_limit: u64,
    pub token_info: TokenInfo,

    pub deposit_withdrawal_cap: WithdrawalCaps,
    pub debt_withdrawal_cap: WithdrawalCaps,

    pub elevation_groups: [u8; 20],
    pub disable_usage_as_coll_outside_emode: u8,

    pub utilization_limit_block_borrowing_above_pct: u8,

    // #[cfg_attr(feature = "serde", serde(with = "serde_bool_u8"))]
    pub autodeleverage_enabled: u8,

    // #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    // #[derivative(Debug = "ignore")]
    pub reserved_1: [u8; 1],

    pub borrow_limit_outside_elevation_group: u64,

    pub borrow_limit_against_this_collateral_in_elevation_group: [u64; 32],

    pub deleveraging_bonus_increase_bps_per_day: u64,
}
#[derive(BorshDeserialize, BorshSerialize, Default, PartialEq, Eq, Debug)]
// #[derivative(Debug)]
// #[zero_copy]
#[repr(C)]
pub struct ReserveFees {
    pub borrow_fee_sf: u64,
    pub flash_loan_fee_sf: u64,
    // #[derivative(Debug = "ignore")]
    pub padding: [u8; 8],
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq)]
// #[zero_copy]
#[repr(C)]
pub struct BorrowRateCurve {
    pub points: [CurvePoint; 11],
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, PartialEq, Eq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct CurvePoint {
    pub utilization_rate_bps: u32,
    pub borrow_rate_bps: u32,
}


#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, Default, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[zero_copy]
#[repr(C)]
pub struct WithdrawalCaps {
    pub config_capacity: i64,
    // #[cfg_attr(
    //     all(feature = "serde", not(feature = "serialize_caps_interval_values")),
    //     serde(skip)
    // )]
    pub current_total: i64,
    // #[cfg_attr(
    //     all(feature = "serde", not(feature = "serialize_caps_interval_values")),
    //     serde(skip)
    // )]
    pub last_interval_start_timestamp: u64,
    pub config_interval_length_seconds: u64,
}
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, Default, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct TokenInfo {
    // #[cfg_attr(feature = "serde", serde(with = "serde_name"))]
    pub name: [u8; 32],

    pub heuristic: PriceHeuristic,

    pub max_twap_divergence_bps: u64,

    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,

    // #[cfg_attr(feature = "serde", serde(default))]
    pub scope_configuration: ScopeConfiguration,

    // #[cfg_attr(feature = "serde", serde(default))]
    pub switchboard_configuration: SwitchboardConfiguration,

    // #[cfg_attr(feature = "serde", serde(default))]
    pub pyth_configuration: PythConfiguration,

    pub block_price_usage: u8,

    // #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    pub reserved: [u8; 7],

    // #[cfg_attr(feature = "serde", serde(skip_serializing, default))]
    pub _padding: [u64; 19],
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq, Default)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct PriceHeuristic {
    pub lower: u64,
    pub upper: u64,
    pub exp: u64,
}
#[derive(BorshDeserialize, BorshSerialize, Default, Debug, PartialEq, Eq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct ScopeConfiguration {
    // #[cfg_attr(feature = "serde", serde(with = "serde_string", default))]
    pub price_feed: Pubkey,
    // #[cfg_attr(feature = "serde", serde(with = "serde_scope_chain"))]
    pub price_chain: [u16; 4],
    // #[cfg_attr(feature = "serde", serde(with = "serde_scope_chain"))]
    pub twap_chain: [u16; 4],
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq, Default)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
// #[zero_copy]
#[repr(C)]
pub struct SwitchboardConfiguration {
    // #[cfg_attr(feature = "serde", serde(with = "serde_string", default))]
    pub price_aggregator: Pubkey,
    // #[cfg_attr(feature = "serde", serde(with = "serde_string", default))]
    pub twap_aggregator: Pubkey,
}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq, Default)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[zero_copy]
// #[repr(transparent)]
pub struct PythConfiguration {
    // #[cfg_attr(feature = "serde", serde(with = "serde_string", default))]
    pub price: Pubkey,
}