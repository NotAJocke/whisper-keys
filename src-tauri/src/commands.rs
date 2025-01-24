use crate::{
    packs::{self, load_pack},
    state::AppState,
};

#[tauri::command]
pub fn list_available_packs(state: tauri::State<AppState>) -> Vec<String> {
    let packs = state.packs.read().unwrap();

    packs.clone()
}

#[tauri::command]
pub fn refresh_available_packs(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    let packs_dir = &state.packs_dir;

    let packs = packs::list_available(packs_dir).map_err(|e| e.to_string())?;
    let mut packs_lock = state.packs.write().unwrap();

    *packs_lock = packs.clone();

    Ok(packs)
}

#[tauri::command]
pub fn select_pack(state: tauri::State<AppState>, pack: String) -> Result<(), String> {
    let pack = load_pack(&state.packs_dir, &pack);

    match pack {
        Ok(pack) => {
            *state.volume.write().unwrap() = Some(pack.volume);
            *state.current_pack.write().unwrap() = Some(pack);

            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_volume(state: tauri::State<AppState>, volume: u16) {
    *state.volume.write().unwrap() = Some(volume);
}

#[tauri::command]
pub fn toggle_mute(state: tauri::State<AppState>) {
    let mut lock = state.muted.write().unwrap();

    *lock = !*lock;
}
