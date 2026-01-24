# 🔐 RBAC Permission System - StaxMed OS

## Übersicht

StaxMed OS verwendet ein **flexibles, permission-basiertes Berechtigungssystem** (RBAC - Role-Based Access Control), das granulare Kontrolle über Zugriffe ermöglicht.

## 🎯 Konzept

### Hierarchie

```
User
  └─ User Roles (mehrere möglich)
       └─ Role Permissions
  └─ Direct User Permissions (optional, überschreibt Rollen)
  └─ Location Permissions (orts-spezifisch)
```

### Komponenten

1. **Permissions** - Atomare Berechtigungen (z.B. `inventory.view.all`)
2. **Roles** - Vorlagen mit Permissions-Sets (z.B. "Lagerleiter")
3. **User Roles** - Zugewiesene Rollen pro User
4. **User Permissions** - Direkte Permissions (überschreiben Rollen)
5. **Location Permissions** - Standort-spezifische Rechte

---

## 📋 Permission-Namenskonvention

Format: `{resource}.{action}.{scope}`

### Beispiele:

```
inventory.view.all          → Alle Inventare anzeigen
inventory.view.own          → Nur eigene/zugewiesene Inventare
inventory.create            → Neue Items erstellen
users.manage_permissions    → User-Berechtigungen verwalten
system.tenants.manage       → Tenants verwalten (nur staxadmin)
```

### Resource-Typen:

- `inventory` - Bestandsverwaltung
- `shipments` - Lieferungen
- `locations` - Standorte
- `users` - Benutzerverwaltung
- `roles` - Rollenverwaltung
- `devices` - Medizingeräte
- `missions` - Einsätze/Verbrauch
- `tenant` - Tenant-Einstellungen
- `system` - System-Verwaltung (nur staxadmin)

### Actions:

- `view` - Anzeigen/Lesen
- `create` - Erstellen
- `update` - Bearbeiten
- `delete` - Löschen
- `manage` - Vollzugriff (alle CRUD-Operationen)
- `report` - Melden/Berichten
- Spezifisch: `manage_permissions`, `maintenance`, etc.

### Scope:

- `all` - Global/Alle
- `own` - Nur eigene/zugewiesene
- `location:{uuid}` - Spezifischer Standort (geplant)
- `warehouse:{uuid}` - Spezifisches Lager (geplant)

---

## 👥 Vordefinierte System-Rollen

### 1. StaxAdmin (Super Administrator)

**Vollständiger System-Zugriff für StaxMed Mitarbeiter**

```
Tenant: StaxMed System (00000000-0000-0000-0000-000000000000)
Permissions:
  ✅ system.tenants.view
  ✅ system.tenants.manage
  ✅ system.licenses.manage
  ✅ system.all
```

**Kann:**
- Alle Tenants sehen und verwalten
- Lizenzen verwalten
- System-Einstellungen vornehmen
- Über Tenant-Grenzen hinweg arbeiten

---

### 2. Tenant Admin (Administrator)

**Vollständiger Zugriff innerhalb des eigenen Tenants**

```
Permissions:
  ✅ inventory.manage
  ✅ shipments.manage
  ✅ locations.* (view, create, update, delete)
  ✅ users.* (view, create, update, delete)
  ✅ users.manage_permissions
  ✅ roles.* (view, create, update, delete)
  ✅ devices.* (view, manage, maintenance)
  ✅ missions.* (view, report)
  ✅ tenant.settings.* (view, update)
```

**Kann:**
- User und Rollen verwalten
- Berechtigungen zuweisen
- Standorte verwalten
- Alle Inventare sehen und bearbeiten
- Tenant-Einstellungen ändern

**Kann NICHT:**
- Andere Tenants sehen
- System-Einstellungen ändern
- StaxAdmin-Rolle vergeben

---

### 3. Logistic Manager (Logistik-Manager)

**Verwaltung von Bestand und Logistik**

```
Permissions:
  ✅ inventory.view.all
  ✅ inventory.create
  ✅ inventory.update
  ✅ shipments.view
  ✅ shipments.create
  ✅ shipments.receive
  ✅ shipments.manage
  ✅ locations.view
  ✅ missions.view
```

**Kann:**
- Alle Inventare sehen und bearbeiten
- Lieferungen erstellen und verwalten
- Wareneingang durchführen
- Standorte anzeigen

**Kann NICHT:**
- User verwalten
- Standorte erstellen/löschen
- Tenant-Einstellungen ändern

---

### 4. Paramedic (Sanitäter)

**Basis-Zugriff für Einsatzkräfte**

