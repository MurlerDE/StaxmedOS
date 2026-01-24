-- Migration: Permission-basiertes RBAC System
-- Ermöglicht flexible, granulare Berechtigungen statt fester Rollen

-- ============================================================================
-- 1. PERMISSIONS TABELLE
-- ============================================================================
-- Definiert alle verfügbaren Berechtigungen im System

CREATE TABLE "permissions" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "name" varchar UNIQUE NOT NULL,  -- z.B. "inventory.create"
  "resource" varchar NOT NULL,      -- z.B. "inventory", "users", "locations"
  "action" varchar NOT NULL,        -- z.B. "view", "create", "update", "delete"
  "scope" varchar DEFAULT 'own',    -- "all", "own", "location:uuid", "warehouse:uuid"
  "description" text,
  "is_system" boolean DEFAULT false, -- Kann nicht gelöscht werden (für staxadmin)
  "created_at" timestamptz DEFAULT now()
);

-- ============================================================================
-- 2. ROLES TABELLE (erweitert bestehende users.role)
-- ============================================================================
-- Definiert Rollen-Vorlagen mit Permissions
-- Die alte "role" Spalte in users bleibt als Referenz

CREATE TABLE "roles" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "tenant_id" uuid,  -- NULL = System-Rolle (staxadmin), sonst Tenant-spezifisch
  "name" varchar NOT NULL,
  "display_name" varchar NOT NULL,
  "description" text,
  "is_system" boolean DEFAULT false,
  "created_at" timestamptz DEFAULT now(),
  CONSTRAINT unique_role_per_tenant UNIQUE(tenant_id, name)
);

-- ============================================================================
-- 3. ROLE_PERMISSIONS - Verknüpfungstabelle
-- ============================================================================
-- Welche Permissions hat eine Rolle?

CREATE TABLE "role_permissions" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "role_id" uuid NOT NULL,
  "permission_id" uuid NOT NULL,
  "granted_at" timestamptz DEFAULT now(),
  CONSTRAINT unique_role_permission UNIQUE(role_id, permission_id)
);

-- ============================================================================
-- 4. USER_ROLES - Verknüpfungstabelle
-- ============================================================================
-- Welche Rollen hat ein User? (User können mehrere Rollen haben)

CREATE TABLE "user_roles" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "role_id" uuid NOT NULL,
  "assigned_by" uuid,
  "assigned_at" timestamptz DEFAULT now(),
  CONSTRAINT unique_user_role UNIQUE(user_id, role_id)
);

-- ============================================================================
-- 5. USER_PERMISSIONS - Direkte User-Permissions
-- ============================================================================
-- Überschreibt/Erweitert Rollen-Permissions für spezielle Fälle

CREATE TABLE "user_permissions" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "permission_id" uuid NOT NULL,
  "granted" boolean DEFAULT true,  -- false = explizit verweigert (override)
  "granted_by" uuid,
  "granted_at" timestamptz DEFAULT now(),
  "expires_at" timestamptz,  -- Temporäre Permissions möglich
  CONSTRAINT unique_user_permission UNIQUE(user_id, permission_id)
);

-- ============================================================================
-- 6. LOCATION_PERMISSIONS - Orts-spezifische Berechtigungen
-- ============================================================================
-- Für granulare Kontrolle: "User darf nur Fahrzeug X sehen"

CREATE TABLE "location_permissions" (
  "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "location_id" uuid NOT NULL,
  "permission_level" varchar NOT NULL DEFAULT 'view', -- view, edit, manage
  "granted_by" uuid,
  "granted_at" timestamptz DEFAULT now(),
  CONSTRAINT unique_user_location UNIQUE(user_id, location_id)
);

-- ============================================================================
-- 7. INDIZES für Performance
-- ============================================================================

