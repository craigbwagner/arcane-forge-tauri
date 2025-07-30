import { useEffect } from "react";
import useCharacterStore from "../../stores/characterStore";
import { BasicDescription } from "../../types/character/BasicDescription";
import { useParams } from "react-router-dom";
import { Card } from "@chakra-ui/react";

export default function CharacterBasicDescription({
	basicDescription,
	name,
}: {
	basicDescription: BasicDescription;
	name: string;
}) {
	return (
		<section>
			<Card.Root>
				<Card.Title className="text-center">
					<h1>{name}</h1>
				</Card.Title>
        <Card.Description className="grid grid-cols-3">
          <p>{basicDescription.race}</p>
          <p>{basicDescription.sex}</p>
          <p>{basicDescription.alignment}</p>
          <p>{basicDescription.size}</p>
          <p>{basicDescription.age}</p>
          <p>{basicDescription.height}</p>
          <p>{basicDescription.weight}</p>
        </Card.Description>
			</Card.Root>
		</section>
	);
}
