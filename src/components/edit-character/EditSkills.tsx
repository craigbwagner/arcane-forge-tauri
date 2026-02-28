import { useFormContext } from "react-hook-form";
import { Card, Input } from "@chakra-ui/react";
import type { CharacterFormData } from "../../schemas/characterSchema";

export default function EditSkills() {
	const { register, watch } = useFormContext<CharacterFormData>();

	const skills = watch("skills");

	if (!skills) return null;

	return (
		<Card.Root>
			<Card.Title>Skills</Card.Title>
			<Card.Body>
				<div className="grid grid-cols-5 gap-2 text-xs font-semibold opacity-60 mb-2">
					<p>Skill</p>
					<p>Ability</p>
					<p>Prof</p>
					<p>Expertise</p>
					<p>Add. Mods</p>
				</div>
				{skills.map((skill, index) => (
					<div
						key={skill.name}
						className="grid grid-cols-5 gap-2 items-center py-1"
					>
						<p className="text-sm">{skill.name}</p>
						<p className="text-sm opacity-70">{skill.abilityName}</p>
						<input
							type="checkbox"
							{...register(`skills.${index}.isProficient`)}
						/>
						<input
							type="checkbox"
							{...register(`skills.${index}.hasExpertise`)}
						/>
						<Input
							type="number"
							size="sm"
							{...register(`skills.${index}.additionalMods`, {
								valueAsNumber: true,
							})}
						/>
					</div>
				))}
			</Card.Body>
		</Card.Root>
	);
}
