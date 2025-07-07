import { Component, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { FullCharacterData } from "../../types/character/FullCharacterData";
import { CharacterService } from "../../services/character.service";

@Component({
  selector: "app-characters",
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: "./characters.component.html",
  styleUrl: "./characters.component.css",
})
export class CharactersComponent implements OnInit {
  characters: FullCharacterData[] = [];
  loading = false;
  error: string | null = null;

  constructor(private characterService: CharacterService) {}

  async ngOnInit() {
    await this.loadCharacters();
  }

  async loadCharacters() {
    this.loading = true;
    this.error = null;

    try {
      this.characters = await this.characterService.getAll();
    } catch (e) {
      this.error = "Failed to load characters.";
      console.error(this.error);
    } finally {
      this.loading = false;
    }
  }

  async refreshCharacters() {
    await this.loadCharacters();
  }

  async createCharacter() {
    await this.characterService.create();
  }
}
