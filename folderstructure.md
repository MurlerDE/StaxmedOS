# Projekt-Ordnerstruktur (aktualisiert)

Nach dem Refactoring spiegelt diese Datei die aktuelle, tatsächlich vorhandene Ordnerstruktur und Konventionen wider. Nutze sie als Referenz für neues Feature-Scaffolding und Code-Placement.

StaxmedOS/
├── .github/                     # CI-Workflows, Issue/PR-Templates
├── .zed/                        # Editor- / Dev-Configs (Zed-specific)
├── .app/                        # Projekt-bezogene Notizen / Status (Stand_...)
├── .env.example                 # Beispiel-Env (DB-URL, Secrets, STAX_MODE)
├── scripts/                     # Hilfs-Skripte (generate types, gen-feature)
│   ├── generate-ts-interfaces.sh
│   └── gen-feature.sh
├── docker-compose.dev.yml       # (optional) lokale Dev-Compose
├── Makefile                     # Lokale Convenience-Targets (optional)
├── README.md
├── folderstructure.md           # Diese Datei

├── backend/                     # Rust workspace (apps & libs)
│   ├── Cargo.toml               # Workspace definition
│   ├── migrations/              # sqlx Migrations
│   ├── apps/
│   │   └── api/                 # Haupt-API (Axum)
│   │       ├── Cargo.toml
│   │       ├── Dockerfile
│   │       └── src/
│   │           ├── main.rs      # Einstiegspunkt & Router
│   │           ├── config.rs    # Env + Config-Laden
│   │           ├── modules/     # Feature-getrennte API-Module (auth, inventory, system...)
│   │           │   ├── auth/
│   │           │   ├── inventory/
│   │           │   └── system/
│   │           └── tracing/     # opentelemetry / tracing Init (optional)
│   └── libs/
│       └── shared_types/        # Gemeinsame Rust-Structs (Serde & ts-rs)
│           ├── Cargo.toml
│           └── src/
│               ├── lib.rs       # Re-exports
│               ├── auth.rs      # Claims, Login-Request
│               └── inventory.rs # Product, Location, Stock-Level

├── frontend/                    # Angular App (Standalone Components, Signals)
│   ├── angular.json
│   ├── package.json
│   ├── proxy.conf.json          # Dev-Proxy (/api -> localhost:3000)
│   ├── README.md
│   └── src/
│       ├── environments/        # environment.prod.ts, environment.pi.ts
│       └── app/
│           ├── app.routes.ts    # Root routes (lazy-loaded features)
│           ├── core/            # Globale Singletons & Services
│           │   ├── services/    # ApiService, ConfigService
│           │   ├── auth/        # AuthService, Interceptor, Guards
│           │   ├── models/      # Von Rust generierte TS-Interfaces (shared-types)
│           │   └── interceptors/# HTTP-Interceptors
│           ├── layout/          # Layout Shells
│           │   ├── app-layout/  # Authenticated app-shell (Sidebar, Header)
│           │   └── public-layout/# Landingpage / public pages
│           ├── shared/          # UI-Baukasten (Buttons, Inputs, Modals, Directives)
│           │   ├── components/
│           │   └── directives/
│           └── features/        # Business-Domains (strict per-feature layout)
│               ├── {feature-name}/
│               │   ├── components/   # Feature-shared components
│               │   ├── services/     # Feature-API / local logic
│               │   ├── views/        # Adaptive views (mobile/tablet/desktop)
│               │   │   ├── mobile/
│               │   │   ├── tablet/
│               │   │   └── desktop/
│               │   ├── {feature-name}.component.ts
│               │   └── {feature-name}-routing.ts
│               ├── landing-page/
│               ├── dashboard/
│               ├── auth/
│               ├── stax-admin/
│               ├── inventory/
│               ├── compliance/
│               ├── planner/
│               └── skills/

└── docs/ (optional)             # Compliance, Architektur, Prozesse


Konventionen & Hinweise
- Feature-Layout: Jeder Feature-Ordner muss die Unterordner `components/`, `services/` und `views/{mobile,tablet,desktop}` enthalten.
- Type-Sharing: `backend/libs/shared_types` ist die Source-of-Truth. TS-Interfaces werden via `ts-rs` generiert und liegen unter `frontend/src/app/core/models`.
- Generierung: Nutze `scripts/generate-ts-interfaces.sh` (oder `make generate-types`) vor dem Commit, falls du Rust-Structs änderst. CI prüft dies automatisch.
- Routing: Haupt-Routing in `frontend/src/app/app.routes.ts` verwendet lazy-loading per Feature.
- DB-Migrationen: Alle sqlx-Migrations liegen unter `backend/migrations` und werden per `sqlx migrate run` angewendet.

Empfohlene Dateien, falls noch nicht vorhanden
- `.env.example` mit DB-URL und minimalen Variablen
- `CONTRIBUTING.md`, `ISSUE_TEMPLATE.md`, `PULL_REQUEST_TEMPLATE.md`
- `.github/workflows/ci.yml` mit Rust/Frontend-Checks