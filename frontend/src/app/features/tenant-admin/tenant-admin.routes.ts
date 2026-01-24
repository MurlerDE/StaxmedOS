import { Routes } from '@angular/router';
import { TenantAdminComponent } from './tenant-admin.component';
import { DesktopComponent } from './views/desktop/desktop.component';

export const TENANT_ADMIN_ROUTES: Routes = [
  {
    path: '',
    component: TenantAdminComponent,
    children: [
      {
        path: '',
        redirectTo: 'users',
        pathMatch: 'full',
      },
      {
        path: 'users',
        component: DesktopComponent,
        title: 'Benutzerverwaltung',
      },
      {
        path: 'roles',
        component: DesktopComponent, // TODO: Separate Roles Component
        title: 'Rollenverwaltung',
      },
      {
        path: 'settings',
        component: DesktopComponent, // TODO: Settings Component
        title: 'Einstellungen',
      },
    ],
  },
];
