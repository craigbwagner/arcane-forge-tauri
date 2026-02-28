import { Card } from "@chakra-ui/react";
import { AbilityScoreProps } from "./CharacterAbilityScores";

export default function CharacterSaves(props: AbilityScoreProps) {
  const { abilityScores } = props;

  return (
		<Card.Root>
			<Card.Title>Saving Throws</Card.Title>
			<Card.Body>
				{abilityScores.map((ability) => (
					<div key={ability.shortName} className='grid grid-cols-3 gap-2'>
						<p>{ability.shortName}</p>
						<p>{ability.isProficient ? "●" : "○"}</p>
						<p>{ability.save >= 0 ? `+${ability.save}` : ability.save}</p>
					</div>
				))}
			</Card.Body>
		</Card.Root>
	);
}
