package ops

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func writeSpecsFile(t *testing.T, root, rel, content string) {
	t.Helper()
	path := filepath.Join(root, rel)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatalf("mkdir %s: %v", path, err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", path, err)
	}
}

func setupSpecsFixture(t *testing.T, root string, unitRequiredLine, testType string) {
	t.Helper()
	writeSpecsFile(t, root, ".loopr/init-state.json", `{"mode":"existing"}`)
	writeSpecsFile(t, root, "specs/spec.md", "# Spec\n\n## Testing Strategy\n- Stack: go test\n")
	writeSpecsFile(t, root, "specs/feature-order.yaml", strings.TrimSpace(`
version: 1
features:
  - slug: alpha
    title: Alpha
    depends_on: []
`)+"\n")
	writeSpecsFile(t, root, "specs/task-order.yaml", strings.TrimSpace(`
version: 1
features:
  - slug: alpha
    title: Alpha
    depends_on: []
    tasks:
      - id: "01"
        title: Task one
`)+"\n")
	writeSpecsFile(t, root, "specs/test-order.yaml", strings.TrimSpace(`
version: 1
features:
  - slug: alpha
    tasks:
      - id: "01"
        title: Task one
        tests:
          - id: "01"
            title: Test one
`)+"\n")
	writeSpecsFile(t, root, "specs/feature-alpha.md", strings.TrimSpace(`
# Feature: Alpha

## Summary

## Invariants / Properties
- 

## PBT Suitability
- Optional
`)+"\n")
	taskBody := strings.TrimSpace(`
# Task: Alpha / Task one

## Task ID
01

## Testing Notes
` + unitRequiredLine + `
`)
	writeSpecsFile(t, root, "specs/feature-alpha-task-01.md", taskBody+"\n")
	testBody := strings.TrimSpace(`
# Test: Test one

## Test ID
01

## Type
` + testType + `

## Purpose
- 
`)
	writeSpecsFile(t, root, "specs/feature-alpha-task-01-test-01.md", testBody+"\n")
}

func TestDoctorSpecsOK(t *testing.T) {
	root := t.TempDir()
	setupSpecsFixture(t, root, "Unit tests required: Yes", "Unit")

	report, err := DoctorSpecs(SpecsDoctorOptions{SpecsDir: filepath.Join(root, "specs")})
	if err != nil {
		t.Fatalf("DoctorSpecs error: %v", err)
	}
	if len(report.Errors) > 0 {
		t.Fatalf("unexpected errors: %#v", report.Errors)
	}
	if len(report.Warnings) > 0 {
		t.Fatalf("unexpected warnings: %#v", report.Warnings)
	}
}

func TestDoctorSpecsWarnsOnMissingUnitTest(t *testing.T) {
	root := t.TempDir()
	setupSpecsFixture(t, root, "Unit tests required: Yes", "Integration")

	report, err := DoctorSpecs(SpecsDoctorOptions{SpecsDir: filepath.Join(root, "specs")})
	if err != nil {
		t.Fatalf("DoctorSpecs error: %v", err)
	}
	if len(report.Errors) > 0 {
		t.Fatalf("unexpected errors: %#v", report.Errors)
	}
	if len(report.Warnings) != 1 || !strings.Contains(report.Warnings[0], "Missing unit test for task") {
		t.Fatalf("warnings = %#v, want missing unit test warning", report.Warnings)
	}
}

func TestDoctorSpecsSkipsUnitRequirementWhenNotSuitable(t *testing.T) {
	root := t.TempDir()
	setupSpecsFixture(t, root, "Unit tests required: No", "Integration")

	report, err := DoctorSpecs(SpecsDoctorOptions{SpecsDir: filepath.Join(root, "specs")})
	if err != nil {
		t.Fatalf("DoctorSpecs error: %v", err)
	}
	if len(report.Errors) > 0 {
		t.Fatalf("unexpected errors: %#v", report.Errors)
	}
	if len(report.Warnings) > 0 {
		t.Fatalf("unexpected warnings: %#v", report.Warnings)
	}
}

func TestDoctorSpecsEnforcesUnitTests(t *testing.T) {
	root := t.TempDir()
	setupSpecsFixture(t, root, "Unit tests required: Yes", "Integration")

	report, err := DoctorSpecs(SpecsDoctorOptions{
		SpecsDir:         filepath.Join(root, "specs"),
		EnforceUnitTests: true,
	})
	if err != nil {
		t.Fatalf("DoctorSpecs error: %v", err)
	}
	if len(report.Errors) != 1 || !strings.Contains(report.Errors[0], "Missing unit test for task") {
		t.Fatalf("errors = %#v, want missing unit test error", report.Errors)
	}
}
