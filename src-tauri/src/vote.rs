use crate::state::AppState;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{ipc::Channel, State};
use zcash_vote::db::load_prop;

#[tauri::command]
pub fn get_sync_height(state: State<'_, Mutex<AppState>>) -> Result<Option<u32>, String> {
    tauri_export!(state, connection, {
        let height = load_prop(&connection, "height")?.map(|h| h.parse::<u32>().unwrap());
        Ok::<_, Error>(height)
    })
}

#[tauri::command]
pub fn get_available_balance(state: State<'_, Mutex<AppState>>) -> Result<u64, String> {
    tauri_export!(state, connection, {
        let balance = connection.query_row(
            "SELECT SUM(value) FROM notes WHERE spent IS NULL",
            [],
            |r| r.get::<_, Option<u64>>(0),
        )?;
        Ok::<_, Error>(balance.unwrap_or_default())
    })
}

#[tauri::command]
pub async fn vote(
    _address: String,
    _amount: u64,
    _channel: Channel<String>,
    _state: State<'_, Mutex<AppState>>,
) -> Result<String, String> {
    // TODO: re-enable when note/cmx tracking is restored
    Err("Voting not available: note tracking disabled".to_string())
}

#[tauri::command]
pub fn fetch_votes(state: State<'_, Mutex<AppState>>) -> Result<Vec<Vote>, String> {
    tauri_export!(state, connection, {
        let mut s = connection
            .prepare("SELECT id_vote, hash, address, amount FROM votes ORDER BY id_vote")?;
        let rows = s.query_map([], |r| {
            Ok((
                r.get::<_, u32>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, u64>(3)?,
            ))
        })?;
        let mut votes = vec![];
        for r in rows {
            let (id_vote, hash, address, amount) = r?;
            votes.push(Vote {
                id: id_vote,
                hash,
                address,
                amount,
            })
        }
        Ok::<_, Error>(votes)
    })
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Vote {
    pub id: u32,
    pub hash: String,
    pub address: String,
    pub amount: u64,
}
