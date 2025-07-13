import { Component, Input, WritableSignal } from '@angular/core';
import { FullCharacterData } from '../../types/character/FullCharacterData';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-skills-display',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './skills-display.component.html',
  styleUrl: './skills-display.component.css'
})
export class SkillsDisplayComponent {
  @Input() character!: FullCharacterData;
}
