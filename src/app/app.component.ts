import { Component } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { Navbar } from "./components/navbar/navbar.component";

@Component({
    selector: "app-root",
    imports: [CommonModule, RouterOutlet, Navbar],
    templateUrl: "./app.component.html",
    styleUrl: "./app.component.css"
})
export class AppComponent {}
