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
        <div className="grid grid-cols-5 gap-2 text-xs font-semibold opacity-60 mb-1">
          <p>Skill</p>
          <p>Ability</p>
          <p>Prof</p>
          <p>Exp</p>
          <p>Mod</p>
        </div>
        <ul>
          {skills.map(skill => (
            <li key={skill.name} className="grid grid-cols-5 gap-2">
              <p>{skill.name}</p>
              <p>{skill.abilityName}</p>
              <p>{skill.isProficient ? "●" : "○"}</p>
              <p>{skill.hasExpertise ? "◆" : "○"}</p>
              <p>{skill.totalMod >= 0 ? `+${skill.totalMod}` : skill.totalMod}</p>
            </li>
          ))}
        </ul>
      </Card.Body>
    </Card.Root>
  )
}
