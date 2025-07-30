import { Card } from "@chakra-ui/react";
import { AbilityScore } from "../../types/character/AbilityScore";

interface CharacterAbilityScoreProps {
	abilityScores: AbilityScore[];
}

export default function CharacterAbilityScores(
	props: CharacterAbilityScoreProps
) {
	const { abilityScores } = props;
	return (
		<Card.Root>
			<Card.Body>
				<div className='grid grid-cols-3'>
					{abilityScores.map((ability) => (
						<div className='text-center'>
							<p>{ability.name}</p>
							<p>
								{ability.totalMod > 0
									? "+" + ability.totalMod
									: ability.totalMod}
							</p>
							<p>{ability.score}</p>
						</div>
					))}
				</div>
			</Card.Body>
		</Card.Root>
	);
}
