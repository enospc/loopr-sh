package ops

import (
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

func TestLoadLoopConfigDefaults(t *testing.T) {
	dir := t.TempDir()
	cfg, err := LoadLoopConfig(filepath.Join(dir, "config"))
	if err != nil {
		t.Fatalf("LoadLoopConfig error: %v", err)
	}
	if !reflect.DeepEqual(cfg, DefaultLoopConfig()) {
		t.Fatalf("config = %#v, want %#v", cfg, DefaultLoopConfig())
	}
}

func TestLoadLoopConfigOverrides(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "config")
	data := []byte("MAX_CALLS_PER_HOUR=50\nCODEX_TIMEOUT_MINUTES=10\nMAX_ITERATIONS=5\nMAX_MISSING_STATUS=4\n")
	if err := os.WriteFile(path, data, 0o644); err != nil {
		t.Fatalf("write config: %v", err)
	}
	cfg, err := LoadLoopConfig(path)
	if err != nil {
		t.Fatalf("LoadLoopConfig error: %v", err)
	}
	if cfg.MaxCallsPerHour != 50 || cfg.CodexTimeoutMinutes != 10 || cfg.MaxIterations != 5 || cfg.MaxMissingStatus != 4 {
		t.Fatalf("config override mismatch: %#v", cfg)
	}
}

func TestLoadLoopConfigUnknownKey(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "config")
	if err := os.WriteFile(path, []byte("UNKNOWN_KEY=1\n"), 0o644); err != nil {
		t.Fatalf("write config: %v", err)
	}
	if _, err := LoadLoopConfig(path); err == nil {
		t.Fatalf("expected error for unknown key")
	}
}
