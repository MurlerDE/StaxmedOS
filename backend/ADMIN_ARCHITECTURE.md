# Admin-Modul Architektur

## 🏗️ Systemarchitektur

```
┌─────────────────────────────────────────────────────────────────┐
│                         FRONTEND                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  React/TypeScript Application                            │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐         │   │
│  │  │  Tenant    │  │  Address   │  │  Contact   │         │   │
│  │  │  Manager   │  │  Manager   │  │  Manager   │   ...   │   │
│  │  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘         │   │
│  │        │                │                │                │   │
│  │        └────────────────┴────────────────┘                │   │
│  │                         │                                 │   │
│  │                    ┌────▼────┐                            │   │
│  │                    │adminApi │ (TypeScript Service)       │   │
│  │                    │ Module  │                            │   │
│  │                    └────┬────┘                            │   │
│  └─────────────────────────┼─────────────────────────────────┘   │
└────────────────────────────┼─────────────────────────────────────┘
                             │
                             │ HTTP/REST + JWT
                             │
┌────────────────────────────▼─────────────────────────────────────┐
│                         BACKEND (Rust/Axum)                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  /api/admin Router                                       │   │
│  │  ┌────────────────────────────────────────────────────┐  │   │
│  │  │  Middleware: JWT Auth + staxadmin Role Check      │  │   │
│  │  └────────────────┬───────────────────────────────────┘  │   │
│  │                   │                                      │   │
│  │  ┌────────────────▼──────────────────────┐              │   │
│  │  │  Handlers (21 Endpunkte)              │              │   │
│  │  │  ┌──────────────┐  ┌──────────────┐   │              │   │
│  │  │  │   Tenants    │  │  Addresses   │   │              │   │
│  │  │  │ - list       │  │ - list       │   │              │   │
│  │  │  │ - get        │  │ - create     │   │              │   │
│  │  │  │ - create     │  │ - update     │   │              │   │
│  │  │  │ - update     │  │ - delete     │   │              │   │
│  │  │  │ - delete     │  └──────────────┘   │              │   │
│  │  │  │ - status     │                     │              │   │
│  │  │  └──────────────┘  ┌──────────────┐   │              │   │
│  │  │                    │  Contacts    │   │              │   │
│  │  │  ┌──────────────┐  │ - list       │   │              │   │
│  │  │  │    Plans     │  │ - create     │   │              │   │
│  │  │  │ - list       │  │ - update     │   │              │   │
│  │  │  │ - switch     │  │ - delete     │   │              │   │
│  │  │  └──────────────┘  └──────────────┘   │              │   │
│  │  │                                        │              │   │
│  │  │  ┌──────────────┐  ┌──────────────┐   │              │   │
│  │  │  │  Payments    │  │ Statistics   │   │              │   │
│  │  │  │ - list       │  │ - overview   │   │              │   │
│  │  │  │ - create     │  └──────────────┘   │              │   │
│  │  │  │ - update     │                     │              │   │
│  │  │  │ - delete     │                     │              │   │
│  │  │  └──────────────┘                     │              │   │
│  │  └────────────────┬──────────────────────┘              │   │
│  └───────────────────┼─────────────────────────────────────┘   │
│                      │                                          │
│  ┌───────────────────▼─────────────────────────────────────┐   │
│  │  Data Layer (SQLx)                                      │   │
│  │  - Type-safe queries                                    │   │
│  │  - Prepared statements                                  │   │
│  │  - Connection pooling                                   │   │
│  └───────────────────┬─────────────────────────────────────┘   │
└──────────────────────┼─────────────────────────────────────────┘
                       │
                       │ PostgreSQL Protocol
                       │
┌──────────────────────▼──────────────────────────────────────────┐
│                      DATABASE (PostgreSQL)                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Core Tables                                             │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   tenants    │  │    plans     │  │    users     │   │   │
│  │  │  (extended)  │  │              │  │              │   │   │
│  │  └──────┬───────┘  └──────────────┘  └──────────────┘   │   │
│  │         │                                                │   │
│  │         │ 1:N relationships                              │   │
│  │         │                                                │   │
│  │  ┌──────▼───────────────────────────────────────────┐   │   │
│  │  │  Admin Extension Tables                          │   │   │
│  │  │  ┌───────────────────┐  ┌───────────────────┐   │   │   │
│  │  │  │ tenant_addresses  │  │ tenant_contacts   │   │   │   │
│  │  │  │ - billing         │  │ - primary         │   │   │   │
│  │  │  │ - shipping        │  │ - billing         │   │   │   │
│  │  │  │ - main            │  │ - technical       │   │   │   │
│  │  │  │ - other           │  │ - support         │   │   │   │
│  │  │  └───────────────────┘  └───────────────────┘   │   │   │
│  │  │                                                  │   │   │
│  │  │  ┌───────────────────┐                          │   │   │
│  │  │  │ tenant_payments   │                          │   │   │
│  │  │  │ - invoice_number  │                          │   │   │
│  │  │  │ - status          │                          │   │   │
│  │  │  │ - amount          │                          │   │   │
│  │  │  └───────────────────┘                          │   │   │
│  │  └──────────────────────────────────────────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 📊 Datenmodell

```
┌──────────────────────────────────────────────────────────────────┐
│                           tenants                                 │
├──────────────────────────────────────────────────────────────────┤
│ id (PK)                                                           │
│ customer_number (UNIQUE) ← Auto-generiert: STX-YY-XXXX           │
│ name                                                              │
│ license_key (UNIQUE)                                             │
│ valid_until                                                      │
│ plan_id (FK) ───────────────┐                                   │
│ status (trial|active|...)   │                                   │
│ ─────────────────────────   │                                   │
│ vat_number          ← NEU   │                                   │
│ tax_id              ← NEU   │                                   │
│ website             ← NEU   │                                   │
│ industry            ← NEU   │                                   │
│ company_size        ← NEU   │                                   │
│ preferred_language  ← NEU   │                                   │
│ notes               ← NEU   │                                   │
│ updated_at          ← NEU   │                                   │
└───────┬──────────────────────┴──────────┐                        │
        │                                 │                        │
        │ 1:N                             │                        │
        │                          ┌──────▼──────┐                 │
        │                          │    plans    │                 │
        │                          ├─────────────┤                 │
        │                          │ id (PK)     │                 │
        │                          │ name        │                 │
        │                          │ max_users   │                 │
        │                          └─────────────┘                 │
        │                                                           │
   ┌────┴──────────┬──────────────┬──────────────┐                │
   │               │              │              │                │
   │ 1:N           │ 1:N          │ 1:N          │ 1:N            │
   │               │              │              │                │
