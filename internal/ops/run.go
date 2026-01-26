package ops

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type RunStep struct {
	Name         string
	Skill        string
	Inputs       []string
	Outputs      []string
	RequiresSeed bool
	AlwaysRun    bool
}

type RunOptions struct {
	LooprRoot string
	From      string
	To        string
	Step      string
	Seed      string
	Force     bool
	Confirm   bool
	Codex     bool
	CodexArgs []string
	Progress  func(ProgressEvent)
}

type RunReport struct {
	Steps       []RunStep
	Executed    []RunStep
	Skipped     []RunStep
	LastSession *CodexSession
}

type ProgressEvent struct {
	Step   RunStep
	Index  int
	Total  int
	Status string
}

const (
	ProgressStart = "start"
	ProgressSkip  = "skip"
	ProgressDone  = "done"
	ProgressError = "error"
)

func RunWorkflow(opts RunOptions) (RunReport, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return RunReport{}, err
	}
	root := ""
	if opts.Codex {
		root, _, err = ResolveLooprRoot(cwd, opts.LooprRoot)
		if err != nil {
			return RunReport{}, err
		}
	} else {
		root, err = resolvePlanRoot(cwd, opts.LooprRoot)
		if err != nil {
			return RunReport{}, err
		}
	}
	appendPrompt := shouldAppendPrompt(opts.CodexArgs)
	if opts.Codex && !appendPrompt && len(opts.CodexArgs) > 0 {
		args := append([]string{"--cd", root}, opts.CodexArgs...)
		_, session, err := RunCodex(args, CodexOptions{LooprRoot: root})
		report := RunReport{LastSession: session}
		if err != nil {
			return report, err
		}
		return report, nil
	}
	handoffPath := ""
	if opts.Codex {
		handoffPath, err = ensureHandoff(root)
		if err != nil {
			return RunReport{}, err
		}
	}

	var steps []RunStep
	if opts.Codex {
		steps, err = planSteps(root, opts)
	} else {
		steps, err = viewSteps(opts)
	}
	if err != nil {
		return RunReport{}, err
	}
	report := RunReport{Steps: steps}

	if !opts.Codex {
		return report, nil
	}

	total := len(steps)
	for _, step := range steps {
		idx := len(report.Executed) + len(report.Skipped) + 1
		shouldRun := step.AlwaysRun || opts.Force
		if !shouldRun {
			ok, err := outputsPresent(root, step)
			if err != nil {
				return report, err
			}
			shouldRun = !ok
		}
		if !shouldRun {
			report.Skipped = append(report.Skipped, step)
			if opts.Progress != nil {
				opts.Progress(ProgressEvent{
					Step:   step,
					Index:  idx,
					Total:  total,
					Status: ProgressSkip,
				})
			}
			continue
		}
		if appendPrompt && step.RequiresSeed && strings.TrimSpace(opts.Seed) == "" {
			return report, fmt.Errorf("seed prompt required for %s (use --seed)", step.Name)
		}
		if opts.Confirm {
			ok, err := confirmStep(step.Name)
			if err != nil {
				return report, err
			}
			if !ok {
				return report, errors.New("run cancelled")
			}
		}

		if opts.Progress != nil {
			opts.Progress(ProgressEvent{
				Step:   step,
				Index:  idx,
				Total:  total,
				Status: ProgressStart,
			})
		}
		args := append([]string{"--cd", root}, opts.CodexArgs...)
		if appendPrompt {
			prompt := buildPrompt(step, opts.Seed, handoffPath)
			args = append(args, prompt)
		}
		_, session, err := RunCodex(args, CodexOptions{LooprRoot: root})
		if err != nil {
			report.LastSession = session
			if opts.Progress != nil {
				opts.Progress(ProgressEvent{
					Step:   step,
					Index:  idx,
					Total:  total,
					Status: ProgressError,
				})
			}
			return report, err
		}
		if opts.Progress != nil {
			opts.Progress(ProgressEvent{
				Step:   step,
				Index:  idx,
				Total:  total,
				Status: ProgressDone,
			})
		}
		report.LastSession = session
		report.Executed = append(report.Executed, step)
	}

	return report, nil
}

func resolvePlanRoot(cwd, override string) (string, error) {
	if root := strings.TrimSpace(override); root != "" {
		return filepath.Abs(root)
	}
	return cwd, nil
}

func planSteps(root string, opts RunOptions) ([]RunStep, error) {
	steps := defaultRunSteps()
	if opts.Step != "" {
		step, ok := findStep(steps, opts.Step)
		if !ok {
			return nil, fmt.Errorf("unknown step: %s", opts.Step)
		}
		return []RunStep{step}, nil
	}
	if opts.From != "" || opts.To != "" {
		return selectRange(steps, opts.From, opts.To)
	}
	for i, step := range steps {
		if step.AlwaysRun {
			continue
		}
		ok, err := outputsPresent(root, step)
		if err != nil {
			return nil, err
		}
		if !ok {
			return steps[i:], nil
		}
	}
	execStep, ok := findStep(steps, "execute")
	if !ok {
		return nil, errors.New("missing execute step")
	}
	return []RunStep{execStep}, nil
}

func viewSteps(opts RunOptions) ([]RunStep, error) {
	steps := defaultRunSteps()
	if opts.Step != "" {
		step, ok := findStep(steps, opts.Step)
		if !ok {
			return nil, fmt.Errorf("unknown step: %s", opts.Step)
		}
		return []RunStep{step}, nil
	}
	if opts.From != "" || opts.To != "" {
		return selectRange(steps, opts.From, opts.To)
	}
	return steps, nil
}

