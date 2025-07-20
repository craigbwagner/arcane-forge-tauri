import { invoke } from "@tauri-apps/api/core";
import { FullCharacterData } from "../types/character/FullCharacterData";

const characterService = {
	async getAll(): Promise<FullCharacterData[]> {
		try {
			let characters = await invoke<Array<FullCharacterData>>(
				"get_all_characters"
			);
      console.log(characters);
			return characters;
		} catch (e) {
			console.error(e);
			throw e;
		}
	},
};

export default characterService;
