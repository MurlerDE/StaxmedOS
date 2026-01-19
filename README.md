StaxmedOS
StaxmedOS ist ein hochmodulares, mandantenfähiges Betriebssystem für die medizinische Logistik und Bestandsverwaltung. Entwickelt für Rettungsdienste (EMS), Arztpraxen und klinische Einrichtungen, kombiniert es die Sicherheit von Rust mit der Flexibilität von Angular.

🚀 Vision
Medizinische Logistik leidet oft unter veralteter Software. StaxmedOS löst dieses Problem durch:
- Modularen Aufbau: Nur die Funktionen nutzen, die wirklich gebraucht werden (z. B. Rettungsdienst-Einsatzerfassung vs. Praxis-Bestandsmanagement).
- Typsicherheit: End-to-End Typsicherheit von der Datenbank bis zum Frontend.
- Offline-First: Dank PWA-Technologie einsatzbereit in Umgebungen mit schlechter Konnektivität (z. B. im ländlichen Rettungsdienst).
- Compliance: Fokus auf MPG/MDR-Konformität durch lückenlose Audit-Trails und Chargen-Tracking.

🛠 Tech-Stack
Ebene	Technologie	Rolle
Backend	Rust (Axum)	Hochperformante, sichere API & Geschäftslogik.
Frontend	Angular (PWA)	Reaktives UI mit Signals & modularer Library-Struktur.
Datenbank	PostgreSQL	Relationales Herzstück mit JSONB-Flexibilität.
Type-Sharing	ts-rs	Automatische Generierung von TS-Interfaces aus Rust-Structs.
Infrastruktur	Proxmox LXC	Ressourceneffizientes Hosting der Datenbank.

📁 Projektstruktur (Manual Monorepo)
Das Projekt ist als manuelles Monorepo organisiert, um maximale Kontrolle ohne Overhead zu gewährleisten.
```
StaxmedOS/
├── backend/
│   ├── apps/
│   │   └── api/                # Das Axum API-Gateway
│   ├── libs/
│   │   ├── shared_types/       # Single Source of Truth für Datenmodelle
│   │   ├── auth/               # JWT & Benutzerverwaltung
│   │   └── inventory/          # Geschäftslogik für StaxInventory
├── frontend/
│   ├── src/app/
│   │   ├── generated/          # Automatisch generierte TS-Interfaces
│   │   ├── core/               # Layout, Auth-Guards, Sidebar-Logik
│   │   └── modules/            # Lazy-loaded Feature-Module
├── migrations/                 # SQLx Datenbank-Migrationen
└── Cargo.toml                  # Cargo Workspace Definition
```

🔑 Kern-Features
1. Mandantenfähigkeit (Multi-Tenancy)
Jeder Datensatz ist strikt einer tenant_id zugeordnet. Die Trennung erfolgt auf Datenbankebene und wird durch eine Rust-Middleware in der API abgesichert.

2. End-to-End Typsicherheit
Durch die Library shared_types im Backend werden Änderungen an Datenstrukturen sofort in das Frontend reflektiert:

Änderung im Rust-Struct.

cargo test generiert neue .ts-Interfaces.

Angular-Compiler meldet Fehler bei Inkompatibilität.

3. Dynamisches Modul-Gating
Das System erkennt beim Login, welche Module (z. B. StaxInventory, StaxMission) für den Mandanten freigeschaltet sind und passt das UI sowie die API-Berechtigungen dynamisch an.

🛠 Installation & Entwicklung
Voraussetzungen
Rust (Stable) & Cargo

Node.js & Angular CLI

PostgreSQL (z. B. im Proxmox LXC)

sqlx-cli für Datenbank-Migrationen

Setup
Repository klonen:

Bash
git clone https://github.com/MurlerDE/StaxmedOS.git
cd StaxmedOS
Datenbank vorbereiten: Erstelle eine .env Datei im Root:

Code-Snippet
DATABASE_URL=postgres://user:password@lxc-ip:5432/staxmed
Migrationen ausführen:

Bash
sqlx migrate run
Interfaces generieren:

Bash
cargo test -p shared_types
Backend starten:

Bash
cargo run -p api
Frontend starten:

Bash
cd frontend
npm install
ng serve

🗺 Roadmap
[ ] Phase 1: Core-System (Auth, Tenants, Basic Layout).

[ ] Phase 2: StaxInventory (UDI-Scanning, Chargen-Tracking).

[ ] Phase 3: StaxMission (Einsatzerfassung & MPG-Checklisten).

[ ] Phase 4: Mandanten-Admin-Panel für individuelle Feld-Konfiguration.

📄 Lizenz
Dieses Projekt ist für die interne Nutzung/Entwicklung durch MurlerDE bestimmt.
