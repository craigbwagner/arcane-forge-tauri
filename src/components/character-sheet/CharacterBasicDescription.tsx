import { BasicDescription } from "../../types/character/BasicDescription";
import { Card } from "@chakra-ui/react";

interface BasicDescriptionProps {
	basicDescription: BasicDescription;
	name: string;
}

export default function CharacterBasicDescription(
	props: BasicDescriptionProps
) {
	const { basicDescription, name } = props;
	return (
		<section>
			<Card.Root>
				<Card.Header className='text-center'>
					<h1>{name}</h1>
				</Card.Header>
				<Card.Body>
					<div className='grid grid-cols-3'>
						<p>{basicDescription.race}</p>
						<p>{basicDescription.sex}</p>
						<p>{basicDescription.alignment}</p>
						<p>{basicDescription.size}</p>
						<p>{basicDescription.age}</p>
						<p>{basicDescription.height}</p>
						<p>{basicDescription.weight}</p>
					</div>
				</Card.Body>
			</Card.Root>
		</section>
	);
}
