# StaxAdmin Frontend - Kundenverwaltung

Diese Dokumentation beschreibt die Frontend-Integration der Kundenverwaltung (Tenants) mit dem Backend.

## 📁 Struktur

```
stax-admin/
├── stax-admin.component.ts       # Hauptkomponente mit Tabs
├── stax-admin.component.html     # Template mit Tab-Navigation
├── stax-admin.component.css      # Styles
├── tenants/                      # Kundenverwaltung
│   ├── tenants.component.ts      # Tenant-Management Logik
│   ├── tenants.component.html    # Tenant UI
│   └── tenants.component.css     # Tenant Styles
└── README.md                     # Diese Datei
```

## 🚀 Features

### Kundenverwaltung (Tenants Tab)

#### ✅ Implementierte Funktionen

1. **Liste aller Kunden**
   - Karten-Ansicht mit wichtigsten Infos
   - Status-Badges (Trial, Aktiv, Gesperrt, Gekündigt)
   - Benutzer-Anzahl pro Kunde
   - Plan-Anzeige

2. **Suche & Filter**
   - Volltextsuche nach Name/Kundennummer
   - Filter nach Status (trial, active, suspended, canceled)
   - Filter nach Plan
   - Paginierung (20 Einträge pro Seite)

3. **Statistiken-Dashboard**
   - Gesamt-Anzahl Kunden
   - Aktive Kunden
   - Kunden in Testphase
   - Gesperrte Kunden
   - Gesamtzahl Benutzer

4. **CRUD-Operationen**
   - ✅ Neuen Kunden anlegen
   - ✅ Kunde bearbeiten
   - ✅ Kunde löschen
   - ✅ Status ändern (Aktivieren/Sperren)
   - ✅ Plan wechseln

5. **Detail-Ansicht**
   - Vollständige Kundeninformationen
   - Kundennummer (automatisch generiert)
   - USt-IdNr., Steuernummer
   - Webseite, Branche, Unternehmensgröße
   - Gültigkeitsdatum
   - Notizen

### Benutzerverwaltung (Users Tab)

Bestehende User-Management-Funktionalität (unverändert).

## 🔌 Backend-Integration

### AdminService

Der `AdminService` (`src/app/core/services/admin.service.ts`) stellt alle API-Funktionen bereit:

```typescript
import { AdminService } from '../../../core/services/admin.service';

// Kunden laden
this.adminService.listTenants({
  status: 'active',
  search: 'München',
  page: 1,
  per_page: 20
}).subscribe(tenants => {
  // Handle tenants
});

// Neuen Kunden anlegen
this.adminService.createTenant({
  name: 'Neue Firma GmbH',
  plan_id: 'plan-uuid',
  status: 'trial',
  vat_number: 'DE123456789'
}).subscribe(tenant => {
  // Handle created tenant
});
```

### TypeScript-Typen

Alle Typen sind aus dem Backend generiert (`src/app/core/models/`):

```typescript
import type {
  TenantListItem,
  TenantDetail,
  CreateTenantRequest,
  UpdateTenantRequest,
  Plan,
  TenantStatistics
} from '../../../core/models';
```

## 📋 API-Endpunkte

Alle Endpunkte sind unter `/api/admin` verfügbar:

### Tenants
- `GET /api/admin/tenants` - Liste mit Filter/Paginierung
- `GET /api/admin/tenants/{id}` - Einzelner Kunde
- `POST /api/admin/tenants` - Neuen Kunden anlegen
- `PUT /api/admin/tenants/{id}` - Kunde aktualisieren
- `DELETE /api/admin/tenants/{id}` - Kunde löschen
- `PATCH /api/admin/tenants/{id}/status` - Status ändern

### Pläne
- `GET /api/admin/plans` - Alle Pläne
- `PUT /api/admin/tenants/{id}/plan` - Plan wechseln

### Statistiken
- `GET /api/admin/statistics` - Dashboard-Statistiken

### Adressen (TODO)
- `GET /api/admin/tenants/{id}/addresses`
- `POST /api/admin/tenants/{id}/addresses`
- `PUT /api/admin/tenants/{tenant_id}/addresses/{address_id}`
- `DELETE /api/admin/tenants/{tenant_id}/addresses/{address_id}`

### Kontakte (TODO)
- `GET /api/admin/tenants/{id}/contacts`
- `POST /api/admin/tenants/{id}/contacts`
- Etc.

