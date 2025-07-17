import { FullCharacterData } from "./../../types/character/FullCharacterData";
import { Component, Input, WritableSignal } from "@angular/core";


@Component({
    selector: "app-character-details-display",
    imports: [],
    templateUrl: "./character-details-display.component.html",
    styleUrl: "./character-details-display.component.css"
})
export class CharacterDetailsDisplayComponent{
  @Input() character!: FullCharacterData;
}
