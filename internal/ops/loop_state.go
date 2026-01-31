package ops

import (
	"encoding/json"
	"os"
)

type loopState struct {
	Iteration               int    `json:"iteration"`
	ConsecutiveDoneSignals  int    `json:"consecutive_done_signals"`
	ConsecutiveTestLoops    int    `json:"consecutive_test_loops"`
	NoProgressCount         int    `json:"no_progress_count"`
	SameErrorCount          int    `json:"same_error_count"`
	MissingStatusCount      int    `json:"missing_status_count"`
	LastErrorSignature      string `json:"last_error_signature,omitempty"`
	LastCompletionIndicator string `json:"last_completion_indicator,omitempty"`
}

func readLoopState(path string) (loopState, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return loopState{}, nil
		}
		return loopState{}, err
	}
	var state loopState
	if err := json.Unmarshal(data, &state); err != nil {
		return loopState{}, err
	}
	return state, nil
}

func writeLoopState(path string, state loopState) error {
	data, err := json.MarshalIndent(state, "", "  ")
	if err != nil {
		return err
	}
	data = append(data, '\n')
	return WriteFileAtomic(path, data, 0o644)
}
