import { Component } from '@angular/core';
import { FullCharacterData } from '../../types/character/FullCharacterData';

@Component({
  selector: 'app-character-details-display',
  standalone: true,
  imports: [],
  templateUrl: './character-details-display.component.html',
  styleUrl: './character-details-display.component.css'
})
export class CharacterDetailsDisplayComponent {

  // getAbilities(): Array<{name: string, score: number}> {
  //   return [
  //     { name: 'STR', score: this.character.abilities.strength },
  //     { name: 'DEX', score: this.character.abilities.dexterity },
  //     { name: 'CON', score: this.character.abilities.constitution },
  //     { name: 'INT', score: this.character.abilities.intelligence },
  //     { name: 'WIS', score: this.character.abilities.wisdom },
  //     { name: 'CHA', score: this.character.abilities.charisma }
  //   ];
  // }

  getModifier(score: number): string {
    const modifier = Math.floor((score - 10) / 2);
    return modifier >= 0 ? `+${modifier}` : `${modifier}`;
  }
}
