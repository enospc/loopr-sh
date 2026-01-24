package skills

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io/fs"
	"path/filepath"
	"sort"
	"strings"
)

type FileEntry struct {
	Skill   string
	SubPath string
	RelPath string
	Data    []byte
	Hash    string
	Mode    fs.FileMode
}

type Skill struct {
	Name  string
	Files []FileEntry
}

type EmbeddedIndex struct {
	Skills       []Skill
	SkillsByName map[string]*Skill
}

func LoadEmbedded(fsys fs.FS, root string) (*EmbeddedIndex, error) {
	sub, err := fs.Sub(fsys, root)
	if err != nil {
		return nil, fmt.Errorf("unable to locate embedded root %q: %w", root, err)
	}

	index := &EmbeddedIndex{SkillsByName: map[string]*Skill{}}

	err = fs.WalkDir(sub, ".", func(path string, d fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if d.IsDir() {
			return nil
		}
		if strings.HasPrefix(filepath.Base(path), ".") {
			return nil
		}

		data, err := fs.ReadFile(sub, path)
		if err != nil {
			return fmt.Errorf("read embedded file %q: %w", path, err)
		}

		parts := strings.SplitN(path, "/", 2)
		if len(parts) != 2 {
			return nil
		}
		skillName := parts[0]
		subPath := parts[1]

		mode := fs.FileMode(0o644)
		if strings.Contains(path, "/scripts/") {
			mode = 0o755
		}

		hash := sha256.Sum256(data)
		entry := FileEntry{
			Skill:   skillName,
			SubPath: subPath,
			RelPath: path,
			Data:    data,
			Hash:    hex.EncodeToString(hash[:]),
			Mode:    mode,
		}

		skill := index.SkillsByName[skillName]
		if skill == nil {
			skill = &Skill{Name: skillName}
			index.SkillsByName[skillName] = skill
		}
		skill.Files = append(skill.Files, entry)
		return nil
	})
	if err != nil {
		return nil, err
	}

	index.Skills = index.Skills[:0]
	for _, skill := range index.SkillsByName {
		index.Skills = append(index.Skills, *skill)
	}
	sort.Slice(index.Skills, func(i, j int) bool {
		return index.Skills[i].Name < index.Skills[j].Name
	})
	return index, nil
}

func HashFile(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}
