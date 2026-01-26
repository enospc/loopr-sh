package skills

import (
	"testing"

	"loopr/internal/agents"
)

func TestEmbeddedLooprCommonIncludesSharedTemplates(t *testing.T) {
	spec, err := agents.Resolve("codex")
	if err != nil {
		t.Fatalf("resolve agent: %v", err)
	}
	index, err := LoadEmbedded(spec.EmbeddedFS, spec.EmbeddedRoot)
	if err != nil {
		t.Fatalf("load embedded: %v", err)
	}
	common := index.SkillsByName["loopr-common"]
	if common == nil {
		t.Fatalf("missing loopr-common in embedded skills")
	}
	required := map[string]bool{
		"COMMON.md":         false,
		"task-template.md":  false,
		"test-templates.md": false,
		"pbt-guidance.md":   false,
	}
	for _, entry := range common.Files {
		if _, ok := required[entry.SubPath]; ok {
			required[entry.SubPath] = true
		}
	}
	for name, found := range required {
		if !found {
			t.Fatalf("loopr-common missing %s in embedded files", name)
		}
	}
}
