import { Outlet } from 'react-router-dom';
import Navbar from '../components/Navbar';

export default function PageLayout() {
  return (
    <div>
      <Navbar />

      <main>
        <Outlet />
      </main>
    </div>
  )
}
