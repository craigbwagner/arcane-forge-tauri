import { useEffect } from "react";
import CharacterSummaryCard from "../components/CharacterSummaryCard";
import useCharacterStore from "../store"

export default function CharactersList() {
  let { characters, getCharacters, loading, error } = useCharacterStore();

  useEffect(() => {
    getCharacters();
  }, [])

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <>
        {characters.map((character) => (
          <CharacterSummaryCard key={character.id} character={character} />
        ))}
    </>
  );
}
