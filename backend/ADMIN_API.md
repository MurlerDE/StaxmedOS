# Admin API Dokumentation

Diese Dokumentation beschreibt alle Endpunkte des Admin-Moduls für die Kundenverwaltung.

**Base URL:** `http://localhost:8080/api/admin`

**Authentication:** Alle Endpunkte erfordern eine gültige JWT-Authentifizierung mit Admin-Rechten (Rolle: `staxadmin`).

---

## 📋 Inhaltsverzeichnis

1. [Kundenverwaltung (Tenants)](#kundenverwaltung-tenants)
2. [Plan-Verwaltung](#plan-verwaltung)
3. [Adressen-Verwaltung](#adressen-verwaltung)
4. [Kontakt-Verwaltung](#kontakt-verwaltung)
5. [Zahlungs-Verwaltung](#zahlungs-verwaltung)
6. [Statistiken](#statistiken)

---

## Kundenverwaltung (Tenants)

### 1. Liste aller Kunden abrufen

**GET** `/api/admin/tenants`

**Query Parameter:**
- `status` (optional): Filter nach Status (`trial`, `active`, `suspended`, `canceled`)
- `plan_id` (optional): Filter nach Plan-ID (UUID)
- `search` (optional): Suche nach Name oder Kundennummer
- `page` (optional): Seitennummer (Standard: 1)
- `per_page` (optional): Einträge pro Seite (Standard: 20, Max: 100)

**Beispiel Request:**
```javascript
const response = await fetch('http://localhost:8080/api/admin/tenants?status=active&page=1&per_page=20', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const tenants = await response.json();
```

**Response (200 OK):**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_number": "STX-25-1001",
    "name": "Muster GmbH",
    "status": "active",
    "plan_id": "660e8400-e29b-41d4-a716-446655440000",
    "plan_name": "Professional",
    "valid_until": "2025-12-31T23:59:59Z",
    "created_at": "2025-01-15T10:30:00Z",
    "user_count": 15
  }
]
```

---

### 2. Einzelnen Kunden abrufen

**GET** `/api/admin/tenants/:id`

**Path Parameter:**
- `id`: Tenant UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}`, {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const tenant = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_number": "STX-25-1001",
  "name": "Muster GmbH",
  "license_key": "XXXX-XXXX-XXXX-XXXX",
  "valid_until": "2025-12-31T23:59:59Z",
  "is_on_premise": false,
  "hardware_hash": null,
  "plan_id": "660e8400-e29b-41d4-a716-446655440000",
  "status": "active",
  "vat_number": "DE123456789",
  "tax_id": "12/345/67890",
  "website": "https://www.muster-gmbh.de",
  "industry": "Healthcare",
  "company_size": "51-200",
  "preferred_language": "de",
  "notes": "Wichtiger Kunde",
  "created_at": "2025-01-15T10:30:00Z",
  "updated_at": "2025-01-20T14:15:00Z"
}
```

---

### 3. Neuen Kunden anlegen

**POST** `/api/admin/tenants`

**Request Body:**
```json
{
  "name": "Neue Rettungsdienst GmbH",
  "plan_id": "660e8400-e29b-41d4-a716-446655440000",
  "status": "trial",
  "vat_number": "DE987654321",
  "tax_id": "98/765/43210",
  "website": "https://www.neue-rettung.de",
  "industry": "Emergency Services",
  "company_size": "11-50",
  "preferred_language": "de",
  "notes": "Testphase bis 31.03.2025",
  "valid_until": "2025-03-31T23:59:59Z"
}
```

**Beispiel Request:**
```javascript
const response = await fetch('http://localhost:8080/api/admin/tenants', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    name: 'Neue Rettungsdienst GmbH',
    plan_id: '660e8400-e29b-41d4-a716-446655440000',
    status: 'trial',
    vat_number: 'DE987654321',
    website: 'https://www.neue-rettung.de',
    industry: 'Emergency Services',
    company_size: '11-50'
  })
});
const newTenant = await response.json();
```

**Response (201 Created):**
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440000",
  "customer_number": "STX-25-1002",
  "name": "Neue Rettungsdienst GmbH",
  "status": "trial",
  ...
}
```

---

### 4. Kunden aktualisieren

**PUT** `/api/admin/tenants/:id`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:** (alle Felder optional)
```json
{
  "name": "Aktualisierter Name",
  "status": "active",
  "vat_number": "DE123456789",
  "tax_id": "12/345/67890",
  "website": "https://www.example.com",
  "industry": "Healthcare",
  "company_size": "51-200",
  "preferred_language": "en",
  "notes": "Aktualisierte Notizen",
  "valid_until": "2026-12-31T23:59:59Z"
}
```

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}`, {
  method: 'PUT',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    status: 'active',
    valid_until: '2026-12-31T23:59:59Z'
  })
});
const updatedTenant = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_number": "STX-25-1001",
  "name": "Aktualisierter Name",
  "status": "active",
  ...
}
```

