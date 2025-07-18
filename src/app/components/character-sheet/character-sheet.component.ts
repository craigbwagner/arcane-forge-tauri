import { Component, inject, signal } from '@angular/core';
import { CharacterDetailsDisplayComponent } from '../character-details-display/character-details-display.component';
import { toSignal } from '@angular/core/rxjs-interop';
import { Router, ActivatedRoute, RouterModule } from '@angular/router';
import { CharacterService } from '../../services/character.service';
import { FullCharacterData } from '../../types/character/FullCharacterData';

import { SkillsDisplayComponent } from "../skills-display/skills-display.component";

@Component({
	selector: "app-character-sheet",
	imports: [
		CharacterDetailsDisplayComponent,
		RouterModule,
		SkillsDisplayComponent,
	],
	templateUrl: "./character-sheet.component.html",
	styleUrl: "./character-sheet.component.css",
	standalone: true,
})
export class CharacterSheetComponent {
	private router = inject(Router);
	private activatedRoute = inject(ActivatedRoute);
	private characterService = inject(CharacterService);
	private routeParams = toSignal(this.activatedRoute.paramMap);

	character = signal<FullCharacterData | undefined>(undefined);
	loading = signal(false);
	error = signal<string | null>(null);

	ngOnInit() {
		const params = this.routeParams();
		const id = params?.get("id");
		if (id) {
			this.loadCharacter(+id);
		}
	}

	async loadCharacter(id: number) {
		this.loading.set(true);
		this.error.set(null);

		try {
			const character = await this.characterService.getById(id);
			this.character.set(character);
		} catch (e) {
			this.error.set("Failed to load character.");
			console.error(e);
		} finally {
			this.loading.set(false);
		}
	}
}
