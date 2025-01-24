import { invoke } from "@tauri-apps/api/core";

export const load = async () => {

  let packs = await invoke("list_available_packs");

  return {
    packs
  };
}
