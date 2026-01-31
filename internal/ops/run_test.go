package ops

import (
	"os"
	"path/filepath"
	"testing"
)

func writeSpecFile(t *testing.T, root, rel string) {
	t.Helper()
	path := filepath.Join(root, rel)
	if err := EnsureDir(filepath.Dir(path), 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", path, err)
	}
	if err := os.WriteFile(path, []byte("ok\n"), 0o644); err != nil {
		t.Fatalf("write %s: %v", path, err)
	}
}

func TestPlanStepsDefaultsToPrd(t *testing.T) {
	root := t.TempDir()
	steps, err := planSteps(root, RunOptions{})
	if err != nil {
		t.Fatalf("planSteps error: %v", err)
	}
	if len(steps) == 0 || steps[0].Name != "prd" {
		t.Fatalf("steps[0] = %#v, want prd", steps)
	}
}

func TestPlanStepsSkipsPrdWhenPresent(t *testing.T) {
	root := t.TempDir()
	writeSpecFile(t, root, "specs/prd.md")

	steps, err := planSteps(root, RunOptions{})
	if err != nil {
		t.Fatalf("planSteps error: %v", err)
	}
	if len(steps) == 0 || steps[0].Name != "spec" {
		t.Fatalf("steps[0] = %#v, want spec", steps)
	}
}

func TestPlanStepsReturnsExecuteWhenAllOutputsPresent(t *testing.T) {
	root := t.TempDir()
	writeSpecFile(t, root, "specs/prd.md")
	writeSpecFile(t, root, "specs/spec.md")
	writeSpecFile(t, root, "specs/feature-order.yaml")
	writeSpecFile(t, root, "specs/feature-demo.md")
	writeSpecFile(t, root, "specs/task-order.yaml")
	writeSpecFile(t, root, "specs/feature-demo-task-01.md")
	writeSpecFile(t, root, "specs/test-order.yaml")
	writeSpecFile(t, root, "specs/feature-demo-task-01-test-01.md")

	steps, err := planSteps(root, RunOptions{})
	if err != nil {
		t.Fatalf("planSteps error: %v", err)
	}
	if len(steps) != 1 || steps[0].Name != "execute" {
		t.Fatalf("steps = %#v, want execute only", steps)
	}
}

func TestRunWorkflowDryRunShowsAllSteps(t *testing.T) {
	root := t.TempDir()
	prev, err := os.Getwd()
	if err != nil {
		t.Fatalf("getwd error: %v", err)
	}
	if err := os.Chdir(root); err != nil {
		t.Fatalf("chdir error: %v", err)
	}
	t.Cleanup(func() {
		_ = os.Chdir(prev)
	})

	report, err := RunWorkflow(RunOptions{Codex: false})
	if err != nil {
		t.Fatalf("RunWorkflow error: %v", err)
	}
	if len(report.Steps) != len(defaultRunSteps()) {
		t.Fatalf("steps = %d, want %d", len(report.Steps), len(defaultRunSteps()))
	}
}

func TestPlanStepsRange(t *testing.T) {
	root := t.TempDir()
	steps, err := planSteps(root, RunOptions{From: "tasks", To: "tests"})
	if err != nil {
		t.Fatalf("planSteps error: %v", err)
	}
	if len(steps) != 2 || steps[0].Name != "tasks" || steps[1].Name != "tests" {
		t.Fatalf("steps = %#v, want tasks->tests", steps)
	}
}

func TestRunWorkflowDryRunDoesNotRequireRepoID(t *testing.T) {
	root := t.TempDir()
	prev, err := os.Getwd()
	if err != nil {
		t.Fatalf("getwd error: %v", err)
	}
	if err := os.Chdir(root); err != nil {
		t.Fatalf("chdir error: %v", err)
	}
	t.Cleanup(func() {
		_ = os.Chdir(prev)
	})

	report, err := RunWorkflow(RunOptions{Codex: false})
	if err != nil {
		t.Fatalf("RunWorkflow error: %v", err)
	}
	if len(report.Steps) == 0 || report.Steps[0].Name != "prd" {
		t.Fatalf("steps[0] = %#v, want prd", report.Steps)
	}
	if _, err := os.Stat(filepath.Join(root, ".loopr", "handoff.md")); err == nil {
		t.Fatalf("handoff.md created during dry run, want none")
	}
}

func TestShouldAppendPrompt(t *testing.T) {
	cases := []struct {
		name string
		args []string
		want bool
	}{
		{name: "no args", args: nil, want: true},
		{name: "help flag", args: []string{"--help"}, want: false},
		{name: "short help", args: []string{"-h"}, want: false},
		{name: "version flag", args: []string{"--version"}, want: false},
		{name: "subcommand", args: []string{"exec"}, want: false},
		{name: "flags with values", args: []string{"--model", "o3"}, want: true},
		{name: "config value", args: []string{"--config", "model=\"o3\""}, want: true},
		{name: "cd flag", args: []string{"--cd", "/tmp"}, want: true},
	}

	for _, tc := range cases {
		got := shouldAppendPrompt(tc.args)
		if got != tc.want {
			t.Fatalf("%s: shouldAppendPrompt(%v) = %v, want %v", tc.name, tc.args, got, tc.want)
		}
	}
}
