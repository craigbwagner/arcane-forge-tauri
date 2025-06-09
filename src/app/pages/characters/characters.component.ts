import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

@Component({
  selector: 'app-characters',
  standalone: true,
  imports: [],
  templateUrl: './characters.component.html',
  styleUrl: './characters.component.css'
})
export class CharactersComponent {
  async createCharacter() : Promise<void> {
    let newCharacterId: number = await invoke('create_character');
  }
}