┌──▼──────────┐ ┌──▼──────────┐ ┌▼─────────┐ ┌──▼──────────┐    │
│tenant_      │ │tenant_      │ │tenant_   │ │   users     │    │
│addresses    │ │contacts     │ │payments  │ │             │    │
├─────────────┤ ├─────────────┤ ├──────────┤ ├─────────────┤    │
│id (PK)      │ │id (PK)      │ │id (PK)   │ │id (PK)      │    │
│tenant_id(FK)│ │tenant_id(FK)│ │tenant_id │ │tenant_id(FK)│    │
│address_type │ │contact_type │ │  (FK)    │ │username     │    │
│company_name │ │salutation   │ │method    │ │role         │    │
│street       │ │first_name   │ │status    │ │...          │    │
│postal_code  │ │last_name    │ │amount    │ └─────────────┘    │
│city         │ │position     │ │currency  │                    │
│country      │ │email        │ │invoice_# │                    │
│is_primary   │ │phone        │ │due_date  │                    │
│             │ │mobile       │ │paid_at   │                    │
│             │ │is_primary   │ │notes     │                    │
└─────────────┘ └─────────────┘ └──────────┘                    │
```

---

## 🔐 Sicherheitsarchitektur

```
┌─────────────────────────────────────────────────────────────┐
│                    Client Request                            │
│  GET /api/admin/tenants                                      │
│  Authorization: Bearer eyJhbGc...                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────────────────────┐
│  Middleware Layer 1: JWT Validation                          │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  1. Extract Token from Authorization Header           │  │
│  │  2. Verify JWT Signature                              │  │
│  │  3. Check Token Expiration                            │  │
│  │  4. Extract User Claims (id, role, tenant_id)         │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────┘
                       │ ✓ Valid Token
                       ▼
