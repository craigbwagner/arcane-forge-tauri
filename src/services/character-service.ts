import { invoke } from "@tauri-apps/api/core";
import { FullCharacterData } from "../types/character/FullCharacterData";
import DataService from "../types/interfaces/DataService";

const characterService = {
	async getAll(): Promise<FullCharacterData[]> {
		try {
			let characters = await invoke<Array<FullCharacterData>>(
				"get_all_characters"
			);
			return characters;
		} catch (error) {
			console.error(error);
			throw error;
		}
	},

	async getById(id: number): Promise<FullCharacterData> {
		try {
			let currentCharacter = await invoke<FullCharacterData>(
				"get_character_by_id",
				{ id }
			);
			return currentCharacter;
		} catch (error) {
			console.error(error);
			throw error;
		}
	},
} satisfies DataService<FullCharacterData>;

export default characterService;
