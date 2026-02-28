import { z } from "zod";

const alignmentSchema = z.enum([
	"LawfulGood",
	"NeutralGood",
	"ChaoticGood",
	"LawfulNeutral",
	"TrueNeutral",
	"ChaoticNeutral",
	"LawfulEvil",
	"NeutralEvil",
	"ChaoticEvil",
]);

const sexSchema = z.enum(["Male", "Female", "Other", "Unspecified"]);

const sizeSchema = z.enum([
	"Tiny",
	"Small",
	"Medium",
	"Large",
	"Huge",
	"Gargantuan",
]);

const abilitySchema = z.enum([
	"Strength",
	"Dexterity",
	"Constitution",
	"Intelligence",
	"Wisdom",
	"Charisma",
]);

const basicDescriptionSchema = z.object({
	race: z.string().min(1, "Race is required"),
	sex: sexSchema,
	size: sizeSchema,
	age: z.number().int().min(0, "Age must be non-negative"),
	height: z.string().min(1, "Height is required"),
	weight: z.number().min(0, "Weight must be non-negative"),
	alignment: alignmentSchema,
});

const abilityScoreSchema = z.object({
	name: z.string(),
	shortName: z.string(),
	isProficient: z.boolean(),
	score: z.number().int().min(1, "Min score is 1").max(30, "Max score is 30"),
	additionalMods: z.number().int(),
	// Server-calculated fields — included but not user-editable
	baseModifier: z.number(),
	totalMod: z.number(),
	save: z.number(),
	additionalSaveMods: z.number().int(),
});

const skillSchema = z.object({
	name: z.string(),
	isProficient: z.boolean(),
	hasExpertise: z.boolean(),
	abilityName: abilitySchema,
	additionalMods: z.number().int(),
	// Server-calculated
	totalMod: z.number(),
});

const combatStatsSchema = z.object({
	// Server-calculated
	initiative: z.number(),
	// User-editable
	initiativeMods: z.number().int(),
	speed: z.number().int().min(0, "Speed must be non-negative"),
	maxHp: z.number().int().min(1, "Max HP must be at least 1"),
	currentHp: z.number().int(),
	tempHp: z.number().int().min(0, "Temp HP must be non-negative"),
	hitDiceRemaining: z.number().int().min(0, "Hit dice must be non-negative"),
});

// Fixed-length tuples matching the TypeScript types from ts-rs
const abilityScoresTuple = z.tuple([
	abilityScoreSchema,
	abilityScoreSchema,
	abilityScoreSchema,
	abilityScoreSchema,
	abilityScoreSchema,
	abilityScoreSchema,
]);

const skillsTuple = z.tuple([
	skillSchema, skillSchema, skillSchema,
	skillSchema, skillSchema, skillSchema,
	skillSchema, skillSchema, skillSchema,
	skillSchema, skillSchema, skillSchema,
	skillSchema, skillSchema, skillSchema,
	skillSchema, skillSchema, skillSchema,
]);

export const characterSchema = z.object({
	id: z.number().int(),
	name: z.string().min(1, "Name is required"),
	creator: z.string().min(1, "Creator is required"),
	// Server-calculated
	proficiencyBonus: z.number(),
	basicDescription: basicDescriptionSchema,
	combatStats: combatStatsSchema,
	languages: z.array(z.string()),
	abilityScores: abilityScoresTuple,
	skills: skillsTuple,
	killList: z.array(z.string()),
	createdAt: z.string(),
	updatedAt: z.string(),
});

export type CharacterFormData = z.infer<typeof characterSchema>;
