package ops

import (
	"io/fs"
	"os"
	"path/filepath"
	"testing"
	"testing/fstest"

	"loopr/internal/agents"
)

func testEmbeddedFS() fs.FS {
	return fstest.MapFS{
		"codex-skills/loopr-demo/README.md": {
			Data: []byte("hello"),
		},
		"codex-skills/loopr-demo/scripts/run.sh": {
			Data: []byte("#!/bin/sh\necho ok\n"),
		},
	}
}

func testAgent(skillsRoot string) agents.Spec {
	return agents.Spec{
		Name:         "codex",
		EmbeddedFS:   testEmbeddedFS(),
		EmbeddedRoot: "codex-skills",
		SkillsRoot: func() (string, error) {
			return skillsRoot, nil
		},
	}
}

func TestInstallWritesSkills(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	report, err := Install(agent, nil, false)
	if err != nil {
		t.Fatalf("Install error: %v", err)
	}
	if len(report.Installed) != 1 || report.Installed[0] != "loopr-demo" {
		t.Fatalf("Installed = %#v, want [loopr-demo]", report.Installed)
	}
	if report.BackupPath != "" {
		t.Fatalf("BackupPath = %q, want empty", report.BackupPath)
	}

	readme := filepath.Join(skillsRoot, "loopr-demo", "README.md")
	if _, err := os.Stat(readme); err != nil {
		t.Fatalf("README missing: %v", err)
	}

	script := filepath.Join(skillsRoot, "loopr-demo", "scripts", "run.sh")
	info, err := os.Stat(script)
	if err != nil {
		t.Fatalf("script missing: %v", err)
	}
	if info.Mode()&0o111 == 0 {
		t.Fatalf("script mode = %v, want executable bit set", info.Mode())
	}
}

func TestInstallBacksUpModifiedSkills(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	if _, err := Install(agent, nil, false); err != nil {
		t.Fatalf("initial Install error: %v", err)
	}

	modified := filepath.Join(skillsRoot, "loopr-demo", "README.md")
	if err := os.WriteFile(modified, []byte("local"), 0o644); err != nil {
		t.Fatalf("write modified README: %v", err)
	}

	report, err := Install(agent, nil, false)
	if err != nil {
		t.Fatalf("Install error: %v", err)
	}
	if report.BackupPath == "" {
		t.Fatalf("BackupPath empty, want backup")
	}
	if len(report.Updated) != 1 || report.Updated[0] != "loopr-demo" {
		t.Fatalf("Updated = %#v, want [loopr-demo]", report.Updated)
	}

	backupReadme := filepath.Join(report.BackupPath, "loopr-demo", "README.md")
	data, err := os.ReadFile(backupReadme)
	if err != nil {
		t.Fatalf("backup README missing: %v", err)
	}
	if string(data) != "local" {
		t.Fatalf("backup README = %q, want %q", string(data), "local")
	}
}

func TestUninstallRemovesSkillsWithBackup(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	if _, err := Install(agent, nil, false); err != nil {
		t.Fatalf("Install error: %v", err)
	}

	report, err := Uninstall(agent, nil, false)
	if err != nil {
		t.Fatalf("Uninstall error: %v", err)
	}
	if report.BackupPath == "" {
		t.Fatalf("BackupPath empty, want backup")
	}
	if len(report.Removed) != 1 || report.Removed[0] != "loopr-demo" {
		t.Fatalf("Removed = %#v, want [loopr-demo]", report.Removed)
	}

	if _, err := os.Stat(filepath.Join(skillsRoot, "loopr-demo")); !os.IsNotExist(err) {
		t.Fatalf("skill still present or unexpected error: %v", err)
	}
	if _, err := os.Stat(filepath.Join(report.BackupPath, "loopr-demo")); err != nil {
		t.Fatalf("backup missing: %v", err)
	}
}

func TestUninstallForceSkipsBackup(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	if _, err := Install(agent, nil, false); err != nil {
		t.Fatalf("Install error: %v", err)
	}

	report, err := Uninstall(agent, nil, true)
	if err != nil {
		t.Fatalf("Uninstall error: %v", err)
	}
	if report.BackupPath != "" {
		t.Fatalf("BackupPath = %q, want empty", report.BackupPath)
	}
	if _, err := os.Stat(filepath.Join(skillsRoot, "loopr-demo")); !os.IsNotExist(err) {
		t.Fatalf("skill still present or unexpected error: %v", err)
	}
	if _, err := os.Stat(filepath.Join(skillsRoot, ".backup")); err == nil {
		t.Fatalf(".backup exists, want none")
	}
}

func TestDoctorReportsMissingAndDrifted(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	report, err := Doctor(agent, nil)
	if err != nil {
		t.Fatalf("Doctor error: %v", err)
	}
	if len(report.Skills) != 1 || report.Skills[0].Status != "missing" {
		t.Fatalf("missing status = %#v, want missing", report.Skills)
	}

	if _, err := Install(agent, nil, false); err != nil {
		t.Fatalf("Install error: %v", err)
	}
	modified := filepath.Join(skillsRoot, "loopr-demo", "README.md")
	if err := os.WriteFile(modified, []byte("local"), 0o644); err != nil {
		t.Fatalf("write modified README: %v", err)
	}

	report, err = Doctor(agent, nil)
	if err != nil {
		t.Fatalf("Doctor error: %v", err)
	}
	if len(report.Skills) != 1 || report.Skills[0].Status != "drifted" {
		t.Fatalf("drifted status = %#v, want drifted", report.Skills)
	}
	if len(report.Skills[0].Drifted) != 1 || report.Skills[0].Drifted[0] != "README.md" {
		t.Fatalf("drifted files = %#v, want README.md", report.Skills[0].Drifted)
	}
}

func TestDoctorReportsExtraSkills(t *testing.T) {
	skillsRoot := t.TempDir()
	agent := testAgent(skillsRoot)

	extraDir := filepath.Join(skillsRoot, "loopr-extra")
	if err := os.MkdirAll(extraDir, 0o755); err != nil {
		t.Fatalf("mkdir extra: %v", err)
	}
	if err := os.WriteFile(filepath.Join(extraDir, "README.md"), []byte("extra"), 0o644); err != nil {
		t.Fatalf("write extra: %v", err)
	}

	report, err := Doctor(agent, nil)
	if err != nil {
		t.Fatalf("Doctor error: %v", err)
	}
	if len(report.ExtraSkills) != 1 || report.ExtraSkills[0] != "loopr-extra" {
		t.Fatalf("ExtraSkills = %#v, want [loopr-extra]", report.ExtraSkills)
	}
}
