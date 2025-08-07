import { Card } from "@chakra-ui/react";
import { AbilityScoreProps } from "./CharacterAbilityScores";

export default function CharacterSaves(props: AbilityScoreProps) {
  const { abilityScores } = props;

  return (
		<Card.Root>
			<Card.Title>Saving Throws</Card.Title>
			<Card.Body>
				{abilityScores.map((ability) => (
					<div className='grid grid-cols-3'>
						<p>{ability.shortName}</p>
						<p>{ability.isProficient ? "true" : "false"}</p>
						<p>{ability.save}</p>
					</div>
				))}
			</Card.Body>
		</Card.Root>
	);
}
