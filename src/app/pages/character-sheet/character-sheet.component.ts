import { Component } from '@angular/core';
import { CharacterDetailsDisplayComponent } from '../../components/character-details-display/character-details-display.component';

@Component({
  selector: 'app-character-sheet',
  standalone: true,
  imports: [CharacterDetailsDisplayComponent],
  templateUrl: './character-sheet.component.html',
  styleUrl: './character-sheet.component.css'
})
export class CharacterSheetComponent {

}
