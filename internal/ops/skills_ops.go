package ops

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"loopr/internal/agents"
	"loopr/internal/skills"
)

type InstallReport struct {
	Installed  []string
	Updated    []string
	Skipped    []string
	BackupPath string
}

type UninstallReport struct {
	Removed    []string
	BackupPath string
}

type SkillReport struct {
	Name    string
	Status  string
	Missing []string
	Drifted []string
}

type DoctorReport struct {
	Skills      []SkillReport
	ExtraSkills []string
}

func DefaultSkillFilter(name string) bool {
	return strings.HasPrefix(name, "loopr-")
}

func parseOnlyList(only []string) map[string]struct{} {
	set := map[string]struct{}{}
	for _, item := range only {
		name := strings.TrimSpace(item)
		if name == "" {
			continue
		}
		set[name] = struct{}{}
	}
	return set
}

func FilterSkills(index *skills.EmbeddedIndex, only []string) []skills.Skill {
	set := parseOnlyList(only)
	var out []skills.Skill
	for _, skill := range index.Skills {
		if len(set) > 0 {
			if _, ok := set[skill.Name]; !ok {
				continue
			}
		} else if !DefaultSkillFilter(skill.Name) {
			continue
		}
		out = append(out, skill)
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Name < out[j].Name })
	return out
}

func Install(agent agents.Spec, only []string, force bool) (InstallReport, error) {
	index, err := skills.LoadEmbedded(agent.EmbeddedFS, agent.EmbeddedRoot)
	if err != nil {
		return InstallReport{}, err
	}
	skillsRoot, err := agent.SkillsRoot()
	if err != nil {
		return InstallReport{}, err
	}
	if err := EnsureDir(skillsRoot, 0o755); err != nil {
		return InstallReport{}, err
	}

	skillList := FilterSkills(index, only)
	if len(skillList) == 0 {
		return InstallReport{}, fmt.Errorf("no matching skills to install")
	}

	skillsToBackup := map[string]struct{}{}
	skillChange := map[string]bool{}
	skillExists := map[string]bool{}

	for _, skill := range skillList {
		skillDir := filepath.Join(skillsRoot, skill.Name)
		if _, err := os.Stat(skillDir); err == nil {
			skillExists[skill.Name] = true
		}
		for _, entry := range skill.Files {
			target := filepath.Join(skillDir, entry.SubPath)
			data, err := os.ReadFile(target)
			if err != nil {
				if !os.IsNotExist(err) {
					return InstallReport{}, fmt.Errorf("read %s: %w", target, err)
				}
				skillChange[skill.Name] = true
				if skillExists[skill.Name] {
					skillsToBackup[skill.Name] = struct{}{}
				}
				continue
			}
			if skills.HashFile(data) != entry.Hash {
				skillChange[skill.Name] = true
				skillsToBackup[skill.Name] = struct{}{}
			}
		}
	}

	var backupPath string
	if len(skillsToBackup) > 0 {
		backupPath = filepath.Join(skillsRoot, ".backup", "loopr-"+time.Now().UTC().Format("20060102-150405"))
		if err := EnsureDir(backupPath, 0o755); err != nil {
			return InstallReport{}, err
		}
		for name := range skillsToBackup {
			src := filepath.Join(skillsRoot, name)
			if _, err := os.Stat(src); err != nil {
				if os.IsNotExist(err) {
					continue
				}
				return InstallReport{}, fmt.Errorf("stat %s: %w", src, err)
			}
			dst := filepath.Join(backupPath, name)
			if err := CopyDir(src, dst); err != nil {
				if force {
					continue
				}
				return InstallReport{}, fmt.Errorf("backup %s: %w", name, err)
			}
		}
	}

	report := InstallReport{BackupPath: backupPath}
	for _, skill := range skillList {
		skillDir := filepath.Join(skillsRoot, skill.Name)
		changed := skillChange[skill.Name]
		for _, entry := range skill.Files {
			target := filepath.Join(skillDir, entry.SubPath)
			data, err := os.ReadFile(target)
			if err == nil && skills.HashFile(data) == entry.Hash {
				info, statErr := os.Stat(target)
				if statErr == nil && info.Mode().Perm() != entry.Mode.Perm() {
					if err := os.Chmod(target, entry.Mode.Perm()); err != nil {
						return InstallReport{}, fmt.Errorf("chmod %s: %w", target, err)
					}
				}
				continue
			}
			if err := WriteFileAtomic(target, entry.Data, entry.Mode); err != nil {
				return InstallReport{}, err
			}
		}
		if changed {
			if skillExists[skill.Name] {
				report.Updated = append(report.Updated, skill.Name)
			} else {
				report.Installed = append(report.Installed, skill.Name)
			}
		} else {
			report.Skipped = append(report.Skipped, skill.Name)
		}
	}

	sort.Strings(report.Installed)
	sort.Strings(report.Updated)
	sort.Strings(report.Skipped)
	return report, nil
}

