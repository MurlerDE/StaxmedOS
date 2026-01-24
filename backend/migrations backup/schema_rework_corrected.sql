-- Erweiterungen sicherstellen (für UUIDs)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 1. SEQUENZEN & FUNKTIONEN
CREATE SEQUENCE IF NOT EXISTS tenant_customer_num_seq START 1000;

CREATE OR REPLACE FUNCTION generate_stax_customer_number()
RETURNS TEXT AS $$
BEGIN
    RETURN 'STX-' || to_char(current_date, 'YY') || '-' || nextval('tenant_customer_num_seq');
END;
$$ LANGUAGE plpgsql;

-- 2. TABELLEN

CREATE TABLE "plans" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "name" varchar NOT NULL,
  "max_users" int DEFAULT 5,
  "created_at" timestamptz DEFAULT now()
);

CREATE TABLE "tenants" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "customer_number" varchar UNIQUE DEFAULT generate_stax_customer_number(),
  "name" varchar NOT NULL,
  "license_key" text UNIQUE,
  "valid_until" timestamptz,
  "is_on_premise" boolean DEFAULT false,
  "hardware_hash" varchar,
  "plan_id" uuid,
  "status" varchar DEFAULT 'trial',
  "created_at" timestamptz DEFAULT now(),
  CONSTRAINT check_tenant_status CHECK (status IN ('trial', 'active', 'suspended', 'canceled'))
);

CREATE TABLE "users" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "username" varchar UNIQUE NOT NULL,
  "password_hash" text NOT NULL,
  "first_name" varchar,
  "last_name" varchar,
  "role" varchar DEFAULT 'paramedic',
  "is_active" boolean DEFAULT true,
  "created_at" timestamptz DEFAULT now(),
  CONSTRAINT check_user_role CHECK (role IN ('admin', 'logistic_manager', 'paramedic'))
);

CREATE TABLE "global_products" (
  "udi_di" varchar PRIMARY KEY,
  "brand_name_en" text,
  "brand_name_de" text,
  "manufacturer" text,
  "category" varchar,
  "is_verified" boolean DEFAULT false,
  "created_at" timestamptz DEFAULT now()
);

CREATE TABLE "locations" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "parent_id" uuid,
  "name" varchar NOT NULL,
  "location_type" varchar NOT NULL,
  "created_at" timestamptz DEFAULT now(),
  CONSTRAINT check_location_type CHECK (location_type IN ('central', 'station', 'vehicle', 'shelf'))
);

CREATE TABLE "stock_targets" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "location_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "month_start" int CHECK (month_start BETWEEN 1 AND 12),
  "min_stock" int DEFAULT 0,
  "target_stock" int DEFAULT 0,
  "max_stock" int DEFAULT 0,
  "created_at" timestamptz DEFAULT now()
);

CREATE TABLE "tenant_product_settings" (
  "tenant_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "custom_alias" varchar,
  "custom_category" varchar,
  "min_stock_global" int,
  "created_at" timestamptz DEFAULT now(),
  PRIMARY KEY ("tenant_id", "udi_di")
);

CREATE TABLE "inventory_items" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "location_id" uuid NOT NULL,
  "batch_number" varchar,
  "expiry_date" date,
  "serial_number" varchar,
  "quantity" int DEFAULT 1,
  "status" varchar DEFAULT 'available',
  "internal_label_id" varchar,
  "scanned_at" timestamptz DEFAULT now(),
  "scanned_by" uuid,
  CONSTRAINT check_inventory_status CHECK (status IN ('available', 'used', 'expired', 'quarantined')),
  CONSTRAINT check_quantity_positive CHECK (quantity >= 0)
);

CREATE TABLE "shipments" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "from_location_id" uuid,
  "to_location_id" uuid,
  "status" varchar DEFAULT 'preparing',
  "created_at" timestamptz DEFAULT now(),
  "created_by" uuid,
  "received_at" timestamptz,
  "received_by" uuid,
  CONSTRAINT check_shipment_status CHECK (status IN ('preparing', 'shipped', 'partially_received', 'completed', 'discrepancy'))
);

CREATE TABLE "shipment_items" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "shipment_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "batch_number" varchar,
  "expiry_date" date,
  "expected_quantity" int NOT NULL,
  "received_quantity" int DEFAULT 0,
  "created_at" timestamptz DEFAULT now(),
  CONSTRAINT check_expected_qty_positive CHECK (expected_quantity > 0),
  CONSTRAINT check_received_qty_not_negative CHECK (received_quantity >= 0)
);

