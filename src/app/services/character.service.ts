import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { FullCharacterData } from "../types/character/FullCharacterData";
import { Service } from "../types/Service";

@Injectable({
	providedIn: "root",
})
export class CharacterService implements Service {
	public async getAll(): Promise<FullCharacterData[]> {
		try {
			let characters = await invoke<FullCharacterData[]>(
				"get_all_characters"
			);
			return characters;
		} catch (e) {
			console.error("Failed to fetch characters:", e);
			throw e;
		}
	}

	public async getById(id: number): Promise<FullCharacterData> {
		try {
			let character = await invoke<FullCharacterData>(
				"get_character_by_id",
				{ id }
			);
			return character;
		} catch (e) {
			console.error("Failed to fetch character:", e);
			throw e;
		}
	}

	public async create(): Promise<FullCharacterData> {
		try {
			let newCharacter = await invoke<FullCharacterData>(
				"create_character"
			);
			return newCharacter;
		} catch (e) {
			console.error("Failed to create new character:", e);
			throw e;
		}
	}
}
