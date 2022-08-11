#![allow(dead_code)]
#![allow(unused_imports)]
use bitcoind_request::command::{
    get_best_block_hash::GetBestBlockHashCommand,
    get_block::{
        self, GetBlockCommand, GetBlockCommandResponse, GetBlockCommandTransactionResponse,
        GetBlockCommandVerbosity,
    },
    get_block_count::{self, GetBlockCountCommand},
    get_block_hash::GetBlockHashCommand,
    get_block_header::GetBlockHeaderCommand,
    get_block_stats::{
        GetBlockStatsCommand, GetBlockStatsCommandResponse,
        GetBlockStatsCommandWithAllStatsResponse, GetBlockStatsCommandWithSelectiveStatsResponse,
        StatsArgumentChoices, TargetBlockArgument,
    },
    get_blockchain_info::GetBlockchainInfoCommand,
    get_chain_tips::GetChainTipsCommand,
    get_chain_tx_stats::GetChainTxStatsCommand,
    get_difficulty::GetDifficultyCommand,
    get_mining_info::GetMiningInfoCommand,
    get_network_hash_ps::GetNetworkHashPsCommand,
    get_raw_transaction::{GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin},
    get_tx_out::GetTxOutCommand,
    get_tx_out_set_info::GetTxOutSetInfoCommand,
    CallableCommand,
};

use bitcoind_request::{Blockhash, BlockhashHexEncoded};

use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use std::{env, time::SystemTimeError};

const BLOCKS_PER_DIFFICULTY_PERIOD: u64 = 2016;

pub struct Seconds(pub i64);

fn timestamp_is_from_more_than_24_hours_ago(timestamp: i64) -> bool {
    let hour_window_to_calculate_for = 24;

    let current_datetime = chrono::offset::Utc::now();
    let window_to_cacluate_duration_in_seconds =
        Duration::seconds(60 * 60 * hour_window_to_calculate_for);
    let datetime_24_hours_ago = current_datetime - window_to_cacluate_duration_in_seconds;
    let datetime_of_block = Utc.timestamp(timestamp as i64, 0);
    datetime_of_block < datetime_24_hours_ago
}

pub fn get_block_height(client: &Client) -> u64 {
    let block_count = GetBlockCountCommand::new().call(client);
    return block_count.0;
}

pub fn get_time_since_last_block_in_seconds(client: &Client) -> i64 {
    let block_count = GetBlockCountCommand::new().call(client);
    let arg = TargetBlockArgument::Height(block_count.0);
    let block_stats_response = GetBlockStatsCommand::new(arg).call(client);
    let time_of_last_block = match block_stats_response {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };
    let current_datetime = chrono::offset::Utc::now();
    let datetime_of_last_block = Utc.timestamp(time_of_last_block as i64, 0);
    let difference = current_datetime.signed_duration_since(datetime_of_last_block);
    difference.num_seconds()
}

pub fn get_average_block_time_for_last_2016_blocks(client: &Client) -> u64 {
    let block_height = GetBlockCountCommand::new().call(client);
    let block_stats_response =
        GetBlockStatsCommand::new(TargetBlockArgument::Height(block_height.0)).call(client);
    let time_of_most_recent_block = match block_stats_response {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };

    let block_stats_response_for_block_2016_old =
        GetBlockStatsCommand::new(TargetBlockArgument::Height(block_height.0 - 2016)).call(client);
    let time_of_block_2016_old = match block_stats_response_for_block_2016_old {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };

    let duration = time_of_most_recent_block - time_of_block_2016_old;
    let average_seconds_per_block = duration / 2016 as u64;
    average_seconds_per_block
}

