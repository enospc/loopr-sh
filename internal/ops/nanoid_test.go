package ops

import (
	"bytes"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestGenerateNanoIDDeterministic(t *testing.T) {
	reader := bytes.NewReader(bytes.Repeat([]byte{0}, repoIDLength))
	id, err := generateNanoID(reader, repoIDLength)
	if err != nil {
		t.Fatalf("generateNanoID error: %v", err)
	}
	if id != strings.Repeat(string(repoIDAlphabet[0]), repoIDLength) {
		t.Fatalf("id = %q, want %q", id, strings.Repeat(string(repoIDAlphabet[0]), repoIDLength))
	}
}

func TestNewSessionPathsRetriesOnCollision(t *testing.T) {
	dir := t.TempDir()
	now := time.Date(2026, 1, 26, 12, 0, 0, 0, time.UTC)

	reader := bytes.NewReader(append(bytes.Repeat([]byte{0}, repoIDLength), bytes.Repeat([]byte{1}, repoIDLength)...))

	base := "session-20260126-120000-uuuuuu"
	collide := filepath.Join(dir, base+".log")
	if err := WriteFileAtomic(collide, []byte("collision"), 0o644); err != nil {
		t.Fatalf("write collision: %v", err)
	}

	logPath, metaPath, err := newSessionPaths(dir, now, reader)
	if err != nil {
		t.Fatalf("newSessionPaths error: %v", err)
	}
	if !strings.Contains(logPath, "session-20260126-120000-ssssss.log") {
		t.Fatalf("logPath = %q, want suffix ssssss", logPath)
	}
	if !strings.HasSuffix(metaPath, ".jsonl") {
		t.Fatalf("metaPath = %q, want .jsonl suffix", metaPath)
	}
	logBase := strings.TrimSuffix(filepath.Base(logPath), ".log")
	metaBase := strings.TrimSuffix(filepath.Base(metaPath), ".jsonl")
	if logBase != metaBase {
		t.Fatalf("log/meta base mismatch: %q vs %q", logBase, metaBase)
	}
}
