import { useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import CharacterBasicDescription from "../components/character-sheet/CharacterBasicDescription";
import useCharacterStore from "../stores/characterStore";
import CharacterCombatStats from "../components/character-sheet/CharacterCombatStats";
import CharacterAbilityScores from "../components/character-sheet/CharacterAbilityScores";
import CharacterSaves from "../components/character-sheet/CharacterSaves";
import CharacterSkills from "../components/character-sheet/CharacterSkills";
import CharacterLanguages from "../components/character-sheet/CharacterLanguages";
import CharacterKillList from "../components/character-sheet/CharacterKillList";
import { Button } from "@chakra-ui/react";

export default function CharacterSheet() {
	let {
		currentCharacter,
		getCurrentCharacter,
		deleteCharacter,
		loading,
		error,
	} = useCharacterStore();
	let { id } = useParams();
	let navigate = useNavigate();

	useEffect(() => {
		if (!id) return;
		getCurrentCharacter(+id);
	}, [id]);

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;
	if (currentCharacter === null) return <div>Could not find character.</div>;

	function handleEditCharacterButton() {
		if (currentCharacter === null) return;
		navigate(`/character/edit/${currentCharacter.id}`);
	}

	async function handleDeleteCharacterButton() {
		if (currentCharacter === null) return;
		let result = await deleteCharacter(currentCharacter.id);

		if (result === true) {
			navigate("/characters-list");
		}
	}

	return (
		<main>
			<div className='flex w-full justify-end gap-2'>
				<Button onClick={handleDeleteCharacterButton}>Delete Character</Button>
				<Button onClick={handleEditCharacterButton}>Edit Character</Button>
			</div>
			<CharacterBasicDescription
				basicDescription={currentCharacter.basicDescription}
				name={currentCharacter.name}
				creator={currentCharacter.creator}
			/>
			<div className='grid grid-cols-2 gap-4'>
				<CharacterAbilityScores
					abilityScores={currentCharacter.abilityScores}
				/>
				<CharacterCombatStats
					combatStats={currentCharacter.combatStats}
					proficiencyBonus={currentCharacter.proficiencyBonus}
				/>
			</div>
			<div className='grid grid-cols-2 gap-4'>
				<CharacterSaves abilityScores={currentCharacter.abilityScores} />
				<CharacterSkills skills={currentCharacter.skills} />
			</div>
			<div className='grid grid-cols-2 gap-4'>
				<CharacterLanguages languages={currentCharacter.languages} />
				<CharacterKillList killList={currentCharacter.killList} />
			</div>
		</main>
	);
}
