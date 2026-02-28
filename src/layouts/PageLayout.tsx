import { Outlet } from 'react-router-dom';
import Navbar from '../components/Navbar';

export default function PageLayout() {
  return (
    <div className="flex flex-col gap-4">
      <Navbar />

      <main>
        <Outlet />
      </main>
    </div>
  )
}
