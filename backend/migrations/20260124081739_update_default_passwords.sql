-- Add migration script here
-- Migration: Update default user passwords with real Argon2 hashes

-- Update StaxAdmin user password (staxadmin / staxadmin123)
UPDATE users
SET password_hash = '$argon2id$v=19$m=19456,t=2,p=1$o9M0eMItq06gq22PlXNTow$v3QglnmEpv6XDl+tApv39qqoSNgsi5caMHTja00XZcE'
WHERE username = 'staxadmin' AND role = 'staxadmin';

-- Update Tenant-Admin user password (admin@testwache.de / admin123)
UPDATE users
SET password_hash = '$argon2id$v=19$m=19456,t=2,p=1$EYi4/7/4tEepRWTPBPz5ig$lkxPtRZ6/vC6xaTYQt8OqcAzuQZGhOGtS79wjCvBSt0'
WHERE username = 'admin@testwache.de' AND role = 'admin';

-- WICHTIG: Standard-Credentials für Development
-- =============================================
-- StaxAdmin (Super Admin):
--   Username: staxadmin
--   Password: staxadmin123
--   Zugriff: Alle Tenants
--
-- Tenant Admin (Test-Wache):
--   Username: admin@testwache.de
--   Password: admin123
--   Zugriff: Nur eigener Tenant
--
-- TODO: Nach Deployment in Production diese Passwörter ÄNDERN!
