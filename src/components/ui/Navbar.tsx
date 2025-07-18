import { Link } from "react-router-dom";

export default function Navbar() {
  return (
    <nav className="flex justify-center gap-8">
      <Link to="/">Dashboard</Link>
      <Link to="characters-list">Characters</Link>
    </nav>
  )
}
