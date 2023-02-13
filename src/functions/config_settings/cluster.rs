/*
    -Change the cluster on the change of the state in the config.sjon file.
    True == Mainnet     False == Devnet
*/

use anchor_client::Cluster;
use anyhow::{Result, Ok};
use crate::functions::{
    config_settings::get_config_settings::get_config_settings,
    constants::config::Config
};

pub fn cluster() -> Result<Cluster> {
    let config: Config = get_config_settings().unwrap();
    let cluster: Cluster = match config.network {
        true => Cluster::Mainnet,
        false => Cluster::Devnet
    };
    Ok(cluster)
}