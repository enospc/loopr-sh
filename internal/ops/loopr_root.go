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
	return "", "", fmt.Errorf("unable to locate specs/.loopr/repo-id (run loopr init)")
}

func ResolveLooprRoot(start string, override string) (string, string, error) {
	if root := strings.TrimSpace(override); root != "" {
		return loadRepoID(root)
	}
	if root := strings.TrimSpace(os.Getenv("LOOPR_ROOT")); root != "" {
		return loadRepoID(root)
	}
	return FindLooprRoot(start)
}

func loadRepoID(root string) (string, string, error) {
	absRoot := root
	if !filepath.IsAbs(root) {
		abs, err := filepath.Abs(root)
		if err != nil {
			return "", "", err
		}
		absRoot = abs
	}
	repoIDPath := filepath.Join(absRoot, "specs", ".loopr", "repo-id")
	data, err := os.ReadFile(repoIDPath)
	if err != nil {
		return "", "", fmt.Errorf("unable to locate specs/.loopr/repo-id under %s (run loopr init)", absRoot)
	}
	repoID := strings.TrimSpace(string(data))
	if repoID == "" {
		return "", "", fmt.Errorf("repo-id is empty at %s", repoIDPath)
	}
	return absRoot, repoID, nil
}
