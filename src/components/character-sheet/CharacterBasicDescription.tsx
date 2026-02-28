import { BasicDescription } from "../../types/character/BasicDescription";
import { Card } from "@chakra-ui/react";
import { ALIGNMENT_LABELS } from "../../utils";

interface BasicDescriptionProps {
	basicDescription: BasicDescription;
	name: string;
	creator: string;
}

export default function CharacterBasicDescription(
	props: BasicDescriptionProps
) {
	const { basicDescription, name, creator } = props;
	return (
		<section>
			<Card.Root>
				<Card.Header className='text-center'>
					<h1>{name || "Unnamed Character"}</h1>
					{creator && <p className='text-sm opacity-70'>Created by {creator}</p>}
				</Card.Header>
				<Card.Body>
					<div className='grid grid-cols-3 gap-4'>
						<div>
							<p className='text-xs font-semibold opacity-60'>Race</p>
							<p>{basicDescription.race}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Sex</p>
							<p>{basicDescription.sex}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Alignment</p>
							<p>{ALIGNMENT_LABELS[basicDescription.alignment]}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Size</p>
							<p>{basicDescription.size}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Age</p>
							<p>{basicDescription.age}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Height</p>
							<p>{basicDescription.height}</p>
						</div>
						<div>
							<p className='text-xs font-semibold opacity-60'>Weight</p>
							<p>{basicDescription.weight} lbs</p>
						</div>
					</div>
				</Card.Body>
			</Card.Root>
		</section>
	);
}
