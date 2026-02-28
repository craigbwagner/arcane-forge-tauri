import { Card } from "@chakra-ui/react";

interface CharacterLanguagesProps {
	languages: string[];
}

export default function CharacterLanguages(props: CharacterLanguagesProps) {
	const { languages } = props;

	return (
		<Card.Root>
			<Card.Title>Languages</Card.Title>
			<Card.Body>
				<p>{languages.length > 0 ? languages.join(", ") : "None"}</p>
			</Card.Body>
		</Card.Root>
	);
}
