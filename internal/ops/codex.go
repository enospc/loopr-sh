package ops

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
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
	cwd, err := os.Getwd()
	if err != nil {
		return 1, nil, err
	}
	root, repoID, err := ResolveLooprRoot(cwd, opts.LooprRoot)
	if err != nil {
		return 1, nil, err
	}

	timestamp := time.Now().UTC().Format("20060102-150405")
	transcriptsDir := filepath.Join(root, "specs", ".loopr", "transcripts", repoID)
	if err := EnsureDir(transcriptsDir, 0o755); err != nil {
		return 1, nil, err
	}

	logPath := filepath.Join(transcriptsDir, fmt.Sprintf("session-%s.log", timestamp))
	metaPath := filepath.Join(transcriptsDir, fmt.Sprintf("session-%s.jsonl", timestamp))

	session := &CodexSession{
		RepoRoot: root,
		RepoID:   repoID,
		LogPath:  logPath,
		MetaPath: metaPath,
		Command:  append([]string{"codex"}, args...),
		Started:  time.Now().UTC(),
	}

	if err := writeMeta(metaPath, map[string]any{
		"event": "start",
		"ts":    session.Started.Format(time.RFC3339Nano),
		"cwd":   cwd,
		"cmd":   session.Command,
		"log":   filepath.Base(logPath),
	}); err != nil {
		return 1, session, err
	}

	code, err := runCodexWithLogging(logPath, args)
	end := time.Now().UTC()
	_ = writeMeta(metaPath, map[string]any{
		"event":     "end",
		"ts":        end.Format(time.RFC3339Nano),
		"exit_code": code,
	})
	return code, session, err
}

func runCodexWithLogging(logPath string, args []string) (int, error) {
	if scriptPath, err := exec.LookPath("script"); err == nil {
		cmdString := shellJoin(append([]string{"codex"}, args...))
		cmd := exec.Command(scriptPath, "-q", "-f", "-c", cmdString, logPath)
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

	cmd := exec.Command("codex", args...)
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