---

### 5. Kunden löschen

**DELETE** `/api/admin/tenants/:id`

**Path Parameter:**
- `id`: Tenant UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}`, {
  method: 'DELETE',
  headers: {
    'Authorization': `Bearer ${jwt_token}`
  }
});
```

**Response (204 No Content)**

**⚠️ Warnung:** Löscht den Kunden und alle zugehörigen Daten (CASCADE).

---

### 6. Kunden-Status aktualisieren

**PATCH** `/api/admin/tenants/:id/status`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:**
```json
{
  "status": "suspended"
}
```

**Mögliche Status-Werte:**
- `trial` - Testphase
- `active` - Aktiv
- `suspended` - Gesperrt
- `canceled` - Gekündigt

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/status`, {
  method: 'PATCH',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    status: 'suspended'
  })
});
const updatedTenant = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "suspended",
  ...
}
```

---

## Plan-Verwaltung

### 7. Alle Pläne abrufen

**GET** `/api/admin/plans`

**Beispiel Request:**
```javascript
const response = await fetch('http://localhost:8080/api/admin/plans', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const plans = await response.json();
```

**Response (200 OK):**
```json
[
  {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "name": "Starter",
    "max_users": 5,
    "created_at": "2025-01-01T00:00:00Z"
  },
  {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "name": "Professional",
    "max_users": 25,
    "created_at": "2025-01-01T00:00:00Z"
  },
  {
    "id": "880e8400-e29b-41d4-a716-446655440000",
    "name": "Enterprise",
    "max_users": null,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

### 8. Plan eines Kunden wechseln

**PUT** `/api/admin/tenants/:id/plan`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:**
```json
{
  "plan_id": "770e8400-e29b-41d4-a716-446655440000"
}
```

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const newPlanId = '770e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/plan`, {
  method: 'PUT',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    plan_id: newPlanId
  })
});
const updatedTenant = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "plan_id": "770e8400-e29b-41d4-a716-446655440000",
  ...
}
```

---

## Adressen-Verwaltung

### 9. Adressen eines Kunden abrufen

**GET** `/api/admin/tenants/:id/addresses`

**Path Parameter:**
- `id`: Tenant UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/addresses`, {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const addresses = await response.json();
```

**Response (200 OK):**
```json
[
  {
    "id": "990e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
    "address_type": "billing",
    "company_name": "Muster GmbH",
    "street": "Musterstraße 123",
    "street_additional": "2. OG",
    "postal_code": "12345",
    "city": "Berlin",
    "state": "Berlin",
    "country": "DE",
    "is_primary": true,
    "created_at": "2025-01-15T10:30:00Z",
    "updated_at": "2025-01-15T10:30:00Z"
  }
]
```

---

### 10. Adresse hinzufügen

**POST** `/api/admin/tenants/:id/addresses`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:**
```json
{
  "address_type": "billing",
  "company_name": "Muster GmbH",
  "street": "Musterstraße 123",
  "street_additional": "2. OG",
  "postal_code": "12345",
  "city": "Berlin",
  "state": "Berlin",
  "country": "DE",
  "is_primary": true
}
```

**Address Types:**
- `billing` - Rechnungsadresse
- `shipping` - Lieferadresse
- `main` - Hauptadresse
- `other` - Sonstige

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/addresses`, {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    address_type: 'billing',
    company_name: 'Muster GmbH',
    street: 'Musterstraße 123',
    postal_code: '12345',
    city: 'Berlin',
    country: 'DE',
    is_primary: true
  })
});
const newAddress = await response.json();
```

**Response (201 Created):**
```json
{
  "id": "990e8400-e29b-41d4-a716-446655440000",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "address_type": "billing",
  ...
}
```

---

### 11. Adresse aktualisieren

**PUT** `/api/admin/tenants/:tenant_id/addresses/:address_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `address_id`: Address UUID

