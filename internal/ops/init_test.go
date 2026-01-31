package ops

import (
	"bytes"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"loopr/internal/version"
)

type initStateFixture struct {
	InitializedAt string   `json:"initialized_at"`
	Mode          string   `json:"mode"`
	Signals       []string `json:"signals"`
	SchemaVersion int      `json:"schema_version"`
	SpecsDir      string   `json:"specs_dir"`
	AllowExisting bool     `json:"allow_existing"`
	LooprVersion  string   `json:"loopr_version"`
	LooprCommit   string   `json:"loopr_commit"`
	LooprDate     string   `json:"loopr_date"`
}

func TestInitGreenfieldCreatesRepoIDAndInitState(t *testing.T) {
	root := t.TempDir()
	now := time.Date(2026, 1, 25, 12, 34, 56, 123456000, time.UTC)

	report, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: false,
		Now:           func() time.Time { return now },
		Rand:          bytes.NewReader([]byte{0, 1, 2, 3, 4, 5}),
	})
	if err != nil {
		t.Fatalf("Init error: %v", err)
	}
	if report.RepoID != "useand" {
		t.Fatalf("RepoID = %q, want useand", report.RepoID)
	}
	if !report.RepoIDCreated {
		t.Fatalf("RepoIDCreated = false, want true")
	}
	if !report.InitStateCreated {
		t.Fatalf("InitStateCreated = false, want true")
	}
	if report.Mode != "greenfield" {
		t.Fatalf("Mode = %q, want greenfield", report.Mode)
	}

	stateBytes, err := os.ReadFile(report.InitStatePath)
	if err != nil {
		t.Fatalf("read init-state: %v", err)
	}
	var state initStateFixture
	if err := json.Unmarshal(stateBytes, &state); err != nil {
		t.Fatalf("unmarshal init-state: %v", err)
	}
	if state.InitializedAt != "2026-01-25T12:34:56.123456+00:00" {
		t.Fatalf("initialized_at = %q, want 2026-01-25T12:34:56.123456+00:00", state.InitializedAt)
	}
	if state.Mode != "greenfield" {
		t.Fatalf("state mode = %q, want greenfield", state.Mode)
	}
	if len(state.Signals) != 0 {
		t.Fatalf("signals = %#v, want empty", state.Signals)
	}
	if state.SchemaVersion != 1 {
		t.Fatalf("schema_version = %d, want 1", state.SchemaVersion)
	}
	if state.SpecsDir != "specs" {
		t.Fatalf("specs_dir = %q, want specs", state.SpecsDir)
	}
	if state.AllowExisting {
		t.Fatalf("allow_existing = true, want false")
	}
	if state.LooprVersion != version.Version {
		t.Fatalf("loopr_version = %q, want %q", state.LooprVersion, version.Version)
	}
	if state.LooprCommit != version.Commit {
		t.Fatalf("loopr_commit = %q, want %q", state.LooprCommit, version.Commit)
	}
	if state.LooprDate != version.Date {
		t.Fatalf("loopr_date = %q, want %q", state.LooprDate, version.Date)
	}

	templateBody, err := os.ReadFile(report.DecisionTemplatePath)
	if err != nil {
		t.Fatalf("read decision template: %v", err)
	}
	for _, heading := range []string{"# Title", "## Date", "## Status", "## Context", "## Decision", "## Alternatives", "## Consequences"} {
		if !strings.Contains(string(templateBody), heading) {
			t.Fatalf("template missing %q", heading)
		}
	}

	if _, err := os.Stat(report.TranscriptsDir); err != nil {
		t.Fatalf("transcripts dir missing: %v", err)
	}

	gitignorePath := filepath.Join(report.LooprDir, ".gitignore")
	gitignoreBody, err := os.ReadFile(gitignorePath)
	if err != nil {
		t.Fatalf("read .gitignore: %v", err)
	}
	if !strings.Contains(string(gitignoreBody), "transcripts/") {
		t.Fatalf(".gitignore missing transcripts/")
	}
}

