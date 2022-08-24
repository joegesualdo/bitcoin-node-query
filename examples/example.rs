#![allow(dead_code)]
#![allow(unused_imports)]
use chrono::{DateTime, TimeZone, Utc};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use std::time::{Duration, SystemTime};
use std::{env, time::SystemTimeError};

use bitcoin_node_query::{
    get_average_block_time_for_last_2016_blocks,
    get_average_block_time_for_since_last_difficulty_adjustement,
    get_average_fees_per_block_over_last_2016_blocks,
    get_average_fees_per_block_over_last_24_hours, get_block_height,
    get_block_height_of_last_difficulty_adjustment, get_block_subsidy_of_most_recent_block,
    get_blocks_count_until_retarget, get_blocks_mined_over_last_24_hours_count, get_chain_size,
    get_current_difficulty_epoch, get_difficulty,
    get_estimated_hash_rate_per_second_for_block_since_last_difficulty_change,
    get_estimated_hash_rate_per_second_for_last_2016_blocks,
    get_estimated_hash_rate_per_second_for_last_epoch, get_estimated_seconds_until_retarget,
    get_fees_as_a_percent_of_reward_for_last_2016_blocks,
    get_fees_as_a_percent_of_reward_for_last_24_hours, get_mempool_transactions_count,
    get_percent_of_vouts_used_segwit_over_last_24_hours, get_time_since_last_block_in_seconds,
    get_total_fee_for_24_hours, get_total_money_supply, get_total_transactions_count,
    get_tps_for_last_30_days, get_transactions_count_over_last_30_days, get_utxo_set_size, Client,
};

pub fn format_duration(seconds: i64) -> String {
    let seconds_formatted = seconds % 60;
    let minutes_formatted = (seconds / 60) % 60;
    format!("{:#?}:{:#?}", minutes_formatted, seconds_formatted)
}

