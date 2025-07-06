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
      return await invoke<FullCharacterData[]>("get_all_characters");
    } catch (e) {
      console.error("Failed to fetch characters:", e);
      throw e;
    }
  }

  public async create(): Promise<FullCharacterData> {
    throw new Error("Method not implemented.");
  }
}
