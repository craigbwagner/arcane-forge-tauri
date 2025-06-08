import { Routes } from "@angular/router";
import { AppComponent } from "./app.component";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { CharactersComponent } from "./pages/characters/characters.component";

export const routes: Routes = [
  {
    path: '',
    component: AppComponent
  },
  {
    path: 'dashboard',
    component: DashboardComponent
  },
  {
    path: 'characters',
    component: CharactersComponent
  }
];
