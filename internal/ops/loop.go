package ops

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

type LoopOptions struct {
	LooprRoot     string
	MaxIterations int
	CodexArgs     []string
	Progress      func(LoopEvent)
}

type LoopReport struct {
	Iterations  int
	ExitReason  string
	LastSession *CodexSession
}

type LoopEvent struct {
	Iteration int
	Status    string
	Details   string
}

const (
	LoopEventStart   = "start"
	LoopEventWaiting = "waiting"
	LoopEventDone    = "done"
	LoopEventExit    = "exit"
	LoopEventError   = "error"
)

type loopStatus struct {
	State       string `json:"state"`
	Iteration   int    `json:"iteration"`
	UpdatedAt   string `json:"updated_at"`
	ExitReason  string `json:"exit_reason,omitempty"`
	LastSummary string `json:"last_summary,omitempty"`
	LastError   string `json:"last_error,omitempty"`
	CallCount   int    `json:"call_count,omitempty"`
	NextResetAt string `json:"next_reset_at,omitempty"`
}

func RunLoop(opts LoopOptions) (LoopReport, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return LoopReport{}, err
	}
	root, _, err := ResolveLooprRoot(cwd, opts.LooprRoot)
	if err != nil {
		return LoopReport{}, err
	}
	handoffPath, err := ensureHandoff(root)
	if err != nil {
		return LoopReport{}, err
	}
	step, ok := findStep(defaultRunSteps(), "execute")
	if !ok {
		return LoopReport{}, errors.New("execute step not found")
	}

	looprDir := filepath.Join(root, ".loopr")
	configPath := filepath.Join(looprDir, "config")
	statePath := filepath.Join(looprDir, "loop-state.json")
	statusPath := filepath.Join(looprDir, "status.json")
	logPath := filepath.Join(looprDir, "loop.log")
	callCountPath := filepath.Join(looprDir, ".call_count")
	lastResetPath := filepath.Join(looprDir, ".last_reset")

	cfg, err := LoadLoopConfig(configPath)
	if err != nil {
		return LoopReport{}, err
	}
	if opts.MaxIterations > 0 {
		cfg.MaxIterations = opts.MaxIterations
	}

	state, err := readLoopState(statePath)
	if err != nil {
		return LoopReport{}, err
	}
	report := LoopReport{}

	for {
		if cfg.MaxIterations > 0 && state.Iteration >= cfg.MaxIterations {
			report.ExitReason = "max_iterations"
			writeLoopStatus(statusPath, loopStatus{
				State:      "complete",
				Iteration:  state.Iteration,
				UpdatedAt:  time.Now().UTC().Format(time.RFC3339Nano),
				ExitReason: report.ExitReason,
			})
			break
		}

		nextIteration := state.Iteration + 1
		if opts.Progress != nil {
			opts.Progress(LoopEvent{Iteration: nextIteration, Status: LoopEventStart})
		}

		callCount, nextResetAt, waitFor, err := enforceRateLimit(callCountPath, lastResetPath, cfg.MaxCallsPerHour)
		if err != nil {
			return report, err
		}
		if waitFor > 0 {
			writeLoopStatus(statusPath, loopStatus{
				State:       "waiting",
				Iteration:   state.Iteration,
				UpdatedAt:   time.Now().UTC().Format(time.RFC3339Nano),
				CallCount:   callCount,
				NextResetAt: nextResetAt.Format(time.RFC3339Nano),
			})
			if opts.Progress != nil {
				opts.Progress(LoopEvent{Iteration: state.Iteration, Status: LoopEventWaiting, Details: waitFor.String()})
			}
			time.Sleep(waitFor)
			callCount, nextResetAt, _, err = enforceRateLimit(callCountPath, lastResetPath, cfg.MaxCallsPerHour)
			if err != nil {
				return report, err
			}
		}

		prompt := buildLoopPrompt(step, handoffPath, nextIteration)
		args := append([]string{"--cd", root}, opts.CodexArgs...)
		args = append(args, prompt)

		var session *CodexSession
		if cfg.CodexTimeoutMinutes > 0 {
			_, session, err = RunCodexWithTimeout(args, CodexOptions{LooprRoot: root}, time.Duration(cfg.CodexTimeoutMinutes)*time.Minute)
		} else {
			_, session, err = RunCodex(args, CodexOptions{LooprRoot: root})
		}
		report.LastSession = session
		state.Iteration = nextIteration
		callCount = incrementCallCount(callCountPath, callCount)

		var status LooprStatus
		statusFound := false
		if session != nil {
			parsed, ok, parseErr := ParseLooprStatusFromLog(session.LogPath)
			if parseErr != nil && err == nil {
				err = parseErr
			}
			status = parsed
			statusFound = ok
		}

		filesModified := status.FilesModified
		if filesModified == 0 {
			filesModified = gitChangeCount(root)
		}
		if err != nil {
			if status.Summary == "" {
				status.Summary = err.Error()
			}
			if status.ErrorCount == 0 {
				status.ErrorCount = 1
			}
			status.Status = "ERROR"
			status.ExitSignal = false
		}

		exitReason, exitState := evaluateLoopExit(cfg, status, statusFound, filesModified, &state)

		statusPayload := loopStatus{
			State:       exitState,
			Iteration:   state.Iteration,
			UpdatedAt:   time.Now().UTC().Format(time.RFC3339Nano),
			ExitReason:  exitReason,
			LastSummary: status.Summary,
			CallCount:   callCount,
		}
		if nextResetAt.After(time.Time{}) {
			statusPayload.NextResetAt = nextResetAt.Format(time.RFC3339Nano)
		}
		if err != nil {
			statusPayload.LastError = err.Error()
			if statusPayload.State == "running" {
				statusPayload.State = "error"
			}
		} else if !statusFound {
			statusPayload.LastError = "missing LOOPR_STATUS block"
		}
		if writeErr := writeLoopStatus(statusPath, statusPayload); writeErr != nil {
			return report, writeErr
		}
		if writeErr := writeLoopState(statePath, state); writeErr != nil {
			return report, writeErr
		}
		appendLoopLog(logPath, state.Iteration, statusPayload.State, statusPayload.ExitReason, statusPayload.LastSummary, statusPayload.LastError)

		if exitReason != "" {
			report.ExitReason = exitReason
			if opts.Progress != nil {
				opts.Progress(LoopEvent{Iteration: state.Iteration, Status: LoopEventExit, Details: exitReason})
			}
			break
		}
		if err != nil {
			if opts.Progress != nil {
				opts.Progress(LoopEvent{Iteration: state.Iteration, Status: LoopEventError, Details: err.Error()})
			}
			return report, err
		}
		if opts.Progress != nil {
			opts.Progress(LoopEvent{Iteration: state.Iteration, Status: LoopEventDone})
		}
	}

	report.Iterations = state.Iteration
	return report, nil
}

