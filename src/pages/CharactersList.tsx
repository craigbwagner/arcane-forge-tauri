import { useEffect } from "react";
import CharacterSummaryCard from "../components/CharacterSummaryCard";
import useCharacterStore from "../stores/characterStore";
import { Button } from "@chakra-ui/react";
import { useNavigate } from "react-router-dom";

export default function CharactersList() {
	let { characters, getCharacters, createCharacter, loading, error } =
		useCharacterStore();
	const navigate = useNavigate();

	useEffect(() => {
		getCharacters();
	}, []);

	async function handleCreateCharacterButtonClicked(): Promise<void> {
		const newCharacter = await createCharacter();
		if (newCharacter == undefined) return;

		navigate(`/character/${newCharacter.id}`);
	}

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;

	return (
		<>
			<Button onClick={handleCreateCharacterButtonClicked}>
				New Character
			</Button>
			<div className='grid grid-cols-3 gap-3'>
				{characters.map((character) => (
					<CharacterSummaryCard key={character.id} character={character} />
				))}
			</div>
		</>
	);
}
