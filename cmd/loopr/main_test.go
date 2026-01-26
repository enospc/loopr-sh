package main

import (
	"io"
	"os"
	"reflect"
	"strings"
	"testing"

	"loopr/internal/agents"
	"loopr/internal/ops"
)

func TestSplitOnDoubleDash(t *testing.T) {
	cases := []struct {
		name      string
		args      []string
		looprArgs []string
		codexArgs []string
	}{
		{
			name:      "delimiter only",
			args:      []string{"--", "--help"},
			looprArgs: nil,
			codexArgs: []string{"--help"},
		},
		{
			name:      "loopr flags then codex args",
			args:      []string{"--loopr-root", "/tmp/root", "--", "--help"},
			looprArgs: []string{"--loopr-root", "/tmp/root"},
			codexArgs: []string{"--help"},
		},
		{
			name:      "no delimiter",
			args:      []string{"--help"},
			looprArgs: []string{"--help"},
			codexArgs: nil,
		},
	}

	for _, tc := range cases {
		looprArgs, codexArgs := splitOnDoubleDash(tc.args)
		if !reflect.DeepEqual(looprArgs, tc.looprArgs) {
			t.Fatalf("%s: looprArgs = %#v, want %#v", tc.name, looprArgs, tc.looprArgs)
		}
		if !reflect.DeepEqual(codexArgs, tc.codexArgs) {
			t.Fatalf("%s: codexArgs = %#v, want %#v", tc.name, codexArgs, tc.codexArgs)
		}
	}
}

func TestSplitListTrimsAndIgnoresEmpty(t *testing.T) {
	list := splitList("loopr-prd, ,loopr-doctor,,")
	expected := []string{"loopr-prd", "loopr-doctor"}
	if !reflect.DeepEqual(list, expected) {
		t.Fatalf("splitList = %#v, want %#v", list, expected)
	}

	empty := splitList(" , , ")
	if empty != nil {
		t.Fatalf("splitList empty = %#v, want nil", empty)
	}
}

func TestResolveAgents(t *testing.T) {
	specs, err := resolveAgents("codex", false)
	if err != nil {
		t.Fatalf("resolveAgents codex error: %v", err)
	}
	if len(specs) != 1 || specs[0].Name != "codex" {
		t.Fatalf("resolveAgents codex = %#v, want codex", specs)
	}

	allSpecs, err := resolveAgents("codex", true)
	if err != nil {
		t.Fatalf("resolveAgents all error: %v", err)
	}
	if len(allSpecs) != 1 || allSpecs[0].Name != "codex" {
		t.Fatalf("resolveAgents all = %#v, want codex", allSpecs)
	}

	if _, err := resolveAgents("unknown", false); err == nil {
		t.Fatalf("resolveAgents unknown = nil error, want error")
	}
}

func TestRunListMatchesDoctorStatus(t *testing.T) {
	skillsBase := t.TempDir()
	t.Setenv("CODEX_HOME", skillsBase)

	agent, err := agents.Resolve("codex")
	if err != nil {
		t.Fatalf("Resolve agent error: %v", err)
	}
	if _, err := ops.Install(agent, []string{"loopr-prd"}, false); err != nil {
		t.Fatalf("Install error: %v", err)
	}

	stdout := os.Stdout
	r, w, err := os.Pipe()
	if err != nil {
		t.Fatalf("pipe error: %v", err)
	}
	os.Stdout = w
	defer func() {
		os.Stdout = stdout
	}()

	runList([]string{"--only", "loopr-prd"})
	_ = w.Close()

	output, err := io.ReadAll(r)
	if err != nil {
		t.Fatalf("read output error: %v", err)
	}

	report, err := ops.Doctor(agent, []string{"loopr-prd"})
	if err != nil {
		t.Fatalf("Doctor error: %v", err)
	}
	if len(report.Skills) != 1 {
		t.Fatalf("Doctor skills = %#v, want 1 entry", report.Skills)
	}
	expectedLine := "loopr-prd\t" + report.Skills[0].Status
	if !strings.Contains(string(output), expectedLine) {
		t.Fatalf("output missing %q: %s", expectedLine, string(output))
	}
}