func buildLoopPrompt(step RunStep, handoffPath string, iteration int) string {
	lines := []string{
		fmt.Sprintf("Loopr loop iteration: %d", iteration),
		fmt.Sprintf("Skill: %s", step.Skill),
		"",
		"Allowed inputs:",
	}
	seen := map[string]struct{}{}
	for _, input := range step.Inputs {
		if _, ok := seen[input]; ok {
			continue
		}
		seen[input] = struct{}{}
		lines = append(lines, fmt.Sprintf("- %s", input))
	}
	lines = append(lines, "", "Required outputs:")
	for _, output := range step.Outputs {
		lines = append(lines, fmt.Sprintf("- %s", output))
	}
	lines = append(lines, "", "Rules:")
	lines = append(lines, "- Read only the allowed inputs.")
	lines = append(lines, "- Do not scan the repo.")
	lines = append(lines, "- If required inputs are missing, stop and ask to run the appropriate step.")
	lines = append(lines, fmt.Sprintf("- Append a completion note to %s (decisions, open questions, tests).", handoffPath))
	lines = append(lines, "- Only set EXIT_SIGNAL: true when all tasks are complete and tests are green.")
	lines = append(lines, "- Always include the status block at the end of your response.")
	lines = append(lines, "", "Status block format (required):")
	lines = append(lines, looprStatusStart)
	lines = append(lines, "STATUS: IN_PROGRESS | COMPLETE | BLOCKED | ERROR")
	lines = append(lines, "EXIT_SIGNAL: true | false")
	lines = append(lines, "WORK_TYPE: tests | code | docs | other")
	lines = append(lines, "FILES_MODIFIED: <int>")
	lines = append(lines, "ERRORS: <int>")
	lines = append(lines, "SUMMARY: <short summary>")
	lines = append(lines, looprStatusEnd)
	lines = append(lines, "")
	lines = append(lines, fmt.Sprintf("Run the skill: %s", step.Skill))
	return strings.Join(lines, "\n")
}

