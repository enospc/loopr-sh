package agents

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"loopr"
)

type Spec struct {
	Name         string
	EmbeddedFS   fs.FS
	EmbeddedRoot string
	SkillsRoot   func() (string, error)
}

func All() []Spec {
	return []Spec{codexSpec()}
}

func Resolve(name string) (Spec, error) {
	switch strings.ToLower(strings.TrimSpace(name)) {
	case "", "codex":
		return codexSpec(), nil
	default:
		return Spec{}, fmt.Errorf("unsupported agent: %s", name)
	}
}

func codexSpec() Spec {
	return Spec{
		Name:         "codex",
		EmbeddedFS:   loopr.EmbeddedSkills,
		EmbeddedRoot: "codex-skills",
		SkillsRoot: func() (string, error) {
			if base := strings.TrimSpace(os.Getenv("CODEX_HOME")); base != "" {
				return filepath.Join(base, "skills"), nil
			}
			home, err := os.UserHomeDir()
			if err != nil {
				return "", errors.New("unable to resolve user home directory")
			}
			return filepath.Join(home, ".codex", "skills"), nil
		},
	}
}
