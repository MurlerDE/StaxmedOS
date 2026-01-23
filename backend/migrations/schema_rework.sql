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
  "max_users" int DEFAULT 5
);

CREATE TABLE "tenants" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "customer_number" varchar UNIQUE DEFAULT generate_stax_customer_number(),
  "name" varchar NOT NULL,
  "license_key" text UNIQUE,
  "valid_until" timestamptz, -- Upgrade: Zeitzonen-sicher
  "is_on_premise" boolean DEFAULT false,
  "hardware_hash" varchar,
  "plan_id" uuid,
  "status" varchar DEFAULT 'trial',
  "created_at" timestamptz DEFAULT now()
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
  "created_at" timestamptz DEFAULT now()
);

CREATE TABLE "global_products" (
  "udi_di" varchar PRIMARY KEY, -- GTIN / DI
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
  "location_type" varchar NOT NULL -- central, vehicle, shelf
);

CREATE TABLE "stock_targets" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "location_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "month_start" int CHECK (month_start BETWEEN 1 AND 12),
  "min_stock" int DEFAULT 0,
  "target_stock" int DEFAULT 0,
  "max_stock" int DEFAULT 0
);

CREATE TABLE "tenant_product_settings" (
  "tenant_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "custom_alias" varchar,
  "custom_category" varchar,
  "min_stock_global" int,
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
  "scanned_by" uuid
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
  "received_by" uuid
);

CREATE TABLE "shipment_items" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "shipment_id" uuid NOT NULL,
  "udi_di" varchar NOT NULL,
  "batch_number" varchar,
  "expected_quantity" int NOT NULL,
  "received_quantity" int DEFAULT 0
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
  "reason" varchar
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
  "is_active" boolean DEFAULT true
);

CREATE TABLE "maintenance_logs" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "device_id" uuid NOT NULL,
  "check_type" varchar,
  "performer" varchar,
  "result" varchar,
  "protocol_url" text,
  "performed_at" timestamptz DEFAULT now()
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

-- 3. INDIZES (Für Performance bei Millionen Datensätzen)
CREATE INDEX idx_inv_items_tenant_udi ON inventory_items(tenant_id, udi_di);
CREATE INDEX idx_inv_items_location ON inventory_items(location_id);
CREATE INDEX idx_inv_items_expiry ON inventory_items(expiry_date);
CREATE INDEX idx_shipments_tenant_status ON shipments(tenant_id, status);
CREATE INDEX idx_movements_item ON stock_movements(item_id);
CREATE INDEX idx_global_products_brand ON global_products(brand_name_de);

-- 4. FOREIGN KEYS (Mit Audit-Trail-Logik)

ALTER TABLE "tenants" ADD FOREIGN KEY ("plan_id") REFERENCES "plans" ("id") ON DELETE SET NULL;

ALTER TABLE "users" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;

ALTER TABLE "locations" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "locations" ADD FOREIGN KEY ("parent_id") REFERENCES "locations" ("id") ON DELETE SET NULL;

ALTER TABLE "inventory_items" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("udi_di") REFERENCES "global_products" ("udi_di");
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id") ON DELETE CASCADE;
ALTER TABLE "inventory_items" ADD FOREIGN KEY ("scanned_by") REFERENCES "users" ("id") ON DELETE SET NULL; -- Audit-Trail erhalten!

ALTER TABLE "shipments" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "shipments" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id") ON DELETE SET NULL;
ALTER TABLE "shipments" ADD FOREIGN KEY ("received_by") REFERENCES "users" ("id") ON DELETE SET NULL;

ALTER TABLE "stock_movements" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "stock_movements" ADD FOREIGN KEY ("moved_by") REFERENCES "users" ("id") ON DELETE SET NULL;

ALTER TABLE "medical_devices" ADD FOREIGN KEY ("inventory_item_id") REFERENCES "inventory_items" ("id") ON DELETE CASCADE;
ALTER TABLE "maintenance_logs" ADD FOREIGN KEY ("device_id") REFERENCES "medical_devices" ("id") ON DELETE CASCADE;

ALTER TABLE "mission_usage" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;
ALTER TABLE "mission_usage" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE SET NULL;