func evaluateLoopExit(cfg LoopConfig, status LooprStatus, statusFound bool, filesModified int, state *loopState) (string, string) {
	if status.PermissionDenied {
		return "permission_denied", "blocked"
	}

	trackMissingStatus(statusFound, state)
	trackCompletion(cfg, status, statusFound, state)
	trackTestLoops(status, state)
	trackProgress(filesModified, state)
	trackErrors(status, state)

	if status.ExitSignal && state.ConsecutiveDoneSignals >= cfg.MaxConsecutiveDoneSignals {
		return "completed", "complete"
	}
	if state.MissingStatusCount >= cfg.MaxMissingStatus {
		return "missing_status", "circuit_open"
	}
	if state.NoProgressCount >= cfg.MaxNoProgress {
		return "circuit_open_no_progress", "circuit_open"
	}
	if state.SameErrorCount >= cfg.MaxSameError {
		return "circuit_open_repeated_error", "circuit_open"
	}
	if state.ConsecutiveTestLoops >= cfg.MaxConsecutiveTestLoops {
		return "circuit_open_test_only", "circuit_open"
	}
	return "", "running"
}

func trackMissingStatus(statusFound bool, state *loopState) {
	if statusFound {
		state.MissingStatusCount = 0
		return
	}
	state.MissingStatusCount++
}

func trackCompletion(cfg LoopConfig, status LooprStatus, statusFound bool, state *loopState) {
	if !statusFound || !status.ExitSignal {
		state.ConsecutiveDoneSignals = 0
		state.LastCompletionIndicator = ""
		return
	}
	completionIndicator := status.Status == "COMPLETE" || containsCompletionKeyword(status.Summary)
	if completionIndicator {
		state.ConsecutiveDoneSignals++
		state.LastCompletionIndicator = status.Summary
	} else {
		state.ConsecutiveDoneSignals = 0
		state.LastCompletionIndicator = ""
	}
}

func trackTestLoops(status LooprStatus, state *loopState) {
	if strings.HasPrefix(status.WorkType, "test") {
		state.ConsecutiveTestLoops++
	} else {
		state.ConsecutiveTestLoops = 0
	}
}

func trackProgress(filesModified int, state *loopState) {
	if filesModified == 0 {
		state.NoProgressCount++
	} else {
		state.NoProgressCount = 0
	}
}

func trackErrors(status LooprStatus, state *loopState) {
	if status.ErrorCount == 0 && status.Status != "ERROR" {
		state.SameErrorCount = 0
		state.LastErrorSignature = ""
		return
	}
	signature := status.Summary
	if signature == "" {
		signature = status.Status
	}
	if signature == state.LastErrorSignature {
		state.SameErrorCount++
	} else {
		state.SameErrorCount = 1
		state.LastErrorSignature = signature
	}
}

func containsCompletionKeyword(summary string) bool {
	s := strings.ToLower(summary)
	return strings.Contains(s, "complete") || strings.Contains(s, "done") || strings.Contains(s, "ready")
}

