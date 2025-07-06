import { FullCharacterData } from "./character/FullCharacterData";

export interface Service {
  getAll(): Promise<FullCharacterData[]>;
  create(): Promise<FullCharacterData>;
}
