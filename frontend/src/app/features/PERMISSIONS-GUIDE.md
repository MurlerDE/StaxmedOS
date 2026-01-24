# 🔐 StaxmedOS Permissions Guide

**Letzte Aktualisierung:** 24. Januar 2026

Dieser Leitfaden zeigt, wie Berechtigungen im Frontend implementiert werden und welche Permissions im Backend verfügbar sind.

---

## 📋 Inhaltsverzeichnis

1. [Überblick](#überblick)
2. [Verfügbare Permissions](#verfügbare-permissions)
3. [AuthService Nutzung](#authservice-nutzung)
4. [Permissions in Components](#permissions-in-components)
5. [Permissions in Templates](#permissions-in-templates)
6. [Route Guards](#route-guards)
7. [Best Practices](#best-practices)
8. [Beispiele](#beispiele)

---

## 🎯 Überblick

StaxmedOS verwendet ein **RBAC (Role-Based Access Control)** System mit granularen Permissions.

### Hierarchie

```
User
 └── user_roles (Tabelle)
      └── roles
           └── role_permissions (Tabelle)
                └── permissions
```

### Rollen (Roles)

| Rolle | Name | Beschreibung |
|-------|------|--------------|
| `staxadmin` | StaxMed Administrator | Vollständiger System-Zugriff (alle Tenants) |
| `admin` | Tenant Administrator | Vollständiger Zugriff innerhalb des Tenants |
| `logistic_manager` | Logistik-Manager | Inventory, Shipments, Locations verwalten |
| `paramedic` | Sanitäter | Basis-Zugriff (Inventory anzeigen, Verbrauch melden) |

---

## 🔑 Verfügbare Permissions

### Format

Permissions folgen dem Schema: `resource.action.scope`

- **resource**: inventory, users, shipments, locations, etc.
- **action**: view, create, update, delete, manage
- **scope**: all, own, location:uuid

### Inventory Permissions

```typescript
'inventory.view.all'      // Alle Inventare anzeigen
'inventory.view.own'      // Nur eigene Inventare anzeigen
'inventory.create'        // Neue Inventory-Items erstellen
'inventory.update'        // Inventory-Items bearbeiten
'inventory.delete'        // Inventory-Items löschen
'inventory.manage'        // Vollständige Inventory-Verwaltung
```

### User Management Permissions

```typescript
'users.view'              // Benutzer anzeigen
'users.create'            // Benutzer erstellen
'users.update'            // Benutzer bearbeiten
'users.delete'            // Benutzer löschen
'users.manage_permissions' // User-Permissions verwalten
```

### Role Management Permissions

```typescript
'roles.view'              // Rollen anzeigen
'roles.create'            // Rollen erstellen
'roles.update'            // Rollen bearbeiten
'roles.delete'            // Rollen löschen
```

### Shipments Permissions

```typescript
'shipments.view'          // Sendungen anzeigen
'shipments.create'        // Sendungen erstellen
'shipments.receive'       // Sendungen empfangen
'shipments.manage'        // Sendungen verwalten
```

### Locations Permissions

```typescript
'locations.view'          // Standorte anzeigen
'locations.create'        // Standorte erstellen
'locations.update'        // Standorte bearbeiten
'locations.delete'        // Standorte löschen
```

### Devices Permissions

```typescript
'devices.view'            // Geräte anzeigen
'devices.manage'          // Geräte verwalten
'devices.maintenance'     // Wartung durchführen
```

### Missions Permissions

```typescript
'missions.view'           // Einsätze anzeigen
'missions.report'         // Einsätze melden
```

### Tenant Settings Permissions

```typescript
'tenant.settings.view'    // Tenant-Einstellungen anzeigen
'tenant.settings.update'  // Tenant-Einstellungen bearbeiten
```

### System Permissions (nur StaxAdmin)

```typescript
'system.tenants.view'     // Alle Tenants anzeigen
'system.tenants.manage'   // Tenants verwalten
'system.licenses.manage'  // Lizenzen verwalten
'system.all'              // Vollständiger System-Zugriff
```

---

## 🔧 AuthService Nutzung

### Service Importieren

```typescript
import { AuthService } from '../../core/services/auth.service';
```

### Im Component Injecten

```typescript
export class MyFeatureComponent {
  private auth = inject(AuthService);
  // oder
  constructor(private auth: AuthService) {}
}
```

### Verfügbare Methoden

```typescript
// Aktueller User
auth.currentUser()           // Signal<User | null>
auth.isAuthenticated()       // Signal<boolean>

// Rollen prüfen
auth.isStaxAdmin()           // boolean - Ist StaxAdmin?
auth.isAdmin()               // boolean - Ist Admin oder StaxAdmin?
auth.hasRole('paramedic')    // boolean - Hat spezifische Rolle?

// Permissions prüfen
auth.hasPermission('users.view')                    // boolean
auth.hasAnyPermission(['users.view', 'users.create']) // boolean
auth.hasAllPermissions(['users.view', 'users.update']) // boolean

// User Info
auth.displayName()           // computed Signal - Anzeigename
auth.initials()             // computed Signal - Initialen
auth.userRole()             // computed Signal - Rolle
```

---

## 📦 Permissions in Components

### Basic Check

```typescript
export class UserManagementComponent {
  auth = inject(AuthService);
  
  ngOnInit() {
    if (!this.auth.hasPermission('users.view')) {
      // Redirect oder Error anzeigen
      this.router.navigate(['/dashboard']);
      return;
    }
    
    this.loadUsers();
  }
}
```

### Mit Signals

```typescript
export class UserManagementComponent {
  auth = inject(AuthService);
  
  // Computed Signals für UI-Logik
  canCreateUsers = computed(() => 
    this.auth.hasPermission('users.create') || this.auth.isAdmin()
  );
  
  canEditUsers = computed(() => 
    this.auth.hasPermission('users.update') || this.auth.isAdmin()
  );
  
  canDeleteUsers = computed(() => 
    this.auth.hasPermission('users.delete') || this.auth.isAdmin()
  );
}
```

### Helper Methods

```typescript
export class InventoryComponent {
  auth = inject(AuthService);
  
  // Helper Methods für bessere Lesbarkeit
  canViewAllInventory(): boolean {
    return this.auth.hasPermission('inventory.view.all') || this.auth.isAdmin();
  }
  
  canEditInventory(): boolean {
    return this.auth.hasPermission('inventory.update') || 
           this.auth.hasPermission('inventory.manage');
  }
  
  showAdminFeatures(): boolean {
    return this.auth.isStaxAdmin();
  }
}
```

---

## 🎨 Permissions in Templates

### Buttons & Actions

```html
<!-- Einfache Permission Check -->
@if (auth.hasPermission('users.create')) {
  <button (click)="createUser()">
    Neuer Benutzer
  </button>
}

<!-- Rolle Check -->
@if (auth.isAdmin()) {
  <button (click)="openAdminPanel()">
    Admin-Panel
  </button>
}

<!-- StaxAdmin Only -->
@if (auth.isStaxAdmin()) {
  <button (click)="manageTenants()">
    Tenants verwalten
  </button>
}
```

### Mit Computed Signals

```html
<!-- In Component: canEditUsers = computed(...) -->
@if (canEditUsers()) {
  <button (click)="editUser(user)">
    Bearbeiten
  </button>
}
```

### Multiple Permissions

```html
<!-- Mehrere Permissions (ODER) -->
@if (auth.hasAnyPermission(['users.update', 'users.delete'])) {
  <div class="actions">
    <!-- Actions -->
  </div>
}

<!-- Mehrere Permissions (UND) -->
@if (auth.hasAllPermissions(['users.view', 'users.update'])) {
  <button>User bearbeiten</button>
}
```

### Sections basierend auf Rolle

```html
<div class="dashboard">
  <!-- Für alle authentifizierten User -->
  @if (auth.isAuthenticated()) {
    <div class="welcome">
      Willkommen, {{ auth.displayName() }}!
    </div>
  }
  
  <!-- Nur für Admins -->
  @if (auth.isAdmin()) {
    <section class="admin-section">
      <h2>Verwaltung</h2>
      <!-- Admin Content -->
    </section>
  }
  
  <!-- Nur für StaxAdmin -->
  @if (auth.isStaxAdmin()) {
    <section class="stax-admin-section">
      <h2>System-Verwaltung</h2>
      <!-- StaxAdmin Content -->
    </section>
  }
</div>
```

### Disabled States

```html
<button 
  (click)="deleteUser(user)"
  [disabled]="!auth.hasPermission('users.delete')"
  [class.opacity-50]="!auth.hasPermission('users.delete')"
  [class.cursor-not-allowed]="!auth.hasPermission('users.delete')">
  Löschen
</button>
```

---

## 🛡️ Route Guards

### Auth Guard (Authentifizierung)

```typescript
// In app.routes.ts
{
  path: 'dashboard',
  loadComponent: () => import('./features/dashboard/dashboard.component'),
  canActivate: [authGuard],  // Nur eingeloggte User
}
```

### Admin Guard (Admin-Rechte)

```typescript
{
  path: 'admin',
  loadComponent: () => import('./features/stax-admin/stax-admin.component'),
  canActivate: [adminGuard],  // Nur Admins (admin + staxadmin)
}
```

### Custom Permission Guard (Beispiel)

```typescript
// guards/permission.guard.ts
import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../services/auth.service';

export const permissionGuard = (permission: string) => {
  return () => {
    const auth = inject(AuthService);
    const router = inject(Router);
    
    if (auth.hasPermission(permission)) {
      return true;
    }
    
    router.navigate(['/dashboard']);
    return false;
  };
};

// Verwendung in Routes:
{
  path: 'users',
  loadComponent: () => import('./features/users/users.component'),
  canActivate: [permissionGuard('users.view')],
}
```

---

## ✅ Best Practices

### 1. **Immer Backend-Validierung**

```typescript
// ❌ FALSCH - Nur Frontend-Check
if (this.auth.hasPermission('users.delete')) {
  this.userService.deleteUser(userId).subscribe(...);
}

// ✅ RICHTIG - Backend validiert auch
this.userService.deleteUser(userId).subscribe({
  next: () => console.log('Gelöscht'),
  error: (err) => {
    if (err.status === 403) {
      this.showError('Keine Berechtigung');
    }
  }
});
```

### 2. **Computed Signals für komplexe Logik**

```typescript
// ✅ Gut - Wiederverwendbar und reaktiv
canManageInventory = computed(() => 
  this.auth.hasPermission('inventory.manage') || 
  (this.auth.hasPermission('inventory.update') && 
   this.auth.hasPermission('inventory.delete'))
);
```

### 3. **Descriptive Helper Methods**

```typescript
// ✅ Lesbar und wartbar
canEditOwnProfile(userId: string): boolean {
  return this.auth.currentUser()?.id === userId || 
         this.auth.hasPermission('users.update');
}
```

### 4. **Graceful Degradation**

```typescript
// ✅ Features ausblenden statt Error
@if (auth.hasPermission('users.create')) {
  <button>Erstellen</button>
} @else {
  <p class="text-gray-500">Keine Berechtigung zum Erstellen</p>
}
```

### 5. **Konsistente Namensgebung**

```typescript
// ✅ Konsistentes Präfix
canViewUsers()
canCreateUsers()
canEditUsers()
canDeleteUsers()
```

---

## 📚 Beispiele

### Beispiel 1: Tenant-Admin Component

```typescript
import { Component, inject, computed } from '@angular/core';
import { AuthService } from '../../core/services/auth.service';

@Component({
  selector: 'app-tenant-admin',
  template: `
    <div class="admin-panel">
      <h1>Tenant-Verwaltung</h1>
      
      @if (canManageUsers()) {
        <section class="users">
          <button (click)="createUser()">Neuer Benutzer</button>
          <!-- User List -->
        </section>
      }
      
      @if (canManageSettings()) {
        <section class="settings">
          <button (click)="editSettings()">Einstellungen</button>
        </section>
      }
      
      @if (auth.isStaxAdmin()) {
        <section class="stax-only">
          <h2>StaxAdmin-Features</h2>
          <!-- Advanced features -->
        </section>
      }
    </div>
  `
})
export class TenantAdminComponent {
  auth = inject(AuthService);
  
  // Permissions als Computed Signals
  canManageUsers = computed(() => 
    this.auth.hasPermission('users.view') || this.auth.isAdmin()
  );
  
  canManageSettings = computed(() => 
    this.auth.hasPermission('tenant.settings.update') || this.auth.isAdmin()
  );
  
  createUser() {
    if (!this.auth.hasPermission('users.create')) {
      alert('Keine Berechtigung');
      return;
    }
    // Create user logic
  }
  
  editSettings() {
    if (!this.canManageSettings()) {
      return;
    }
    // Edit settings logic
  }
}
```

### Beispiel 2: Inventory Component mit Scope

```typescript
@Component({
  selector: 'app-inventory',
  // ...
})
export class InventoryComponent implements OnInit {
  auth = inject(AuthService);
  inventoryService = inject(InventoryService);
  
  items = signal<InventoryItem[]>([]);
  
  // Verschiedene View-Scopes
  canViewAllInventory = computed(() => 
    this.auth.hasPermission('inventory.view.all') || this.auth.isAdmin()
  );
  
  canViewOwnInventory = computed(() => 
    this.auth.hasPermission('inventory.view.own')
  );
  
  ngOnInit() {
    // Lade basierend auf Permissions
    if (this.canViewAllInventory()) {
      this.loadAllInventory();
    } else if (this.canViewOwnInventory()) {
      this.loadOwnInventory();
    } else {
      this.router.navigate(['/dashboard']);
    }
  }
  
  canEditItem(item: InventoryItem): boolean {
    // Logik: Entweder Admin oder Owner des Items
    return this.auth.isAdmin() || 
           (this.auth.hasPermission('inventory.update') && 
            item.created_by === this.auth.currentUser()?.id);
  }
}
```

### Beispiel 3: Conditional Navigation

```typescript
// sidebar.component.ts
export class SidebarComponent {
  auth = inject(AuthService);
  
  menuItems = computed(() => {
    const items = [
      { path: '/dashboard', label: 'Dashboard', icon: 'home' },
    ];
    
    if (this.auth.hasPermission('inventory.view.all') || 
        this.auth.hasPermission('inventory.view.own')) {
      items.push({ 
        path: '/inventory', 
        label: 'Inventar', 
        icon: 'inventory' 
      });
    }
    
    if (this.auth.hasPermission('users.view') || this.auth.isAdmin()) {
      items.push({ 
        path: '/tenant-admin', 
        label: 'Benutzerverwaltung', 
        icon: 'users' 
      });
    }
    
    if (this.auth.isStaxAdmin()) {
      items.push({ 
        path: '/admin', 
        label: 'System-Admin', 
        icon: 'settings' 
      });
    }
    
    return items;
  });
}
```

---

## 🔄 Migrations & Updates

### Neue Permission hinzufügen

1. **Backend**: Migration erstellen
```sql
INSERT INTO permissions (name, resource, action, scope, description)
VALUES ('inventory.export', 'inventory', 'export', 'all', 'Inventory exportieren');
```

2. **Backend**: Rolle zuweisen
```sql
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'admin' AND p.name = 'inventory.export';
```

3. **Frontend**: Permission Guide aktualisieren (diese Datei)

4. **Frontend**: In Component nutzen
```typescript
canExport = computed(() => 
  this.auth.hasPermission('inventory.export') || this.auth.isAdmin()
);
```

---

## 📞 Support

Bei Fragen oder Problemen:
1. Prüfe die verfügbaren Permissions im Backend (`/api/v1/users/permissions/available`)
2. Teste Permissions mit verschiedenen Rollen
3. Kontaktiere das Backend-Team für neue Permissions

---

**Happy Coding! 🚀**