CREATE INDEX idx_permissions_resource ON permissions(resource);
CREATE INDEX idx_permissions_name ON permissions(name);
CREATE INDEX idx_roles_tenant ON roles(tenant_id);
CREATE INDEX idx_role_permissions_role ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission ON role_permissions(permission_id);
CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);
CREATE INDEX idx_user_permissions_user ON user_permissions(user_id);
CREATE INDEX idx_user_permissions_granted ON user_permissions(user_id) WHERE granted = true;
CREATE INDEX idx_user_permissions_expires ON user_permissions(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_location_permissions_user ON location_permissions(user_id);
CREATE INDEX idx_location_permissions_location ON location_permissions(location_id);

-- ============================================================================
-- 8. FOREIGN KEYS
-- ============================================================================

ALTER TABLE "roles" ADD FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE;

ALTER TABLE "role_permissions" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON DELETE CASCADE;
ALTER TABLE "role_permissions" ADD FOREIGN KEY ("permission_id") REFERENCES "permissions" ("id") ON DELETE CASCADE;

ALTER TABLE "user_roles" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
ALTER TABLE "user_roles" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON DELETE CASCADE;
ALTER TABLE "user_roles" ADD FOREIGN KEY ("assigned_by") REFERENCES "users" ("id") ON DELETE SET NULL;

ALTER TABLE "user_permissions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
ALTER TABLE "user_permissions" ADD FOREIGN KEY ("permission_id") REFERENCES "permissions" ("id") ON DELETE CASCADE;
ALTER TABLE "user_permissions" ADD FOREIGN KEY ("granted_by") REFERENCES "users" ("id") ON DELETE SET NULL;

ALTER TABLE "location_permissions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
ALTER TABLE "location_permissions" ADD FOREIGN KEY ("location_id") REFERENCES "locations" ("id") ON DELETE CASCADE;
ALTER TABLE "location_permissions" ADD FOREIGN KEY ("granted_by") REFERENCES "users" ("id") ON DELETE SET NULL;

-- ============================================================================
-- 9. SYSTEM-PERMISSIONS (Basis-Set)
-- ============================================================================

-- Inventory Permissions
INSERT INTO permissions (name, resource, action, scope, description, is_system) VALUES
-- Inventory - Basis
('inventory.view.all', 'inventory', 'view', 'all', 'Alle Inventare anzeigen', true),
('inventory.view.own', 'inventory', 'view', 'own', 'Nur eigene/zugewiesene Inventare anzeigen', true),
('inventory.create', 'inventory', 'create', 'all', 'Neue Inventory Items erstellen', true),
('inventory.update', 'inventory', 'update', 'all', 'Inventory Items bearbeiten', true),
('inventory.delete', 'inventory', 'delete', 'all', 'Inventory Items löschen', true),
('inventory.manage', 'inventory', 'manage', 'all', 'Vollständige Inventory-Verwaltung', true),

-- Shipments
('shipments.view', 'shipments', 'view', 'all', 'Lieferungen anzeigen', true),
('shipments.create', 'shipments', 'create', 'all', 'Neue Lieferungen erstellen', true),
('shipments.receive', 'shipments', 'receive', 'all', 'Lieferungen empfangen', true),
('shipments.manage', 'shipments', 'manage', 'all', 'Vollständige Shipment-Verwaltung', true),

-- Locations
('locations.view', 'locations', 'view', 'all', 'Alle Standorte anzeigen', true),
('locations.create', 'locations', 'create', 'all', 'Neue Standorte erstellen', true),
('locations.update', 'locations', 'update', 'all', 'Standorte bearbeiten', true),
('locations.delete', 'locations', 'delete', 'all', 'Standorte löschen', true),

-- Users
('users.view', 'users', 'view', 'all', 'User anzeigen', true),
('users.create', 'users', 'create', 'all', 'Neue User erstellen', true),
('users.update', 'users', 'update', 'all', 'User bearbeiten', true),
('users.delete', 'users', 'delete', 'all', 'User löschen', true),
('users.manage_permissions', 'users', 'manage_permissions', 'all', 'User-Berechtigungen verwalten', true),

-- Roles
('roles.view', 'roles', 'view', 'all', 'Rollen anzeigen', true),
('roles.create', 'roles', 'create', 'all', 'Neue Rollen erstellen', true),
('roles.update', 'roles', 'update', 'all', 'Rollen bearbeiten', true),
('roles.delete', 'roles', 'delete', 'all', 'Rollen löschen', true),

-- Medical Devices
('devices.view', 'devices', 'view', 'all', 'Medizingeräte anzeigen', true),
('devices.manage', 'devices', 'manage', 'all', 'Medizingeräte verwalten', true),
('devices.maintenance', 'devices', 'maintenance', 'all', 'Wartungen durchführen', true),

-- Mission/Usage
('missions.view', 'missions', 'view', 'all', 'Einsätze anzeigen', true),
('missions.report', 'missions', 'report', 'all', 'Verbrauch melden', true),

-- Tenant Settings
('tenant.settings.view', 'tenant', 'settings.view', 'all', 'Tenant-Einstellungen anzeigen', true),
('tenant.settings.update', 'tenant', 'settings.update', 'all', 'Tenant-Einstellungen bearbeiten', true),

-- System Admin (nur für staxadmin)
('system.tenants.view', 'system', 'tenants.view', 'all', 'Alle Tenants anzeigen', true),
('system.tenants.manage', 'system', 'tenants.manage', 'all', 'Tenants verwalten', true),
('system.licenses.manage', 'system', 'licenses.manage', 'all', 'Lizenzen verwalten', true),
('system.all', 'system', 'all', 'all', 'Vollständiger System-Zugriff', true);

-- ============================================================================
-- 10. SYSTEM-ROLLEN (vordefiniert)
-- ============================================================================

-- StaxAdmin Role (System-Rolle, kein Tenant)
INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
VALUES (
  '00000000-0000-0000-0000-000000000001'::uuid,
  NULL,
  'staxadmin',
  'StaxMed Administrator',
  'Vollständiger System-Zugriff für StaxMed Mitarbeiter',
  true
);

-- Tenant Admin Role Template (wird pro Tenant kopiert)
INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
VALUES (
  '00000000-0000-0000-0000-000000000002'::uuid,
  NULL,
  'admin',
  'Tenant Administrator',
  'Vollständiger Zugriff innerhalb des Tenants',
  true
);

-- Logistic Manager Template
INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
VALUES (
  '00000000-0000-0000-0000-000000000003'::uuid,
  NULL,
  'logistic_manager',
  'Logistik-Manager',
  'Verwaltung von Inventory, Shipments und Locations',
  true
);

-- Paramedic Template
INSERT INTO roles (id, tenant_id, name, display_name, description, is_system)
VALUES (
  '00000000-0000-0000-0000-000000000004'::uuid,
  NULL,
  'paramedic',
  'Sanitäter',
  'Basis-Zugriff: Verbrauch melden, Inventory anzeigen',
  true
);

-- ============================================================================
-- 11. ROLLE-PERMISSION ZUWEISUNGEN
-- ============================================================================

-- StaxAdmin bekommt ALLE System-Permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT
  '00000000-0000-0000-0000-000000000001'::uuid,
  id
FROM permissions
WHERE name LIKE 'system.%';

-- Tenant Admin bekommt alle Tenant-relevanten Permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT
  '00000000-0000-0000-0000-000000000002'::uuid,
  id
FROM permissions
WHERE name IN (
  'inventory.manage',
  'shipments.manage',
  'locations.view', 'locations.create', 'locations.update', 'locations.delete',
  'users.view', 'users.create', 'users.update', 'users.delete', 'users.manage_permissions',
  'roles.view', 'roles.create', 'roles.update', 'roles.delete',
  'devices.view', 'devices.manage', 'devices.maintenance',
  'missions.view', 'missions.report',
  'tenant.settings.view', 'tenant.settings.update'
);

-- Logistic Manager Permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT
  '00000000-0000-0000-0000-000000000003'::uuid,
  id
FROM permissions
WHERE name IN (
  'inventory.view.all', 'inventory.create', 'inventory.update',
  'shipments.view', 'shipments.create', 'shipments.receive', 'shipments.manage',
  'locations.view',
  'missions.view'
);

-- Paramedic Permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT
  '00000000-0000-0000-0000-000000000004'::uuid,
  id
FROM permissions
WHERE name IN (
  'inventory.view.own',
  'missions.report',
  'locations.view'
);

-- ============================================================================
-- 12. MIGRIERE BESTEHENDE USER ZU NEUEM SYSTEM
-- ============================================================================

-- Weise bestehenden Usern ihre Rollen zu basierend auf users.role
INSERT INTO user_roles (user_id, role_id, assigned_at)
SELECT
  u.id,
  CASE u.role
    WHEN 'staxadmin' THEN '00000000-0000-0000-0000-000000000001'::uuid
    WHEN 'admin' THEN '00000000-0000-0000-0000-000000000002'::uuid
    WHEN 'logistic_manager' THEN '00000000-0000-0000-0000-000000000003'::uuid
    WHEN 'paramedic' THEN '00000000-0000-0000-0000-000000000004'::uuid
    ELSE '00000000-0000-0000-0000-000000000004'::uuid -- Default zu paramedic
  END,
  now()
FROM users u
ON CONFLICT (user_id, role_id) DO NOTHING;

-- ============================================================================
-- 13. HELPER FUNCTION: Check User Permission
-- ============================================================================

CREATE OR REPLACE FUNCTION user_has_permission(
  p_user_id uuid,
  p_permission_name varchar
)
RETURNS boolean AS $$
DECLARE
  has_perm boolean;
BEGIN
  -- Prüfe direkte User-Permissions (können Rollen überschreiben)
  SELECT EXISTS(
    SELECT 1
    FROM user_permissions up
    JOIN permissions p ON up.permission_id = p.id
    WHERE up.user_id = p_user_id
      AND p.name = p_permission_name
      AND up.granted = true
      AND (up.expires_at IS NULL OR up.expires_at > now())
  ) INTO has_perm;

  IF has_perm THEN
    RETURN true;
  END IF;

  -- Prüfe Permissions über Rollen
  SELECT EXISTS(
    SELECT 1
    FROM user_roles ur
    JOIN role_permissions rp ON ur.role_id = rp.role_id
    JOIN permissions p ON rp.permission_id = p.id
    WHERE ur.user_id = p_user_id
      AND p.name = p_permission_name
  ) INTO has_perm;

  RETURN has_perm;
END;
$$ LANGUAGE plpgsql STABLE;

-- ============================================================================
-- 14. HELPER FUNCTION: Get User Permissions
-- ============================================================================

CREATE OR REPLACE FUNCTION get_user_permissions(p_user_id uuid)
RETURNS TABLE(permission_name varchar, source varchar) AS $$
BEGIN
  RETURN QUERY
  -- Permissions über Rollen
  SELECT DISTINCT
    p.name as permission_name,
    'role:' || r.name as source
  FROM user_roles ur
  JOIN roles r ON ur.role_id = r.id
  JOIN role_permissions rp ON r.id = rp.role_id
  JOIN permissions p ON rp.permission_id = p.id
  WHERE ur.user_id = p_user_id

  UNION

  -- Direkte User-Permissions
  SELECT
    p.name as permission_name,
    'direct' as source
  FROM user_permissions up
  JOIN permissions p ON up.permission_id = p.id
  WHERE up.user_id = p_user_id
    AND up.granted = true
    AND (up.expires_at IS NULL OR up.expires_at > now());
END;
$$ LANGUAGE plpgsql STABLE;

-- ============================================================================
-- 15. COMMENTS
-- ============================================================================

COMMENT ON TABLE permissions IS 'Definiert alle verfügbaren Berechtigungen im System';
COMMENT ON TABLE roles IS 'Rollen-Vorlagen mit zugewiesenen Permissions';
COMMENT ON TABLE role_permissions IS 'Verknüpfung: Welche Permissions hat eine Rolle?';
COMMENT ON TABLE user_roles IS 'Verknüpfung: Welche Rollen hat ein User?';
COMMENT ON TABLE user_permissions IS 'Direkte User-Permissions (überschreibt Rollen)';
COMMENT ON TABLE location_permissions IS 'Orts-spezifische Berechtigungen für granulare Kontrolle';

COMMENT ON COLUMN permissions.name IS 'Eindeutiger Name im Format: resource.action.scope (z.B. inventory.view.all)';
COMMENT ON COLUMN permissions.scope IS 'all = global, own = nur eigene, oder location:uuid / warehouse:uuid';
COMMENT ON COLUMN permissions.is_system IS 'System-Permissions können nicht gelöscht werden';

COMMENT ON COLUMN roles.tenant_id IS 'NULL = System-Rolle (für alle Tenants), sonst Tenant-spezifisch';
COMMENT ON COLUMN roles.is_system IS 'System-Rollen (staxadmin, admin, etc.) können nicht gelöscht werden';

COMMENT ON COLUMN user_permissions.granted IS 'false = Permission explizit verweigert (überschreibt Rolle)';
COMMENT ON COLUMN user_permissions.expires_at IS 'Temporäre Permissions möglich (NULL = unbegrenzt)';

COMMENT ON FUNCTION user_has_permission IS 'Prüft ob User eine bestimmte Permission hat (über Rolle oder direkt)';
COMMENT ON FUNCTION get_user_permissions IS 'Gibt alle Permissions eines Users zurück mit Quelle (Rolle oder direkt)';

-- ============================================================================
-- 16. WICHTIGE HINWEISE
-- ============================================================================

-- Die alte users.role Spalte bleibt als Fallback erhalten
-- Sie wird beim Login verwendet um initial die Rolle zu bestimmen
-- Neue User sollten über user_roles Tabelle zugewiesen werden

-- Permission-Namenskonvention:
-- Format: {resource}.{action}.{scope}
-- Beispiele:
--   inventory.view.all       - Alle Inventare sehen
--   inventory.create         - Inventory erstellen
--   users.manage_permissions - User-Berechtigungen verwalten
--   system.tenants.manage    - Tenants verwalten (nur staxadmin)

-- Wildcard-Support kann später hinzugefügt werden:
--   inventory.*   - Alle Inventory-Permissions
--   *.view        - Alle View-Permissions