pub fn get_average_block_time_for_since_last_difficulty_adjustement(client: &Client) -> u64 {
    let block_height = GetBlockCountCommand::new().call(client);
    let block_stats_response =
        GetBlockStatsCommand::new(TargetBlockArgument::Height(block_height.0)).call(client);
    let time_of_most_recent_block = match block_stats_response {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };

    let block_height_of_last_difficulty_adjustment =
        get_block_height_of_last_difficulty_adjustment(client);
    let block_stats_response_for_last_difficulty_ajustment_block = GetBlockStatsCommand::new(
        TargetBlockArgument::Height(block_height_of_last_difficulty_adjustment),
    )
    .call(client);
    let time_of_last_difficulty_adjustment_block =
        match block_stats_response_for_last_difficulty_ajustment_block {
            GetBlockStatsCommandResponse::AllStats(response) => response.time,
            GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
        };

    let blocks_since_last_retarget =
        BLOCKS_PER_DIFFICULTY_PERIOD as f64 - get_blocks_count_until_retarget(client);

    let duration = time_of_most_recent_block - time_of_last_difficulty_adjustment_block;
    let average_seconds_per_block = duration / blocks_since_last_retarget as u64;
    average_seconds_per_block
}

pub fn get_total_money_supply(client: &Client) -> u64 {
    // calls to gettxoutsetinfo are erroring out due to this: https://github.com/apoelstra/rust-jsonrpc/issues/67
    let tx_out_set_info = GetTxOutSetInfoCommand::new().call(client);
    tx_out_set_info.total_amount
}

// gets the chain size in bytes
pub fn get_chain_size(client: &Client) -> u64 {
    let blockchain_info = GetBlockchainInfoCommand::new().call(client);
    blockchain_info.size_on_disk
}

pub fn get_utxo_set_size(client: &Client) -> u64 {
    let tx_out_set_info = GetTxOutSetInfoCommand::new().call(client);
    tx_out_set_info.txouts
}

pub fn get_total_transactions_count(client: &Client) -> u64 {
    let chain_tx_stats = GetChainTxStatsCommand::new().call(client);
    chain_tx_stats.txcount
}

pub fn get_tps_for_last_30_days(client: &Client) -> f64 {
    // This defaults to getting about 30 days worth of of data
    let chain_tx_stats = GetChainTxStatsCommand::new().call(client);
    let seconds_in_interval = chain_tx_stats.window_interval;
    let transactions_count_in_window = chain_tx_stats.window_tx_count as f64;
    let elapsed_seconds_in_window = seconds_in_interval as f64;
    let tps = transactions_count_in_window / elapsed_seconds_in_window;
    tps
}

// takes a long time
pub fn get_transactions_count_over_last_30_days(client: &Client) -> u64 {
    let chain_tx_stats = GetChainTxStatsCommand::new().call(client);
    chain_tx_stats.window_tx_count
}

pub fn get_total_fee_for_block_at_height(client: &Client, height: u64) -> u64 {
    let block_stats = GetBlockStatsCommand::new(TargetBlockArgument::Height(height))
        .add_selective_stats(vec![StatsArgumentChoices::TotalFee])
        .call(client);
    let total_fee = match block_stats {
        GetBlockStatsCommandResponse::AllStats(response) => response.totalfee,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.totalfee.unwrap(),
    };
    total_fee
}
fn get_subsidy_for_block_at_height(client: &Client, height: u64) -> u64 {
    let block_stats = GetBlockStatsCommand::new(TargetBlockArgument::Height(height))
        .add_selective_stats(vec![StatsArgumentChoices::Subsidy])
        .call(client);
    let subsidy = match block_stats {
        GetBlockStatsCommandResponse::AllStats(response) => response.subsidy,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.subsidy.unwrap(),
    };
    subsidy
}

fn get_timestamp_of_block_at_height(client: &Client, height: u64) -> u64 {
    let block_stats = GetBlockStatsCommand::new(TargetBlockArgument::Height(height))
        .add_selective_stats(vec![StatsArgumentChoices::Time])
        .call(client);
    let time = match block_stats {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };
    time
}

// takes a long time
pub fn get_total_fee_for_24_hours(client: &Client) -> u64 {
    let last_block_height = get_block_height(&client);

    let mut total_fee = 0;
    let mut current_block_height = last_block_height;
    let mut next_block_timestamp = chrono::offset::Utc::now().timestamp();
    // Calculate fee while the blocktime is within the 24 hour window.
    while !timestamp_is_from_more_than_24_hours_ago(next_block_timestamp) {
        // TODO: Very inneficient as we're calling "getblockstats" command twice. Could just do it
        // once.
        let fee = get_total_fee_for_block_at_height(client, current_block_height);
        let time = get_timestamp_of_block_at_height(client, current_block_height);

        total_fee = total_fee + fee;

        current_block_height = current_block_height - 1;
        next_block_timestamp = time as i64;
    }
    total_fee
}

