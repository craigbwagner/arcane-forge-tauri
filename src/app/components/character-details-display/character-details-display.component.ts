import { FullCharacterData } from "./../../types/character/FullCharacterData";
import { Component, Input } from "@angular/core";
import { Card } from "primeng/card";


@Component({
	selector: "app-character-details-display",
	imports: [Card],
	templateUrl: "./character-details-display.component.html",
	styleUrl: "./character-details-display.component.css",
})
export class CharacterDetailsDisplayComponent {
	@Input() character!: FullCharacterData;
}
