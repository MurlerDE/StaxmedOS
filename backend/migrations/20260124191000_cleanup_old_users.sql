-- Migration: Cleanup old test users and recreate with proper RBAC

-- ============================================================================
-- 1. LÖSCHE ALTE TEST-USER (vor RBAC-System erstellt)
-- ============================================================================

-- Lösche alle User außer dem System-StaxAdmin
-- (Behalte nur User mit dem speziellen StaxAdmin tenant_id)
DELETE FROM user_roles
WHERE user_id IN (
    SELECT id FROM users
    WHERE tenant_id != '00000000-0000-0000-0000-000000000000'::uuid
);

DELETE FROM user_permissions
WHERE user_id IN (
    SELECT id FROM users
    WHERE tenant_id != '00000000-0000-0000-0000-000000000000'::uuid
);

DELETE FROM users
WHERE tenant_id != '00000000-0000-0000-0000-000000000000'::uuid;

-- ============================================================================
-- 2. STELLE SICHER, DASS STAXADMIN RICHTIG KONFIGURIERT IST
-- ============================================================================

-- StaxAdmin Rolle in user_roles eintragen (falls noch nicht vorhanden)
INSERT INTO user_roles (user_id, role_id, assigned_at)
SELECT
    u.id,
    '00000000-0000-0000-0000-000000000001'::uuid, -- staxadmin role
    now()
FROM users u
WHERE u.username = 'staxadmin'
  AND u.role = 'staxadmin'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur
    WHERE ur.user_id = u.id
    AND ur.role_id = '00000000-0000-0000-0000-000000000001'::uuid
  );

-- ============================================================================
-- 3. ERSTELLE NEUE TEST-USER MIT KORREKTEM RBAC-SETUP
-- ============================================================================

-- Tenant Admin für Test-Wache (neu anlegen mit RBAC)
DO $$
DECLARE
    v_tenant_id uuid;
    v_user_id uuid;
    v_admin_role_id uuid;
BEGIN
    -- Hole Test-Tenant ID
    SELECT id INTO v_tenant_id FROM tenants WHERE name = 'StaxMed Test-Wache' LIMIT 1;

    IF v_tenant_id IS NOT NULL THEN
        -- Erstelle Admin-User für diesen Tenant (falls noch nicht vorhanden)
        INSERT INTO users (id, tenant_id, username, password_hash, role, is_active, first_name, last_name)
        VALUES (
            gen_random_uuid(),
            v_tenant_id,
            'admin@testwache.de',
            '$argon2id$v=19$m=19456,t=2,p=1$EYi4/7/4tEepRWTPBPz5ig$lkxPtRZ6/vC6xaTYQt8OqcAzuQZGhOGtS79wjCvBSt0', -- admin123
            'admin',
            true,
            'Admin',
            'Test-Wache'
        )
        ON CONFLICT (username) DO NOTHING
        RETURNING id INTO v_user_id;

        -- Falls User schon existierte, hole ID
        IF v_user_id IS NULL THEN
            SELECT id INTO v_user_id FROM users WHERE username = 'admin@testwache.de';
        END IF;

        -- Weise Admin-Rolle zu
        -- Erstelle tenant-spezifische Admin-Rolle falls nicht vorhanden
        INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
        VALUES (
            gen_random_uuid(),
            v_tenant_id,
            'admin',
            'Tenant Administrator',
            'Vollständiger Zugriff innerhalb des Tenants',
            true
        )
        ON CONFLICT DO NOTHING
        RETURNING id INTO v_admin_role_id;

        -- Falls Rolle schon existiert
        IF v_admin_role_id IS NULL THEN
            SELECT id INTO v_admin_role_id FROM roles WHERE tenant_id = v_tenant_id AND name = 'admin' LIMIT 1;
        END IF;

        -- Kopiere Permissions von Template-Admin-Rolle zur Tenant-Admin-Rolle
        INSERT INTO role_permissions (role_id, permission_id)
        SELECT
            v_admin_role_id,
            rp.permission_id
        FROM role_permissions rp
        WHERE rp.role_id = '00000000-0000-0000-0000-000000000002'::uuid -- Template admin role
        ON CONFLICT DO NOTHING;

        -- Weise User die Admin-Rolle zu
        INSERT INTO user_roles (user_id, role_id, assigned_at)
        VALUES (v_user_id, v_admin_role_id, now())
        ON CONFLICT DO NOTHING;

        RAISE NOTICE 'Tenant Admin created/updated for Test-Wache';
    END IF;