func enforceRateLimit(callCountPath, lastResetPath string, maxCalls int) (int, time.Time, time.Duration, error) {
	now := time.Now().UTC()
	callCount, lastReset, err := readRateLimit(callCountPath, lastResetPath, now)
	if err != nil {
		return 0, time.Time{}, 0, err
	}
	if !fileExists(callCountPath) || !fileExists(lastResetPath) {
		if err := writeRateLimit(callCountPath, lastResetPath, callCount, lastReset); err != nil {
			return 0, time.Time{}, 0, err
		}
	}
	if now.Sub(lastReset) >= time.Hour {
		callCount = 0
		lastReset = now
		if err := writeRateLimit(callCountPath, lastResetPath, callCount, lastReset); err != nil {
			return 0, time.Time{}, 0, err
		}
	}
	if callCount >= maxCalls {
		nextReset := lastReset.Add(time.Hour)
		waitFor := time.Until(nextReset)
		if waitFor < 0 {
			waitFor = 0
		}
		return callCount, nextReset, waitFor, nil
	}
	return callCount, lastReset.Add(time.Hour), 0, nil
}

func readRateLimit(callCountPath, lastResetPath string, now time.Time) (int, time.Time, error) {
	callCount := 0
	if data, err := os.ReadFile(callCountPath); err == nil {
		callCount = parseInt(string(data))
	} else if !os.IsNotExist(err) {
		return 0, time.Time{}, err
	}
	lastReset := now
	if data, err := os.ReadFile(lastResetPath); err == nil {
		parsed, err := time.Parse(time.RFC3339Nano, strings.TrimSpace(string(data)))
		if err != nil {
			return 0, time.Time{}, err
		}
		lastReset = parsed
	} else if !os.IsNotExist(err) {
		return 0, time.Time{}, err
	}
	return callCount, lastReset, nil
}

func writeRateLimit(callCountPath, lastResetPath string, callCount int, lastReset time.Time) error {
	if err := WriteFileAtomic(callCountPath, []byte(fmt.Sprintf("%d\n", callCount)), 0o644); err != nil {
		return err
	}
	return WriteFileAtomic(lastResetPath, []byte(lastReset.Format(time.RFC3339Nano)+"\n"), 0o644)
}

func incrementCallCount(callCountPath string, callCount int) int {
	callCount++
	_ = WriteFileAtomic(callCountPath, []byte(fmt.Sprintf("%d\n", callCount)), 0o644)
	return callCount
}

func writeLoopStatus(path string, status loopStatus) error {
	data, err := json.MarshalIndent(status, "", "  ")
	if err != nil {
		return err
	}
	data = append(data, '\n')
	return WriteFileAtomic(path, data, 0o644)
}

func appendLoopLog(path string, iteration int, state, reason, summary, errMsg string) {
	line := fmt.Sprintf("%s\titer=%d\tstate=%s", time.Now().UTC().Format(time.RFC3339Nano), iteration, state)
	if reason != "" {
		line += "\treason=" + reason
	}
	if summary != "" {
		line += "\tsummary=" + sanitizeLogValue(summary)
	}
	if errMsg != "" {
		line += "\terror=" + sanitizeLogValue(errMsg)
	}
	line += "\n"
	_ = appendFile(path, line)
}

func sanitizeLogValue(value string) string {
	value = strings.ReplaceAll(value, "\n", " ")
	value = strings.ReplaceAll(value, "\t", " ")
	return strings.TrimSpace(value)
}

func appendFile(path, content string) error {
	if err := EnsureDir(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	file, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return err
	}
	defer file.Close()
	_, err = file.WriteString(content)
	return err
}

func gitChangeCount(root string) int {
	if _, err := exec.LookPath("git"); err != nil {
		return 0
	}
	if !fileExists(filepath.Join(root, ".git")) {
		return 0
	}
	cmd := exec.Command("git", "-C", root, "status", "--porcelain")
	out, err := cmd.Output()
	if err != nil {
		return 0
	}
	lines := strings.Split(strings.TrimSpace(string(out)), "\n")
	if len(lines) == 1 && strings.TrimSpace(lines[0]) == "" {
		return 0
	}
	return len(lines)
}
