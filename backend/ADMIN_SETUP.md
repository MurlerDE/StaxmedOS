# Admin-Modul Setup

Dieses Dokument beschreibt die Einrichtung und Verwendung des Admin-Moduls für die Kundenverwaltung.

## 📋 Übersicht

Das Admin-Modul ermöglicht die Verwaltung von:
- **Tenants (Kunden)**: Anlegen, Bearbeiten, Löschen, Status-Verwaltung
- **Pläne**: Wechsel zwischen verschiedenen Subscription-Plänen
- **Adressen**: Verwaltung von Rechnungs-, Liefer- und sonstigen Adressen
- **Kontakte**: Ansprechpartner mit verschiedenen Rollen
- **Zahlungen**: Zahlungshistorie und Rechnungsverwaltung
- **Statistiken**: Übersicht über alle Kunden und deren Status

## 🚀 Installation

### 1. Migration ausführen

Die Migration für Kundendaten muss zuerst ausgeführt werden:

```bash
cd backend
sqlx migrate run
```

Dies erstellt folgende neue Tabellen:
- `tenant_addresses` - Adressen für Tenants
- `tenant_contacts` - Ansprechpartner für Tenants
- `tenant_payments` - Zahlungsinformationen
- Erweitert `tenants` um zusätzliche Felder (vat_number, tax_id, etc.)

### 2. Dependencies installieren

Die `rust_decimal` Dependency wurde bereits zur `Cargo.toml` hinzugefügt:

```toml
rust_decimal = { version = "1.36", features = ["serde"] }
```

Führen Sie einen Build durch, um die Dependencies zu installieren:

```bash
cargo build
```

### 3. Backend starten

```bash
cargo run
```

Das Admin-Modul ist nun verfügbar unter: `http://localhost:8080/api/admin`

## 🔐 Authentifizierung

Alle Admin-Endpunkte erfordern Authentifizierung mit der Rolle `staxadmin`.

### Standard-Admin-Zugang

Nach der Migration existiert folgender Admin-User:

- **Username:** `admin@staxmed.com`
- **Passwort:** `Admin123!Secure`
- **Rolle:** `staxadmin`

⚠️ **WICHTIG:** Ändern Sie das Passwort nach dem ersten Login!

### Login-Request

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin@staxmed.com",
    "password": "Admin123!Secure"
  }'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "...",
    "username": "admin@staxmed.com",
    "role": "staxadmin"
  }
}
```

## 📚 API-Dokumentation

Die vollständige API-Dokumentation finden Sie in: **`ADMIN_API.md`**

### Schnellstart-Beispiele

#### 1. Alle Kunden abrufen

```bash
curl -X GET http://localhost:8080/api/admin/tenants \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

#### 2. Neuen Kunden anlegen

```bash
curl -X POST http://localhost:8080/api/admin/tenants \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Neue Rettungsdienst GmbH",
    "plan_id": "PLAN_UUID",
    "status": "trial",
    "vat_number": "DE123456789",
    "website": "https://www.neue-rettung.de",
    "industry": "Emergency Services",
    "company_size": "11-50"
  }'
```

#### 3. Adresse hinzufügen

```bash
curl -X POST http://localhost:8080/api/admin/tenants/TENANT_UUID/addresses \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "address_type": "billing",
    "company_name": "Muster GmbH",
    "street": "Musterstraße 123",
    "postal_code": "12345",
    "city": "Berlin",
    "country": "DE",
    "is_primary": true
  }'
```

#### 4. Plan wechseln

```bash
curl -X PUT http://localhost:8080/api/admin/tenants/TENANT_UUID/plan \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "NEW_PLAN_UUID"
  }'
```

## 🛠️ Entwicklung

### Modul-Struktur

```
backend/apps/src/modules/admin/
└── mod.rs              # Hauptmodul mit allen Endpunkten
```

### Neue Endpunkte hinzufügen

