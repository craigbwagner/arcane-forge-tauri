import { FullCharacterData } from "./../../types/character/FullCharacterData";
import { Component, Input, WritableSignal } from "@angular/core";
import { CommonModule } from "@angular/common";

@Component({
	selector: "app-character-details-display",
	standalone: true,
	imports: [CommonModule],
	templateUrl: "./character-details-display.component.html",
	styleUrl: "./character-details-display.component.css",
})
export class CharacterDetailsDisplayComponent{
  @Input() character!: FullCharacterData;
}
