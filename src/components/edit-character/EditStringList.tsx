import { useFormContext } from "react-hook-form";
import { Button, Card, Input } from "@chakra-ui/react";
import type { CharacterFormData } from "../../schemas/characterSchema";

interface EditStringListProps {
	fieldName: "languages" | "killList";
	title: string;
	addLabel: string;
}

export default function EditStringList(props: EditStringListProps) {
	const { fieldName, title, addLabel } = props;
	const { watch, setValue, register } = useFormContext<CharacterFormData>();

	const items = watch(fieldName);

	if (!items) return null;

	function handleAdd() {
		setValue(fieldName, [...items, ""]);
	}

	function handleRemove(index: number) {
		setValue(
			fieldName,
			items.filter((_, i) => i !== index)
		);
	}

	return (
		<Card.Root>
			<Card.Title>{title}</Card.Title>
			<Card.Body className="flex flex-col gap-2">
				{items.map((_, index) => (
					<div key={index} className="flex gap-2">
						<Input {...register(`${fieldName}.${index}`)} />
						<Button
							variant="outline"
							size="sm"
							onClick={() => handleRemove(index)}
						>
							Remove
						</Button>
					</div>
				))}
				<Button variant="outline" size="sm" onClick={handleAdd}>
					{addLabel}
				</Button>
			</Card.Body>
		</Card.Root>
	);
}
