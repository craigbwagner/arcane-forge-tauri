import type { Alignment } from "./types/character/Alignment";
import type { Sex } from "./types/character/Sex";
import type { Size } from "./types/character/Size";

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === 'string') return error;
  return 'An unknown error occurred';
}

const ALIGNMENT_LABELS: Record<Alignment, string> = {
  LawfulGood: "Lawful Good",
  NeutralGood: "Neutral Good",
  ChaoticGood: "Chaotic Good",
  LawfulNeutral: "Lawful Neutral",
  TrueNeutral: "True Neutral",
  ChaoticNeutral: "Chaotic Neutral",
  LawfulEvil: "Lawful Evil",
  NeutralEvil: "Neutral Evil",
  ChaoticEvil: "Chaotic Evil",
};

const SEX_LABELS: Record<Sex, string> = {
  Male: "Male",
  Female: "Female",
  Other: "Other",
  Unspecified: "Unspecified",
};

const SIZE_LABELS: Record<Size, string> = {
  Tiny: "Tiny",
  Small: "Small",
  Medium: "Medium",
  Large: "Large",
  Huge: "Huge",
  Gargantuan: "Gargantuan",
};

export { getErrorMessage, ALIGNMENT_LABELS, SEX_LABELS, SIZE_LABELS };
