package ops

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"gopkg.in/yaml.v3"
)

type SpecsDoctorOptions struct {
	SpecsDir         string
	EnforceUnitTests bool
}

type SpecsDoctorReport struct {
	Warnings     []string
	Errors       []string
	FeatureSlugs []string
	Tasks        map[string][]string
}

func DoctorSpecs(opts SpecsDoctorOptions) (SpecsDoctorReport, error) {
	specsDir := strings.TrimSpace(opts.SpecsDir)
	if specsDir == "" {
		specsDir = "specs"
	}
	report := SpecsDoctorReport{Tasks: map[string][]string{}}
	addErr := func(msg string) {
		report.Errors = append(report.Errors, msg)
	}
	addWarn := func(msg string) {
		report.Warnings = append(report.Warnings, msg)
	}

	featureOrderPath := filepath.Join(specsDir, "feature-order.yaml")
	taskOrderPath := filepath.Join(specsDir, "task-order.yaml")
	testOrderPath := filepath.Join(specsDir, "test-order.yaml")
	specPath := filepath.Join(specsDir, "spec.md")
	looprDir := filepath.Join(filepath.Dir(specsDir), ".loopr")
	initStatePath := filepath.Join(looprDir, "init-state.json")

	mode := "existing"
	if data, err := os.ReadFile(initStatePath); err == nil {
		var initState map[string]any
		if err := json.Unmarshal(data, &initState); err != nil {
			addErr(fmt.Sprintf("Failed to parse %s: %v", initStatePath, err))
		} else if value, ok := initState["mode"].(string); ok {
			switch value {
			case "greenfield", "existing":
				mode = value
			default:
				addErr(fmt.Sprintf("%s: mode must be 'greenfield' or 'existing' (got %q)", initStatePath, value))
			}
		}
	} else if os.IsNotExist(err) {
		addWarn(fmt.Sprintf("Missing %s; assuming existing mode", initStatePath))
	} else {
		return report, err
	}

	if !fileExists(featureOrderPath) {
		addErr(fmt.Sprintf("Missing %s", featureOrderPath))
	}
	if !fileExists(taskOrderPath) {
		addErr(fmt.Sprintf("Missing %s", taskOrderPath))
	}
	if !fileExists(testOrderPath) {
		addErr(fmt.Sprintf("Missing %s", testOrderPath))
	}

	featureOrder := loadYAML(featureOrderPath, addErr)
	taskOrder := loadYAML(taskOrderPath, addErr)
	testOrder := loadYAML(testOrderPath, addErr)

	if data, err := os.ReadFile(specPath); err == nil {
		if !strings.Contains(string(data), "## Testing Strategy") {
			addWarn(fmt.Sprintf("%s: missing '## Testing Strategy' section", specPath))
		}
	} else if os.IsNotExist(err) {
		addWarn(fmt.Sprintf("Missing %s; skipping testing strategy checks", specPath))
	} else {
		return report, err
	}

	featureSlugs := []string{}
	featureTitles := map[string]string{}
	if featureOrder != nil {
		root, ok := asStringMap(featureOrder)
		if !ok {
			addErr("feature-order.yaml: root must be a map")
		} else {
			features, ok := asList(root["features"])
			if !ok || len(features) == 0 {
				addErr("feature-order.yaml: 'features' must be a non-empty list")
			} else {
				seen := map[string]struct{}{}
				for idx, item := range features {
					entry, ok := asStringMap(item)
					if !ok {
						addErr(fmt.Sprintf("feature-order.yaml: feature[%d] must be a map", idx))
						continue
					}
					slug, ok := entry["slug"].(string)
					if !ok || strings.TrimSpace(slug) == "" {
						addErr(fmt.Sprintf("feature-order.yaml: feature[%d].slug must be a string", idx))
						continue
					}
					if _, exists := seen[slug]; exists {
						addErr(fmt.Sprintf("feature-order.yaml: duplicate feature slug '%s'", slug))
						continue
					}
					seen[slug] = struct{}{}
					featureSlugs = append(featureSlugs, slug)
					if title, ok := entry["title"].(string); ok {
						featureTitles[slug] = title
					}
					dependsOn := entry["depends_on"]
					if dependsOn != nil {
						list, ok := asList(dependsOn)
						if !ok {
							addErr(fmt.Sprintf("feature-order.yaml: feature[%d].depends_on must be a list of strings", idx))
						} else if !allStrings(list) {
							addErr(fmt.Sprintf("feature-order.yaml: feature[%d].depends_on must be a list of strings", idx))
						}
					}
				}
			}
		}
	}

	if mode == "greenfield" && len(featureSlugs) > 0 && featureSlugs[0] != "foundation" {
		addErr("feature-order.yaml: first feature must be 'foundation' when mode=greenfield")
	}

	for _, slug := range featureSlugs {
		featurePath := filepath.Join(specsDir, fmt.Sprintf("feature-%s.md", slug))
		if !fileExists(featurePath) {
			addErr(fmt.Sprintf("Missing feature file: %s", featurePath))
			continue
		}
		data, err := os.ReadFile(featurePath)
		if err != nil {
			addErr(fmt.Sprintf("Failed to read %s: %v", featurePath, err))
			continue
		}
		text := string(data)
		if !strings.Contains(text, "## Invariants / Properties") {
			addWarn(fmt.Sprintf("%s: missing '## Invariants / Properties' section", featurePath))
		}
		if !strings.Contains(text, "## PBT Suitability") {
			addWarn(fmt.Sprintf("%s: missing '## PBT Suitability' section", featurePath))
		}
	}

	idRe := regexp.MustCompile(`^[0-9]+$`)
	expectedTasks := map[string][]string{}
	unitRequired := map[string]map[string]bool{}
	taskHasUnit := map[string]map[string]bool{}

	if taskOrder != nil {
		root, ok := asStringMap(taskOrder)
		if !ok {
			addErr("task-order.yaml: root must be a map")
		} else {
			features, ok := asList(root["features"])
			if !ok || len(features) == 0 {
				addErr("task-order.yaml: 'features' must be a non-empty list")
			} else {
				for idx, item := range features {
					entry, ok := asStringMap(item)
					if !ok {
						addErr(fmt.Sprintf("task-order.yaml: feature[%d] must be a map", idx))
						continue
					}
					slug, ok := entry["slug"].(string)
					if !ok || strings.TrimSpace(slug) == "" {
						addErr(fmt.Sprintf("task-order.yaml: feature[%d].slug must be a string", idx))
						continue
					}
					if len(featureSlugs) > 0 {
						if _, ok := featureTitles[slug]; !ok && !contains(featureSlugs, slug) {
							addErr(fmt.Sprintf("task-order.yaml: unknown feature slug '%s'", slug))
						}
					}
					tasks, ok := asList(entry["tasks"])
					if !ok || len(tasks) == 0 {
						addErr(fmt.Sprintf("task-order.yaml: feature '%s' must include non-empty tasks", slug))
						continue
					}
					seenTasks := map[string]struct{}{}
					taskIDs := []string{}
					for tIdx, t := range tasks {
						taskEntry, ok := asStringMap(t)
						if !ok {
							addErr(fmt.Sprintf("task-order.yaml: feature '%s' task[%d] must be a map", slug, tIdx))
							continue
						}
						tid, ok := taskEntry["id"].(string)
						if !ok || !idRe.MatchString(tid) {
							addErr(fmt.Sprintf("task-order.yaml: feature '%s' task[%d].id must be numeric string", slug, tIdx))
							continue
						}
						if _, exists := seenTasks[tid]; exists {
							addErr(fmt.Sprintf("task-order.yaml: feature '%s' has duplicate task id '%s'", slug, tid))
							continue
						}
						seenTasks[tid] = struct{}{}
						taskIDs = append(taskIDs, tid)
						taskPath := filepath.Join(specsDir, fmt.Sprintf("feature-%s-task-%s.md", slug, tid))
						if !fileExists(taskPath) {
							addErr(fmt.Sprintf("Missing task file: %s", taskPath))
							setUnitRequired(unitRequired, slug, tid, true)
						} else {
							data, err := os.ReadFile(taskPath)
							if err != nil {
								addErr(fmt.Sprintf("Failed to read %s: %v", taskPath, err))
								setUnitRequired(unitRequired, slug, tid, true)
							} else {
								setUnitRequired(unitRequired, slug, tid, unitTestsRequired(string(data)))
							}
						}
					}
					expectedTasks[slug] = taskIDs
				}
			}
		}
	}

	for slug, taskIDs := range expectedTasks {
		taskHasUnit[slug] = map[string]bool{}
		for _, tid := range taskIDs {
			taskHasUnit[slug][tid] = false
			if _, ok := unitRequired[slug]; !ok {
				unitRequired[slug] = map[string]bool{}
			}
			if _, ok := unitRequired[slug][tid]; !ok {
				unitRequired[slug][tid] = true
			}
		}
	}

	if testOrder != nil {
		root, ok := asStringMap(testOrder)
		if !ok {
			addErr("test-order.yaml: root must be a map")
		} else {
			features, ok := asList(root["features"])
			if !ok || len(features) == 0 {
				addErr("test-order.yaml: 'features' must be a non-empty list")
			} else {
				for idx, item := range features {
					entry, ok := asStringMap(item)
					if !ok {
						addErr(fmt.Sprintf("test-order.yaml: feature[%d] must be a map", idx))
						continue
					}
					slug, ok := entry["slug"].(string)
					if !ok || strings.TrimSpace(slug) == "" {
						addErr(fmt.Sprintf("test-order.yaml: feature[%d].slug must be a string", idx))
						continue
					}
					if len(expectedTasks) > 0 {
						if _, ok := expectedTasks[slug]; !ok {
							addErr(fmt.Sprintf("test-order.yaml: unknown feature slug '%s'", slug))
							continue
						}
					}
					tasks, ok := asList(entry["tasks"])
					if !ok || len(tasks) == 0 {
						addErr(fmt.Sprintf("test-order.yaml: feature '%s' must include non-empty tasks", slug))
						continue
					}
					for tIdx, t := range tasks {
						taskEntry, ok := asStringMap(t)
						if !ok {
							addErr(fmt.Sprintf("test-order.yaml: feature '%s' task[%d] must be a map", slug, tIdx))
							continue
						}
						tid, ok := taskEntry["id"].(string)
						if !ok || !idRe.MatchString(tid) {
							addErr(fmt.Sprintf("test-order.yaml: feature '%s' task[%d].id must be numeric string", slug, tIdx))
							continue
						}
						if len(expectedTasks) > 0 {
							if ids, ok := expectedTasks[slug]; ok && !contains(ids, tid) {
								addErr(fmt.Sprintf("test-order.yaml: feature '%s' references unknown task id '%s'", slug, tid))
							}
						}
						tests, ok := asList(taskEntry["tests"])
						if !ok || len(tests) == 0 {
							addWarn(fmt.Sprintf("test-order.yaml: feature '%s' task '%s' has no tests", slug, tid))
							continue
						}
						seenTests := map[string]struct{}{}
						for teIdx, te := range tests {
							testEntry, ok := asStringMap(te)
							if !ok {
								addErr(fmt.Sprintf("test-order.yaml: feature '%s' task '%s' test[%d] must be a map", slug, tid, teIdx))
								continue
							}
							teid, ok := testEntry["id"].(string)
							if !ok || !idRe.MatchString(teid) {
								addErr(fmt.Sprintf("test-order.yaml: feature '%s' task '%s' test[%d].id must be numeric string", slug, tid, teIdx))
								continue
							}
							if _, exists := seenTests[teid]; exists {
								addErr(fmt.Sprintf("test-order.yaml: feature '%s' task '%s' has duplicate test id '%s'", slug, tid, teid))
								continue
							}
							seenTests[teid] = struct{}{}
							testPath := filepath.Join(specsDir, fmt.Sprintf("feature-%s-task-%s-test-%s.md", slug, tid, teid))
							if !fileExists(testPath) {
								addErr(fmt.Sprintf("Missing test file: %s", testPath))
								continue
							}
							data, err := os.ReadFile(testPath)
							if err != nil {
								addErr(fmt.Sprintf("Failed to read %s: %v", testPath, err))
								continue
							}
							if isUnitTestType(parseTestType(string(data))) {
								if _, ok := taskHasUnit[slug]; ok {
									taskHasUnit[slug][tid] = true
								}
							}
						}
					}
				}
			}
		}
	}

	for slug, taskIDs := range expectedTasks {
		for _, tid := range taskIDs {
			if !unitRequired[slug][tid] {
				continue
			}
			if taskHasUnit[slug][tid] {
				continue
			}
			taskPath := filepath.Join(specsDir, fmt.Sprintf("feature-%s-task-%s.md", slug, tid))
			msg := fmt.Sprintf("Missing unit test for task: %s", taskPath)
			if opts.EnforceUnitTests {
				addErr(msg)
			} else {
				addWarn(msg)
			}
		}
	}

	report.FeatureSlugs = featureSlugs
	report.Tasks = expectedTasks
	return report, nil
}

