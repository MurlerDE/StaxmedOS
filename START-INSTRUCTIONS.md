# StaxmedOS - Start-Anleitung

## 🚀 Schnellstart

### 1. Backend starten

**Option A: Mit Batch-Datei (Windows)**
```bash
start-backend.bat
```

**Option B: Manuell**
```bash
cd backend
cargo run
```

Das Backend sollte nach dem Kompilieren auf `http://127.0.0.1:3000` laufen.

### 2. Frontend starten

In einem **neuen Terminal**:
```bash
cd frontend
npm start
# oder
ng serve
```

Das Frontend läuft auf `http://localhost:4200`

### 3. Backend testen

```bash
test-backend.bat
```

Oder manuell:
```bash
curl http://localhost:3000/api/v1/healthy
```

Erwartete Ausgabe: `StaxmedOS API v1 - Online`

---

## 🔧 Letzte Fixes (24.01.2026)

### Backend-Änderungen
1. **Pfadparameter-Syntax** aktualisiert (`:param` → `{param}`) für Axum v0.7
2. **SQL-Typen korrigiert** in `users/mod.rs`:
   - `PermissionInfo`: `permission_name` und `source` sind jetzt NOT NULL
   - `UserRoleInfo`: `role_name` und `display_name` sind jetzt NOT NULL
3. **StaxAdmin Permissions** erweitert: StaxAdmin hat jetzt ALLE Permissions (nicht nur `system.*`)

### Frontend-Änderungen
1. **Route hinzugefügt**: `/tenant-admin` für Benutzerverwaltung innerhalb eines Tenants
   - `/tenant-admin/users` - Benutzerverwaltung
   - `/tenant-admin/roles` - Rollenverwaltung
   - `/tenant-admin/settings` - Einstellungen

---

## 📋 Bekannte Benutzer

### StaxAdmin (System-Administrator)
- **Username**: `stax`
- **Passwort**: In der Datenbank gespeichert
- **Zugriff**: Voller System-Zugriff, kann alle Tenants sehen

### Tenant-Admin
- **Zugriff**: Verwaltung von Benutzern und Rollen innerhalb des eigenen Tenants
- Route: `/tenant-admin`

---

## 🐛 Fehlerbehebung

### "ECONNREFUSED" Fehler
- **Problem**: Backend läuft nicht
- **Lösung**: Backend starten mit `start-backend.bat`

### "401 Unauthorized" Fehler
- **Problem**: Token ist abgelaufen
- **Lösung**: Neu anmelden (zur Landing Page `/` navigieren)

### "500 Internal Server Error" beim User-Abruf
- **Problem**: Backend läuft mit alter Version
- **Lösung**: 
  1. Backend stoppen: `taskkill //F //IM backend.exe`
  2. Neu kompilieren: `cd backend && cargo build`
  3. Neu starten: `cargo run`

### "403 Forbidden" als Admin
- **Problem**: Fehlende Permissions in der Datenbank
- **Lösung**: Migration ausführen:
  ```bash
  cd backend
  sqlx migrate run --database-url "postgres://staxadmin:Se8%5EG8s7zP%247TkE@172.16.181.190:5432/staxmed_db"
  ```

---

## 🗄️ Datenbank-Verbindung

Die Datenbank-Konfiguration befindet sich in `backend/.env`:
```
DATABASE_URL=postgres://staxadmin:Se8%5EG8s7zP%247TkE@172.16.181.190:5432/staxmed_db
```

### Migrationen ausführen
```bash
cd backend
sqlx migrate run
```

### Migrationen Status prüfen
```bash
cd backend
sqlx migrate info
```

---

## 📦 API-Endpunkte

Alle API-Endpunkte sind unter `/api/v1` verfügbar:

### Authentifizierung
- `POST /api/v1/auth/login` - Anmelden
- `POST /api/v1/auth/register` - Registrieren
- `GET /api/v1/auth/me` - Aktuellen Benutzer abrufen

### Benutzerverwaltung
- `GET /api/v1/users` - Alle Benutzer auflisten
- `POST /api/v1/users` - Neuen Benutzer erstellen
- `GET /api/v1/users/{user_id}` - Benutzerdetails abrufen
- `PUT /api/v1/users/{user_id}` - Benutzer aktualisieren
- `DELETE /api/v1/users/{user_id}` - Benutzer löschen
- `PUT /api/v1/users/{user_id}/password` - Passwort ändern
- `GET /api/v1/users/{user_id}/roles` - Benutzerrollen abrufen
- `POST /api/v1/users/{user_id}/roles` - Rolle zuweisen
- `DELETE /api/v1/users/{user_id}/roles/{role_id}` - Rolle entfernen
- `GET /api/v1/users/roles/available` - Verfügbare Rollen abrufen

### Inventory
- `GET /api/v1/inventory` - Inventar auflisten

### System
- `GET /api/v1/healthy` - Health Check

---

## 🔐 Berechtigungssystem

Das System verwendet ein RBAC (Role-Based Access Control) System:

### Rollen
- **staxadmin** - System-Administrator (alle Permissions)
- **admin** - Tenant-Administrator (alle Tenant-Permissions)
- **logistic_manager** - Logistik-Manager (Inventory, Shipments, Locations)
- **paramedic** - Sanitäter (Basis-Zugriff)

### Permissions
Format: `resource.action.scope`
- Beispiele: `users.view`, `users.create`, `inventory.manage`, `system.tenants.view`

---

## 📝 Entwickler-Notizen

### Code-Änderungen kompilieren
```bash
cd backend
cargo build
```

### Frontend neu bauen
```bash
cd frontend
npm run build
```

### Tests ausführen
```bash
cd backend
cargo test
```

---

Bei Fragen oder Problemen, siehe die detaillierten Logs in der Konsole.