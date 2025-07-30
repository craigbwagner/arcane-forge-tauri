import { useEffect } from "react";
import { useParams } from "react-router-dom";
import CharacterBasicDescription from "../components/character-sheet/CharacterBasicDescription";
import useCharacterStore from "../stores/characterStore";
import CharacterCombatStats from "../components/character-sheet/CharacterCombatStats";

export default function CharacterSheet() {
	let { currentCharacter, getCurrentCharacter, loading, error } =
		useCharacterStore();
	let { id } = useParams();

	useEffect(() => {
		getCurrentCharacter(+id!);
	}, []);

	if (loading) return <div>Loading...</div>;
	if (error) return <div>Error: {error}</div>;
	if (currentCharacter === null) return <div>Could not find character.</div>;

	return (
		<main>
			<CharacterBasicDescription
				basicDescription={currentCharacter.basicDescription}
				name={currentCharacter.name}
			/>
			<CharacterCombatStats combatStats={currentCharacter.combatStats} />
		</main>
	);
}