**Request Body:** (alle Felder optional)
```json
{
  "street": "Neue Straße 456",
  "postal_code": "54321",
  "city": "München",
  "is_primary": false
}
```

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const addressId = '990e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/addresses/${addressId}`, {
  method: 'PUT',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    street: 'Neue Straße 456',
    postal_code: '54321',
    city: 'München'
  })
});
const updatedAddress = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "990e8400-e29b-41d4-a716-446655440000",
  "street": "Neue Straße 456",
  "postal_code": "54321",
  "city": "München",
  ...
}
```

---

### 12. Adresse löschen

**DELETE** `/api/admin/tenants/:tenant_id/addresses/:address_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `address_id`: Address UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const addressId = '990e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/addresses/${addressId}`, {
  method: 'DELETE',
  headers: {
    'Authorization': `Bearer ${jwt_token}`
  }
});
```

**Response (204 No Content)**

---

## Kontakt-Verwaltung

### 13. Kontakte eines Kunden abrufen

**GET** `/api/admin/tenants/:id/contacts`

**Path Parameter:**
- `id`: Tenant UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/contacts`, {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const contacts = await response.json();
```

**Response (200 OK):**
```json
[
  {
    "id": "aa0e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
    "contact_type": "primary",
    "salutation": "mr",
    "first_name": "Max",
    "last_name": "Mustermann",
    "position": "Geschäftsführer",
    "email": "max.mustermann@muster-gmbh.de",
    "phone": "+49 30 12345678",
    "mobile": "+49 171 1234567",
    "fax": "+49 30 12345679",
    "is_primary": true,
    "notes": "Hauptansprechpartner",
    "created_at": "2025-01-15T10:30:00Z",
    "updated_at": "2025-01-15T10:30:00Z"
  }
]
```

---

### 14. Kontakt hinzufügen

**POST** `/api/admin/tenants/:id/contacts`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:**
```json
{
  "contact_type": "primary",
  "salutation": "mr",
  "first_name": "Max",
  "last_name": "Mustermann",
  "position": "Geschäftsführer",
  "email": "max.mustermann@muster-gmbh.de",
  "phone": "+49 30 12345678",
  "mobile": "+49 171 1234567",
  "fax": "+49 30 12345679",
  "is_primary": true,
  "notes": "Hauptansprechpartner"
}
```

**Contact Types:**
- `primary` - Hauptansprechpartner
- `billing` - Buchhaltung
- `technical` - Technischer Ansprechpartner
- `support` - Support-Kontakt
- `other` - Sonstige

**Salutation:**
- `mr` - Herr
- `ms` - Frau
- `dr` - Dr.
- `prof` - Prof.

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/contacts`, {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    contact_type: 'primary',
    salutation: 'mr',
    first_name: 'Max',
    last_name: 'Mustermann',
    email: 'max.mustermann@muster-gmbh.de',
    phone: '+49 30 12345678',
    is_primary: true
  })
});
const newContact = await response.json();
```

**Response (201 Created):**
```json
{
  "id": "aa0e8400-e29b-41d4-a716-446655440000",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "contact_type": "primary",
  ...
}
```

---

### 15. Kontakt aktualisieren

**PUT** `/api/admin/tenants/:tenant_id/contacts/:contact_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `contact_id`: Contact UUID