func TestInitRejectsNonGreenfieldWithoutAllowExisting(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "go.mod"), []byte("module example.com/test"), 0o644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}

	_, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: false,
	})
	if err == nil {
		t.Fatalf("Init error = nil, want error")
	}
	var ngErr NonGreenfieldError
	if !errors.As(err, &ngErr) {
		t.Fatalf("Init error = %v, want NonGreenfieldError", err)
	}
	if len(ngErr.Signals) == 0 || ngErr.Signals[0] != "go.mod" {
		t.Fatalf("signals = %#v, want go.mod", ngErr.Signals)
	}
	if _, err := os.Stat(filepath.Join(root, ".loopr", "init-state.json")); err == nil {
		t.Fatalf("init-state created, want none")
	}
}

func TestInitAllowExistingCreatesExistingMode(t *testing.T) {
	root := t.TempDir()
	if err := os.WriteFile(filepath.Join(root, "go.mod"), []byte("module example.com/test"), 0o644); err != nil {
		t.Fatalf("write go.mod: %v", err)
	}

	report, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: true,
		Rand:          bytes.NewReader([]byte{0, 1, 2, 3, 4, 5}),
	})
	if err != nil {
		t.Fatalf("Init error: %v", err)
	}
	if report.Mode != "existing" {
		t.Fatalf("Mode = %q, want existing", report.Mode)
	}
	if len(report.Signals) != 1 || report.Signals[0] != "go.mod" {
		t.Fatalf("Signals = %#v, want [go.mod]", report.Signals)
	}
}

func TestInitReusesRepoIDWhenPresent(t *testing.T) {
	root := t.TempDir()
	repoIDPath := filepath.Join(root, ".loopr", "repo-id")
	if err := os.MkdirAll(filepath.Dir(repoIDPath), 0o755); err != nil {
		t.Fatalf("mkdir loopr dir: %v", err)
	}
	if err := os.WriteFile(repoIDPath, []byte("abc123\n"), 0o644); err != nil {
		t.Fatalf("write repo-id: %v", err)
	}

	report, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: false,
	})
	if err != nil {
		t.Fatalf("Init error: %v", err)
	}
	if report.RepoID != "abc123" {
		t.Fatalf("RepoID = %q, want abc123", report.RepoID)
	}
	if report.RepoIDCreated {
		t.Fatalf("RepoIDCreated = true, want false")
	}
	if report.Mode != "existing" {
		t.Fatalf("Mode = %q, want existing", report.Mode)
	}
}

func TestInitCreatesRepoIDWhenInitStateExists(t *testing.T) {
	root := t.TempDir()
	initStatePath := filepath.Join(root, ".loopr", "init-state.json")
	if err := os.MkdirAll(filepath.Dir(initStatePath), 0o755); err != nil {
		t.Fatalf("mkdir loopr dir: %v", err)
	}
	if err := os.WriteFile(initStatePath, []byte("{\"mode\":\"existing\"}\n"), 0o644); err != nil {
		t.Fatalf("write init-state: %v", err)
	}

	report, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: false,
		Rand:          bytes.NewReader([]byte{0, 1, 2, 3, 4, 5}),
	})
	if err != nil {
		t.Fatalf("Init error: %v", err)
	}
	if report.RepoID != "useand" {
		t.Fatalf("RepoID = %q, want useand", report.RepoID)
	}
	if !report.RepoIDCreated {
		t.Fatalf("RepoIDCreated = false, want true")
	}
	if report.InitStateCreated {
		t.Fatalf("InitStateCreated = true, want false")
	}
}

func TestInitRejectsInvalidRepoID(t *testing.T) {
	root := t.TempDir()
	repoIDPath := filepath.Join(root, ".loopr", "repo-id")
	if err := os.MkdirAll(filepath.Dir(repoIDPath), 0o755); err != nil {
		t.Fatalf("mkdir loopr dir: %v", err)
	}
	if err := os.WriteFile(repoIDPath, []byte("______\n"), 0o644); err != nil {
		t.Fatalf("write repo-id: %v", err)
	}

	_, err := Init(InitOptions{
		Root:          root,
		SpecsDir:      "specs",
		AllowExisting: false,
	})
	if err == nil {
		t.Fatalf("Init error = nil, want error")
	}
	if !strings.Contains(err.Error(), "must be 6 characters from the NanoID alphabet") {
		t.Fatalf("error = %q, want repo-id format message", err.Error())
	}
}
