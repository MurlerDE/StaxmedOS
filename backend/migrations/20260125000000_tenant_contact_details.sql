-- Migration: Tenant Contact Details and Addresses
-- Erweitert die Tenants-Tabelle um Kontaktdaten und Adressen für die Kundenverwaltung

-- ============================================================================
-- 1. TENANT ADDRESSES TABLE
-- ============================================================================
-- Speichert Adressen für Kunden (Rechnungsadresse, Lieferadresse, etc.)

CREATE TABLE "tenant_addresses" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  "address_type" VARCHAR NOT NULL, -- 'billing', 'shipping', 'main'
  "company_name" VARCHAR,
  "street" VARCHAR NOT NULL,
  "street_additional" VARCHAR,
  "postal_code" VARCHAR NOT NULL,
  "city" VARCHAR NOT NULL,
  "state" VARCHAR,
  "country" VARCHAR NOT NULL DEFAULT 'DE',
  "is_primary" BOOLEAN DEFAULT false,
  "created_at" TIMESTAMPTZ DEFAULT now(),
  "updated_at" TIMESTAMPTZ DEFAULT now(),
  CONSTRAINT check_address_type CHECK (address_type IN ('billing', 'shipping', 'main', 'other'))
);

-- Index für schnellere Abfragen nach Tenant
CREATE INDEX idx_tenant_addresses_tenant_id ON tenant_addresses(tenant_id);

-- Index für primäre Adressen
CREATE INDEX idx_tenant_addresses_primary ON tenant_addresses(tenant_id, is_primary) WHERE is_primary = true;

-- ============================================================================
-- 2. TENANT CONTACTS TABLE
-- ============================================================================
-- Speichert Ansprechpartner für Kunden

CREATE TABLE "tenant_contacts" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  "contact_type" VARCHAR NOT NULL, -- 'primary', 'billing', 'technical', 'other'
  "salutation" VARCHAR, -- 'mr', 'ms', 'dr', 'prof'
  "first_name" VARCHAR NOT NULL,
  "last_name" VARCHAR NOT NULL,
  "position" VARCHAR,
  "email" VARCHAR NOT NULL,
  "phone" VARCHAR,
  "mobile" VARCHAR,
  "fax" VARCHAR,
  "is_primary" BOOLEAN DEFAULT false,
  "notes" TEXT,
  "created_at" TIMESTAMPTZ DEFAULT now(),
  "updated_at" TIMESTAMPTZ DEFAULT now(),
  CONSTRAINT check_contact_type CHECK (contact_type IN ('primary', 'billing', 'technical', 'support', 'other')),
  CONSTRAINT check_salutation CHECK (salutation IN ('mr', 'ms', 'dr', 'prof', NULL))
);

-- Index für schnellere Abfragen nach Tenant
CREATE INDEX idx_tenant_contacts_tenant_id ON tenant_contacts(tenant_id);

-- Index für primäre Kontakte
CREATE INDEX idx_tenant_contacts_primary ON tenant_contacts(tenant_id, is_primary) WHERE is_primary = true;

-- ============================================================================
-- 3. TENANT PAYMENT INFORMATION TABLE
-- ============================================================================
-- Speichert Zahlungsinformationen und -historie

CREATE TABLE "tenant_payments" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  "payment_method" VARCHAR NOT NULL, -- 'invoice', 'credit_card', 'sepa', 'paypal'
  "payment_status" VARCHAR NOT NULL DEFAULT 'pending', -- 'pending', 'paid', 'overdue', 'failed'
  "amount" DECIMAL(10, 2) NOT NULL,
  "currency" VARCHAR(3) DEFAULT 'EUR',
  "billing_period_start" DATE,
  "billing_period_end" DATE,
  "due_date" DATE,
  "paid_at" TIMESTAMPTZ,
  "invoice_number" VARCHAR UNIQUE,
  "notes" TEXT,
  "created_at" TIMESTAMPTZ DEFAULT now(),
  "updated_at" TIMESTAMPTZ DEFAULT now(),
  CONSTRAINT check_payment_method CHECK (payment_method IN ('invoice', 'credit_card', 'sepa', 'paypal', 'bank_transfer')),
  CONSTRAINT check_payment_status CHECK (payment_status IN ('pending', 'paid', 'overdue', 'failed', 'cancelled'))
);

-- Index für schnellere Abfragen nach Tenant
CREATE INDEX idx_tenant_payments_tenant_id ON tenant_payments(tenant_id);

-- Index für Zahlungsstatus
CREATE INDEX idx_tenant_payments_status ON tenant_payments(payment_status);

-- Index für Fälligkeitsdatum
CREATE INDEX idx_tenant_payments_due_date ON tenant_payments(due_date);

-- ============================================================================
-- 4. ERWEITERTE TENANT-FELDER
-- ============================================================================
-- Fügt zusätzliche Felder zur tenants-Tabelle hinzu

ALTER TABLE tenants
  ADD COLUMN IF NOT EXISTS vat_number VARCHAR,
  ADD COLUMN IF NOT EXISTS tax_id VARCHAR,
  ADD COLUMN IF NOT EXISTS website VARCHAR,
  ADD COLUMN IF NOT EXISTS industry VARCHAR,
  ADD COLUMN IF NOT EXISTS company_size VARCHAR,
  ADD COLUMN IF NOT EXISTS preferred_language VARCHAR(5) DEFAULT 'de',
  ADD COLUMN IF NOT EXISTS notes TEXT,
  ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT now();

-- Index für schnellere Textsuche
CREATE INDEX idx_tenants_name_search ON tenants USING gin(to_tsvector('german', name));

-- ============================================================================
-- 5. TRIGGER FÜR UPDATED_AT
-- ============================================================================
-- Automatisches Update des updated_at Feldes

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tenants_updated_at
  BEFORE UPDATE ON tenants
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tenant_addresses_updated_at
  BEFORE UPDATE ON tenant_addresses
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tenant_contacts_updated_at
  BEFORE UPDATE ON tenant_contacts
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tenant_payments_updated_at
  BEFORE UPDATE ON tenant_payments
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- 6. CONSTRAINTS FÜR PRIMÄRE ADRESSEN/KONTAKTE
-- ============================================================================
-- Sicherstellen, dass nur ein primärer Kontakt/eine primäre Adresse pro Typ existiert

CREATE UNIQUE INDEX idx_tenant_addresses_unique_primary
  ON tenant_addresses(tenant_id, address_type)
  WHERE is_primary = true;

CREATE UNIQUE INDEX idx_tenant_contacts_unique_primary
  ON tenant_contacts(tenant_id, contact_type)
  WHERE is_primary = true;

-- ============================================================================
-- 7. KOMMENTARE FÜR DOKUMENTATION
-- ============================================================================

COMMENT ON TABLE tenant_addresses IS 'Speichert verschiedene Adressen für Tenants (Rechnungs-, Lieferadresse, etc.)';
COMMENT ON TABLE tenant_contacts IS 'Speichert Ansprechpartner für Tenants mit verschiedenen Rollen';
COMMENT ON TABLE tenant_payments IS 'Zahlungshistorie und -informationen für Tenants';

COMMENT ON COLUMN tenants.vat_number IS 'Umsatzsteuer-Identifikationsnummer';
COMMENT ON COLUMN tenants.tax_id IS 'Steuernummer';
COMMENT ON COLUMN tenants.company_size IS 'Unternehmensgröße (z.B. 1-10, 11-50, 51-200, 201+)';
COMMENT ON COLUMN tenants.industry IS 'Branche des Unternehmens';