**Request Body:** (alle Felder optional)
```json
{
  "email": "neue.email@muster-gmbh.de",
  "phone": "+49 30 98765432",
  "position": "CTO"
}
```

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const contactId = 'aa0e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/contacts/${contactId}`, {
  method: 'PUT',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    email: 'neue.email@muster-gmbh.de',
    phone: '+49 30 98765432'
  })
});
const updatedContact = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "aa0e8400-e29b-41d4-a716-446655440000",
  "email": "neue.email@muster-gmbh.de",
  "phone": "+49 30 98765432",
  ...
}
```

---

### 16. Kontakt löschen

**DELETE** `/api/admin/tenants/:tenant_id/contacts/:contact_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `contact_id`: Contact UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const contactId = 'aa0e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/contacts/${contactId}`, {
  method: 'DELETE',
  headers: {
    'Authorization': `Bearer ${jwt_token}`
  }
});
```

**Response (204 No Content)**

---

## Zahlungs-Verwaltung

### 17. Zahlungen eines Kunden abrufen

**GET** `/api/admin/tenants/:id/payments`

**Path Parameter:**
- `id`: Tenant UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/payments`, {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const payments = await response.json();
```

**Response (200 OK):**
```json
[
  {
    "id": "bb0e8400-e29b-41d4-a716-446655440000",
    "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
    "payment_method": "invoice",
    "payment_status": "paid",
    "amount": "299.99",
    "currency": "EUR",
    "billing_period_start": "2025-01-01",
    "billing_period_end": "2025-01-31",
    "due_date": "2025-02-14",
    "paid_at": "2025-02-10T14:30:00Z",
    "invoice_number": "RE-2025-001",
    "notes": "Rechnung per E-Mail versendet",
    "created_at": "2025-01-31T23:59:00Z",
    "updated_at": "2025-02-10T14:30:00Z"
  }
]
```

---

### 18. Zahlung hinzufügen

**POST** `/api/admin/tenants/:id/payments`

**Path Parameter:**
- `id`: Tenant UUID

**Request Body:**
```json
{
  "payment_method": "invoice",
  "amount": "299.99",
  "currency": "EUR",
  "billing_period_start": "2025-02-01",
  "billing_period_end": "2025-02-28",
  "due_date": "2025-03-14",
  "invoice_number": "RE-2025-002",
  "notes": "Monatliche Abrechnung"
}
```

**Payment Methods:**
- `invoice` - Rechnung
- `credit_card` - Kreditkarte
- `sepa` - SEPA-Lastschrift
- `paypal` - PayPal
- `bank_transfer` - Banküberweisung

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/payments`, {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    payment_method: 'invoice',
    amount: '299.99',
    currency: 'EUR',
    billing_period_start: '2025-02-01',
    billing_period_end: '2025-02-28',
    due_date: '2025-03-14',
    invoice_number: 'RE-2025-002'
  })
});
const newPayment = await response.json();
```

**Response (201 Created):**
```json
{
  "id": "bb0e8400-e29b-41d4-a716-446655440000",
  "tenant_id": "550e8400-e29b-41d4-a716-446655440000",
  "payment_status": "pending",
  ...
}
```

---

### 19. Zahlung aktualisieren

**PUT** `/api/admin/tenants/:tenant_id/payments/:payment_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `payment_id`: Payment UUID

**Request Body:** (alle Felder optional)
```json
{
  "payment_status": "paid",
  "paid_at": "2025-02-10T14:30:00Z",
  "notes": "Zahlung eingegangen"
}
```