func Doctor(agent agents.Spec, only []string) (DoctorReport, error) {
	index, err := skills.LoadEmbedded(agent.EmbeddedFS, agent.EmbeddedRoot)
	if err != nil {
		return DoctorReport{}, err
	}
	skillsRoot, err := agent.SkillsRoot()
	if err != nil {
		return DoctorReport{}, err
	}

	skillList := FilterSkills(index, only)
	if len(skillList) == 0 {
		return DoctorReport{}, fmt.Errorf("no matching skills to check")
	}

	report := DoctorReport{}
	seen := map[string]struct{}{}

	for _, skill := range skillList {
		seen[skill.Name] = struct{}{}
		skillDir := filepath.Join(skillsRoot, skill.Name)
		_, err := os.Stat(skillDir)
		if err != nil {
			report.Skills = append(report.Skills, SkillReport{Name: skill.Name, Status: "missing"})
			continue
		}

		sr := SkillReport{Name: skill.Name, Status: "installed"}
		for _, entry := range skill.Files {
			target := filepath.Join(skillDir, entry.SubPath)
			data, err := os.ReadFile(target)
			if err != nil {
				sr.Missing = append(sr.Missing, entry.SubPath)
				continue
			}
			if skills.HashFile(data) != entry.Hash {
				sr.Drifted = append(sr.Drifted, entry.SubPath)
				continue
			}
			info, statErr := os.Stat(target)
			if statErr == nil && info.Mode().Perm() != entry.Mode.Perm() {
				sr.Drifted = append(sr.Drifted, entry.SubPath)
			}
		}
		if len(sr.Missing) > 0 || len(sr.Drifted) > 0 {
			sr.Status = "drifted"
		}
		sort.Strings(sr.Missing)
		sort.Strings(sr.Drifted)
		report.Skills = append(report.Skills, sr)
	}

	entries, err := os.ReadDir(skillsRoot)
	if err == nil {
		for _, entry := range entries {
			if !entry.IsDir() {
				continue
			}
			name := entry.Name()
			if !DefaultSkillFilter(name) {
				continue
			}
			if _, ok := seen[name]; !ok {
				report.ExtraSkills = append(report.ExtraSkills, name)
			}
		}
	}

	sort.Slice(report.Skills, func(i, j int) bool { return report.Skills[i].Name < report.Skills[j].Name })
	sort.Strings(report.ExtraSkills)
	return report, nil
}

func Uninstall(agent agents.Spec, only []string, force bool) (UninstallReport, error) {
	index, err := skills.LoadEmbedded(agent.EmbeddedFS, agent.EmbeddedRoot)
	if err != nil {
		return UninstallReport{}, err
	}
	skillsRoot, err := agent.SkillsRoot()
	if err != nil {
		return UninstallReport{}, err
	}

	skillList := FilterSkills(index, only)
	if len(skillList) == 0 {
		return UninstallReport{}, fmt.Errorf("no matching skills to uninstall")
	}

	var backupPath string
	if !force {
		backupPath = filepath.Join(skillsRoot, ".backup", "loopr-"+time.Now().UTC().Format("20060102-150405"))
		if err := EnsureDir(backupPath, 0o755); err != nil {
			return UninstallReport{}, err
		}
	}

	report := UninstallReport{BackupPath: backupPath}
	for _, skill := range skillList {
		path := filepath.Join(skillsRoot, skill.Name)
		if _, err := os.Stat(path); err != nil {
			if os.IsNotExist(err) {
				continue
			}
			return UninstallReport{}, err
		}
		if !force {
			dst := filepath.Join(backupPath, skill.Name)
			if err := CopyDir(path, dst); err != nil {
				return UninstallReport{}, fmt.Errorf("backup %s: %w", skill.Name, err)
			}
		}
		if err := os.RemoveAll(path); err != nil {
			return UninstallReport{}, fmt.Errorf("remove %s: %w", path, err)
		}
		report.Removed = append(report.Removed, skill.Name)
	}
	return report, nil
}
