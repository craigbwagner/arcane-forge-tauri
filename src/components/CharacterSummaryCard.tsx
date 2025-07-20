import { Card, Link } from "@chakra-ui/react";
import { FullCharacterData } from "../types/character/FullCharacterData";

export default function CharacterSummaryCard({character}: { character:FullCharacterData }) {
  return (
    <Card.Root>
      <Card.Header>
        <Link href={`/character/${character.id}`}><h1>{character.name}</h1></Link>
      </Card.Header>
      <Card.Body>
        <p>{character.basicDescription.race}</p>
        <p>Sex: {character.basicDescription.sex}</p>
      </Card.Body>
    </Card.Root>
  )
}
