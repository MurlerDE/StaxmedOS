-- Add migration script here
-- Migration: Füge staxadmin Rolle und System-Tenant hinzu

-- 1. Erweitere CHECK Constraint für role um 'staxadmin'
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_user_role;
ALTER TABLE users ADD CONSTRAINT check_user_role
  CHECK (role IN ('staxadmin', 'admin', 'logistic_manager', 'paramedic'));

-- 2. Erstelle System-Tenant für StaxMed-Admins (falls nicht existiert)
INSERT INTO tenants (id, name, status, customer_number)
VALUES (
  '00000000-0000-0000-0000-000000000000'::uuid,
  'StaxMed System',
  'active',
  'STX-SYSTEM'
)
ON CONFLICT (id) DO NOTHING;

-- 3. Erstelle StaxAdmin-User (Passwort muss über /dev/hash endpoint generiert werden!)
-- WICHTIG: Nach Deployment das Passwort ändern!
-- Standard-Passwort: staxadmin123
INSERT INTO users (id, tenant_id, username, password_hash, role, is_active, first_name, last_name)
VALUES (
  gen_random_uuid(),
  '00000000-0000-0000-0000-000000000000'::uuid,
  'staxadmin',
  '$argon2id$v=19$m=19456,t=2,p=1$VEVTVF9TQUxU$4e3p5Kz855E6O8v3l6X9Q3v8k7m2', -- Dummy Hash - ÄNDERN!
  'staxadmin',
  true,
  'StaxMed',
  'Administrator'
)
ON CONFLICT (username) DO NOTHING;

-- 4. Aktualisiere Kommentare
COMMENT ON COLUMN users.role IS 'staxadmin (Super Admin für alle Tenants), admin (Tenant Admin), logistic_manager, paramedic (Sanitäter)';
COMMENT ON TABLE tenants IS 'Zentrale Mandantentabelle. System-Tenant (00000000-0000-0000-0000-000000000000) ist für StaxMed-Admins reserviert.';
