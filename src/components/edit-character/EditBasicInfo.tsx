import { useFormContext } from "react-hook-form";
import { Card, Field, Input } from "@chakra-ui/react";
import { ALIGNMENT_LABELS, SEX_LABELS, SIZE_LABELS } from "../../utils";
import type { CharacterFormData } from "../../schemas/characterSchema";

export default function EditBasicInfo() {
	const {
		register,
		formState: { errors },
	} = useFormContext<CharacterFormData>();

	return (
		<Card.Root>
			<Card.Title>Basic Info</Card.Title>
			<Card.Body className="flex flex-col gap-4">
				<div className="grid grid-cols-2 gap-4">
					<Field.Root invalid={!!errors.name}>
						<Field.Label>Name</Field.Label>
						<Input {...register("name")} />
						<Field.ErrorText>{errors.name?.message}</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.creator}>
						<Field.Label>Creator</Field.Label>
						<Input {...register("creator")} />
						<Field.ErrorText>{errors.creator?.message}</Field.ErrorText>
					</Field.Root>
				</div>

				<div className="grid grid-cols-3 gap-4">
					<Field.Root invalid={!!errors.basicDescription?.race}>
						<Field.Label>Race</Field.Label>
						<Input {...register("basicDescription.race")} />
						<Field.ErrorText>
							{errors.basicDescription?.race?.message}
						</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.basicDescription?.sex}>
						<Field.Label>Sex</Field.Label>
						<select
							{...register("basicDescription.sex")}
							className="w-full rounded border px-2 py-1"
						>
							{Object.entries(SEX_LABELS).map(([value, label]) => (
								<option key={value} value={value}>
									{label}
								</option>
							))}
						</select>
						<Field.ErrorText>
							{errors.basicDescription?.sex?.message}
						</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.basicDescription?.alignment}>
						<Field.Label>Alignment</Field.Label>
						<select
							{...register("basicDescription.alignment")}
							className="w-full rounded border px-2 py-1"
						>
							{Object.entries(ALIGNMENT_LABELS).map(([value, label]) => (
								<option key={value} value={value}>
									{label}
								</option>
							))}
						</select>
						<Field.ErrorText>
							{errors.basicDescription?.alignment?.message}
						</Field.ErrorText>
					</Field.Root>
				</div>

				<div className="grid grid-cols-4 gap-4">
					<Field.Root invalid={!!errors.basicDescription?.size}>
						<Field.Label>Size</Field.Label>
						<select
							{...register("basicDescription.size")}
							className="w-full rounded border px-2 py-1"
						>
							{Object.entries(SIZE_LABELS).map(([value, label]) => (
								<option key={value} value={value}>
									{label}
								</option>
							))}
						</select>
						<Field.ErrorText>
							{errors.basicDescription?.size?.message}
						</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.basicDescription?.age}>
						<Field.Label>Age</Field.Label>
						<Input
							type="number"
							{...register("basicDescription.age", { valueAsNumber: true })}
						/>
						<Field.ErrorText>
							{errors.basicDescription?.age?.message}
						</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.basicDescription?.height}>
						<Field.Label>Height</Field.Label>
						<Input {...register("basicDescription.height")} />
						<Field.ErrorText>
							{errors.basicDescription?.height?.message}
						</Field.ErrorText>
					</Field.Root>

					<Field.Root invalid={!!errors.basicDescription?.weight}>
						<Field.Label>Weight</Field.Label>
						<Input
							type="number"
							{...register("basicDescription.weight", { valueAsNumber: true })}
						/>
						<Field.ErrorText>
							{errors.basicDescription?.weight?.message}
						</Field.ErrorText>
					</Field.Root>
				</div>
			</Card.Body>
		</Card.Root>
	);
}
