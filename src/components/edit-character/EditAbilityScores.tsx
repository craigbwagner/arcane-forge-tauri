import { useFormContext } from "react-hook-form";
import { Card, Field, Input } from "@chakra-ui/react";
import type { CharacterFormData } from "../../schemas/characterSchema";

export default function EditAbilityScores() {
	const {
		register,
		watch,
		formState: { errors },
	} = useFormContext<CharacterFormData>();

	const abilityScores = watch("abilityScores");

	if (!abilityScores) return null;

	return (
		<Card.Root>
			<Card.Title>Ability Scores</Card.Title>
			<Card.Body className="grid grid-cols-3 gap-4">
				{abilityScores.map((ability, index) => (
					<Card.Root key={ability.shortName} variant="outline">
						<Card.Body className="flex flex-col gap-2">
							<p className="font-semibold text-center">{ability.name}</p>

							<Field.Root
								invalid={!!errors.abilityScores?.[index]?.score}
							>
								<Field.Label>Score</Field.Label>
								<Input
									type="number"
									{...register(`abilityScores.${index}.score`, {
										valueAsNumber: true,
									})}
								/>
								<Field.ErrorText>
									{errors.abilityScores?.[index]?.score?.message}
								</Field.ErrorText>
							</Field.Root>

							<Field.Root
								invalid={!!errors.abilityScores?.[index]?.additionalMods}
							>
								<Field.Label>Additional Mods</Field.Label>
								<Input
									type="number"
									{...register(`abilityScores.${index}.additionalMods`, {
										valueAsNumber: true,
									})}
								/>
								<Field.ErrorText>
									{errors.abilityScores?.[index]?.additionalMods?.message}
								</Field.ErrorText>
							</Field.Root>

							<Field.Root>
								<label className="flex items-center gap-2">
									<input
										type="checkbox"
										{...register(`abilityScores.${index}.isProficient`)}
									/>
									<span>Save Proficiency</span>
								</label>
							</Field.Root>
						</Card.Body>
					</Card.Root>
				))}
			</Card.Body>
		</Card.Root>
	);
}
