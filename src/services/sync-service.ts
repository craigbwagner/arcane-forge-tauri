import { invoke } from "@tauri-apps/api/core";
import { FullCharacterData } from "../types/character/FullCharacterData";

const syncService = {
	async pushToCloud(): Promise<number> {
		try {
			return await invoke<number>("push_to_cloud");
		} catch (error) {
			console.error(error);
			throw error;
		}
	},

	async pullFromCloud(): Promise<FullCharacterData[]> {
		try {
			return await invoke<FullCharacterData[]>("pull_from_cloud");
		} catch (error) {
			console.error(error);
			throw error;
		}
	},

	async checkSyncStatus(): Promise<boolean> {
		try {
			return await invoke<boolean>("check_sync_status");
		} catch (error) {
			console.error(error);
			return false;
		}
	},
};

export default syncService;
