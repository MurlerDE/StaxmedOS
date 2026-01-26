# Admin-Modul Quick Start Guide

> **Status:** ✅ Production Ready (Version 1.0.1)

## 🚀 In 5 Minuten einsatzbereit

### Schritt 1: Migration ausführen

```bash
cd backend
sqlx migrate run
```

**Erwartete Ausgabe:**
```
Applied 20260125000000/migrate tenant contact details (XXXms)
```

---

### Schritt 2: Backend starten

```bash
cargo run
```

**Erwartete Ausgabe:**
```
Server listening on http://0.0.0.0:8080
```

---

### Schritt 3: Login und Token erhalten

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin@staxmed.com",
    "password": "Admin123!Secure"
  }'
```

**Response:**
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

💾 **Token speichern:**
```bash
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

### Schritt 4: Ersten Kunden anlegen

```bash
curl -X POST http://localhost:8080/api/admin/tenants \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Rettungsdienst München GmbH",
    "plan_id": "PLAN_UUID_HIER",
    "vat_number": "DE123456789",
    "website": "https://www.rettung-muenchen.de",
    "industry": "Emergency Services",
    "company_size": "51-200"
  }'
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_number": "STX-25-1001",
  "name": "Rettungsdienst München GmbH",
  "status": "trial",
  ...
}
```

---

### Schritt 5: Adresse hinzufügen

```bash
TENANT_ID="550e8400-e29b-41d4-a716-446655440000"

curl -X POST http://localhost:8080/api/admin/tenants/$TENANT_ID/addresses \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "address_type": "billing",
    "company_name": "Rettungsdienst München GmbH",
    "street": "Musterstraße 123",
    "postal_code": "80331",
    "city": "München",
    "country": "DE",
    "is_primary": true
  }'
```

---

### Schritt 6: Ansprechpartner hinzufügen

```bash
curl -X POST http://localhost:8080/api/admin/tenants/$TENANT_ID/contacts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "contact_type": "primary",
    "salutation": "mr",
    "first_name": "Max",
    "last_name": "Mustermann",
    "position": "Geschäftsführer",
    "email": "max.mustermann@rettung-muenchen.de",
    "phone": "+49 89 12345678",
    "mobile": "+49 171 1234567",
    "is_primary": true
  }'
```

---

### Schritt 7: Alle Kunden abrufen

```bash
curl -X GET http://localhost:8080/api/admin/tenants \
  -H "Authorization: Bearer $TOKEN"
```

---

### Schritt 8: Statistiken abrufen

```bash
curl -X GET http://localhost:8080/api/admin/statistics \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**
```json
{
  "total_tenants": 1,
  "active_tenants": 0,
  "trial_tenants": 1,
  "suspended_tenants": 0,
  "total_users": 1,
  "tenants_by_plan": [...]
}
```

---

## 📋 Verfügbare Endpunkte (Übersicht)

### Kunden-Verwaltung
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/tenants` | Alle Kunden (mit Filter/Suche) |
| GET | `/api/admin/tenants/{id}` | Einzelner Kunde |
| POST | `/api/admin/tenants` | Neuen Kunden anlegen |
| PUT | `/api/admin/tenants/{id}` | Kunden aktualisieren |
| DELETE | `/api/admin/tenants/{id}` | Kunden löschen |
| PATCH | `/api/admin/tenants/{id}/status` | Status ändern |

### Plan-Verwaltung
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/plans` | Alle Pläne |
| PUT | `/api/admin/tenants/{id}/plan` | Plan wechseln |

### Adressen
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/tenants/{id}/addresses` | Alle Adressen |
| POST | `/api/admin/tenants/{id}/addresses` | Adresse hinzufügen |
| PUT | `/api/admin/tenants/{tenant_id}/addresses/{address_id}` | Aktualisieren |
| DELETE | `/api/admin/tenants/{tenant_id}/addresses/{address_id}` | Löschen |

