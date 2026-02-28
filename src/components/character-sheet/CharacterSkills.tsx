import { Card } from "@chakra-ui/react";
import { Skill } from "../../types/character/Skill";

interface SkillsProps {
  skills: Skill[]
}

export default function CharacterSkills(props: SkillsProps) {
  const { skills } = props;

  return (
    <Card.Root>
      <Card.Title>
        Skills
      </Card.Title>
      <Card.Body>
        <ul>
          {skills.map(skill => (
            <li key={skill.name} className="grid grid-cols-5">
              <p>{skill.name}</p>
              <p>{skill.abilityName}</p>
              <p>Proficient: {skill.isProficient ? "true" : "false" }</p>
              <p>Expertise: {skill.hasExpertise ? "true" : "false" }</p>
              <p>{skill.totalMod}</p>
            </li>
          ))}
        </ul>
      </Card.Body>
    </Card.Root>
  )
}
