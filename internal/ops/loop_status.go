package ops

import (
	"os"
	"strconv"
	"strings"
)

const (
	looprStatusStart = "---LOOPR_STATUS---"
	looprStatusEnd   = "---END_LOOPR_STATUS---"
)

type LooprStatus struct {
	Status           string
	ExitSignal       bool
	WorkType         string
	FilesModified    int
	ErrorCount       int
	Summary          string
	PermissionDenied bool
}

func ParseLooprStatusFromLog(path string) (LooprStatus, bool, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return LooprStatus{}, false, err
	}
	status, ok := ParseLooprStatus(string(data))
	return status, ok, nil
}

func ParseLooprStatus(log string) (LooprStatus, bool) {
	idx := strings.LastIndex(log, looprStatusStart)
	if idx == -1 {
		return LooprStatus{}, false
	}
	segment := log[idx+len(looprStatusStart):]
	if end := strings.Index(segment, looprStatusEnd); end >= 0 {
		segment = segment[:end]
	}
	status := LooprStatus{
		Status: "UNKNOWN",
	}
	lines := strings.Split(segment, "\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		parts := strings.SplitN(line, ":", 2)
		if len(parts) != 2 {
			continue
		}
		key := strings.ToUpper(strings.TrimSpace(parts[0]))
		val := strings.TrimSpace(parts[1])
		switch key {
		case "STATUS":
			status.Status = strings.ToUpper(val)
		case "EXIT_SIGNAL":
			status.ExitSignal = parseBool(val)
		case "WORK_TYPE":
			status.WorkType = strings.ToLower(val)
		case "FILES_MODIFIED":
			status.FilesModified = parseInt(val)
		case "ERRORS":
			status.ErrorCount = parseInt(val)
		case "SUMMARY":
			status.Summary = val
		case "PERMISSION_DENIALS":
			status.PermissionDenied = parseBool(val)
		}
	}
	return status, true
}

func parseBool(value string) bool {
	switch strings.ToLower(strings.TrimSpace(value)) {
	case "true", "yes", "1", "y":
		return true
	default:
		return false
	}
}

func parseInt(value string) int {
	parsed, err := strconv.Atoi(strings.TrimSpace(value))
	if err != nil {
		return 0
	}
	return parsed
}
