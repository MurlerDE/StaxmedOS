-- Add migration script here
CREATE TABLE "tenants" (
  "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
  "name" varchar NOT NULL,
  "license_key" text UNIQUE,
  "valid_until" timestamp,
  "is_on_premise" boolean DEFAULT false,
  "hardware_hash" varchar,
  "plan_id" uuid,
  "status" varchar
);

CREATE TABLE "plans" (
  "id" UUID PRIMARY KEY,
  "name" varchar,
  "max_users" int
);

CREATE TABLE "users" (
  "id" UUID PRIMARY KEY,
  "tenant_id" uuid,
  "username" varchar UNIQUE,
  "password_hash" text,
  "first_name" varchar,
  "last_name" varchar,
  "role" varchar,
  "is_active" boolean DEFAULT true
);

CREATE TABLE "global_products" (
  "udi_di" varchar PRIMARY KEY,
  "brand_name_en" text,
  "brand_name_de" text,
  "manufacturer" text,
  "category" varchar,
  "is_verified" boolean DEFAULT false
);

CREATE TABLE "locations" (
  "id" UUID PRIMARY KEY,
  "tenant_id" uuid,
  "parent_id" uuid,
  "name" varchar,
  "location_type" varchar
);

CREATE TABLE "stock_targets" (
  "id" UUID PRIMARY KEY,
  "location_id" uuid,
  "udi_di" varchar,
  "month_start" int,
  "min_stock" int,
  "target_stock" int,
  "max_stock" int
);

CREATE TABLE "tenant_product_settings" (
  "tenant_id" uuid,
  "udi_di" varchar,
  "custom_alias" varchar,
  "custom_category" varchar,
  "min_stock_global" int,
  PRIMARY KEY ("tenant_id", "udi_di")
);

CREATE TABLE "inventory_items" (
  "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
  "tenant_id" uuid,
  "udi_di" varchar,
  "location_id" uuid,
  "batch_number" varchar,
  "expiry_date" date,
  "serial_number" varchar,
  "quantity" int DEFAULT 1,
  "status" varchar DEFAULT 'available',
  "internal_label_id" varchar,
  "scanned_at" timestamp DEFAULT (now()),
  "scanned_by" uuid
);

CREATE TABLE "shipments" (
  "id" UUID PRIMARY KEY,
  "tenant_id" uuid,
  "from_location_id" uuid,
  "to_location_id" uuid,
  "status" varchar,
  "created_at" timestamp DEFAULT (now()),
  "created_by" uuid,
  "received_at" timestamp,
  "received_by" uuid
);

CREATE TABLE "shipment_items" (
  "id" UUID PRIMARY KEY,
  "shipment_id" uuid,
  "udi_di" varchar,
  "batch_number" varchar,
  "expected_quantity" int,
  "received_quantity" int DEFAULT 0
);

CREATE TABLE "shipment_discrepancies" (
  "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
  "tenant_id" uuid,
  "shipment_id" uuid,
  "shipment_item_id" uuid,
  "type" varchar,
  "expected_qty" int,
  "received_qty" int,
  "status" varchar DEFAULT 'open',
  "reported_by" uuid,
  "reported_at" timestamp DEFAULT (now()),
  "comment" text,
  "resolved_by" uuid,
  "resolution_note" text,
  "resolved_at" timestamp
);

CREATE TABLE "stock_movements" (
  "id" UUID PRIMARY KEY,
  "tenant_id" uuid,
  "item_id" uuid,
  "from_location" uuid,
  "to_location" uuid,
  "quantity" int,
  "moved_by" uuid,
  "moved_at" timestamp DEFAULT (now()),
  "reason" varchar
);

CREATE TABLE "medical_devices" (
  "id" UUID PRIMARY KEY,
  "inventory_item_id" uuid,
  "internal_inventory_number" varchar UNIQUE,
  "stk_interval_months" int,
  "mtk_interval_months" int,
  "last_maintenance_date" date,
  "next_maintenance_date" date,
  "manual_url" text,
  "is_active" boolean DEFAULT true
);

CREATE TABLE "maintenance_logs" (
  "id" UUID PRIMARY KEY,
  "device_id" uuid,
  "check_type" varchar,
  "performer" varchar,
  "result" varchar,
  "protocol_url" text,
  "performed_at" timestamp
);

CREATE TABLE "mission_usage" (
  "id" UUID PRIMARY KEY,
  "tenant_id" uuid,
  "mission_number" varchar,
  "custom_fields" jsonb DEFAULT '{}',
  "vehicle_id" uuid,
  "user_id" uuid,
  "item_id" uuid,
  "consumed_at" timestamp DEFAULT (now())
);

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

COMMENT ON COLUMN "shipment_discrepancies"."type" IS 'missing, damaged, wrong_article, expired_on_arrival';

COMMENT ON COLUMN "shipment_discrepancies"."status" IS 'open, investigating, resolved, written_off';

