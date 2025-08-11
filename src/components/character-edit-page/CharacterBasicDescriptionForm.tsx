import { Stack, Fieldset, Field, Input, Card } from "@chakra-ui/react";
import { useFormContext } from "react-hook-form";
import { CharacterFormData } from "../../types/schemas/CharacterFormData";

export default function CharacterBasicDescriptionForm() {
  const { register, formState: {errors}} = useFormContext<CharacterFormData>();

	return (
		<Card.Root>
			<Card.Header>Character Basic Details</Card.Header>
			<Card.Body>
				<Fieldset.Content>
					<Field.Root>
						<Field.Label>Character Name</Field.Label>
						<Input {...register("name")} />
					</Field.Root>

					<Field.Root>
						<Field.Label>Creator</Field.Label>
						<Input {...register("creator")} />
					</Field.Root>
				</Fieldset.Content>
			</Card.Body>
		</Card.Root>
	);
}
