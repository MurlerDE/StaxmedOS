-- Migration: Fix StaxAdmin Password and Login

-- ============================================================================
-- 1. AKTUALISIERE STAXADMIN PASSWORT
-- ============================================================================

-- Setze StaxAdmin Passwort zurück (staxadmin / staxadmin123)
UPDATE users
SET password_hash = '$argon2id$v=19$m=19456,t=2,p=1$o9M0eMItq06gq22PlXNTow$v3QglnmEpv6XDl+tApv39qqoSNgsi5caMHTja00XZcE'
WHERE username = 'staxadmin'
  AND role = 'staxadmin'
  AND tenant_id = '00000000-0000-0000-0000-000000000000'::uuid;

-- Falls StaxAdmin nicht existiert, erstelle ihn neu
INSERT INTO users (id, tenant_id, username, password_hash, role, is_active, first_name, last_name)
VALUES (
  '00000000-0000-0000-0000-000000000001'::uuid,
  '00000000-0000-0000-0000-000000000000'::uuid,
  'staxadmin',
  '$argon2id$v=19$m=19456,t=2,p=1$o9M0eMItq06gq22PlXNTow$v3QglnmEpv6XDl+tApv39qqoSNgsi5caMHTja00XZcE', -- staxadmin123
  'staxadmin',
  true,
  'System',
  'Administrator'
)
ON CONFLICT (username) DO UPDATE
SET password_hash = EXCLUDED.password_hash,
    is_active = true;

-- ============================================================================
-- 2. STELLE SICHER DASS STAXADMIN DIE RICHTIGE ROLLE HAT
-- ============================================================================

-- Weise StaxAdmin-Rolle zu (falls nicht vorhanden)
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
  )
ON CONFLICT DO NOTHING;

-- ============================================================================
-- 3. PRÜFE REGISTRIERTE USER UND WEISE STANDARD-ROLLEN ZU
-- ============================================================================

-- Für jeden User der über Register erstellt wurde und keine Rolle hat:
-- Weise die entsprechende Tenant-Admin-Rolle zu
DO $$
DECLARE
    v_user RECORD;
    v_admin_role_id uuid;
BEGIN
    -- Durchlaufe alle User die keine Rolle in user_roles haben
    FOR v_user IN
        SELECT u.id, u.tenant_id, u.role
        FROM users u
        WHERE u.tenant_id != '00000000-0000-0000-0000-000000000000'::uuid
        AND NOT EXISTS (
            SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id
        )
    LOOP
        -- Finde oder erstelle die passende Rolle für den Tenant
        IF v_user.role = 'admin' THEN
            -- Finde Admin-Rolle für den Tenant
            SELECT id INTO v_admin_role_id
            FROM roles
            WHERE tenant_id = v_user.tenant_id
              AND name = 'admin'
            LIMIT 1;

            -- Falls keine Tenant-spezifische Admin-Rolle existiert, erstelle sie
            IF v_admin_role_id IS NULL THEN
                INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
                VALUES (
                    gen_random_uuid(),
                    v_user.tenant_id,
                    'admin',
                    'Tenant Administrator',
                    'Vollständiger Zugriff innerhalb des Tenants',
                    true
                )
                RETURNING id INTO v_admin_role_id;

                -- Kopiere Permissions von Template-Admin-Rolle
                INSERT INTO role_permissions (role_id, permission_id)
                SELECT
                    v_admin_role_id,
                    rp.permission_id
                FROM role_permissions rp
                WHERE rp.role_id = '00000000-0000-0000-0000-000000000002'::uuid -- Template admin role
                ON CONFLICT DO NOTHING;
            END IF;

            -- Weise User die Rolle zu
            INSERT INTO user_roles (user_id, role_id, assigned_at)
            VALUES (v_user.id, v_admin_role_id, now())
            ON CONFLICT DO NOTHING;

            RAISE NOTICE 'Admin role assigned to user % in tenant %', v_user.id, v_user.tenant_id;
        END IF;
    END LOOP;
END $$;

-- ============================================================================
-- ZUSAMMENFASSUNG
-- ============================================================================

-- StaxAdmin Credentials:
--   Username: staxadmin
--   Password: staxadmin123
--
-- Alle über Register erstellten Admins haben jetzt ihre Rollen zugewiesen
