package ops

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func writeRepoID(t *testing.T, root, repoID string) {
	t.Helper()
	path := filepath.Join(root, ".loopr")
	if err := os.MkdirAll(path, 0o755); err != nil {
		t.Fatalf("mkdir loopr dir: %v", err)
	}
	if err := os.WriteFile(filepath.Join(path, "repo-id"), []byte(repoID), 0o644); err != nil {
		t.Fatalf("write repo-id: %v", err)
	}
}

func TestResolveLooprRootPrefersOverride(t *testing.T) {
	rootA := t.TempDir()
	rootB := t.TempDir()
	writeRepoID(t, rootA, "aaaaaa")
	writeRepoID(t, rootB, "bbbbbb")

	resolvedRoot, repoID, err := ResolveLooprRoot(t.TempDir(), rootB)
	if err != nil {
		t.Fatalf("ResolveLooprRoot error: %v", err)
	}
	absB, _ := filepath.Abs(rootB)
	if resolvedRoot != absB {
		t.Fatalf("root = %q, want %q", resolvedRoot, absB)
	}
	if repoID != "bbbbbb" {
		t.Fatalf("repoID = %q, want bbbbbb", repoID)
	}
}

func TestResolveLooprRootRejectsInvalidOverrideRepoID(t *testing.T) {
	root := t.TempDir()
	writeRepoID(t, root, "______")

	_, _, err := ResolveLooprRoot(t.TempDir(), root)
	if err == nil {
		t.Fatalf("ResolveLooprRoot error = nil, want error")
	}
	if !strings.Contains(err.Error(), "must be 6 characters from the NanoID alphabet") {
		t.Fatalf("error = %q, want repo-id format message", err.Error())
	}
}

func TestResolveLooprRootSearchesUpwards(t *testing.T) {
	root := t.TempDir()
	writeRepoID(t, root, "cccccc")

	nested := filepath.Join(root, "a", "b")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}

	resolvedRoot, repoID, err := ResolveLooprRoot(nested, "")
	if err != nil {
		t.Fatalf("ResolveLooprRoot error: %v", err)
	}
	absRoot, _ := filepath.Abs(root)
	if resolvedRoot != absRoot {
		t.Fatalf("root = %q, want %q", resolvedRoot, absRoot)
	}
	if repoID != "cccccc" {
		t.Fatalf("repoID = %q, want cccccc", repoID)
	}
}

func TestResolveLooprRootRejectsInvalidAncestorRepoID(t *testing.T) {
	root := t.TempDir()
	writeRepoID(t, root, "______")

	nested := filepath.Join(root, "a", "b")
	if err := os.MkdirAll(nested, 0o755); err != nil {
		t.Fatalf("mkdir nested: %v", err)
	}

	_, _, err := ResolveLooprRoot(nested, "")
	if err == nil {
		t.Fatalf("ResolveLooprRoot error = nil, want error")
	}
	if !strings.Contains(err.Error(), "must be 6 characters from the NanoID alphabet") {
		t.Fatalf("error = %q, want repo-id format message", err.Error())
	}
}

func TestResolveLooprRootMissing(t *testing.T) {
	_, _, err := ResolveLooprRoot(t.TempDir(), "")
	if err == nil {
		t.Fatalf("ResolveLooprRoot error = nil, want error")
	}
	if !strings.Contains(err.Error(), ".loopr/repo-id") {
		t.Fatalf("error = %q, want missing repo-id message", err.Error())
	}
	if !strings.Contains(err.Error(), "loopr init") {
		t.Fatalf("error = %q, want loopr init hint", err.Error())
	}
}
