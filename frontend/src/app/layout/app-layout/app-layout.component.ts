import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet, RouterLink, Router } from '@angular/router';
import { AuthService } from '../../core/services/auth.service';

@Component({
  selector: 'app-layout',
  standalone: true,
  imports: [CommonModule, RouterOutlet, RouterLink],
  templateUrl: './app-layout.component.html',
  styleUrl: './app-layout.component.css',
})
export class AppLayoutComponent {
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
