package ops

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func FindLooprRoot(start string) (string, string, error) {
	current := start
	for {
		repoIDPath := filepath.Join(current, "specs", ".loopr", "repo-id")
		data, err := os.ReadFile(repoIDPath)
		if err == nil {
			repoID := strings.TrimSpace(string(data))
			if repoID == "" {
				return "", "", fmt.Errorf("repo-id is empty at %s", repoIDPath)
			}
			return current, repoID, nil
		}
		parent := filepath.Dir(current)
		if parent == current {
			break
		}
		current = parent
	}
	return "", "", fmt.Errorf("unable to locate specs/.loopr/repo-id (run loopr-init)")
}
