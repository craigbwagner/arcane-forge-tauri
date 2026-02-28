import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { useForm, FormProvider } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@chakra-ui/react";
import useCharacterStore from "../stores/characterStore";
import {
	characterSchema,
	type CharacterFormData,
} from "../schemas/characterSchema";
import EditBasicInfo from "../components/edit-character/EditBasicInfo";
import EditCombatStats from "../components/edit-character/EditCombatStats";
import EditAbilityScores from "../components/edit-character/EditAbilityScores";
import EditSkills from "../components/edit-character/EditSkills";
import EditStringList from "../components/edit-character/EditStringList";

export default function EditCharacterPage() {
	const { id } = useParams();
	const navigate = useNavigate();
	const { currentCharacter, getCurrentCharacter, updateCharacter, loading, error } =
		useCharacterStore();

	const methods = useForm<CharacterFormData>({
		resolver: zodResolver(characterSchema),
	});

	// Load the character when the route id changes
	useEffect(() => {
		if (!id) return;
		getCurrentCharacter(+id);
	}, [id]);

	// Reset form values when character data arrives
	useEffect(() => {
		if (!currentCharacter) return;
		methods.reset(currentCharacter);
	}, [currentCharacter]);

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;
	if (!currentCharacter) return <div>Could not find character.</div>;

	async function onSubmit(data: CharacterFormData) {
		try {
			const updated = await updateCharacter(data);
			navigate(`/character/${updated.id}`);
		} catch (err) {
			console.error("Failed to save character:", err);
		}
	}

	return (
		<FormProvider {...methods}>
			<form
				onSubmit={methods.handleSubmit(onSubmit)}
				className="flex flex-col gap-4"
			>
				<div className="flex justify-end gap-2">
					<Button
						variant="outline"
						onClick={() => navigate(`/character/${currentCharacter.id}`)}
					>
						Cancel
					</Button>
					<Button type="submit">Save</Button>
				</div>

				<EditBasicInfo />

				<div className="grid grid-cols-2 gap-4">
					<EditAbilityScores />
					<EditCombatStats />
				</div>

				<EditSkills />

				<div className="grid grid-cols-2 gap-4">
					<EditStringList
						fieldName="languages"
						title="Languages"
						addLabel="Add Language"
					/>
					<EditStringList
						fieldName="killList"
						title="Kill List"
						addLabel="Add Kill"
					/>
				</div>
			</form>
		</FormProvider>
	);
}