fn main() {
    /////////////////////////////////////////////////////////////////////
    //////////Blockchain Data //////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let url = env::var("BITCOIND_URL").expect("BITCOIND_URL env variable not set");
    let client = Client::new(&url, &username, &password).expect("failed to create client");

    // let block_height = get_block_height(&client);
    // println!("BLOCK HEIGHT: {:#?}", block_height);

    // let seconds_since_last_block = get_time_since_last_block_in_seconds(&client);
    // println!(
    //     "TIME SINCE LAST BLOCK: {}",
    //     format_duration(seconds_since_last_block)
    // );

    // let average_seconds_per_block_last_2016_blocks =
    //     get_average_block_time_for_last_2016_blocks(&client);
    // println!(
    //     "AVERAGE BLOCK TIME (2016): {}",
    //     format_duration(average_seconds_per_block_last_2016_blocks as i64)
    // );
    // let average_seconds_per_block_since_last_difficulty_adjustment =
    //     get_average_block_time_for_since_last_difficulty_adjustement(&client);
    // println!(
    //     "AVERAGE BLOCK TIME (SINCE LAST_DIFFICULTY ADJUSTMENT): {}",
    //     format_duration(average_seconds_per_block_since_last_difficulty_adjustment as i64)
    // );

    // // Errors out because: https://github.com/apoelstra/rust-jsonrpc/issues/67
    // // let total_money_supply = get_total_money_supply(&client);
    // // println!("TOTAL MONEY SUPPLY: {:#?}", total_money_supply);

    // let chain_size = get_chain_size(&client);
    // let chain_size_in_gbs = chain_size as f64 / 1_000_000_000.0;
    // println!("CHAIN SIZE: {:#?}GB", chain_size_in_gbs);

    // // Errors out because: https://github.com/apoelstra/rust-jsonrpc/issues/67
    // // let utxo_set_size = get_utxo_set_size(&client);
    // // println!("UTXO SET SIZE: {:#?}", utxo_set_size);

    // /////////////////////////////////////////////////////////////////////
    // //////////Blockchain Data //////////////////////////////////////////
    // /////////////////////////////////////////////////////////////////////
    // let total_transactions_count = get_total_transactions_count(&client);
    // println!("TOTAL TRANSACTIONS COUNT: {:#?}", total_transactions_count);

    // let tps_for_last_30_days = get_tps_for_last_30_days(&client);
    // println!(
    //     "TRANSACTIONS PER SECOND (last 30 days): {:#?} tx/s",
    //     tps_for_last_30_days
    // );

    // // takes a long time
    // let transactions_count_over_last_30_days = get_transactions_count_over_last_30_days(&client);
    // println!(
    //     "TRANSACTIONS COUNT OVER LAST 30 DAYS: {:#?}",
    //     transactions_count_over_last_30_days
    // );

    // // takes a long time
    // let total_fee_for_24_hours = get_total_fee_for_24_hours(&client);
    // println!(
    //     "TOTAL FEE FOR LAST 24 hours: {:#?} btc",
    //     total_fee_for_24_hours as f64 / 100_000_000.0
    // );

    // let difficulty = get_difficulty(&client);
    // let trillion: u64 = 1_000_000_000_000;
    // let difficulty_per_trillion: f64 = difficulty as f64 / trillion as f64;
    // println!("Difficulty: {:.2}x10^12", difficulty_per_trillion);

    // let current_difficulty_epoch = get_current_difficulty_epoch(&client);
    // println!("CURRENT EPOCH: {:?}", current_difficulty_epoch);
    // let block_height_of_last_difficulty_adjustment =
    //     get_block_height_of_last_difficulty_adjustment(&client);
    // println!(
    //     "BLOCK HEIGHT OF LAST DIFFICULTY: {:?}",
    //     block_height_of_last_difficulty_adjustment
    // );

    // let mempool_transaction_count = get_mempool_transactions_count(&client);
    // println!("MEMPOOL TRANSACTION COUNT: {:?}", mempool_transaction_count);

    // let hash_rate_since_last_difficulty_change =
    //     get_estimated_hash_rate_per_second_for_block_since_last_difficulty_change(&client);
    // println!(
    //     "ESTIMATED HASH RATE SINCE LAST DIFFICULTY CHANGE: {}",
    //     hash_rate_since_last_difficulty_change
    // );
    // let hash_rate_for_last_2016_blocks =
    //     get_estimated_hash_rate_per_second_for_last_2016_blocks(&client);
    // println!(
    //     "ESTIMATED HASH RATE FOR LAST 2016 BLOCKS: {}",
    //     hash_rate_for_last_2016_blocks
    // );
    // let hash_rate_for_last_epoch = get_estimated_hash_rate_per_second_for_last_epoch(&client);
    // println!(
    //     "ESTIMATED HASH RATE FOR LAST EPOCH: {}",
    //     hash_rate_for_last_epoch
    // );
    // let blocks_till_difficulty_adjustment = get_blocks_count_until_retarget(&client);
    // println!(
    //     "BLOCKS TILL DIFFICULTY ADJUSTMENT: {}",
    //     blocks_till_difficulty_adjustment
    // );
    // let estimated_seconds_until_retarget = get_estimated_seconds_until_retarget(&client);
    // println!(
    //     "ESTIMATED SECONDS UNTIL RETARGET: {}",
    //     estimated_seconds_until_retarget
    // );

    // // takes a long time
    // let blocks_mined_over_last_24_hours = get_blocks_mined_over_last_24_hours_count(&client);
    // println!(
    //     "BLOCKS MINED OVER LAST 24 HOURS: {}",
    //     blocks_mined_over_last_24_hours
    // );
    // // takes a long time
    // let average_fees_per_block_over_last_24_hours =
    //     get_average_fees_per_block_over_last_24_hours(&client);
    // println!(
    //     "AVERAGE FEES PER BLOCK OVER LAST 24 HOURS: {} btc",
    //     average_fees_per_block_over_last_24_hours as f64 / 100_000_000.0
    // );
    // // takes a long time
    // let average_fees_per_block_over_last_2016_blocks =
    //     get_average_fees_per_block_over_last_2016_blocks(&client);
    // println!(
    //     "AVERAGE FEES PER BLOCK OVER LAST 2016 BLOCKS: {} btc",
    //     average_fees_per_block_over_last_2016_blocks as f64 / 100_000_000.0
    // );

    // //takes a long time
    // let fees_as_a_percent_of_reward_for_last_24_hours_ =
    //     get_fees_as_a_percent_of_reward_for_last_24_hours(&client);
    // println!(
    //     "FEES AS A PERCENT OF REWARD OVER THE LAST 24 HOURS: {}",
    //     fees_as_a_percent_of_reward_for_last_24_hours_
    // );
    // // takes a long time
    // let fees_as_a_percent_of_reward_for_last_2016_blocks =
    //     get_fees_as_a_percent_of_reward_for_last_2016_blocks(&client);
    // println!(
    //     "FEES AS A PERCENT OF REWARD OVER THE LAST 2016 BLOCKS: {}",
    //     fees_as_a_percent_of_reward_for_last_2016_blocks
    // );

    // let block_subsidy_of_most_recent_block = get_block_subsidy_of_most_recent_block(&client);
    // println!(
    //     "BLOCK SUBSIDY OF MOST RECENT BLOCK: {} btc",
    //     block_subsidy_of_most_recent_block as f64 / 100_000_000.0
    // );

    // I expect about 37.4% Script hash transactions over the last 90 days
    //let script_hash = get_percent_of_vouts_used_scripthash_over_last_90_days(&client);
    //println!("scripthash (starts with 3)%: {:#?}", script_hash);

    // I expect about 37.4% Script hash transactions over the last 90 days
    // SEGWIT ADOPTION
    // TODO: check the is_segwith_adress function and make sure it's exactly how you want to
    // define what segwith type you're looking for
    let (
        percent_based_on_vouts,
        percent_based_on_vins_or_vouts,
        percent_based_on_transaction_hexes,
        percent_of_payments_spending_segwit_per_day,
        percent_of_segwit_spending_transactions_per_day,
    ) = get_percent_of_vouts_used_segwit_over_last_24_hours(&client);
    println!("segwit percent (vouts): {:#?}", percent_based_on_vouts);
    println!(
        "segwit percent (vouts or vins): {:#?}",
        percent_based_on_vins_or_vouts
    );
    // https://bitbo.io/
    println!(
        "segwit percent (transaction_hexes): {:#?}",
        percent_based_on_transaction_hexes
    );
    // https://transactionfee.info/charts/payments-spending-segwit/
    println!(
        "percent of payments spending segwit_per_day: {:#?}",
        percent_of_payments_spending_segwit_per_day
    );
    // https://transactionfee.info/charts/transactions-spending-segwit/
    println!(
        "percent of segwit spending transactions per day: {:#?}",
        percent_of_segwit_spending_transactions_per_day
    );
}
