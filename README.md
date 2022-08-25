☠️⚠️ Work In Progress ⚠️☠️
# Bitcoin Node Query 
> Request information from a Bitcoin node

This library provides helpful functions to query common information about the bitcoin network.

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
bitcoin-node-query = "0.1.4"
```

## Usage:
```rust
use bitcoin_node_query::{
  Client,
  get_block_height,
  get_time_since_last_block_in_seconds,
  get_average_block_time_for_last_2016_blocks
}

// Create a Client.
let bitcoind_password: &str = ...
let bitcoind_username: &str = ...
let bitcoind_url = "127.0.0.1:8332"
let client = Client::new(
        &bitcoind_url,
        &bitcoind_username,
        &bitcoind_password
    ).expect("failed to create client");

// get block height
let block_height = get_block_height(&client);
println!("Block height: {:#?}", block_height);

// Get how many seconds ago the last block was mined 
let seconds_since_last_block = get_time_since_last_block_in_seconds(&client);
println!(
    "Seconds since last block:",
    format_duration(seconds_since_last_block)
);

// Get the average seconds it took for each of the last 2016 blocks to be mined
let average_seconds_per_block_last_2016_blocks =
get_average_block_time_for_last_2016_blocks(&client);
println!(
    "Average block time for last 2016 blocks",
    format_duration(average_seconds_per_block_last_2016_blocks as i64)
);

```
## API

Find a list of all the functions available in the [documentation](https://docs.rs/bitcoin-node-query/0.1.2/bitcoin_node_query/)

## Related
- [bitcoind-request](https://github.com/joegesualdo/bitcoind-request) - Type-safe wrapper around bitcoind RPC commands
- [bitcoin-terminal-dashboard](https://github.com/joegesualdo/bitcoin-terminal-dashboard) - Bitcoin Dashboard in the terminal

## License
MIT © [Joe Gesualdo]()
