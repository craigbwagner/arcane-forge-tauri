import { useFormContext } from "react-hook-form";
import { Card, Field, Input } from "@chakra-ui/react";
import type { CharacterFormData } from "../../schemas/characterSchema";

export default function EditCombatStats() {
	const {
		register,
		formState: { errors },
	} = useFormContext<CharacterFormData>();

	return (
		<Card.Root>
			<Card.Title>Combat Stats</Card.Title>
			<Card.Body className="grid grid-cols-3 gap-4">
				<Field.Root invalid={!!errors.combatStats?.initiativeMods}>
					<Field.Label>Initiative Mods</Field.Label>
					<Input
						type="number"
						{...register("combatStats.initiativeMods", {
							valueAsNumber: true,
						})}
					/>
					<Field.ErrorText>
						{errors.combatStats?.initiativeMods?.message}
					</Field.ErrorText>
				</Field.Root>

				<Field.Root invalid={!!errors.combatStats?.speed}>
					<Field.Label>Speed</Field.Label>
					<Input
						type="number"
						{...register("combatStats.speed", { valueAsNumber: true })}
					/>
					<Field.ErrorText>
						{errors.combatStats?.speed?.message}
					</Field.ErrorText>
				</Field.Root>

				<Field.Root invalid={!!errors.combatStats?.maxHp}>
					<Field.Label>Max HP</Field.Label>
					<Input
						type="number"
						{...register("combatStats.maxHp", { valueAsNumber: true })}
					/>
					<Field.ErrorText>
						{errors.combatStats?.maxHp?.message}
					</Field.ErrorText>
				</Field.Root>

				<Field.Root invalid={!!errors.combatStats?.currentHp}>
					<Field.Label>Current HP</Field.Label>
					<Input
						type="number"
						{...register("combatStats.currentHp", { valueAsNumber: true })}
					/>
					<Field.ErrorText>
						{errors.combatStats?.currentHp?.message}
					</Field.ErrorText>
				</Field.Root>

				<Field.Root invalid={!!errors.combatStats?.tempHp}>
					<Field.Label>Temp HP</Field.Label>
					<Input
						type="number"
						{...register("combatStats.tempHp", { valueAsNumber: true })}
					/>
					<Field.ErrorText>
						{errors.combatStats?.tempHp?.message}
					</Field.ErrorText>
				</Field.Root>

				<Field.Root invalid={!!errors.combatStats?.hitDiceRemaining}>
					<Field.Label>Hit Dice Remaining</Field.Label>
					<Input
						type="number"
						{...register("combatStats.hitDiceRemaining", {
							valueAsNumber: true,
						})}
					/>
					<Field.ErrorText>
						{errors.combatStats?.hitDiceRemaining?.message}
					</Field.ErrorText>
				</Field.Root>
			</Card.Body>
		</Card.Root>
	);
}