┌──────────────────────────────────────────────────────────────┐
│  Middleware Layer 2: Role Authorization                      │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  1. Check if role == "staxadmin"                       │  │
│  │  2. Reject if user is not staxadmin                    │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────┘
                       │ ✓ staxadmin Role
                       ▼
┌──────────────────────────────────────────────────────────────┐
│  Handler: Process Request                                    │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  1. Validate Input Parameters                          │  │
│  │  2. Execute Type-Safe Query (SQLx)                     │  │
│  │  3. Return JSON Response                               │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
                  [Response]
```

---

## 🔄 Request Flow Beispiel

### Szenario: Neuen Kunden anlegen

```
┌────────┐                                                    ┌────────┐
│Frontend│                                                    │Backend │
└───┬────┘                                                    └───┬────┘
    │                                                             │
    │ 1. POST /api/admin/tenants                                 │
    │    { name: "Neue Firma", plan_id: "..." }                  │
    │────────────────────────────────────────────────────────────>│
    │                                                             │
    │                                          2. JWT Validation  │
    │                                             ✓ staxadmin    │
    │                                                             │
    │                                     3. Validate Input Data  │
    │                                             ✓ OK           │
    │                                                             │
    │                          4. INSERT INTO tenants (...) ─────┐│
    │                             customer_number auto-generated ││
    │                                                             ││
    │                                                     ┌───────▼┴──────┐
    │                                                     │   PostgreSQL  │
    │                                                     │               │
    │                                                     │ - Create Row  │
    │                                                     │ - Run Trigger │
    │                                                     │ - Return Data │
    │                                                     └───────┬───────┘
    │                          5. RETURNING * ◄───────────────────┘│
    │                             (with customer_number)           │
    │                                                             │
    │ 6. Response: 201 Created                                    │
    │    { id: "...", customer_number: "STX-25-1001", ... }       │
    │<────────────────────────────────────────────────────────────│
    │                                                             │
    │ 7. Update UI with new tenant                                │
    │                                                             │
```

---

## 📦 Modul-Struktur

```
backend/
├── apps/src/modules/
│   ├── admin/
│   │   └── mod.rs ...................... Haupt-Modul (831 Zeilen)
│   │       ├── Structs (Tenant, Address, Contact, Payment)
│   │       ├── Router Definition (21 Routes)
│   │       ├── Tenant Handlers (6)
│   │       ├── Plan Handlers (2)
│   │       ├── Address Handlers (4)
│   │       ├── Contact Handlers (4)
│   │       ├── Payment Handlers (4)
│   │       └── Statistics Handler (1)
│   │
│   └── users/ ...................... User-Management (separat)
│       └── mod.rs .................. Benutzer innerhalb Tenants
│
├── migrations/
│   └── 20260125000000_tenant_contact_details.sql
│       ├── CREATE TABLE tenant_addresses
│       ├── CREATE TABLE tenant_contacts
│       ├── CREATE TABLE tenant_payments
│       ├── ALTER TABLE tenants (add fields)
│       ├── CREATE INDEXES
│       └── CREATE TRIGGERS
│
├── Cargo.toml ...................... Dependencies
│   └── rust_decimal = "1.36" ........ Für Zahlungsbeträge
│
└── Dokumentation/
    ├── ADMIN_API.md ................ API-Referenz (954 Zeilen)
    ├── ADMIN_SETUP.md .............. Setup-Guide (286 Zeilen)
    ├── ADMIN_MODULE_SUMMARY.md ..... Zusammenfassung (250 Zeilen)
    └── ADMIN_ARCHITECTURE.md ....... Diese Datei
