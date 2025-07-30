import { Card } from "@chakra-ui/react";
import { CombatStats } from "../../types/character/CombatStats";

export default function CharacterCombatStats({
	combatStats,
}: {
	combatStats: CombatStats;
}) {
	return (
		<div className='grid grid-cols-2'>
			<Card.Root>
				<Card.Body>
					<div className="grid grid-cols-2">
						<p>
							Initiative: {combatStats.initiative + combatStats.initiativeMods}
						</p>
						<p>Speed: {combatStats.speed}</p>
					</div>
				</Card.Body>
			</Card.Root>
			<Card.Root>
				<Card.Body>
					<div className='grid grid-cols-2'>
						<p>
							HP: {combatStats.currentHp}/{combatStats.maxHp}
						</p>
						<p>Temp HP: {combatStats.tempHp}</p>
						<p>Remaining Hit Die: {combatStats.hitDiceRemaining}</p>
					</div>
				</Card.Body>
			</Card.Root>
		</div>
	);
}