**Payment Status:**
- `pending` - Ausstehend
- `paid` - Bezahlt
- `overdue` - Überfällig
- `failed` - Fehlgeschlagen
- `cancelled` - Storniert

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const paymentId = 'bb0e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/payments/${paymentId}`, {
  method: 'PUT',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    payment_status: 'paid',
    paid_at: new Date().toISOString()
  })
});
const updatedPayment = await response.json();
```

**Response (200 OK):**
```json
{
  "id": "bb0e8400-e29b-41d4-a716-446655440000",
  "payment_status": "paid",
  "paid_at": "2025-02-10T14:30:00Z",
  ...
}
```

---

### 20. Zahlung löschen

**DELETE** `/api/admin/tenants/:tenant_id/payments/:payment_id`

**Path Parameter:**
- `tenant_id`: Tenant UUID
- `payment_id`: Payment UUID

**Beispiel Request:**
```javascript
const tenantId = '550e8400-e29b-41d4-a716-446655440000';
const paymentId = 'bb0e8400-e29b-41d4-a716-446655440000';

const response = await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}/payments/${paymentId}`, {
  method: 'DELETE',
  headers: {
    'Authorization': `Bearer ${jwt_token}`
  }
});
```

**Response (204 No Content)**

---

## Statistiken

### 21. Statistik-Übersicht abrufen

**GET** `/api/admin/statistics`

**Beispiel Request:**
```javascript
const response = await fetch('http://localhost:8080/api/admin/statistics', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json'
  }
});
const statistics = await response.json();
```

**Response (200 OK):**
```json
{
  "total_tenants": 42,
  "active_tenants": 35,
  "trial_tenants": 5,
  "suspended_tenants": 2,
  "total_users": 387,
  "tenants_by_plan": [
    {
      "plan_name": "Professional",
      "count": 25
    },
    {
      "plan_name": "Enterprise",
      "count": 10
    },
    {
      "plan_name": "Starter",
      "count": 7
    }
  ]
}
```

---

## ⚠️ Error Handling

Alle Endpunkte können folgende HTTP-Statuscodes zurückgeben:

| Status Code | Bedeutung |
|-------------|-----------|
| `200 OK` | Erfolgreiche Anfrage |
| `201 Created` | Ressource erfolgreich erstellt |
| `204 No Content` | Erfolgreiche Löschung |
| `400 Bad Request` | Ungültige Anfrage oder fehlende Parameter |
| `401 Unauthorized` | Fehlende oder ungültige Authentifizierung |
| `403 Forbidden` | Keine Berechtigung für diese Aktion |
| `404 Not Found` | Ressource nicht gefunden |
| `500 Internal Server Error` | Serverfehler |

**Error Response Format:**
```json
{
  "error": "Fehlermeldung hier"
}
```

**Beispiel Error Handling:**
```javascript
try {
  const response = await fetch('http://localhost:8080/api/admin/tenants', {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${jwt_token}`,
      'Content-Type': 'application/json'
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`HTTP ${response.status}: ${errorText}`);
  }

  const tenants = await response.json();
  return tenants;
} catch (error) {
  console.error('API Error:', error);
  // Fehlerbehandlung (z.B. Benachrichtigung anzeigen)
}
```

---

## 🔒 Authentifizierung

Alle Admin-Endpunkte erfordern einen gültigen JWT-Token mit `staxadmin`-Rolle.

**Token erhalten:**
```javascript
// 1. Login durchführen
const loginResponse = await fetch('http://localhost:8080/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'admin@staxmed.com',
    password: 'your-password'
  })
});

const { token } = await loginResponse.json();

// 2. Token speichern
localStorage.setItem('jwt_token', token);

