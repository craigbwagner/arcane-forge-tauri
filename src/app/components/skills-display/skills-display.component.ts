import { Component, Input } from '@angular/core';
import { FullCharacterData } from '../../types/character/FullCharacterData';


@Component({
    selector: 'app-skills-display',
    imports: [],
    templateUrl: './skills-display.component.html',
    styleUrl: './skills-display.component.css'
})
export class SkillsDisplayComponent {
  @Input() character!: FullCharacterData;
}
