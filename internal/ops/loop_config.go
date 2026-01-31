package ops

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type LoopConfig struct {
	MaxCallsPerHour           int
	CodexTimeoutMinutes       int
	MaxIterations             int
	MaxConsecutiveDoneSignals int
	MaxNoProgress             int
	MaxSameError              int
	MaxConsecutiveTestLoops   int
	MaxMissingStatus          int
}

func DefaultLoopConfig() LoopConfig {
	return LoopConfig{
		MaxCallsPerHour:           100,
		CodexTimeoutMinutes:       15,
		MaxIterations:             50,
		MaxConsecutiveDoneSignals: 2,
		MaxNoProgress:             3,
		MaxSameError:              5,
		MaxConsecutiveTestLoops:   3,
		MaxMissingStatus:          2,
	}
}

func LoadLoopConfig(path string) (LoopConfig, error) {
	cfg := DefaultLoopConfig()
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return cfg, nil
		}
		return cfg, err
	}
	scanner := bufio.NewScanner(strings.NewReader(string(data)))
	lineNo := 0
	for scanner.Scan() {
		lineNo++
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		parts := strings.SplitN(line, "=", 2)
		if len(parts) != 2 {
			return cfg, fmt.Errorf("invalid config line %d: %q (expected KEY=VALUE)", lineNo, line)
		}
		key := strings.TrimSpace(parts[0])
		val := strings.TrimSpace(parts[1])
		if hash := strings.Index(val, "#"); hash >= 0 {
			val = strings.TrimSpace(val[:hash])
		}
		if val == "" {
			return cfg, fmt.Errorf("empty value for %s on line %d", key, lineNo)
		}
		if err := applyLoopConfigValue(&cfg, key, val, lineNo); err != nil {
			return cfg, err
		}
	}
	if err := scanner.Err(); err != nil {
		return cfg, err
	}
	return cfg, nil
}

func applyLoopConfigValue(cfg *LoopConfig, key, val string, lineNo int) error {
	switch key {
	case "MAX_CALLS_PER_HOUR":
		return setLoopConfigInt(&cfg.MaxCallsPerHour, key, val, lineNo, true)
	case "CODEX_TIMEOUT_MINUTES":
		return setLoopConfigInt(&cfg.CodexTimeoutMinutes, key, val, lineNo, true)
	case "MAX_ITERATIONS":
		return setLoopConfigInt(&cfg.MaxIterations, key, val, lineNo, false)
	case "MAX_CONSECUTIVE_DONE_SIGNALS":
		return setLoopConfigInt(&cfg.MaxConsecutiveDoneSignals, key, val, lineNo, true)
	case "MAX_NO_PROGRESS":
		return setLoopConfigInt(&cfg.MaxNoProgress, key, val, lineNo, true)
	case "MAX_SAME_ERROR":
		return setLoopConfigInt(&cfg.MaxSameError, key, val, lineNo, true)
	case "MAX_CONSECUTIVE_TEST_LOOPS":
		return setLoopConfigInt(&cfg.MaxConsecutiveTestLoops, key, val, lineNo, true)
	case "MAX_MISSING_STATUS":
		return setLoopConfigInt(&cfg.MaxMissingStatus, key, val, lineNo, true)
	default:
		return fmt.Errorf("unknown config key %q on line %d", key, lineNo)
	}
}

func setLoopConfigInt(dst *int, key, val string, lineNo int, mustBePositive bool) error {
	parsed, err := strconv.Atoi(val)
	if err != nil {
		return fmt.Errorf("invalid int for %s on line %d: %q", key, lineNo, val)
	}
	if mustBePositive && parsed <= 0 {
		return fmt.Errorf("%s must be > 0 on line %d", key, lineNo)
	}
	if !mustBePositive && parsed < 0 {
		return fmt.Errorf("%s must be >= 0 on line %d", key, lineNo)
	}
	*dst = parsed
	return nil
}