// 3. Token bei jedem Request verwenden
const response = await fetch('http://localhost:8080/api/admin/tenants', {
  headers: {
    'Authorization': `Bearer ${token}`
  }
});
```

---

## 💡 Best Practices

### 1. Token-Verwaltung
```javascript
// Token Refresh Funktion
async function ensureValidToken() {
  const token = localStorage.getItem('jwt_token');
  if (!token) {
    window.location.href = '/login';
    return null;
  }
  
  // Optional: Token-Ablauf prüfen
  const payload = JSON.parse(atob(token.split('.')[1]));
  if (payload.exp * 1000 < Date.now()) {
    localStorage.removeItem('jwt_token');
    window.location.href = '/login';
    return null;
  }
  
  return token;
}
```

### 2. Paginierung
```javascript
// Paginierte Daten laden
async function loadTenants(page = 1, perPage = 20) {
  const response = await fetch(
    `http://localhost:8080/api/admin/tenants?page=${page}&per_page=${perPage}`,
    {
      headers: { 'Authorization': `Bearer ${await ensureValidToken()}` }
    }
  );
  return response.json();
}
```

### 3. Suche mit Debouncing
```javascript
import { debounce } from 'lodash';

const searchTenants = debounce(async (searchTerm) => {
  const response = await fetch(
    `http://localhost:8080/api/admin/tenants?search=${encodeURIComponent(searchTerm)}`,
    {
      headers: { 'Authorization': `Bearer ${await ensureValidToken()}` }
    }
  );
  const results = await response.json();
  // Ergebnisse anzeigen
}, 300);
```

### 4. Optimistisches UI-Update
```javascript
async function deleteTenant(tenantId) {
  // 1. Sofort aus UI entfernen (optimistisch)
  setTenants(prev => prev.filter(t => t.id !== tenantId));
  
  try {
    // 2. API Call
    await fetch(`http://localhost:8080/api/admin/tenants/${tenantId}`, {
      method: 'DELETE',
      headers: { 'Authorization': `Bearer ${await ensureValidToken()}` }
    });
    
    // 3. Erfolgsbenachrichtigung
    showNotification('Kunde gelöscht', 'success');
  } catch (error) {
    // 4. Bei Fehler: Rückgängig machen
    loadTenants(); // Daten neu laden
    showNotification('Fehler beim Löschen', 'error');
  }
}
```

### 5. Batch-Operationen
```javascript
async function updateMultipleTenants(tenantIds, updates) {
  const promises = tenantIds.map(id =>
    fetch(`http://localhost:8080/api/admin/tenants/${id}`, {
      method: 'PUT',
      headers: {
        'Authorization': `Bearer ${await ensureValidToken()}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(updates)
    })
  );
  
  const results = await Promise.allSettled(promises);
  
  const succeeded = results.filter(r => r.status === 'fulfilled').length;
  const failed = results.filter(r => r.status === 'rejected').length;
  
  showNotification(`${succeeded} erfolgreich, ${failed} fehlgeschlagen`);
}
```

---

## 📊 Datenmodell-Übersicht

### Tenant Status
- `trial` - Testphase (Standard für neue Kunden)
- `active` - Aktiv (bezahlter Kunde)
- `suspended` - Gesperrt (z.B. bei Zahlungsverzug)
- `canceled` - Gekündigt

### Company Size Options
- `1-10`
- `11-50`
- `51-200`
- `201-500`
- `501+`

### Industry Options
- `Healthcare`
- `Emergency Services`
- `Medical Services`
- `Hospital`
- `Other`

---

## 🔗 Weiterführende Dokumentation

- **RBAC-System:** Siehe `backend/PERMISSIONS.md`
- **User Management:** Siehe User-Modul (`/api/users`)
- **Security:** Siehe `backend/SECURITY.md`

---

**Letzte Aktualisierung:** 2025-01-25  
**API Version:** 1.0

---

## 📌 Vollständiges React/TypeScript Beispiel

Hier ist ein vollständiges Beispiel für die Integration im Frontend:

```typescript
// src/services/adminApi.ts

interface Tenant {
  id: string;
  customer_number?: string;
  name: string;
  status?: string;
  plan_id?: string;
  plan_name?: string;
  valid_until?: string;
  created_at?: string;
}

interface CreateTenantData {
  name: string;
  plan_id: string;
  status?: string;
  vat_number?: string;
  website?: string;
  industry?: string;
  company_size?: string;
}

const API_BASE_URL = 'http://localhost:8080/api/admin';

// Hilfsfunktion für API-Calls
async function apiCall<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const token = localStorage.getItem('jwt_token');
  
  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
      ...options.headers,
    },
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }

  if (response.status === 204) {
    return null as T;
  }

  return response.json();
}

// Tenant Management
export const adminApi = {
  tenants: {
    list: (params?: {
      status?: string;
      plan_id?: string;
      search?: string;
      page?: number;
      per_page?: number;
    }) => {
      const query = new URLSearchParams(
        Object.entries(params || {})
          .filter(([_, v]) => v !== undefined)
          .map(([k, v]) => [k, String(v)])
      );
      return apiCall<Tenant[]>(`/tenants?${query}`);
    },

    get: (id: string) => apiCall<Tenant>(`/tenants/${id}`),

    create: (data: CreateTenantData) =>
      apiCall<Tenant>('/tenants', {
        method: 'POST',
        body: JSON.stringify(data),
      }),

    update: (id: string, data: Partial<CreateTenantData>) =>
      apiCall<Tenant>(`/tenants/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      }),

    delete: (id: string) =>
      apiCall<null>(`/tenants/${id}`, { method: 'DELETE' }),

    updateStatus: (id: string, status: string) =>
      apiCall<Tenant>(`/tenants/${id}/status`, {
        method: 'PATCH',
        body: JSON.stringify({ status }),
      }),

    switchPlan: (id: string, plan_id: string) =>
      apiCall<Tenant>(`/tenants/${id}/plan`, {
        method: 'PUT',
        body: JSON.stringify({ plan_id }),
      }),
  },

  plans: {
    list: () => apiCall<any[]>('/plans'),
  },

  addresses: {
    list: (tenantId: string) =>
      apiCall<any[]>(`/tenants/${tenantId}/addresses`),

    create: (tenantId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/addresses`, {
        method: 'POST',
        body: JSON.stringify(data),
      }),

    update: (tenantId: string, addressId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/addresses/${addressId}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      }),

    delete: (tenantId: string, addressId: string) =>
      apiCall<null>(`/tenants/${tenantId}/addresses/${addressId}`, {
        method: 'DELETE',
      }),
  },

  contacts: {
    list: (tenantId: string) =>
      apiCall<any[]>(`/tenants/${tenantId}/contacts`),

    create: (tenantId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/contacts`, {
        method: 'POST',
        body: JSON.stringify(data),
      }),

    update: (tenantId: string, contactId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/contacts/${contactId}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      }),

    delete: (tenantId: string, contactId: string) =>
      apiCall<null>(`/tenants/${tenantId}/contacts/${contactId}`, {
        method: 'DELETE',
      }),
  },

  payments: {
    list: (tenantId: string) =>
      apiCall<any[]>(`/tenants/${tenantId}/payments`),

    create: (tenantId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/payments`, {
        method: 'POST',
        body: JSON.stringify(data),
      }),

    update: (tenantId: string, paymentId: string, data: any) =>
      apiCall<any>(`/tenants/${tenantId}/payments/${paymentId}`, {
        method: 'PUT',
        body: JSON.stringify(data),
      }),

    delete: (tenantId: string, paymentId: string) =>
      apiCall<null>(`/tenants/${tenantId}/payments/${paymentId}`, {
        method: 'DELETE',
      }),
  },

  statistics: {
    get: () => apiCall<any>('/statistics'),
  },
};

// Beispiel-Verwendung in einer React-Komponente:
// const tenants = await adminApi.tenants.list({ status: 'active' });
// const newTenant = await adminApi.tenants.create({ name: 'Test', plan_id: 'xyz' });