END $$;

-- Erstelle einen Test-Paramedic
DO $$
DECLARE
    v_tenant_id uuid;
    v_user_id uuid;
    v_paramedic_role_id uuid;
BEGIN
    -- Hole Test-Tenant ID
    SELECT id INTO v_tenant_id FROM tenants WHERE name = 'StaxMed Test-Wache' LIMIT 1;

    IF v_tenant_id IS NOT NULL THEN
        -- Erstelle Paramedic-User
        INSERT INTO users (id, tenant_id, username, password_hash, role, is_active, first_name, last_name)
        VALUES (
            gen_random_uuid(),
            v_tenant_id,
            'paramedic@testwache.de',
            '$argon2id$v=19$m=19456,t=2,p=1$EYi4/7/4tEepRWTPBPz5ig$lkxPtRZ6/vC6xaTYQt8OqcAzuQZGhOGtS79wjCvBSt0', -- admin123
            'paramedic',
            true,
            'Max',
            'Mustermann'
        )
        ON CONFLICT (username) DO NOTHING
        RETURNING id INTO v_user_id;

        -- Falls User schon existierte
        IF v_user_id IS NULL THEN
            SELECT id INTO v_user_id FROM users WHERE username = 'paramedic@testwache.de';
        END IF;

        -- Erstelle tenant-spezifische Paramedic-Rolle falls nicht vorhanden
        INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
        VALUES (
            gen_random_uuid(),
            v_tenant_id,
            'paramedic',
            'Sanitäter',
            'Basis-Zugriff: Verbrauch melden, Inventory anzeigen',
            true
        )
        ON CONFLICT DO NOTHING
        RETURNING id INTO v_paramedic_role_id;

        IF v_paramedic_role_id IS NULL THEN
            SELECT id INTO v_paramedic_role_id FROM roles WHERE tenant_id = v_tenant_id AND name = 'paramedic' LIMIT 1;
        END IF;

        -- Kopiere Permissions von Template-Paramedic-Rolle
        INSERT INTO role_permissions (role_id, permission_id)
        SELECT
            v_paramedic_role_id,
            rp.permission_id
        FROM role_permissions rp
        WHERE rp.role_id = '00000000-0000-0000-0000-000000000004'::uuid -- Template paramedic role
        ON CONFLICT DO NOTHING;

        -- Weise User die Paramedic-Rolle zu
        INSERT INTO user_roles (user_id, role_id, assigned_at)
        VALUES (v_user_id, v_paramedic_role_id, now())
        ON CONFLICT DO NOTHING;

        RAISE NOTICE 'Paramedic user created/updated for Test-Wache';
    END IF;
END $$;

-- ============================================================================
-- ZUSAMMENFASSUNG DER TEST-CREDENTIALS
-- ============================================================================

-- StaxAdmin (System-Administrator):
--   Username: staxadmin
--   Password: staxadmin123
--   Zugriff: Alle Tenants, alle Permissions

-- Tenant Admin (Test-Wache):
--   Username: admin@testwache.de
--   Password: admin123
--   Zugriff: Vollständiger Zugriff innerhalb Test-Wache Tenant

-- Paramedic (Test-Wache):
--   Username: paramedic@testwache.de
--   Password: admin123
--   Zugriff: Basis-Funktionen (Inventory anzeigen, Verbrauch melden)
