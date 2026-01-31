package ops

import (
	"bytes"
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"loopr/internal/agents"
	"loopr/internal/skills"
	"loopr/internal/version"
)

type CodexSession struct {
	RepoRoot string
	RepoID   string
	LogPath  string
	MetaPath string
	Command  []string
	Started  time.Time
}

type CodexOptions struct {
	LooprRoot string
}

func RunCodex(args []string, opts CodexOptions) (int, *CodexSession, error) {
	return runCodexInternal(args, opts, 0)
}

func RunCodexWithTimeout(args []string, opts CodexOptions, timeout time.Duration) (int, *CodexSession, error) {
	return runCodexInternal(args, opts, timeout)
}

func runCodexInternal(args []string, opts CodexOptions, timeout time.Duration) (int, *CodexSession, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return 1, nil, err
	}
	root, repoID, err := ResolveLooprRoot(cwd, opts.LooprRoot)
	if err != nil {
		return 1, nil, err
	}

	transcriptsDir := filepath.Join(root, ".loopr", "transcripts", repoID)
	if err := EnsureDir(transcriptsDir, 0o755); err != nil {
		return 1, nil, err
	}

	logPath, metaPath, err := newSessionPaths(transcriptsDir, time.Now().UTC(), randReader())
	if err != nil {
		return 1, nil, err
	}

	session := &CodexSession{
		RepoRoot: root,
		RepoID:   repoID,
		LogPath:  logPath,
		MetaPath: metaPath,
		Command:  append([]string{"codex"}, args...),
		Started:  time.Now().UTC(),
	}

	embeddedHash, err := embeddedSkillsHash()
	if err != nil {
		return 1, session, err
	}

	startMeta := map[string]any{
		"event":                "start",
		"ts":                   session.Started.Format(time.RFC3339Nano),
		"cwd":                  cwd,
		"cmd":                  session.Command,
		"log":                  filepath.Base(logPath),
		"loopr_version":        version.Version,
		"loopr_commit":         version.Commit,
		"loopr_date":           version.Date,
		"repo_root":            root,
		"repo_id":              repoID,
		"skills_embedded_hash": embeddedHash,
	}

	if commit, dirty := gitInfo(root); commit != "" {
		startMeta["git_commit"] = commit
		if dirty != nil {
			startMeta["git_dirty"] = *dirty
		}
	}

	if installedHash, err := installedSkillsHash(); err == nil && installedHash != "" {
		startMeta["skills_installed_hash"] = installedHash
	}

	if err := writeMeta(metaPath, startMeta); err != nil {
		return 1, session, err
	}

	code, err := runCodexWithLoggingTimeout(logPath, args, timeout)
	end := time.Now().UTC()
	_ = writeMeta(metaPath, map[string]any{
		"event":     "end",
		"ts":        end.Format(time.RFC3339Nano),
		"exit_code": code,
	})
	return code, session, err
}

func newSessionPaths(dir string, now time.Time, reader io.Reader) (string, string, error) {
	timestamp := now.UTC().Format("20060102-150405")
	for i := 0; i < 10; i++ {
		suffix, err := generateNanoID(reader, sessionIDLength)
		if err != nil {
			return "", "", err
		}
		base := fmt.Sprintf("session-%s-%s", timestamp, suffix)
		logPath := filepath.Join(dir, base+".log")
		metaPath := filepath.Join(dir, base+".jsonl")
		if exists(logPath) || exists(metaPath) {
			continue
		}
		return logPath, metaPath, nil
	}
	return "", "", fmt.Errorf("unable to allocate unique session paths")
}

func randReader() io.Reader {
	return rand.Reader
}

func exists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func runCodexWithLogging(logPath string, args []string) (int, error) {
	return runCodexWithLoggingTimeout(logPath, args, 0)
}

