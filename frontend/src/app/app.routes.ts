import { Routes } from '@angular/router';
import { authGuard, adminGuard, publicGuard } from './core/guards/auth.guard';

export const routes: Routes = [
  // Authenticated routes (App Layout with Sidebar) - these come FIRST
  {
    path: 'dashboard',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [authGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/dashboard/dashboard.component').then((m) => m.DashboardComponent),
      },
    ],
  },

  {
    path: 'inventory',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [authGuard],
    children: [
      {
        path: '',
        loadChildren: () =>
          import('./features/inventory/inventory.routes').then((m) => m.INVENTORY_ROUTES),
      },
    ],
  },

  {
    path: 'compliance',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [authGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/compliance/compliance.component').then((m) => m.ComplianceComponent),
      },
    ],
  },

  {
    path: 'planner',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [authGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/planner/planner.component').then((m) => m.PlannerComponent),
      },
    ],
  },

  {
    path: 'skills',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [authGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/skills/skills.component').then((m) => m.SkillsComponent),
      },
    ],
  },

  // Tenant Admin routes (Authenticated + Admin only - for managing users within tenant)
  {
    path: 'tenant-admin',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [adminGuard],
    children: [
      {
        path: '',
        loadChildren: () =>
          import('./features/tenant-admin/tenant-admin.routes').then((m) => m.TENANT_ADMIN_ROUTES),
      },
    ],
  },

  // Admin routes (Authenticated + Admin only - for StaxAdmin system-wide management)
  {
    path: 'admin',
    loadComponent: () =>
      import('./layout/app-layout/app-layout.component').then((m) => m.AppLayoutComponent),
    canActivate: [adminGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/stax-admin/stax-admin.component').then((m) => m.StaxAdminComponent),
      },
    ],
  },

  // Public routes (Landing Page) - comes LAST so it's the fallback
  {
    path: '',
    loadComponent: () =>
      import('./layout/public-layout/public-layout.component').then((m) => m.PublicLayoutComponent),
    canActivate: [publicGuard],
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/landing-page/landing-page.component').then(
            (m) => m.LandingPageComponent,
          ),
      },
    ],
  },

  // Wildcard - redirect to root
  { path: '**', redirectTo: '', pathMatch: 'full' },
];
