import { FormProvider, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useCharacterStore from "../stores/characterStore";
import {
	CharacterFormData,
	characterSchema,
} from "../types/schemas/CharacterFormData";
import {
	Button,
	Field,
	Fieldset,
	Input,
	Stack,
	Tabs,
	VStack,
} from "@chakra-ui/react";
import CharacterBasicDescriptionForm from "../components/character-edit-page/CharacterBasicDescriptionForm";

export default function EditCharacterPage() {
	let { currentCharacter, getCurrentCharacter, loading, error } =
		useCharacterStore();
	let navigate = useNavigate();
	let { id } = useParams();

	const methods = useForm<CharacterFormData>({
		resolver: zodResolver(characterSchema),
	});

	useEffect(() => {
		getCurrentCharacter(+id!);
	}, []);

	useEffect(() => {
		if (currentCharacter) {
			methods.reset({
				name: currentCharacter.name,
				creator: currentCharacter.creator,
			});
		}
	}, [currentCharacter, methods.reset]);

	function onSubmit(data: CharacterFormData) {
		console.log("Save to Zustand:", data);
	}

	function handleReturnToCharacterButton() {
		navigate(`/character/${id}`);
	}

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;
	if (currentCharacter === null) return <div>Could not find character.</div>;

	return (
		<FormProvider {...methods}>
			<VStack
				as='form'
				onSubmit={methods.handleSubmit(onSubmit)}
				align='stretch'>
				<Button onClick={handleReturnToCharacterButton}>
					Return to Character Page
				</Button>
				<Fieldset.Root>
					<Tabs.Root defaultValue='Basic Info'>
						<Tabs.List bg='bg.muted' rounded='13' p='1'>
							<Tabs.Trigger value='Basic Info'>Basic Info</Tabs.Trigger>
							<Tabs.Indicator rounded='12' />
						</Tabs.List>
						<Tabs.Content value='Basic Info'>
							<CharacterBasicDescriptionForm />
						</Tabs.Content>
					</Tabs.Root>
					<Fieldset.Legend>Character Data</Fieldset.Legend>
				</Fieldset.Root>
				<Button type='submit'>Save Character</Button>
			</VStack>
		</FormProvider>
	);
}