COMMENT ON COLUMN "shipment_discrepancies"."comment" IS 'z.B. Karton war nass oder Packung fehlte';

COMMENT ON COLUMN "shipment_discrepancies"."resolution_note" IS 'z.B. Nachlieferung veranlasst oder Bestand korrigiert';

COMMENT ON COLUMN "stock_movements"."reason" IS 'transfer, usage, correction, disposal';

COMMENT ON TABLE "medical_devices" IS 'Zusatzdaten für langlebige Medizingeräte (Assets).';

COMMENT ON COLUMN "medical_devices"."inventory_item_id" IS 'Link zum physischen Item';

COMMENT ON COLUMN "medical_devices"."internal_inventory_number" IS 'Deine interne Inventarnummer';

COMMENT ON COLUMN "medical_devices"."stk_interval_months" IS 'Sicherheitstechnische Kontrolle';

COMMENT ON COLUMN "medical_devices"."mtk_interval_months" IS 'Medizintechnische Kontrolle';

COMMENT ON COLUMN "medical_devices"."manual_url" IS 'Link zur digitalen Gebrauchsanweisung';

COMMENT ON COLUMN "medical_devices"."is_active" IS 'Gerät einsatzbereit oder gesperrt?';

COMMENT ON COLUMN "maintenance_logs"."check_type" IS 'STK, MTK, Reparatur, Einweisung';

COMMENT ON COLUMN "maintenance_logs"."performer" IS 'Wer hat die Prüfung durchgeführt (z.B. externer Dienstleister)';

COMMENT ON COLUMN "maintenance_logs"."result" IS 'bestanden, Mängel, stillgelegt';

COMMENT ON COLUMN "maintenance_logs"."protocol_url" IS 'Link zum PDF-Prüfbericht';

COMMENT ON TABLE "mission_usage" IS 'Verknüpft Lagerabgänge mit Einsätzen (MPG-Konformität).';

COMMENT ON COLUMN "mission_usage"."mission_number" IS 'Einsatznummer für MPG-Rückverfolgbarkeit';

COMMENT ON COLUMN "mission_usage"."custom_fields" IS 'Benutzerdefinierte Felder';

COMMENT ON COLUMN "mission_usage"."vehicle_id" IS 'ID des Fahrzeugs (Location vom Typ vehicle)';

COMMENT ON COLUMN "mission_usage"."user_id" IS 'Sanitäter, der den Artikel entnommen hat';

COMMENT ON COLUMN "mission_usage"."item_id" IS 'Das spezifische verbrauchte Item';

ALTER TABLE "tenants" ADD FOREIGN KEY ("plan_id") REFERENCES "plans" ("id");

ALTER TABLE "users" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "locations" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "locations" ADD FOREIGN KEY ("parent_id") REFERENCES "locations" ("id");

ALTER TABLE "stock_targets" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id");

ALTER TABLE "stock_targets" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di");

ALTER TABLE "tenant_product_settings" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "tenant_product_settings" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di");

ALTER TABLE "inventory_items" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "inventory_items" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di");

ALTER TABLE "inventory_items" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id");

ALTER TABLE "inventory_items" ADD FOREIGN KEY ("scanned_by") REFERENCES "users" ("id");

ALTER TABLE "shipments" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "shipments" ADD FOREIGN KEY ("from_location_id") REFERENCES "locations" ("id");

ALTER TABLE "shipments" ADD FOREIGN KEY ("to_location_id") REFERENCES "locations" ("id");

ALTER TABLE "shipments" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "shipments" ADD FOREIGN KEY ("received_by") REFERENCES "users" ("id");

ALTER TABLE "shipment_items" ADD FOREIGN KEY ("shipment_id") REFERENCES "shipments" ("id");

ALTER TABLE "shipment_items" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di");

ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("shipment_id") REFERENCES "shipments" ("id");

ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("shipment_item_id") REFERENCES "shipment_items" ("id");

ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("reported_by") REFERENCES "users" ("id");

ALTER TABLE "shipment_discrepancies" ADD FOREIGN KEY ("resolved_by") REFERENCES "users" ("id");

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("item_id") REFERENCES "inventory_items" ("id");

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("from_location") REFERENCES "locations" ("id");

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("to_location") REFERENCES "locations" ("id");

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("moved_by") REFERENCES "users" ("id");

ALTER TABLE "medical_devices" ADD FOREIGN KEY ("inventory_item_id") REFERENCES "inventory_items" ("id");

ALTER TABLE "maintenance_logs" ADD FOREIGN KEY ("device_id") REFERENCES "medical_devices" ("id");

ALTER TABLE "mission_usage" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id");

ALTER TABLE "mission_usage" ADD FOREIGN KEY ("vehicle_id") REFERENCES "locations" ("id");

ALTER TABLE "mission_usage" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "mission_usage" ADD FOREIGN KEY ("item_id") REFERENCES "inventory_items" ("id");
