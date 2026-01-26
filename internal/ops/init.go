package ops

import (
	"crypto/rand"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"loopr/internal/version"
)

var defaultAllowFiles = []string{
	"readme*",
	"license*",
	"changelog*",
	"contributing*",
	"code_of_conduct*",
	"security*",
	"notice*",
	".gitignore",
	".gitattributes",
	".editorconfig",
	"agents.md",
}

var defaultAllowDirs = map[string]struct{}{
	".git":    {},
	".github": {},
	".vscode": {},
	"docs":    {},
	"specs":   {},
}

const (
	repoIDLength   = 6
	repoIDAlphabet = "useandom26T198340PX75pxJACKVERYMINDBUSHWOLFGQZbfghjklqvwyzrict"
)

type InitOptions struct {
	Root          string
	SpecsDir      string
	AllowExisting bool
	Now           func() time.Time
	Rand          io.Reader
}

type InitReport struct {
	Root                    string
	SpecsDir                string
	LooprDir                string
	RepoID                  string
	RepoIDCreated           bool
	InitStatePath           string
	InitStateCreated        bool
	Mode                    string
	Signals                 []string
	DecisionsDir            string
	DecisionTemplatePath    string
	DecisionTemplateCreated bool
	TranscriptsDir          string
	SessionLogPath          string
	SessionMetaPath         string
}

type NonGreenfieldError struct {
	Root    string
	Signals []string
}

func (e NonGreenfieldError) Error() string {
	return fmt.Sprintf("non-greenfield signals detected under %s (rerun with --allow-existing)", e.Root)
}