```
Permissions:
  ✅ inventory.view.own
  ✅ missions.report
  ✅ locations.view
```

**Kann:**
- Eigene/zugewiesene Inventare anzeigen
- Verbrauch nach Einsatz melden
- Standorte anzeigen

**Kann NICHT:**
- Andere Inventare sehen
- Bestand ändern
- Lieferungen verwalten

---

## 🛠️ Verwendung im Backend

### Permission Check in Route Handler

```rust
use crate::modules::auth::RequiresPermission;

// Einfacher Permission-Check
async fn create_inventory(
    RequiresPermission("inventory.create"): RequiresPermission,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateInventoryRequest>,
) -> Result<Json<Inventory>, (StatusCode, String)> {
    // User hat inventory.create Permission
    // Implementierung...
}

// Mehrere Permissions (OR)
async fn view_sensitive_data(
    RequiresPermission("admin.view"): RequiresPermission,
    RequiresPermission("audit.view"): RequiresPermission, // Eine von beiden reicht
) -> Result<Json<Data>, (StatusCode, String)> {
    // ...
}
```

### Permission Check in Business Logic

```rust
// In Service/Repository
async fn has_permission(user_id: Uuid, permission: &str, pool: &PgPool) -> bool {
    sqlx::query_scalar!(
        "SELECT user_has_permission($1, $2)",
        user_id,
        permission
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false)
}

// Verwendung
if !has_permission(user_id, "inventory.delete", &pool).await {
    return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
}
```

### Alle User-Permissions abrufen

```rust
let permissions = sqlx::query!(
    "SELECT * FROM get_user_permissions($1)",
    user_id
)
.fetch_all(&pool)
.await?;

for perm in permissions {
    println!("{} (von {})", perm.permission_name, perm.source);
}
```

---

## 🎨 Verwendung im Frontend

### AuthService

```typescript
// In Component/Service
if (auth.hasPermission('inventory.create')) {
    // Zeige "Neu" Button
}

if (auth.hasAnyPermission(['admin.view', 'staxadmin'])) {
    // Admin-Bereich
}
```

### Directive (geplant)

```html
<!-- Button nur anzeigen wenn Permission vorhanden -->
<button *appHasPermission="'inventory.create'">
    Neues Item
</button>

<!-- Mehrere Permissions (OR) -->
<div *appHasAnyPermission="['inventory.view.all', 'inventory.view.own']">
    Inventory-Liste
</div>
```

### Route Guard (geplant)

```typescript
{
    path: 'inventory/create',
    component: CreateInventoryComponent,
    canActivate: [PermissionGuard],
    data: {
        permission: 'inventory.create'
    }
}
```

---

## 📐 Erweiterte Szenarien

### 1. Custom Rollen erstellen (als Tenant Admin)

```sql
-- Neue Rolle erstellen
INSERT INTO roles (tenant_id, name, display_name, description)
VALUES (
    :tenant_id,
    'warehouse_supervisor',
    'Lagerleiter',
    'Verantwortlich für Hauptlager'
);

-- Permissions zuweisen
INSERT INTO role_permissions (role_id, permission_id)
SELECT
    :new_role_id,
    id
FROM permissions
WHERE name IN (
    'inventory.view.all',
    'inventory.create',
    'inventory.update',
    'locations.view'
);

-- Rolle einem User zuweisen
INSERT INTO user_roles (user_id, role_id, assigned_by)
VALUES (:user_id, :role_id, :admin_user_id);
```

---

### 2. Temporäre Permissions

```sql
-- User darf für 7 Tage Lieferungen empfangen
INSERT INTO user_permissions (user_id, permission_id, granted, expires_at, granted_by)
VALUES (
    :user_id,
    (SELECT id FROM permissions WHERE name = 'shipments.receive'),
    true,
    now() + interval '7 days',
    :admin_user_id
);
```

---

### 3. Permission explizit verweigern (Override)

```sql
-- User hat über Rolle "inventory.delete", aber explizit verweigern
INSERT INTO user_permissions (user_id, permission_id, granted, granted_by)
VALUES (
    :user_id,
    (SELECT id FROM permissions WHERE name = 'inventory.delete'),
    false,  -- Explizit verweigert
    :admin_user_id
);
```

---

### 4. Standort-spezifische Rechte

```sql
-- User darf nur RTW-1 (Fahrzeug) verwalten
INSERT INTO location_permissions (user_id, location_id, permission_level, granted_by)
VALUES (
    :user_id,
    :rtw1_location_id,
    'manage',
    :admin_user_id
);

-- Abfrage: Welche Locations darf User sehen?
SELECT l.*
FROM locations l
JOIN location_permissions lp ON l.id = lp.location_id
WHERE lp.user_id = :user_id
  AND lp.permission_level IN ('view', 'edit', 'manage');
```

