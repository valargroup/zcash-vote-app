use anyhow::{Error, Result};
use orchard::vote::Ballot;
use rusqlite::Connection;
use std::sync::Mutex;

use bip0039::Mnemonic;
use tauri::State;
use zcash_address::unified::Encoding;
use zcash_vote::election::{Election, BALLOT_VK};

use crate::{db::mark_spent, state::AppState};

#[tauri::command]
pub fn validate_key(key: String) -> bool {
    if Mnemonic::from_phrase(&key).is_ok() {
        return true;
    }
    if zcash_address::unified::Ufvk::decode(&key).is_ok() {
        return true;
    }
    false
}

#[tauri::command]
pub fn validate_ballot(ballot: String, state: State<Mutex<AppState>>) -> Result<(), String> {
    tauri_export!(state, _connection, {
        let election = &state.election;
        let ballot = serde_json::from_str::<Ballot>(&ballot)?;
        orchard::vote::validate_ballot(ballot, election.signature_required, &BALLOT_VK)?;
        Ok::<_, Error>(())
    })
}

pub fn handle_ballot(
    connection: &Connection,
    _election: &Election,
    height: u32,
    ballot: &Ballot,
) -> Result<()> {
    for action in ballot.data.actions.iter() {
        mark_spent(connection, height, &action.nf)?;
    }
    Ok(())
}
