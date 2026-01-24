package loopr

import "embed"

// EmbeddedSkills contains the source-of-truth Loopr skills for supported agents.
//go:embed codex-skills/** claude-skills/**
var EmbeddedSkills embed.FS