pub fn get_difficulty(client: &Client) -> f64 {
    let difficulty = GetDifficultyCommand::new().call(client);
    difficulty.0
}

pub fn get_current_difficulty_epoch(client: &Client) -> u64 {
    let block_count = get_block_height(client);
    let epoch = (block_count / BLOCKS_PER_DIFFICULTY_PERIOD) + 1;
    epoch
}
pub fn get_block_height_of_last_difficulty_adjustment(client: &Client) -> u64 {
    let last_epoch = get_current_difficulty_epoch(client) - 1;
    let block_height_of_last_difficulty_adjustment = last_epoch * 2016;
    block_height_of_last_difficulty_adjustment
}

pub fn get_mempool_transactions_count(client: &Client) -> u64 {
    let mining_info = GetMiningInfoCommand::new().call(client);
    let mempool_transaction_count = mining_info.pooledtx;
    mempool_transaction_count
}

pub fn get_estimated_hash_rate_per_second_for_block_since_last_difficulty_change(
    client: &Client,
) -> f64 {
    let hash_rate = GetNetworkHashPsCommand::new()
        .set_n_blocks(bitcoind_request::command::get_network_hash_ps::BlocksToIncludeArg::BlocksSinceLastDifficultyChange)
        .call(client);
    hash_rate.0
}

pub fn get_estimated_hash_rate_per_second_for_last_2016_blocks(client: &Client) -> f64 {
    let blocks_to_calculate = 2016;
    let hash_rate = GetNetworkHashPsCommand::new()
        .set_n_blocks(
            bitcoind_request::command::get_network_hash_ps::BlocksToIncludeArg::NBlocks(
                blocks_to_calculate,
            ),
        )
        .call(client);
    hash_rate.0
}
pub fn get_estimated_hash_rate_per_second_for_last_epoch(client: &Client) -> f64 {
    let block_height_of_last_difficulty_adjustment =
        get_block_height_of_last_difficulty_adjustment(client);
    let hash_rate = GetNetworkHashPsCommand::new()
        .set_n_blocks(
            bitcoind_request::command::get_network_hash_ps::BlocksToIncludeArg::NBlocks(
                BLOCKS_PER_DIFFICULTY_PERIOD,
            ),
        )
        .set_height(
            bitcoind_request::command::get_network_hash_ps::HeightArg::Height(
                block_height_of_last_difficulty_adjustment,
            ),
        )
        .call(client);
    hash_rate.0
}

pub fn get_blocks_count_until_retarget(client: &Client) -> f64 {
    let block_count = get_block_height(client);
    let percent_of_epoch_complete: f64 =
        (block_count as f64 / BLOCKS_PER_DIFFICULTY_PERIOD as f64) % 1.0;
    let percent_of_epoch_to_go: f64 = 1.0 - percent_of_epoch_complete;
    let blocks_until_retarget = percent_of_epoch_to_go * (BLOCKS_PER_DIFFICULTY_PERIOD as f64);
    blocks_until_retarget
}

pub fn get_estimated_seconds_until_retarget(client: &Client) -> f64 {
    // TODO: Could we get a more accurate prediction if we use average block times in the current
    // epoch?
    // let average_block_time_for_current_epoch =
    get_average_block_time_for_since_last_difficulty_adjustement(client);
    let blocks_count_until_retarget = get_blocks_count_until_retarget(client);
    10.0 * 60.0 * blocks_count_until_retarget
}