CREATE TABLE "shipment_discrepancies" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "shipment_id" uuid NOT NULL,
  "shipment_item_id" uuid,
  "type" varchar NOT NULL,
  "quantity_difference" int,
  "status" varchar DEFAULT 'open',
  "comment" text,
  "resolution_note" text,
  "reported_at" timestamptz DEFAULT now(),
  "reported_by" uuid,
  "resolved_at" timestamptz,
  "resolved_by" uuid,
  CONSTRAINT check_discrepancy_type CHECK (type IN ('missing', 'damaged', 'wrong_article', 'expired_on_arrival', 'excess')),
  CONSTRAINT check_discrepancy_status CHECK (status IN ('open', 'investigating', 'resolved', 'written_off'))
);

CREATE TABLE "stock_movements" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "item_id" uuid NOT NULL,
  "from_location" uuid,
  "to_location" uuid,
  "quantity" int NOT NULL,
  "moved_by" uuid,
  "moved_at" timestamptz DEFAULT now(),
  "reason" varchar,
  CONSTRAINT check_movement_reason CHECK (reason IN ('transfer', 'usage', 'correction', 'disposal', 'return'))
);

CREATE TABLE "medical_devices" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "inventory_item_id" uuid NOT NULL,
  "internal_inventory_number" varchar UNIQUE,
  "stk_interval_months" int,
  "mtk_interval_months" int,
  "last_maintenance_date" date,
  "next_maintenance_date" date,
  "manual_url" text,
  "is_active" boolean DEFAULT true,
  "created_at" timestamptz DEFAULT now()
);

CREATE TABLE "maintenance_logs" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "device_id" uuid NOT NULL,
  "check_type" varchar,
  "performer" varchar,
  "result" varchar,
  "protocol_url" text,
  "notes" text,
  "performed_at" timestamptz DEFAULT now(),
  CONSTRAINT check_maintenance_type CHECK (check_type IN ('STK', 'MTK', 'repair', 'calibration', 'training')),
  CONSTRAINT check_maintenance_result CHECK (result IN ('passed', 'defects', 'decommissioned', 'pending'))
);

CREATE TABLE "mission_usage" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "mission_number" varchar NOT NULL,
  "custom_fields" jsonb DEFAULT '{}',
  "vehicle_id" uuid,
  "user_id" uuid,
  "item_id" uuid,
  "consumed_at" timestamptz DEFAULT now()
);

CREATE TABLE "staxbox_licenses" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "hardware_id" varchar(100) UNIQUE NOT NULL,
  "tenant_id" uuid,
  "license_key" varchar(100) UNIQUE,
  "valid_until" timestamptz,
  "last_heartbeat" timestamptz,
  "is_active" boolean DEFAULT true,
  "created_at" timestamptz DEFAULT now()
);

-- 3. INDIZES (Für Performance bei Millionen Datensätzen)
CREATE INDEX idx_inv_items_tenant_udi ON inventory_items(tenant_id, udi_di);
CREATE INDEX idx_inv_items_location ON inventory_items(location_id);
CREATE INDEX idx_inv_items_expiry ON inventory_items(expiry_date);
CREATE INDEX idx_inv_items_status ON inventory_items(status) WHERE status != 'available';
CREATE INDEX idx_shipments_tenant_status ON shipments(tenant_id, status);
CREATE INDEX idx_shipments_created_at ON shipments(created_at DESC);
CREATE INDEX idx_movements_item ON stock_movements(item_id);
CREATE INDEX idx_movements_tenant ON stock_movements(tenant_id, moved_at DESC);
CREATE INDEX idx_global_products_brand ON global_products(brand_name_de);
CREATE INDEX idx_global_products_category ON global_products(category);
CREATE INDEX idx_staxbox_hardware ON staxbox_licenses(hardware_id);
CREATE INDEX idx_staxbox_tenant ON staxbox_licenses(tenant_id);
CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_locations_tenant ON locations(tenant_id);
CREATE INDEX idx_locations_parent ON locations(parent_id);
CREATE INDEX idx_stock_targets_location ON stock_targets(location_id);
CREATE INDEX idx_shipment_items_shipment ON shipment_items(shipment_id);
CREATE INDEX idx_discrepancies_shipment ON shipment_discrepancies(shipment_id);
CREATE INDEX idx_discrepancies_status ON shipment_discrepancies(status) WHERE status != 'resolved';
CREATE INDEX idx_maintenance_device ON maintenance_logs(device_id, performed_at DESC);
CREATE INDEX idx_mission_tenant ON mission_usage(tenant_id, consumed_at DESC);