func loadYAML(path string, addErr func(string)) any {
	if !fileExists(path) {
		return nil
	}
	data, err := os.ReadFile(path)
	if err != nil {
		addErr(fmt.Sprintf("Failed to read %s: %v", path, err))
		return nil
	}
	var out any
	if err := yaml.Unmarshal(data, &out); err != nil {
		addErr(fmt.Sprintf("Failed to parse %s: %v", path, err))
		return nil
	}
	return out
}

func asStringMap(value any) (map[string]any, bool) {
	switch typed := value.(type) {
	case map[string]any:
		return typed, true
	case map[any]any:
		out := map[string]any{}
		for key, val := range typed {
			ks, ok := key.(string)
			if !ok {
				return nil, false
			}
			out[ks] = val
		}
		return out, true
	default:
		return nil, false
	}
}

func asList(value any) ([]any, bool) {
	switch typed := value.(type) {
	case []any:
		return typed, true
	default:
		return nil, false
	}
}

func allStrings(items []any) bool {
	for _, item := range items {
		if _, ok := item.(string); !ok {
			return false
		}
	}
	return true
}

func parseTestType(text string) string {
	lines := strings.Split(text, "\n")
	for i, line := range lines {
		if strings.EqualFold(strings.TrimSpace(line), "## Type") {
			for _, next := range lines[i+1:] {
				value := strings.TrimSpace(next)
				if value != "" {
					return value
				}
			}
			return ""
		}
	}
	return ""
}

func isUnitTestType(value string) bool {
	return strings.Contains(strings.ToLower(value), "unit")
}

func unitTestsRequired(text string) bool {
	lines := strings.Split(text, "\n")
	for _, line := range lines {
		clean := strings.TrimSpace(line)
		if clean == "" {
			continue
		}
		clean = strings.TrimLeft(clean, "-*+ ")
		lower := strings.ToLower(clean)
		if !strings.HasPrefix(lower, "unit tests required") {
			continue
		}
		value := lower
		if idx := strings.IndexAny(lower, ":?"); idx != -1 {
			value = strings.TrimSpace(lower[idx+1:])
		}
		if strings.Contains(value, "not suitable") || strings.HasPrefix(value, "no") || value == "n/a" || value == "na" {
			return false
		}
		if strings.Contains(value, "yes") || strings.Contains(value, "required") {
			return true
		}
		return true
	}
	return true
}

func contains(items []string, value string) bool {
	for _, item := range items {
		if item == value {
			return true
		}
	}
	return false
}

func setUnitRequired(target map[string]map[string]bool, slug, taskID string, value bool) {
	if _, ok := target[slug]; !ok {
		target[slug] = map[string]bool{}
	}
	target[slug][taskID] = value
}
