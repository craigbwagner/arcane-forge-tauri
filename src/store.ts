import { FullCharacterData } from "./types/character/FullCharacterData";
import { create } from "zustand";
import characterService from "./services/character-service";
import { getErrorMessage } from "./utils";

interface CharacterState {
  currentCharacter: FullCharacterData | null;
  characters: FullCharacterData[];
  loading: boolean;
  error: string | null;

  getCharacters(): Promise<void>;
}

const useCharacterStore = create<CharacterState>((set) => ({
  currentCharacter: null,
  characters: [],
  loading: false,
  error: null,

  getCharacters: async () => {
    set({loading: true, error: null});
    try {
      const characters = await characterService.getAll();
      set(() => ({ characters }));
      set({loading: false});
    } catch (error) {
      set({error: getErrorMessage(error), loading: false});
    }
  },
}))

export default useCharacterStore;
