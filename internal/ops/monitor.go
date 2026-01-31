package ops

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type MonitorOptions struct {
	LooprRoot string
	Interval  time.Duration
	Once      bool
}

func RunMonitor(opts MonitorOptions) error {
	cwd, err := os.Getwd()
	if err != nil {
		return err
	}
	root, _, err := ResolveLooprRoot(cwd, opts.LooprRoot)
	if err != nil {
		return err
	}
	interval := opts.Interval
	if interval <= 0 {
		interval = 5 * time.Second
	}
	statusPath := filepath.Join(root, ".loopr", "status.json")

	lastLine := ""
	warned := false
	for {
		status, raw, err := readLoopStatus(statusPath)
		if err != nil {
			if os.IsNotExist(err) {
				if !warned {
					fmt.Fprintf(os.Stderr, "waiting for %s (run loopr loop)\n", statusPath)
					warned = true
				}
			} else {
				fmt.Fprintf(os.Stderr, "monitor read error: %v\n", err)
			}
		} else {
			line := formatMonitorLine(status, raw)
			if line != "" && line != lastLine {
				fmt.Println(line)
				lastLine = line
			}
		}
		if opts.Once {
			return nil
		}
		time.Sleep(interval)
	}
}

func readLoopStatus(path string) (loopStatus, []byte, error) {
	raw, err := os.ReadFile(path)
	if err != nil {
		return loopStatus{}, nil, err
	}
	var status loopStatus
	if err := json.Unmarshal(raw, &status); err != nil {
		return loopStatus{}, raw, err
	}
	return status, raw, nil
}

func formatMonitorLine(status loopStatus, raw []byte) string {
	if status.State == "" && len(raw) > 0 {
		return strings.TrimSpace(string(raw))
	}
	parts := []string{
		fmt.Sprintf("state=%s", status.State),
		fmt.Sprintf("iter=%d", status.Iteration),
	}
	if status.ExitReason != "" {
		parts = append(parts, "reason="+sanitizeLogValue(status.ExitReason))
	}
	if status.LastSummary != "" {
		parts = append(parts, "summary="+sanitizeLogValue(status.LastSummary))
	}
	if status.LastError != "" {
		parts = append(parts, "error="+sanitizeLogValue(status.LastError))
	}
	return strings.Join(parts, " ")
}
