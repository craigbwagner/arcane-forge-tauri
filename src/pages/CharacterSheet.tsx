import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import CharacterBasicDescription from "../components/character-sheet/CharacterBasicDescription";
import useCharacterStore from "../stores/characterStore";
import CharacterCombatStats from "../components/character-sheet/CharacterCombatStats";
import CharacterAbilityScores from "../components/character-sheet/CharacterAbilityScores";
import CharacterSaves from "../components/character-sheet/CharacterSaves";
import CharacterSkills from "../components/character-sheet/CharacterSkills";
import { Button } from "@chakra-ui/react";

export default function CharacterSheet() {
	let { currentCharacter, getCurrentCharacter, loading, error } =
		useCharacterStore();
	let { id } = useParams();
	let navigate = useNavigate();

	useEffect(() => {
		getCurrentCharacter(+id!);
	}, []);

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;
	if (currentCharacter === null) return <div>Could not find character.</div>;

	function handleEditCharacterButton() {
		if (currentCharacter === null) return;
		navigate(`/character/edit/${currentCharacter.id}`);
	}

	return (
		<main>
			<div className='flex w-full justify-end'>
				<Button onClick={handleEditCharacterButton}>Edit Character</Button>
			</div>
			<CharacterBasicDescription
				basicDescription={currentCharacter.basicDescription}
				name={currentCharacter.name}
			/>
			<div className='grid grid-cols-2'>
				<CharacterAbilityScores
					abilityScores={currentCharacter.abilityScores}
				/>
				<CharacterCombatStats combatStats={currentCharacter.combatStats} />
			</div>
			<div className='grid grid-cols-2'>
				<CharacterSaves abilityScores={currentCharacter.abilityScores} />
				<CharacterSkills skills={currentCharacter.skills} />
			</div>
		</main>
	);
}
