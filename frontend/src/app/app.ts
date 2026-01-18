import { Component, inject, computed } from '@angular/core';
import { RouterOutlet, RouterLink, Router } from '@angular/router';
import { AuthService } from './services/auth.service';
import { CommonModule } from '@angular/common';
import { LoginComponent } from './components/login/login.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, RouterLink, CommonModule, LoginComponent],
  templateUrl: './app.html',
})
export class App {
  auth = inject(AuthService);
  private router = inject(Router);

  // Zentrale Definition der Module
  modules = computed(() => [
    {
      id: 'inventory',
      name: 'StaxInventory',
      route: '/inventory',
      active: this.auth.activeModules().includes('inventory'),
    },
    {
      id: 'compliance',
      name: 'StaxCompliance',
      route: '/compliance',
      active: this.auth.activeModules().includes('compliance'),
    },
    {
      id: 'skills',
      name: 'StaxSkills',
      route: '/skills',
      active: this.auth.activeModules().includes('skills'),
    },
    {
      id: 'planner',
      name: 'StaxPlanner',
      route: '/planner',
      active: this.auth.activeModules().includes('planner'),
    },
  ]);

  // Hilfsfunktion für aktives Styling in der Sidebar
  isRouteActive(route: string): boolean {
    return this.router.url === route;
  }
}