---

## 🔄 Migration bestehender User

Alle existierenden User wurden automatisch migriert:

```sql
-- Mapping: alte role → neue Rolle
staxadmin        → Role: StaxAdmin (system.*)
admin            → Role: Tenant Admin (inventory.manage, users.*, ...)
logistic_manager → Role: Logistic Manager (inventory.*, shipments.*)
paramedic        → Role: Paramedic (inventory.view.own, missions.report)
```

Die alte `users.role` Spalte bleibt als Fallback erhalten.

---

## ⚠️ Best Practices

### 1. Principle of Least Privilege
Weise nur die minimal notwendigen Permissions zu.

```sql
-- ❌ Schlecht: Alles erlauben
INSERT INTO role_permissions (role_id, permission_id)
SELECT :role_id, id FROM permissions;

-- ✅ Gut: Spezifisch
INSERT INTO role_permissions (role_id, permission_id)
SELECT :role_id, id FROM permissions
WHERE name IN ('inventory.view.own', 'missions.report');
```

---

### 2. Rollen statt direkte Permissions
Nutze Rollen für wiederkehrende Permission-Sets.

```sql
-- ✅ Besser: Rolle erstellen und mehrfach zuweisen
-- Statt: Jedem User einzeln 10 Permissions geben
```

---

### 3. Dokumentiere Custom Rollen

```sql
INSERT INTO roles (tenant_id, name, display_name, description)
VALUES (
    :tenant_id,
    'night_shift_supervisor',
    'Nachtschicht-Leiter',
    'Verantwortlich für Nachtschicht: Kann Verbrauch melden und Notfall-Bestellungen tätigen'
    -- ✅ Klare Beschreibung!
);
```

---

### 4. Regelmäßig Permission-Audits

```sql
-- Welche Permissions hat User X?
SELECT * FROM get_user_permissions(:user_id);

-- Wer hat admin-Rechte?
SELECT DISTINCT u.username, r.name
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
WHERE r.name IN ('admin', 'staxadmin');
```

---

## 🚀 Geplante Erweiterungen

### Phase 2: Wildcard-Support
```
inventory.*         → Alle Inventory-Permissions
*.view              → Alle View-Permissions
locations.*.all     → Alle Location-Permissions mit scope "all"
```

### Phase 3: Permission-Groups
```
Group: "logistics"
  ├─ inventory.*
  ├─ shipments.*
  └─ locations.view
```

### Phase 4: Conditional Permissions
```
IF time >= 22:00 AND time <= 06:00 THEN deny("inventory.create")
IF location_type = "vehicle" THEN allow("inventory.view")
```

---

## 📚 API Endpoints (geplant)

### User Permissions verwalten

```http
# Alle Permissions eines Users
GET /api/v1/users/{user_id}/permissions

# Permission zuweisen
POST /api/v1/users/{user_id}/permissions
{
    "permission": "inventory.create",
    "expires_at": "2024-12-31T23:59:59Z"  // Optional
}

# Permission entziehen
DELETE /api/v1/users/{user_id}/permissions/{permission_id}
```

### Rollen verwalten

```http
# Alle Rollen des Tenants
GET /api/v1/roles

# Neue Rolle erstellen
POST /api/v1/roles
{
    "name": "warehouse_manager",
    "display_name": "Lagerleiter",
    "description": "Verantwortlich für Lagerverwaltung",
    "permissions": [
        "inventory.view.all",
        "inventory.create",
        "inventory.update"
    ]
}

# Rolle einem User zuweisen
POST /api/v1/users/{user_id}/roles
{
    "role_id": "uuid"
}
```

---

## 🧪 Testing

### Permission-Check testen

```rust
#[tokio::test]
async fn test_user_has_permission() {
    let pool = setup_test_db().await;
    
    // User mit Paramedic-Rolle
    let user_id = create_test_user(&pool, "paramedic").await;
    
    // Hat inventory.view.own
    assert!(has_permission(user_id, "inventory.view.own", &pool).await);
    
    // Hat NICHT inventory.delete
    assert!(!has_permission(user_id, "inventory.delete", &pool).await);
}
```

---

## 📞 Support

Bei Fragen zum Permission-System:
- **Docs:** https://docs.staxmed.de/rbac
- **Email:** dev@staxmed.de
- **Issue Tracker:** https://github.com/staxmed/staxmed-os/issues

---

**Letzte Aktualisierung:** 2026-01-24  
**Version:** 2.0.0 (RBAC Migration)