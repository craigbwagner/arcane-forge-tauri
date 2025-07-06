import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { FullCharacterData } from "../../types/character/FullCharacterData";

@Component({
  selector: "app-characters",
  standalone: true,
  imports: [],
  templateUrl: "./characters.component.html",
  styleUrl: "./characters.component.css",
})
export class CharactersComponent {
  async fetchCharacters(): Promise<FullCharacterData> {
    return await invoke("get_all_characters");
  }

  async createCharacter(): Promise<FullCharacterData> {
    return await invoke("create_character");
  }
}
