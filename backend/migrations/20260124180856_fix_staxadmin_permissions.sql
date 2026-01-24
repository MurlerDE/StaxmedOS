-- Migration: Fix StaxAdmin Permissions
-- StaxAdmin sollte ALLE Permissions haben, nicht nur system.*

-- Gebe StaxAdmin alle Permissions (außer die, die er schon hat)
INSERT INTO role_permissions (role_id, permission_id)
SELECT
  '00000000-0000-0000-0000-000000000001'::uuid as role_id,
  p.id as permission_id
FROM permissions p
WHERE NOT EXISTS (
  SELECT 1
  FROM role_permissions rp
  WHERE rp.role_id = '00000000-0000-0000-0000-000000000001'::uuid
  AND rp.permission_id = p.id
);
