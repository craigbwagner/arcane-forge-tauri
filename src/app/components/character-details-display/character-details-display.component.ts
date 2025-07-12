import { Component, effect, inject, OnInit, signal } from "@angular/core";
import { FullCharacterData } from "../../types/character/FullCharacterData";
import { CommonModule } from "@angular/common";
import { ActivatedRoute, Router, RouterModule } from "@angular/router";
import { CharacterService } from "../../services/character.service";
import { toSignal } from "@angular/core/rxjs-interop";

@Component({
	selector: "app-character-details-display",
	standalone: true,
	imports: [CommonModule, RouterModule],
	templateUrl: "./character-details-display.component.html",
	styleUrl: "./character-details-display.component.css",
})
export class CharacterDetailsDisplayComponent implements OnInit {
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