### Zahlungen (TODO)
- `GET /api/admin/tenants/{id}/payments`
- `POST /api/admin/tenants/{id}/payments`
- Etc.

## 🎨 UI-Komponenten

### Modals

1. **Create Modal** - Neuen Kunden anlegen
2. **Edit Modal** - Kunde bearbeiten
3. **Detail Modal** - Vollständige Kundeninformationen
4. **Plan Switch Modal** - Plan wechseln
5. **Delete Confirmation** - Lösch-Bestätigung

### Status-Badges

Automatische Farbcodierung:
- **Trial** (Testphase): Blau
- **Active** (Aktiv): Grün
- **Suspended** (Gesperrt): Rot
- **Canceled** (Gekündigt): Grau

## 🔐 Authentifizierung

Alle Requests erfordern:
- JWT-Token im `Authorization` Header
- Rolle: `staxadmin`

Der AdminService fügt automatisch den Token hinzu (via HTTP Interceptor).

## 🛠️ Entwicklung

### Neue Features hinzufügen

1. **Backend-Endpunkt** hinzufügen (siehe `backend/apps/src/modules/admin/mod.rs`)
2. **TypeScript-Typen** regenerieren: `cd backend && cargo test -p shared_types`
3. **Service-Methode** in `admin.service.ts` hinzufügen
4. **UI-Komponente** erstellen/erweitern

### Beispiel: Adressen-Management hinzufügen

```typescript
// In tenants.component.ts
navigateToAddresses(tenantId: string) {
  this.router.navigate(['/stax-admin/tenants', tenantId, 'addresses']);
}
```

Dann neue Komponente erstellen:
```bash
ng generate component features/stax-admin/addresses
```

## 📊 Datenfluss

```
Component (tenants.component.ts)
    ↓
Service (admin.service.ts)
    ↓
HTTP Request → Backend API
    ↓
Backend (Rust/Axum)
    ↓
Database (PostgreSQL)
```

## 🐛 Debugging

### Backend nicht erreichbar?

```typescript
// In environment.ts
export const environment = {
  production: false,
  apiUrl: 'http://localhost:8080/api'
};
```

### Token-Probleme?

Prüfen Sie den HTTP Interceptor in `src/app/core/interceptors/`.

### Typen stimmen nicht?

TypeScript-Typen neu generieren:
```bash
cd backend
cargo test -p shared_types
```

## 📝 TODO / Erweiterungen

### Kurzfristig
- [ ] Adressen-Verwaltung UI
- [ ] Kontakt-Verwaltung UI
- [ ] Zahlungs-Verwaltung UI
- [ ] Export-Funktionen (CSV, PDF)

### Mittelfristig
- [ ] Bulk-Operationen (mehrere Kunden gleichzeitig)
- [ ] Erweiterte Suche mit mehr Filtern
- [ ] Sortierung nach verschiedenen Feldern
- [ ] Tenant-Detail-Seite mit Tabs (Adressen, Kontakte, Zahlungen)

### Langfristig
- [ ] Audit-Log (wer hat was geändert)
- [ ] E-Mail-Benachrichtigungen
- [ ] Automatische Rechnungserstellung
- [ ] Dashboard mit Charts
- [ ] Tenant-Aktivitäten verfolgen

## 🔗 Weitere Dokumentation

- **Backend API**: `backend/ADMIN_API.md`
- **Backend Setup**: `backend/ADMIN_SETUP.md`
- **Architektur**: `backend/ADMIN_ARCHITECTURE.md`
- **TypeScript Models**: `frontend/src/app/core/models/README.md`

## 💡 Best Practices

1. **Error Handling**: Alle API-Calls mit Error-Handler
2. **Loading States**: UI zeigt Loading-Spinner während Requests
3. **Success Messages**: Bestätigungen nach erfolgreichen Aktionen
4. **Optimistic UI**: Sofortige UI-Updates, Rollback bei Fehler
5. **Type Safety**: Immer TypeScript-Typen aus Backend verwenden

## 🎯 Verwendung

```typescript
// In einer Komponente
import { AdminService } from '@core/services/admin.service';
import type { TenantListItem } from '@core/models';

private adminService = inject(AdminService);
tenants = signal<TenantListItem[]>([]);

loadTenants() {
  this.adminService.listTenants({ status: 'active' }).subscribe({
    next: (tenants) => this.tenants.set(tenants),
    error: (err) => console.error(err)
  });
}
```

---

**Version**: 1.0.0  
**Letzte Aktualisierung**: 2025-01-25  
**Status**: ✅ Production Ready