type initState struct {
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

func Init(opts InitOptions) (InitReport, error) {
	root := strings.TrimSpace(opts.Root)
	if root == "" {
		root = "."
	}
	specsDir := strings.TrimSpace(opts.SpecsDir)
	if specsDir == "" {
		specsDir = "specs"
	}
	absRoot, err := filepath.Abs(root)
	if err != nil {
		return InitReport{}, err
	}

	looprSpecs := filepath.Join(absRoot, specsDir)
	looprDir := filepath.Join(looprSpecs, ".loopr")
	initStatePath := filepath.Join(looprDir, "init-state.json")
	repoIDPath := filepath.Join(looprDir, "repo-id")
	decisionsDir := filepath.Join(looprSpecs, "decisions")
	decisionTemplate := filepath.Join(decisionsDir, "template.md")

	now := opts.Now
	if now == nil {
		now = time.Now
	}
	randReader := opts.Rand
	if randReader == nil {
		randReader = rand.Reader
	}

	report := InitReport{
		Root:                 absRoot,
		SpecsDir:             looprSpecs,
		LooprDir:             looprDir,
		InitStatePath:        initStatePath,
		DecisionsDir:         decisionsDir,
		DecisionTemplatePath: decisionTemplate,
	}

	initStateExists := fileExists(initStatePath)
	repoID, repoIDExists, err := readRepoID(repoIDPath)
	if err != nil {
		return report, err
	}

	var signals []string
	if !initStateExists {
		signals, err = scanSignals(absRoot)
		if err != nil {
			return report, err
		}
	}

	if !initStateExists {
		if !repoIDExists {
			if len(signals) > 0 && !opts.AllowExisting {
				return report, NonGreenfieldError{Root: absRoot, Signals: signals}
			}
		}
		mode := "greenfield"
		if repoIDExists || len(signals) > 0 {
			mode = "existing"
		}
		if err := EnsureDir(looprDir, 0o755); err != nil {
			return report, err
		}
		if err := ensureLooprGitignore(looprDir); err != nil {
			return report, err
		}
		if !repoIDExists {
			repoID, err = generateRepoID(randReader)
			if err != nil {
				return report, err
			}
			if err := WriteFileAtomic(repoIDPath, []byte(repoID+"\n"), 0o644); err != nil {
				return report, err
			}
			report.RepoIDCreated = true
		}
		transcriptsDir := filepath.Join(looprDir, "transcripts", repoID)
		if err := EnsureDir(transcriptsDir, 0o755); err != nil {
			return report, err
		}
		created, err := ensureDecisionTemplate(decisionsDir, decisionTemplate)
		if err != nil {
			return report, err
		}
		report.DecisionTemplateCreated = created

		state := initState{
			InitializedAt: formatInitTime(now().UTC()),
			Mode:          mode,
			Signals:       signals,
			SchemaVersion: 1,
			SpecsDir:      specsDir,
			AllowExisting: opts.AllowExisting,
			LooprVersion:  version.Version,
			LooprCommit:   version.Commit,
			LooprDate:     version.Date,
		}
		data, err := json.MarshalIndent(state, "", "  ")
		if err != nil {
			return report, err
		}
		data = append(data, '\n')
		if err := WriteFileAtomic(initStatePath, data, 0o644); err != nil {
			return report, err
		}
		report.InitStateCreated = true
		report.Mode = mode
		report.Signals = signals
	} else {
		if err := EnsureDir(looprDir, 0o755); err != nil {
			return report, err
		}
		if err := ensureLooprGitignore(looprDir); err != nil {
			return report, err
		}
		if !repoIDExists {
			repoID, err = generateRepoID(randReader)
			if err != nil {
				return report, err
			}
			if err := WriteFileAtomic(repoIDPath, []byte(repoID+"\n"), 0o644); err != nil {
				return report, err
			}
			report.RepoIDCreated = true
		}
		transcriptsDir := filepath.Join(looprDir, "transcripts", repoID)
		if err := EnsureDir(transcriptsDir, 0o755); err != nil {
			return report, err
		}
		created, err := ensureDecisionTemplate(decisionsDir, decisionTemplate)
		if err != nil {
			return report, err
		}
		report.DecisionTemplateCreated = created
	}

	report.RepoID = repoID
	report.TranscriptsDir = filepath.Join(looprDir, "transcripts", repoID)

	nowTime := now().UTC()
	timestamp := nowTime.Format("20060102-150405")
	report.SessionLogPath = filepath.Join(report.TranscriptsDir, fmt.Sprintf("session-%s.log", timestamp))
	report.SessionMetaPath = filepath.Join(report.TranscriptsDir, fmt.Sprintf("session-%s.jsonl", timestamp))

	return report, nil
}

func ensureLooprGitignore(looprDir string) error {
	path := filepath.Join(looprDir, ".gitignore")
	if fileExists(path) {
		return nil
	}
	body := strings.Join([]string{
		"# Loopr transcripts are local-only.",
		"transcripts/",
		"session-*.log",
		"session-*.jsonl",
		"",
	}, "\n")
	return WriteFileAtomic(path, []byte(body), 0o644)
}

func scanSignals(root string) ([]string, error) {
	entries, err := os.ReadDir(root)
	if err != nil {
		return nil, err
	}
	var signals []string
	for _, entry := range entries {
		name := entry.Name()
		lower := strings.ToLower(name)
		if entry.IsDir() {
			if _, ok := defaultAllowDirs[lower]; ok {
				continue
			}
			signals = append(signals, name+"/")
			continue
		}
		if isAllowedFile(lower) {
			continue
		}
		signals = append(signals, name)
	}
	sort.Strings(signals)
	return signals, nil
}

func isAllowedFile(lowerName string) bool {
	for _, pattern := range defaultAllowFiles {
		matched, err := path.Match(pattern, lowerName)
		if err == nil && matched {
			return true
		}
	}
	return false
}

func ensureDecisionTemplate(dir, templatePath string) (bool, error) {
	if err := EnsureDir(dir, 0o755); err != nil {
		return false, err
	}
	if fileExists(templatePath) {
		return false, nil
	}
	body := strings.Join([]string{
		"# Title",
		"",
		"## Date",
		"",
		"## Status",
		"",
		"## Context",
		"",
		"## Decision",
		"",
		"## Alternatives",
		"",
		"## Consequences",
		"",
	}, "\n")
	if err := WriteFileAtomic(templatePath, []byte(body), 0o644); err != nil {
		return false, err
	}
	return true, nil
}

func readRepoID(path string) (string, bool, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return "", false, nil
		}
		return "", false, err
	}
	value := strings.TrimSpace(string(data))
	if value == "" {
		return "", true, fmt.Errorf("repo-id is empty at %s", path)
	}
	if !validRepoID(value) {
		return "", true, fmt.Errorf("repo-id %q at %s must be %d characters from the NanoID alphabet", value, path, repoIDLength)
	}
	return value, true, nil
}

func validRepoID(value string) bool {
	if len(value) != repoIDLength {
		return false
	}
	for i := 0; i < len(value); i++ {
		if strings.IndexByte(repoIDAlphabet, value[i]) == -1 {
			return false
		}
	}
	return true
}

func generateRepoID(reader io.Reader) (string, error) {
	const mask = byte(63)
	out := make([]byte, 0, repoIDLength)
	buf := make([]byte, repoIDLength)
	for len(out) < repoIDLength {
		need := repoIDLength - len(out)
		if need > len(buf) {
			need = len(buf)
		}
		if _, err := io.ReadFull(reader, buf[:need]); err != nil {
			return "", err
		}
		for _, b := range buf[:need] {
			idx := int(b & mask)
			if idx >= len(repoIDAlphabet) {
				continue
			}
			out = append(out, repoIDAlphabet[idx])
			if len(out) == repoIDLength {
				break
			}
		}
	}
	return string(out), nil
}

func formatInitTime(t time.Time) string {
	t = t.UTC()
	if t.Nanosecond() == 0 {
		return t.Format("2006-01-02T15:04:05") + "+00:00"
	}
	usec := t.Nanosecond() / 1000
	return fmt.Sprintf("%s.%06d+00:00", t.Format("2006-01-02T15:04:05"), usec)
}

func fileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}
