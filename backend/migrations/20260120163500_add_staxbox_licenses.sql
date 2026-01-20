-- Add staxbox_licenses table for StaxBox license management
CREATE TABLE staxbox_licenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hardware_id VARCHAR(100) UNIQUE NOT NULL, -- z.B. CPU-Seriennummer vom Pi
    tenant_id UUID REFERENCES tenants(id),
    license_key VARCHAR(100) UNIQUE,
    valid_until TIMESTAMPTZ,
    last_heartbeat TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index für schnelle Heartbeat-Updates
CREATE INDEX idx_staxbox_hardware ON staxbox_licenses(hardware_id);

COMMENT ON TABLE staxbox_licenses IS 'Verwaltung der StaxBox-Lizenzen für On-Premise Installationen';
COMMENT ON COLUMN staxbox_licenses.hardware_id IS 'Eindeutige Hardware-ID (z.B. CPU-Seriennummer vom Raspberry Pi)';
COMMENT ON COLUMN staxbox_licenses.tenant_id IS 'Verknüpfung zum Mandanten, der diese Box nutzt';
COMMENT ON COLUMN staxbox_licenses.license_key IS 'Verschlüsselter Lizenzschlüssel';
COMMENT ON COLUMN staxbox_licenses.valid_until IS 'Ablaufdatum der Lizenz';
COMMENT ON COLUMN staxbox_licenses.last_heartbeat IS 'Zeitpunkt des letzten Heartbeat-Signals von der Box';
COMMENT ON COLUMN staxbox_licenses.is_active IS 'Gibt an, ob die Lizenz aktiv ist';