// takes a long time
pub fn get_blocks_mined_over_last_24_hours_count(client: &Client) -> u64 {
    let block_count = get_block_height(client);

    // defaults to current time
    let mut next_block_timestamp: u64 = get_timestamp_of_block_at_height(client, block_count);
    let mut traversed_blocks_count = 0;
    // Calculate fee while the blocktime is within the 24 hour window.
    while !timestamp_is_from_more_than_24_hours_ago(next_block_timestamp as i64) {
        traversed_blocks_count = traversed_blocks_count + 1;

        let block_height_to_calculate = block_count - traversed_blocks_count;
        let time = get_timestamp_of_block_at_height(client, block_height_to_calculate);

        next_block_timestamp = time as u64;
    }

    traversed_blocks_count
}

// takes a long time
pub fn get_average_fees_per_block_over_last_24_hours(client: &Client) -> u64 {
    let block_count = get_block_height(client);

    // defaults to current time
    let mut next_block_timestamp: u64 = get_timestamp_of_block_at_height(client, block_count);
    let mut traversed_blocks_count = 0;
    let mut total_fee = 0;
    // Calculate fee while the blocktime is within the 24 hour window.
    while !timestamp_is_from_more_than_24_hours_ago(next_block_timestamp as i64) {
        let block_fee =
            get_total_fee_for_block_at_height(client, block_count - traversed_blocks_count);
        traversed_blocks_count = traversed_blocks_count + 1;
        total_fee = total_fee + block_fee;

        let next_block_height_to_calculate = block_count - traversed_blocks_count;
        let time = get_timestamp_of_block_at_height(client, next_block_height_to_calculate);

        next_block_timestamp = time as u64;
    }

    total_fee / traversed_blocks_count
}

// takes a long time
pub fn get_average_fees_per_block_over_last_2016_blocks(client: &Client) -> u64 {
    let block_count = get_block_height(client);
    // defaults to current time
    let mut total_fee = 0;

    for i in 0..=2016 {
        // Calculate fee while the blocktime is within the 24 hour window.
        let block_fee = get_total_fee_for_block_at_height(client, block_count - i);
        total_fee = total_fee + block_fee;
    }

    total_fee / 2016
}

pub fn get_fees_as_a_percent_of_reward_for_last_24_hours(client: &Client) -> f64 {
    let block_count = get_block_height(client);

    // defaults to current time
    let mut next_block_timestamp: u64 = get_timestamp_of_block_at_height(client, block_count);
    let mut traversed_blocks_count = 0;
    let mut total_fee = 0.0;
    let mut total_subsidy = 0.0;
    // Calculate fee while the blocktime is within the 24 hour window.
    while !timestamp_is_from_more_than_24_hours_ago(next_block_timestamp as i64) {
        let block_fee =
            get_total_fee_for_block_at_height(client, block_count - traversed_blocks_count);
        let block_subsidy =
            get_subsidy_for_block_at_height(client, block_count - traversed_blocks_count);
        traversed_blocks_count = traversed_blocks_count + 1;
        total_fee = total_fee as f64 + block_fee as f64;
        total_subsidy = total_subsidy as f64 + block_subsidy as f64;

        let next_block_height_to_calculate = block_count - traversed_blocks_count;
        let time = get_timestamp_of_block_at_height(client, next_block_height_to_calculate);

        next_block_timestamp = time as u64;
    }

    total_fee / (total_subsidy + total_fee)
}

pub fn get_fees_as_a_percent_of_reward_for_last_2016_blocks(client: &Client) -> f64 {
    let block_count = get_block_height(client);
    // defaults to current time
    let mut total_fee = 0.0;
    let mut total_subsidy = 0.0;

    for i in 0..=2016 {
        // Calculate fee while the blocktime is within the 24 hour window.
        let block_fee = get_total_fee_for_block_at_height(client, block_count - i);
        let block_subsidy = get_subsidy_for_block_at_height(client, block_count);
        total_fee = total_fee as f64 + block_fee as f64;
        total_subsidy = total_subsidy as f64 + block_subsidy as f64;
    }

    total_fee / (total_subsidy + total_fee)
}

pub fn get_block_subsidy_of_most_recent_block(client: &Client) -> u64 {
    let block_count = get_block_height(client);
    let block_subsidy = get_subsidy_for_block_at_height(client, block_count);
    block_subsidy
}
