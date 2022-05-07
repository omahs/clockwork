use std::sync::Arc;

use solana_client_helpers::Client;
use solana_sdk::pubkey::Pubkey;

use crate::{
    cli::CliError,
    utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
};

pub fn create(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_pda = cronos_sdk::scheduler::state::Daemon::pda(owner);
    let fee_pda = cronos_sdk::scheduler::state::Fee::pda(daemon_pda.0);
    let ix = cronos_sdk::scheduler::instruction::daemon_new(daemon_pda, fee_pda, owner);
    sign_and_submit(client, &[ix]);
    get(client, &daemon_pda.0)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let daemon_data = cronos_sdk::scheduler::state::Daemon::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", daemon_data);
    Ok(())
}