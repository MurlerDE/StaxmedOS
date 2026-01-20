```
StaxmedOS/
├── .zed/                        # Deine Editor-Konfiguration
├── .env                         # Zentrale Schaltstelle (Secrets, DB-URL, STAX_MODE)
│
├── backend/                     # Rust Workspace
│   ├── Cargo.toml               # Definiert Workspace (apps & libs)
│   ├── migrations/              # SQL-Migrationen (Initial, Auth, StaxBox)
│   ├── apps/
│   │   └── api/                 # Das Haupt-Backend (Axum)
│   │       ├── src/
│   │       │   ├── main.rs      # Einstiegspunkt & Router-Versionierung (/v1)
│   │       │   ├── config.rs    # Lädt STAX_MODE & Secrets aus .env
│   │       │   ├── license/     # StaxBox-Logik (worker.rs für Heartbeat)
│   │       │   └── modules/     # API-Endpunkte nach Features getrennt
│   │       │       ├── auth/    # Login & Token-Ausstellung
│   │       │       ├── inventory/ # Lager-Logik & DB-Abfragen
│   │       │       └── system/  # SaaS-Master Logik (Empfang Heartbeat)
│   ├── libs/
│   │   └── shared_types/        # Gemeinsame Rust-Structs (Serde & TS-RS)
│   │       └── src/
│   │           ├── lib.rs       # Re-exports
│   │           ├── auth.rs      # Claims, Login-Request
│   │           └── inventory.rs # Product, Location, Stock-Level
│
├── frontend/                    # Angular App (v17/18+)
│   ├── angular.json             # Proxy-Konfiguration hier verlinken
│   ├── proxy.conf.json          # Löst CORS-Probleme im Dev-Modus (/api -> localhost:3000)
│   ├── src/
│   │   ├── environments/        # SaaS vs. StaxBox Config
│   │   │   ├── environment.prod.ts
│   │   │   └── environment.pi.ts
│   │   └── app/
│   │       ├── core/            # Globale "Singletons"
│   │       │   ├── auth/        # AuthService, Interceptor, Guard
│   │       │   ├── services/    # ConfigService (SaaS/Pi), ApiService
│   │       │   ├── models/      # Die von Rust generierten TS-Interfaces
│   │       │   └── interceptors/# Hängt JWT an jeden Request
│   │       │
│   │       ├── shared/          # UI-Baukasten (Das "Krimskrams")
│   │       │   ├── components/  # Buttons, Inputs, Modals
│   │       │   └── directives/  # z.B. *ifPermission
│   │       │
│   │       ├── layout/          # Layout-Hüllen (Shells)
│   │       │   ├── app-layout/  # Layout MIT Sidebar & Header (Privat)
│   │       │   └── public-layout/# Layout NUR Content & Footer (Landingpage)
│   │       │
│   │       ├── features/        # Die Business-Domänen
│   │       │   ├── landing-page/# Marketing-Ansicht (Public)
│   │       │   ├── auth/        # Login-View & Logout
│   │       │   ├── stax-admin/  # Nur für dich (Tenant-Management, Cloud Only)
│   │       │   └── inventory/   # DAS LAGER MODUL (Adaptive)
│   │       │       ├── inventory.component.ts # Die adaptive Weiche
│   │       │       ├── inventory-routing.ts
│   │       │       ├── services/    # Modul-spezifische API-Calls
│   │       │       └── views/       # Spezialisierte UI pro Gerät
│   │       │           ├── inv-mobile/  # Scan & Go
│   │       │           ├── inv-tablet/  # Inventur-Modus
│   │       │           └── inv-desktop/ # Stammdaten & Analyse
│   │       └── app.routes.ts    # Haupt-Routing (Lazy Loading der Features)
│
└── docker-compose.yml           # (Zukünftig) Startet DB, Backend, Frontend
```