-- UNIQUE-Index für Inventory Items (verhindert Duplikate bei identischen Chargen am selben Ort)
CREATE UNIQUE INDEX idx_inv_unique_item ON inventory_items(tenant_id, udi_di, location_id, COALESCE(batch_number, ''), COALESCE(expiry_date::text, ''))
WHERE serial_number IS NULL;

-- UNIQUE-Index für serielle Items
CREATE UNIQUE INDEX idx_inv_unique_serial ON inventory_items(tenant_id, udi_di, serial_number)
WHERE serial_number IS NOT NULL;

-- 4. FOREIGN KEYS (Mit Audit-Trail-Logik)

-- Plans
ALTER TABLE "tenants" ADD FOREIGN KEY ("plan_id") REFERENCES "plans" ("id") ON DELETE SET NULL;

-- Users
ALTER TABLE "users" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;

-- Locations
ALTER TABLE "locations" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "locations" ADD FOREIGN KEY ("parent_id") REFERENCES "locations" ("id") ON DELETE SET NULL;

-- Stock Targets
ALTER TABLE "stock_targets" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id") ON DELETE CASCADE;
ALTER TABLE "stock_targets" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di") ON DELETE CASCADE;

-- Tenant Product Settings
ALTER TABLE "tenant_product_settings" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "tenant_product_settings" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di") ON DELETE CASCADE;

-- Inventory Items
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di") ON DELETE RESTRICT;
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id") ON DELETE CASCADE;
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("scanned_by") REFERENCES "users" ("id") ON DELETE SET NULL;

-- Shipments
ALTER TABLE "shipments" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "shipments" ADD FOREIGN KEY ("from_location_id") REFERENCES "locations" ("id") ON DELETE SET NULL;
ALTER TABLE "shipments" ADD FOREIGN KEY ("to_location_id") REFERENCES "locations" ("id") ON DELETE SET NULL;
ALTER TABLE "shipments" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id") ON DELETE SET NULL;
ALTER TABLE "shipments" ADD FOREIGN KEY ("received_by") REFERENCES "users" ("id") ON DELETE SET NULL;

-- Shipment Items
ALTER TABLE "shipment_items" ADD FOREIGN KEY ("shipment_id") REFERENCES "shipments" ("id") ON DELETE CASCADE;
ALTER TABLE "shipment_items" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di") ON DELETE RESTRICT;

-- Shipment Discrepancies
ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("shipment_id") REFERENCES "shipments" ("id") ON DELETE CASCADE;
ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("shipment_item_id") REFERENCES "shipment_items" ("id") ON DELETE SET NULL;
ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("reported_by") REFERENCES "users" ("id") ON DELETE SET NULL;
ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("resolved_by") REFERENCES "users" ("id") ON DELETE SET NULL;

-- Stock Movements
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("item_id") REFERENCES "inventory_items" ("id") ON DELETE CASCADE;
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("from_location") REFERENCES "locations" ("id") ON DELETE SET NULL;
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("to_location") REFERENCES "locations" ("id") ON DELETE SET NULL;
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("moved_by") REFERENCES "users" ("id") ON DELETE SET NULL;

-- Medical Devices
ALTER TABLE "medical_devices" ADD FOREIGN KEY ("inventory_item_id") REFERENCES "inventory_items" ("id") ON DELETE CASCADE;

-- Maintenance Logs
ALTER TABLE "maintenance_logs" ADD FOREIGN KEY ("device_id") REFERENCES "medical_devices" ("id") ON DELETE CASCADE;

-- Mission Usage
ALTER TABLE "mission_usage" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "mission_usage" ADD FOREIGN KEY ("vehicle_id") REFERENCES "locations" ("id") ON DELETE SET NULL;
ALTER TABLE "mission_usage" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE SET NULL;
ALTER TABLE "mission_usage" ADD FOREIGN KEY ("item_id") REFERENCES "inventory_items" ("id") ON DELETE SET NULL;

-- StaxBox Licenses
ALTER TABLE "staxbox_licenses" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE SET NULL;

-- 5. COMMENTS

