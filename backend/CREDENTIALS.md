# 🔐 StaxMed OS - Standard Login-Credentials (Development)

## ⚠️ WICHTIG: Nur für Development!

Diese Credentials sind **NUR für die lokale Entwicklung** gedacht und müssen in Production geändert werden!

---

## 👤 Standard-Benutzer

### 1. **StaxAdmin** (Super Administrator)

Vollständiger System-Zugriff auf alle Tenants und System-Einstellungen.

```
Username: staxadmin
Password: staxadmin123
Rolle:    staxadmin
Tenant:   StaxMed System (00000000-0000-0000-0000-000000000000)
```

**Berechtigungen:**
- ✅ Zugriff auf ALLE Tenants
- ✅ System-weite Einstellungen
- ✅ Tenant-Verwaltung (erstellen, löschen, bearbeiten)
- ✅ User-Verwaltung über alle Tenants hinweg
- ✅ Lizenz-Verwaltung

---

### 2. **Tenant Admin** (Test-Wache)

Administrator für die Test-Organisation. Kann nur innerhalb seines eigenen Tenants arbeiten.

```
Username: admin@testwache.de
Password: admin123
Rolle:    admin
Tenant:   StaxMed Test-Wache
```

**Berechtigungen:**
- ✅ User-Verwaltung (nur eigener Tenant)
- ✅ Location-Verwaltung
- ✅ Inventory-Management
- ✅ Shipment-Verwaltung
- ✅ Tenant-Settings
- ❌ Kein Zugriff auf andere Tenants
- ❌ Keine System-Einstellungen

---

## 🆕 Neue Organisation registrieren

Über den `/register` Endpoint oder das Frontend-Formular kann sich jeder eine neue Organisation anlegen:

1. Gehe zu **Register** Tab im Login-Modal
2. Gib Username und Passwort ein
3. Automatisch wird:
   - ✅ Ein neuer Tenant erstellt
   - ✅ Der User wird zum **Admin** des neuen Tenants
   - ✅ Status: `trial` (Testphase)

---

## 🔧 Rollen-Hierarchie

```
staxadmin (Super Admin)
  ├─ Zugriff auf ALLE Tenants
  ├─ System-weite Verwaltung
  └─ Lizenz-Management
  
admin (Tenant Admin)
  ├─ Zugriff nur auf EIGENEN Tenant
  ├─ User-Verwaltung (eigener Tenant)
  └─ Vollständige Tenant-Verwaltung
  
logistic_manager (Logistik-Manager)
  ├─ Inventory-Verwaltung
  ├─ Shipment-Verwaltung
  └─ Stock-Management
  
paramedic (Sanitäter)
  ├─ Verbrauch melden
  ├─ Inventory anzeigen
  └─ Mission-Tracking
```

---

## 🔒 Sicherheits-Hinweise

### Für Production Deployment:

1. **Passwörter SOFORT ändern!**
   ```sql
   -- Neuen Hash generieren via /dev/hash Endpoint
   UPDATE users 
   SET password_hash = '<NEUER_ARGON2_HASH>' 
   WHERE username = 'staxadmin';
   ```

2. **`/dev/hash` Endpoint DEAKTIVIEREN!**
   - Entferne die Route in `mod.rs`
   - Oder nur in Development-Mode verfügbar machen

3. **System-Tenant schützen:**
   ```sql
   -- Verhindere versehentliches Löschen
   UPDATE tenants 
   SET status = 'active' 
   WHERE id = '00000000-0000-0000-0000-000000000000';
   ```

4. **SSL/TLS aktivieren:**
   - Alle API-Calls über HTTPS
   - JWT nur über sichere Verbindungen

5. **Rate Limiting implementieren:**
   - Max. 5 Login-Versuche pro Minute
   - Account-Lockout nach 10 Fehlversuchen

6. **Token-Rotation:**
   - Refresh-Token Mechanismus
   - Kurze Access-Token Lebensdauer (aktuell: 1h)

---

## 📝 Passwort-Hash generieren (Development)

```bash
# Backend muss laufen
curl -X POST http://localhost:4200/api/v1/auth/dev/hash \
  -H "Content-Type: application/json" \
  -d '{"password":"dein_neues_passwort"}'
```

Antwort:
```json
{
  "hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
}
```

---

## 🧪 Testing

### Login testen:
```bash
# StaxAdmin Login
curl -X POST http://localhost:4200/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"staxadmin","password":"staxadmin123"}'

# Tenant Admin Login
curl -X POST http://localhost:4200/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin@testwache.de","password":"admin123"}'
```

### Registrierung testen:
```bash
curl -X POST http://localhost:4200/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test@example.com","password":"test123456"}'
```

---

## 📧 Support

Bei Fragen zur Authentifizierung oder Rollen-Verwaltung:
- Email: dev@staxmed.de
- Docs: https://docs.staxmed.de

---

**Letzte Aktualisierung:** 2026-01-24
**Version:** 1.0.0