1. Fügen Sie die Route im `router()` hinzu:
```rust
.route("/tenants/:id/custom", get(custom_handler))
```

2. Implementieren Sie den Handler:
```rust
async fn custom_handler(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseType>, (StatusCode, String)> {
    // Implementation
}
```

### Datenbank-Queries

Verwenden Sie `sqlx::query_as!` für typsichere Queries:

```rust
let tenant = sqlx::query_as!(
    TenantDetail,
    "SELECT * FROM tenants WHERE id = $1",
    id
)
.fetch_one(&pool)
.await
.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
```

## 📊 Datenbank-Schema

### Wichtige Tabellen

#### `tenants` (erweitert)
- `vat_number` - Umsatzsteuer-ID
- `tax_id` - Steuernummer
- `website` - Webseite
- `industry` - Branche
- `company_size` - Unternehmensgröße
- `preferred_language` - Bevorzugte Sprache
- `notes` - Interne Notizen

#### `tenant_addresses`
- Verschiedene Adresstypen: `billing`, `shipping`, `main`, `other`
- `is_primary` - Primäre Adresse pro Typ

#### `tenant_contacts`
- Verschiedene Kontakttypen: `primary`, `billing`, `technical`, `support`
- `is_primary` - Primärer Kontakt pro Typ
- Anrede: `mr`, `ms`, `dr`, `prof`

#### `tenant_payments`
- Zahlungsmethoden: `invoice`, `credit_card`, `sepa`, `paypal`, `bank_transfer`
- Status: `pending`, `paid`, `overdue`, `failed`, `cancelled`

## 🔍 Testing

### Manuelles Testen

Verwenden Sie Tools wie:
- **curl** (Kommandozeile)
- **Postman** (GUI)
- **Insomnia** (GUI)
- **Thunder Client** (VS Code Extension)

### Beispiel-Test-Workflow

1. Login durchführen und Token erhalten
2. Neuen Kunden anlegen
3. Adresse hinzufügen
4. Kontakt hinzufügen
5. Zahlung erfassen
6. Plan wechseln
7. Status ändern
8. Statistiken abrufen

## ⚠️ Wichtige Hinweise

### Sicherheit

- ✅ Alle Endpunkte sind durch JWT-Authentifizierung geschützt
- ✅ Nur Nutzer mit Rolle `staxadmin` haben Zugriff
- ⚠️ Ändern Sie das Standard-Admin-Passwort
- ⚠️ Verwenden Sie HTTPS in Produktion

### Datenintegrität

- Beim Löschen eines Tenants werden alle zugehörigen Daten gelöscht (CASCADE)
- Pro Adresstyp kann nur eine primäre Adresse existieren
- Pro Kontakttyp kann nur ein primärer Kontakt existieren

### Performance

- Nutzen Sie Paginierung bei großen Datenmengen
- Indizes sind bereits für häufige Queries vorhanden
- Suche nutzt Full-Text-Search (GIN-Index)

## 🔗 Verwandte Dokumentation

- **API-Dokumentation:** `ADMIN_API.md`
- **RBAC-System:** `PERMISSIONS.md`
- **Security:** `SECURITY.md`
- **User-Modul:** `backend/apps/src/modules/users/`

## 📝 Changelog

### Version 1.0 (2025-01-25)
- ✨ Initiales Release
- ✨ Tenant-Verwaltung (CRUD)
- ✨ Adressen-Verwaltung
- ✨ Kontakt-Verwaltung
- ✨ Zahlungs-Verwaltung
- ✨ Plan-Switching
- ✨ Statistiken
- ✨ Vollständige API-Dokumentation

## 🤝 Support

Bei Fragen oder Problemen:
1. Prüfen Sie die API-Dokumentation (`ADMIN_API.md`)
2. Prüfen Sie die Logs: `cargo run` zeigt Fehler an
3. Prüfen Sie die Datenbank-Migration: `sqlx migrate info`

## 📄 Lizenz

Teil des StaxMed-OS Projekts.