COMMENT ON TABLE "tenants" IS 'Zentrale Mandantentabelle. Jede Anfrage im Backend muss gegen eine tenant_id gefiltert werden.';
COMMENT ON COLUMN "tenants"."name" IS 'Name der Rettungswache / Organisation';
COMMENT ON COLUMN "tenants"."license_key" IS 'Verschlüsselter Schlüssel für Phone-Home Validierung';
COMMENT ON COLUMN "tenants"."valid_until" IS 'Ablaufdatum der Lizenz';
COMMENT ON COLUMN "tenants"."is_on_premise" IS 'True, wenn Server lokal beim Kunden läuft';
COMMENT ON COLUMN "tenants"."hardware_hash" IS 'Eindeutiger Fingerabdruck des Zielservers für Kopierschutz';
COMMENT ON COLUMN "tenants"."plan_id" IS 'Verknüpfung zum gewählten Preismodell';
COMMENT ON COLUMN "tenants"."status" IS 'active, trial, suspended, canceled';

COMMENT ON TABLE "plans" IS 'Definiert die Leistungsklassen deines SaaS-Produkts.';
COMMENT ON COLUMN "plans"."name" IS 'z.B. Basic, Pro, Enterprise';
COMMENT ON COLUMN "plans"."max_users" IS 'Limit für Benutzeraccounts';

COMMENT ON TABLE "users" IS 'Benutzerverwaltung pro Mandant.';
COMMENT ON COLUMN "users"."password_hash" IS 'Argon2id oder bcrypt Hash';
COMMENT ON COLUMN "users"."role" IS 'admin, logistic_manager, paramedic (Sanitäter)';

COMMENT ON TABLE "global_products" IS 'Zentraler Artikelstamm. Wird bei neuen Scans via API (GUDID) befüllt.';
COMMENT ON COLUMN "global_products"."udi_di" IS 'GTIN / Device Identifier (GS1 Standard)';
COMMENT ON COLUMN "global_products"."brand_name_en" IS 'Originalbezeichnung aus FDA/EUDAMED';
COMMENT ON COLUMN "global_products"."brand_name_de" IS 'Durch LLM/KI übersetzte deutsche Bezeichnung';
COMMENT ON COLUMN "global_products"."manufacturer" IS 'Herstellername';
COMMENT ON COLUMN "global_products"."category" IS 'Verschlagwortung für Filter (z.B. Infusion, Beatmung)';
COMMENT ON COLUMN "global_products"."is_verified" IS 'Wurde der Datensatz manuell geprüft?';

COMMENT ON TABLE "locations" IS 'Abbildung der physischen Struktur der Rettungswache.';
COMMENT ON COLUMN "locations"."parent_id" IS 'Ermöglicht Hierarchie: Wache > Fahrzeug > Fach';
COMMENT ON COLUMN "locations"."name" IS 'z.B. Zentrallager, RTW-1, Schrank A';
COMMENT ON COLUMN "locations"."location_type" IS 'central, station, vehicle, shelf';

COMMENT ON TABLE "stock_targets" IS 'Steuerung der Saisonalität. Ein Artikel kann pro Monat unterschiedliche Ziele haben.';
COMMENT ON COLUMN "stock_targets"."month_start" IS 'Monatszahl (1-12) für saisonale Planung';
COMMENT ON COLUMN "stock_targets"."min_stock" IS 'Bestand, bei dem Warnung erfolgt';
COMMENT ON COLUMN "stock_targets"."target_stock" IS 'Soll-Bestand für automatische Nachfüllliste';
COMMENT ON COLUMN "stock_targets"."max_stock" IS 'Kapazitätsgrenze des Lagerortes';

COMMENT ON TABLE "tenant_product_settings" IS 'Hier werden globale Produkte für den spezifischen Mandanten "personalisiert".';
COMMENT ON COLUMN "tenant_product_settings"."custom_alias" IS 'Lokaler Name, z.B. Braunüle';
COMMENT ON COLUMN "tenant_product_settings"."custom_category" IS 'Eigene Kategorisierung, falls die globale nicht passt';
COMMENT ON COLUMN "tenant_product_settings"."min_stock_global" IS 'Optional: Globaler Mindestbestand für alle Lagerorte dieses Mandanten';

COMMENT ON TABLE "inventory_items" IS 'Ein Datensatz gruppiert identische Artikel (gleiche Charge/Verfall) an einem Ort.';
COMMENT ON COLUMN "inventory_items"."batch_number" IS 'Charge (AI 10)';
COMMENT ON COLUMN "inventory_items"."expiry_date" IS 'Verfall (AI 17)';
COMMENT ON COLUMN "inventory_items"."serial_number" IS 'Nur bei Einzelgeräten gefüllt';
COMMENT ON COLUMN "inventory_items"."quantity" IS 'Aktuelle Menge dieser Charge an diesem Ort';
COMMENT ON COLUMN "inventory_items"."status" IS 'available, used, expired, quarantined';
COMMENT ON COLUMN "inventory_items"."internal_label_id" IS 'Für QR-Codes bei Artikeln ohne GS1 Matrix';