```

---

## 🎯 Verantwortlichkeiten

### Admin-Modul (`/api/admin`)
- ✅ **Kunden-Management** (Tenants)
  - Anlegen, Bearbeiten, Löschen von Mandanten
  - Status-Verwaltung (trial → active → suspended)
  - Plan-Wechsel
  - Firmen-Stammdaten

- ✅ **Adress-Verwaltung**
  - Mehrere Adressen pro Kunde
  - Rechnungs-, Liefer-, Hauptadressen

- ✅ **Kontakt-Verwaltung**
  - Ansprechpartner mit Rollen
  - Email, Telefon, Position

- ✅ **Zahlungs-Management**
  - Rechnungen erstellen
  - Zahlungsstatus verfolgen
  - Historie

- ✅ **Reporting**
  - Statistiken über alle Kunden
  - Gruppierungen nach Status/Plan

### Users-Modul (`/api/users`)
- ✅ **Benutzer-Verwaltung** (innerhalb Tenant)
  - Benutzer anlegen/bearbeiten
  - Rollen zuweisen
  - Permissions verwalten

**Unterschied:**
- **Admin**: Verwaltet die **Kunden** (Tenants)
- **Users**: Verwaltet **Benutzer innerhalb** eines Kunden

---

## 🔧 Technologie-Stack

### Backend
- **Framework:** Axum (Rust)
- **Database:** PostgreSQL
- **ORM:** SQLx (compile-time checked queries)
- **Auth:** JWT (jsonwebtoken crate)
- **Serialization:** Serde + serde_json
- **Types:** UUID, Chrono, rust_decimal

### Frontend (Integration)
- **Language:** TypeScript
- **Framework:** React (empfohlen)
- **HTTP Client:** Fetch API
- **State:** React Hooks / Context / Redux (nach Wahl)

---

## 📈 Skalierung & Performance

### Database Optimierung
```sql
-- Indizes für häufige Queries
CREATE INDEX idx_tenant_addresses_tenant_id ON tenant_addresses(tenant_id);
CREATE INDEX idx_tenant_contacts_tenant_id ON tenant_contacts(tenant_id);
CREATE INDEX idx_tenant_payments_tenant_id ON tenant_payments(tenant_id);

-- Unique Index für primäre Kontakte/Adressen
CREATE UNIQUE INDEX idx_tenant_contacts_unique_primary
  ON tenant_contacts(tenant_id, contact_type)
  WHERE is_primary = true;

-- Full-Text-Search für Suche
CREATE INDEX idx_tenants_name_search
  ON tenants USING gin(to_tsvector('german', name));
```

### Connection Pooling
- SQLx verwaltet automatisch einen Connection Pool
- Konfigurierbar über Umgebungsvariablen
- Minimiert Datenbankverbindungs-Overhead

### Paginierung
- Alle List-Endpunkte unterstützen `page` & `per_page`
- Standard: 20 Items pro Seite
- Maximum: 100 Items pro Seite

---

## 🧪 Testing-Strategie

### Unit Tests (TODO)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_tenant() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_tenant_validation() {
        // Test implementation
    }
}
```

### Integration Tests (TODO)
- API Endpoint Tests
- Database Migration Tests
- Authentication Tests

### Manual Testing
- Siehe `ADMIN_SETUP.md` für curl-Beispiele
- Postman Collection (TODO)

---

## 🚀 Deployment

### Migration ausführen
```bash
# Development
sqlx migrate run

# Production
DATABASE_URL=postgres://user:pass@prod-host/db sqlx migrate run
```

### Environment Variables
```bash
DATABASE_URL=postgres://user:pass@host/database
JWT_SECRET=your-secret-key-here
RUST_LOG=info
```

### Docker (TODO)
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/backend /usr/local/bin/
CMD ["backend"]
```

---

## 📝 Changelog

### Version 1.0.0 (2025-01-25)
- ✨ Initial Release
- ✅ 21 API Endpoints
- ✅ 4 Database Tables
- ✅ Complete Documentation
- ✅ TypeScript Integration Guide

---

**Erstellt:** 2025-01-25  
**Version:** 1.0.0  
**Autor:** StaxMed Development Team