### Kontakte
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/tenants/{id}/contacts` | Alle Kontakte |
| POST | `/api/admin/tenants/{id}/contacts` | Kontakt hinzufügen |
| PUT | `/api/admin/tenants/{tenant_id}/contacts/{contact_id}` | Aktualisieren |
| DELETE | `/api/admin/tenants/{tenant_id}/contacts/{contact_id}` | Löschen |

### Zahlungen
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/tenants/{id}/payments` | Alle Zahlungen |
| POST | `/api/admin/tenants/{id}/payments` | Zahlung erfassen |
| PUT | `/api/admin/tenants/{tenant_id}/payments/{payment_id}` | Aktualisieren |
| DELETE | `/api/admin/tenants/{tenant_id}/payments/{payment_id}` | Löschen |

### Statistiken
| Methode | Endpunkt | Beschreibung |
|---------|----------|--------------|
| GET | `/api/admin/statistics` | Übersicht aller Kunden |

---

## 🎯 Häufige Aufgaben

### Kunden von Trial auf Active setzen

```bash
curl -X PATCH http://localhost:8080/api/admin/tenants/$TENANT_ID/status \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status": "active"}'
```

### Nach aktiven Kunden suchen

```bash
curl -X GET "http://localhost:8080/api/admin/tenants?status=active" \
  -H "Authorization: Bearer $TOKEN"
```

### Kunden nach Name suchen

```bash
curl -X GET "http://localhost:8080/api/admin/tenants?search=München" \
  -H "Authorization: Bearer $TOKEN"
```

### Plan wechseln

```bash
curl -X PUT http://localhost:8080/api/admin/tenants/$TENANT_ID/plan \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"plan_id": "NEW_PLAN_UUID"}'
```

---

## 🐛 Troubleshooting

### Problem: "401 Unauthorized"
**Lösung:** Token abgelaufen oder ungültig. Neu einloggen.

### Problem: "403 Forbidden"
**Lösung:** User hat nicht die Rolle `staxadmin`. Richtigen Admin-User verwenden.

### Problem: "404 Not Found"
**Lösung:** 
- Prüfen ob Tenant-ID korrekt ist
- Prüfen ob URL korrekt ist (z.B. `/api/admin/tenants` statt `/admin/tenants`)

### Problem: "500 Internal Server Error"
**Lösung:**
- Backend-Logs prüfen
- Migration ausgeführt? → `sqlx migrate run`
- Datenbank erreichbar? → `.env` Datei prüfen

---

## 📚 Frontend-Integration

### TypeScript Service (aus ADMIN_API.md)

```typescript
const API_BASE_URL = 'http://localhost:8080/api/admin';

async function getTenants(token: string) {
  const response = await fetch(`${API_BASE_URL}/tenants`, {
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    }
  });
  return response.json();
}

async function createTenant(token: string, data: any) {
  const response = await fetch(`${API_BASE_URL}/tenants`, {
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

**Vollständiges Service-Modul:** Siehe `ADMIN_API.md` Zeile 1198+

---

## 📖 Weitere Dokumentation

| Dokument | Beschreibung |
|----------|--------------|
| `ADMIN_API.md` | Vollständige API-Referenz (954 Zeilen) |
| `ADMIN_SETUP.md` | Detaillierte Installation & Setup |
| `ADMIN_ARCHITECTURE.md` | System-Architektur & Diagramme |
| `ADMIN_MODULE_SUMMARY.md` | Feature-Übersicht |
| `ADMIN_CHANGELOG.md` | Versions-Historie |
| `ADMIN_QUICKSTART.md` | **Dieses Dokument** |

---

## ✅ Checkliste

- [ ] Migration ausgeführt (`sqlx migrate run`)
- [ ] Backend gestartet (`cargo run`)
- [ ] Login erfolgreich (Token erhalten)
- [ ] Ersten Kunden angelegt
- [ ] Adresse hinzugefügt
- [ ] Kontakt hinzugefügt
- [ ] Statistiken abgerufen
- [ ] Frontend-Integration begonnen

---

**Erstellt:** 2025-01-25  
**Version:** 1.0.1  
**Nächste Schritte:** Siehe `ADMIN_API.md` für vollständige API-Dokumentation

🎉 **Viel Erfolg mit dem Admin-Modul!**