COMMENT ON TABLE "shipments" IS 'Klammert mehrere Artikel zu einer Lieferung zusammen.';
COMMENT ON COLUMN "shipments"."status" IS 'preparing, shipped, partially_received, completed, discrepancy';
COMMENT ON COLUMN "shipments"."created_by" IS 'Der Lagerist';
COMMENT ON COLUMN "shipments"."received_by" IS 'Der Sanitäter auf der Wache';

COMMENT ON TABLE "shipment_items" IS 'Ermöglicht den Soll-Ist-Abgleich beim Empfang.';
COMMENT ON COLUMN "shipment_items"."expected_quantity" IS 'Menge laut Lagerist';
COMMENT ON COLUMN "shipment_items"."received_quantity" IS 'Tatsächlich gescannte Menge auf der Wache';

COMMENT ON TABLE "shipment_discrepancies" IS 'Zentraler Ort für Logistikfehler. Muss vom Admin oder Lagerleiter abgearbeitet werden.';
COMMENT ON COLUMN "shipment_discrepancies"."type" IS 'missing, damaged, wrong_article, expired_on_arrival, excess';
COMMENT ON COLUMN "shipment_discrepancies"."status" IS 'open, investigating, resolved, written_off';
COMMENT ON COLUMN "shipment_discrepancies"."comment" IS 'z.B. Karton war nass oder Packung fehlte';
COMMENT ON COLUMN "shipment_discrepancies"."resolution_note" IS 'z.B. Nachlieferung veranlasst oder Bestand korrigiert';

COMMENT ON TABLE "stock_movements" IS 'Audit-Trail für alle Bestandsbewegungen.';
COMMENT ON COLUMN "stock_movements"."reason" IS 'transfer, usage, correction, disposal, return';

COMMENT ON TABLE "medical_devices" IS 'Zusatzdaten für langlebige Medizingeräte (Assets).';
COMMENT ON COLUMN "medical_devices"."inventory_item_id" IS 'Link zum physischen Item';
COMMENT ON COLUMN "medical_devices"."internal_inventory_number" IS 'Deine interne Inventarnummer';
COMMENT ON COLUMN "medical_devices"."stk_interval_months" IS 'Sicherheitstechnische Kontrolle';
COMMENT ON COLUMN "medical_devices"."mtk_interval_months" IS 'Medizintechnische Kontrolle';
COMMENT ON COLUMN "medical_devices"."manual_url" IS 'Link zur digitalen Gebrauchsanweisung';
COMMENT ON COLUMN "medical_devices"."is_active" IS 'Gerät einsatzbereit oder gesperrt?';

COMMENT ON TABLE "maintenance_logs" IS 'Prüf- und Wartungshistorie für Medizingeräte.';
COMMENT ON COLUMN "maintenance_logs"."check_type" IS 'STK, MTK, repair, calibration, training';
COMMENT ON COLUMN "maintenance_logs"."performer" IS 'Wer hat die Prüfung durchgeführt (z.B. externer Dienstleister)';
COMMENT ON COLUMN "maintenance_logs"."result" IS 'passed, defects, decommissioned, pending';
COMMENT ON COLUMN "maintenance_logs"."protocol_url" IS 'Link zum PDF-Prüfbericht';

COMMENT ON TABLE "mission_usage" IS 'Verknüpft Lagerabgänge mit Einsätzen (MPG-Konformität).';
COMMENT ON COLUMN "mission_usage"."mission_number" IS 'Einsatznummer für MPG-Rückverfolgbarkeit';
COMMENT ON COLUMN "mission_usage"."custom_fields" IS 'Benutzerdefinierte Felder (JSONB)';
COMMENT ON COLUMN "mission_usage"."vehicle_id" IS 'ID des Fahrzeugs (Location vom Typ vehicle)';
COMMENT ON COLUMN "mission_usage"."user_id" IS 'Sanitäter, der den Artikel entnommen hat';
COMMENT ON COLUMN "mission_usage"."item_id" IS 'Das spezifische verbrauchte Item';

COMMENT ON TABLE "staxbox_licenses" IS 'Lizenzverwaltung für StaxBox-Hardware (Raspberry Pi basierte Scan-Terminals).';
COMMENT ON COLUMN "staxbox_licenses"."hardware_id" IS 'Eindeutige Hardware-ID (z.B. CPU-Seriennummer)';
COMMENT ON COLUMN "staxbox_licenses"."last_heartbeat" IS 'Letzter Verbindungscheck für Online-Monitoring';