func defaultRunSteps() []RunStep {
	return []RunStep{
		{
			Name:         "prd",
			Skill:        "loopr-prd",
			Inputs:       []string{"specs/.loopr/handoff.md"},
			Outputs:      []string{"specs/prd.md"},
			RequiresSeed: true,
		},
		{
			Name:    "spec",
			Skill:   "loopr-specify",
			Inputs:  []string{"specs/.loopr/handoff.md", "specs/prd.md"},
			Outputs: []string{"specs/spec.md"},
		},
		{
			Name:    "features",
			Skill:   "loopr-features",
			Inputs:  []string{"specs/.loopr/handoff.md", "specs/spec.md", "specs/.loopr/init-state.json"},
			Outputs: []string{"specs/feature-order.yaml", "specs/feature-*.md"},
		},
		{
			Name:    "tasks",
			Skill:   "loopr-tasks",
			Inputs:  []string{"specs/.loopr/handoff.md", "specs/feature-order.yaml", "specs/feature-*.md", "specs/.loopr/init-state.json"},
			Outputs: []string{"specs/task-order.yaml", "specs/feature-*-task-*.md"},
		},
		{
			Name:    "tests",
			Skill:   "loopr-tests",
			Inputs:  []string{"specs/.loopr/handoff.md", "specs/task-order.yaml", "specs/feature-*-task-*.md"},
			Outputs: []string{"specs/test-order.yaml", "specs/feature-*-task-*-test-*.md"},
		},
		{
			Name:      "execute",
			Skill:     "loopr-execute",
			Inputs:    []string{"specs/.loopr/handoff.md", "specs/task-order.yaml", "specs/test-order.yaml", "specs/feature-*-task-*.md", "specs/feature-*-task-*-test-*.md"},
			Outputs:   []string{"specs/implementation-progress.md"},
			AlwaysRun: true,
		},
	}
}

func outputsPresent(root string, step RunStep) (bool, error) {
	for _, output := range step.Outputs {
		path := filepath.Join(root, output)
		if strings.Contains(output, "*") {
			matches, err := filepath.Glob(path)
			if err != nil {
				return false, err
			}
			if len(matches) == 0 {
				return false, nil
			}
			continue
		}
		if !fileExists(path) {
			return false, nil
		}
	}
	return true, nil
}

func selectRange(steps []RunStep, from, to string) ([]RunStep, error) {
	start := 0
	end := len(steps) - 1
	if from != "" {
		idx := indexOfStep(steps, from)
		if idx < 0 {
			return nil, fmt.Errorf("unknown step: %s", from)
		}
		start = idx
	}
	if to != "" {
		idx := indexOfStep(steps, to)
		if idx < 0 {
			return nil, fmt.Errorf("unknown step: %s", to)
		}
		end = idx
	}
	if start > end {
		return nil, fmt.Errorf("invalid step range: %s to %s", from, to)
	}
	return steps[start : end+1], nil
}

func indexOfStep(steps []RunStep, name string) int {
	for i, step := range steps {
		if step.Name == name {
			return i
		}
	}
	return -1
}

func findStep(steps []RunStep, name string) (RunStep, bool) {
	idx := indexOfStep(steps, name)
	if idx < 0 {
		return RunStep{}, false
	}
	return steps[idx], true
}

func buildPrompt(step RunStep, seed, handoffPath string) string {
	lines := []string{
		fmt.Sprintf("Loopr step: %s", step.Name),
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
	if step.RequiresSeed {
		lines = append(lines, "", "Seed prompt:")
		lines = append(lines, seed)
	}
	lines = append(lines, "", "Rules:")
	lines = append(lines, "- Read only the allowed inputs.")
	lines = append(lines, "- Do not scan the repo.")
	lines = append(lines, "- If required inputs are missing, stop and ask to run the appropriate step.")
	lines = append(lines, fmt.Sprintf("- Append a completion note to %s (decisions, open questions, tests).", handoffPath))
	lines = append(lines, "")
	lines = append(lines, fmt.Sprintf("Run the skill: %s", step.Skill))
	return strings.Join(lines, "\n")
}

func ensureHandoff(root string) (string, error) {
	path := filepath.Join(root, "specs", ".loopr", "handoff.md")
	if fileExists(path) {
		return path, nil
	}
	if err := EnsureDir(filepath.Dir(path), 0o755); err != nil {
		return "", err
	}
	header := fmt.Sprintf("# Loopr Handoff\n\nInitialized: %s\n\n", time.Now().UTC().Format(time.RFC3339Nano))
	if err := WriteFileAtomic(path, []byte(header), 0o644); err != nil {
		return "", err
	}
	return path, nil
}

func confirmStep(name string) (bool, error) {
	fmt.Printf("Run step %s? [y/N]: ", name)
	reader := bufio.NewReader(os.Stdin)
	line, err := reader.ReadString('\n')
	if err != nil && !errors.Is(err, os.ErrClosed) {
		return false, err
	}
	answer := strings.TrimSpace(strings.ToLower(line))
	return answer == "y" || answer == "yes", nil
}

var codexSubcommands = map[string]struct{}{
	"exec":       {},
	"review":     {},
	"login":      {},
	"logout":     {},
	"mcp":        {},
	"mcp-server": {},
	"app-server": {},
	"completion": {},
	"sandbox":    {},
	"apply":      {},
	"resume":     {},
	"fork":       {},
	"cloud":      {},
	"features":   {},
	"help":       {},
}

func shouldAppendPrompt(args []string) bool {
	for _, arg := range args {
		switch arg {
		case "-h", "--help", "-V", "--version":
			return false
		}
		if _, ok := codexSubcommands[arg]; ok {
			return false
		}
	}
	return true
}