func runCodexWithLoggingTimeout(logPath string, args []string, timeout time.Duration) (int, error) {
	ctx := context.Background()
	if timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, timeout)
		defer cancel()
	}
	if scriptPath, err := exec.LookPath("script"); err == nil {
		cmdString := shellJoin(append([]string{"codex"}, args...))
		cmd := exec.CommandContext(ctx, scriptPath, "-q", "-f", "-c", cmdString, logPath)
		cmd.Stdin = os.Stdin
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		err = cmd.Run()
		return exitCode(err), err
	}

	file, err := os.OpenFile(logPath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
	if err != nil {
		return 1, err
	}
	defer file.Close()

	stdout := io.MultiWriter(os.Stdout, file)
	stderr := io.MultiWriter(os.Stderr, file)

	cmd := exec.CommandContext(ctx, "codex", args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = stdout
	cmd.Stderr = stderr
	cmd.Env = os.Environ()
	err = cmd.Run()
	return exitCode(err), err
}

func shellJoin(args []string) string {
	var out []string
	for _, arg := range args {
		if arg == "" {
			out = append(out, "''")
			continue
		}
		if strings.IndexFunc(arg, func(r rune) bool {
			switch r {
			case ' ', '\t', '\n', '\r', '\\', '\'', '"':
				return true
			default:
				return false
			}
		}) == -1 {
			out = append(out, arg)
			continue
		}
		escaped := strings.ReplaceAll(arg, "'", "'\\''")
		out = append(out, "'"+escaped+"'")
	}
	return strings.Join(out, " ")
}

func exitCode(err error) int {
	if err == nil {
		return 0
	}
	if exitErr, ok := err.(*exec.ExitError); ok {
		return exitErr.ExitCode()
	}
	return 1
}

func writeMeta(path string, payload map[string]any) error {
	data, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	file, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := file.Write(append(data, '\n')); err != nil {
		return err
	}
	return nil
}

func embeddedSkillsHash() (string, error) {
	spec, err := agents.Resolve("codex")
	if err != nil {
		return "", err
	}
	index, err := skills.LoadEmbedded(spec.EmbeddedFS, spec.EmbeddedRoot)
	if err != nil {
		return "", err
	}
	skillList := FilterSkills(index, nil)
	if len(skillList) == 0 {
		return "", fmt.Errorf("no embedded skills found")
	}
	entries := make([]string, 0, len(skillList))
	for _, skill := range skillList {
		for _, entry := range skill.Files {
			perm := fmt.Sprintf("%#o", entry.Mode.Perm())
			entries = append(entries, fmt.Sprintf("%s:%s:%s", entry.RelPath, entry.Hash, perm))
		}
	}
	return hashLines(entries), nil
}

func installedSkillsHash() (string, error) {
	spec, err := agents.Resolve("codex")
	if err != nil {
		return "", err
	}
	skillsRoot, err := spec.SkillsRoot()
	if err != nil {
		return "", err
	}
	if _, err := os.Stat(skillsRoot); err != nil {
		if os.IsNotExist(err) {
			return "", nil
		}
		return "", err
	}
	index, err := skills.LoadEmbedded(spec.EmbeddedFS, spec.EmbeddedRoot)
	if err != nil {
		return "", err
	}
	skillList := FilterSkills(index, nil)
	if len(skillList) == 0 {
		return "", nil
	}
	entries := make([]string, 0, len(skillList))
	for _, skill := range skillList {
		for _, entry := range skill.Files {
			target := filepath.Join(skillsRoot, skill.Name, entry.SubPath)
			data, err := os.ReadFile(target)
			if err != nil {
				if os.IsNotExist(err) {
					entries = append(entries, fmt.Sprintf("%s:missing", entry.RelPath))
					continue
				}
				return "", fmt.Errorf("read installed skill %s: %w", target, err)
			}
			hash := skills.HashFile(data)
			perm := "unknown"
			if info, err := os.Stat(target); err == nil {
				perm = fmt.Sprintf("%#o", info.Mode().Perm())
			}
			entries = append(entries, fmt.Sprintf("%s:%s:%s", entry.RelPath, hash, perm))
		}
	}
	return hashLines(entries), nil
}

func hashLines(lines []string) string {
	sort.Strings(lines)
	hasher := sha256.New()
	for _, line := range lines {
		hasher.Write([]byte(line))
		hasher.Write([]byte{'\n'})
	}
	return hex.EncodeToString(hasher.Sum(nil))
}

func gitInfo(root string) (string, *bool) {
	if _, err := exec.LookPath("git"); err != nil {
		return "", nil
	}
	commitCmd := exec.Command("git", "-C", root, "rev-parse", "HEAD")
	commitOut, err := commitCmd.Output()
	if err != nil {
		return "", nil
	}
	commit := strings.TrimSpace(string(commitOut))
	if commit == "" {
		return "", nil
	}
	statusCmd := exec.Command("git", "-C", root, "status", "--porcelain")
	statusOut, err := statusCmd.Output()
	if err != nil {
		return commit, nil
	}
	dirty := len(bytes.TrimSpace(statusOut)) > 0
	return commit, &dirty
}
