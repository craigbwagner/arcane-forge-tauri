import { Routes } from "@angular/router";
import { AppComponent } from "./app.component";
import { DashboardComponent } from "./pages/dashboard/dashboard.component";
import { CharactersComponent } from "./pages/characters/characters.component";
import { CharacterSheetComponent } from "./components/character-sheet/character-sheet.component";

export const routes: Routes = [
  {
    path: '',
    redirectTo: 'dashboard',
    pathMatch: 'full'
  },
  {
    path: 'dashboard',
    component: DashboardComponent
  },
  {
    path: 'characters',
    component: CharactersComponent
  },
  {
    path: 'character/:id',
    component: CharacterSheetComponent
  }
];
