import { Card } from "@chakra-ui/react";

interface CharacterKillListProps {
	killList: string[];
}

export default function CharacterKillList(props: CharacterKillListProps) {
	const { killList } = props;

	return (
		<Card.Root>
			<Card.Title>Kill List</Card.Title>
			<Card.Body>
				{killList.length > 0 ? (
					<ul className="list-disc list-inside">
						{killList.map((kill, index) => (
							<li key={index}>{kill}</li>
						))}
					</ul>
				) : (
					<p>No kills recorded</p>
				)}
			</Card.Body>
		</Card.Root>
	);
}
