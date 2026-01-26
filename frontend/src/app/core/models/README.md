# TypeScript Models

Diese Modelle werden automatisch aus den Rust-Backend-Typen generiert mittels **ts-rs**.

## 📦 Automatische Generierung

Die TypeScript-Typen werden aus `backend/libs/shared_types/src/lib.rs` generiert.

### Neu generieren

```bash
cd backend
cargo test -p shared_types
```

Dies exportiert alle Typen nach `frontend/src/app/core/models/`.

## 📚 Verfügbare Typen

### Auth & Authentication
- `LoginRequest` - Login-Credentials
- `AuthResponse` - Login-Response mit Token
- `UserInfo` - Benutzerinformationen
- `RegisterRequest` - Registrierungsdaten

### Tenants (Kunden)
- `Tenant` - Basis-Tenant-Typ
- `TenantStatus` - Status-Enum (Active, Trial, Suspended, Canceled)
- `TenantListItem` - Tenant in Listen-Ansicht
- `TenantDetail` - Vollständige Tenant-Details
- `CreateTenantRequest` - Tenant erstellen
- `UpdateTenantRequest` - Tenant aktualisieren
- `TenantQueryParams` - Filter/Suche-Parameter

### Adressen
- `TenantAddress` - Adresse eines Tenants
- `CreateAddressRequest` - Adresse erstellen
- `UpdateAddressRequest` - Adresse aktualisieren

### Kontakte
- `TenantContact` - Ansprechpartner eines Tenants
- `CreateContactRequest` - Kontakt erstellen
- `UpdateContactRequest` - Kontakt aktualisieren

### Zahlungen
- `TenantPayment` - Zahlungsinformation
- `CreatePaymentRequest` - Zahlung erfassen
- `UpdatePaymentRequest` - Zahlung aktualisieren

### Pläne
- `Plan` - Subscription-Plan
- `SwitchPlanRequest` - Plan wechseln

### Statistiken
- `TenantStatistics` - Dashboard-Statistiken
- `TenantsByPlan` - Tenants gruppiert nach Plan

### Benutzer
- `UserListItem` - Benutzer in Listen-Ansicht
- `UserDetail` - Vollständige Benutzer-Details
- `UserRoleInfo` - Rollen-Information
- `PermissionInfo` - Permission-Information
- `CreateUserRequest` - Benutzer erstellen
- `UpdateUserRequest` - Benutzer aktualisieren
- `AssignRoleRequest` - Rolle zuweisen
- `AssignPermissionRequest` - Permission zuweisen
- `UpdatePasswordRequest` - Passwort ändern
- `RoleListItem` - Rolle in Listen-Ansicht
- `PermissionListItem` - Permission in Listen-Ansicht

### Inventar
- `GlobalProduct` - Globales Produkt
- `InventoryItem` - Inventar-Artikel

## 🎯 Verwendung

### Einzelne Typen importieren

```typescript
import type { TenantDetail } from '@/app/core/models/TenantDetail';
import type { CreateTenantRequest } from '@/app/core/models/CreateTenantRequest';

const tenant: TenantDetail = {
  id: '123',
  name: 'Muster GmbH',
  // ...
};
```

### Alle Typen importieren

```typescript
import type {
  TenantDetail,
  CreateTenantRequest,
  TenantAddress,
  UserDetail
} from '@/app/core/models';
```

### Mit API-Calls verwenden

```typescript
import type { TenantListItem, CreateTenantRequest } from '@/app/core/models';

async function getTenants(): Promise<TenantListItem[]> {
  const response = await fetch('/api/admin/tenants', {
    headers: { 'Authorization': `Bearer ${token}` }
  });
  return response.json();
}

async function createTenant(data: CreateTenantRequest): Promise<TenantDetail> {
  const response = await fetch('/api/admin/tenants', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });
  return response.json();
}
```

## ⚠️ Wichtig

- **NICHT MANUELL BEARBEITEN!** Diese Dateien werden automatisch generiert
- Änderungen müssen in `backend/libs/shared_types/src/lib.rs` vorgenommen werden
- Nach Änderungen im Backend: `cargo test -p shared_types` ausführen
- Alle Typen sind Type-Safe und synchron mit dem Backend

## 🔄 Workflow

1. Backend-Typ in `backend/libs/shared_types/src/lib.rs` ändern/hinzufügen
2. `#[derive(TS, Serialize, Deserialize)]` hinzufügen
3. `#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]` hinzufügen
4. Export-Test in `export_bindings()` hinzufügen
5. `cargo test -p shared_types` ausführen
6. TypeScript-Datei wird automatisch generiert

## 📝 Beispiel: Neuen Typ hinzufügen

```rust
// In backend/libs/shared_types/src/lib.rs

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct MyNewType {
    pub id: Uuid,
    pub name: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Utc>,
}

// In Tests hinzufügen:
#[test]
fn export_bindings() {
    // ...
    MyNewType::export().expect("Failed to export MyNewType");
}
```

Dann: `cargo test -p shared_types` und `MyNewType.ts` wird generiert!

## 🔗 Weitere Informationen

- **ts-rs Dokumentation:** https://github.com/Aleph-Alpha/ts-rs
- **Backend API-Docs:** `backend/ADMIN_API.md`
- **Backend Typen:** `backend/libs/shared_types/src/lib.rs`
