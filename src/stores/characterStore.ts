import { FullCharacterData } from "../types/character/FullCharacterData";
import { create } from "zustand";
import characterService from "../services/character-service";
import { getErrorMessage } from "../utils";

interface CharacterState {
	currentCharacter: FullCharacterData | null;
	characters: FullCharacterData[];
	loading: boolean;
	error: string | null;

	getCharacters(): Promise<void>;
	getCurrentCharacter(id: number): Promise<void>;
	clearCurrentCharacter(): void;
	createCharacter(): Promise<FullCharacterData | undefined>;
	deleteCharacter(id: number): Promise<boolean>;
}

const useCharacterStore = create<CharacterState>((set, get) => ({
	currentCharacter: null,
	characters: [],
	loading: false,
	error: null,

	getCharacters: async () => {
		set({ loading: true, error: null });
		try {
			const characters = await characterService.getAll();
			set(() => ({ characters }));
			set({ loading: false });
		} catch (error) {
			set({ error: getErrorMessage(error), loading: false });
		}
	},
	getCurrentCharacter: async (id: number) => {
		set({ loading: true, error: null });
		try {
			const currentCharacter = await characterService.getById(id);
			set(() => ({ currentCharacter }));
			set({ loading: false });
		} catch (error) {
			set({ error: getErrorMessage(error), loading: false });
		}
	},
	clearCurrentCharacter: () => {
		set(() => ({ currentCharacter: null }));
	},
	createCharacter: async (): Promise<FullCharacterData> => {
		set({ error: null });

		try {
			return await characterService.create();
		} catch (error) {
			set({ error: getErrorMessage(error) });
			throw error;
		}
	},
	deleteCharacter: async (id: number): Promise<boolean> => {
		set({ error: null });

		try {
			return await characterService.delete(id);
		} catch (error) {
			set({ error: getErrorMessage(error) });
			throw error;
		}
	},
}));

export default useCharacterStore;
