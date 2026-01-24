package main

import (
	"flag"
	"fmt"
	"os"
	"strings"

	"loopr/internal/agents"
	"loopr/internal/ops"
	"loopr/internal/version"
)

func main() {
	if len(os.Args) < 2 {
		usage()
		os.Exit(2)
	}

	switch os.Args[1] {
	case "install":
		runInstall(os.Args[2:])
	case "doctor":
		runDoctor(os.Args[2:])
	case "list":
		runList(os.Args[2:])
	case "uninstall":
		runUninstall(os.Args[2:])
	case "codex":
		runCodex(os.Args[2:])
	case "version":
		runVersion()
	case "-h", "--help", "help":
		usage()
	default:
		fmt.Fprintf(os.Stderr, "unknown command: %s\n", os.Args[1])
		usage()
		os.Exit(2)
	}
}

func usage() {
	fmt.Println("loopr <command> [options]")
	fmt.Println("")
	fmt.Println("Commands:")
	fmt.Println("  install     Install loopr skills")
	fmt.Println("  doctor      Validate installed skills")
	fmt.Println("  list        List skills and status")
	fmt.Println("  uninstall   Remove loopr skills")
	fmt.Println("  codex       Run Codex with transcript logging")
	fmt.Println("  version     Show version info")
}

func parseAgents(fs *flag.FlagSet) ([]agents.Spec, error) {
	agent := fs.String("agent", "codex", "target agent (default: codex)")
	all := fs.Bool("all", false, "operate on all supported agents")
	if err := fs.Parse(fs.Args()); err != nil {
		return nil, err
	}
	if *all {
		return agents.All(), nil
	}
	spec, err := agents.Resolve(*agent)
	if err != nil {
		return nil, err
	}
	return []agents.Spec{spec}, nil
}

func parseOnly(fs *flag.FlagSet) []string {
	only := fs.String("only", "", "comma-separated list of skills to target")
	_ = fs.Parse(fs.Args())
	if *only == "" {
		return nil
	}
	parts := strings.Split(*only, ",")
	var out []string
	for _, part := range parts {
		name := strings.TrimSpace(part)
		if name == "" {
			continue
		}
		out = append(out, name)
	}
	return out
}

func runInstall(args []string) {
	fs := flag.NewFlagSet("install", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	agent := fs.String("agent", "codex", "target agent (default: codex)")
	all := fs.Bool("all", false, "operate on all supported agents")
	only := fs.String("only", "", "comma-separated list of skills to install")
	force := fs.Bool("force", false, "overwrite without backup if backup fails")
	verbose := fs.Bool("verbose", false, "show per-skill details")
	if err := fs.Parse(args); err != nil {
		os.Exit(2)
	}

	agentSpecs, err := resolveAgents(*agent, *all)
	if err != nil {
		fail(err)
	}
	onlyList := splitList(*only)

	for _, spec := range agentSpecs {
		report, err := ops.Install(spec, onlyList, *force)
		if err != nil {
			fail(err)
		}
		fmt.Printf("Agent: %s\n", spec.Name)
		printInstallReport(report, *verbose)
	}
}

func runDoctor(args []string) {
	fs := flag.NewFlagSet("doctor", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	agent := fs.String("agent", "codex", "target agent (default: codex)")
	all := fs.Bool("all", false, "operate on all supported agents")
	only := fs.String("only", "", "comma-separated list of skills to check")
	verbose := fs.Bool("verbose", false, "show file-level drift details")
	if err := fs.Parse(args); err != nil {
		os.Exit(2)
	}

	agentSpecs, err := resolveAgents(*agent, *all)
	if err != nil {
		fail(err)
	}
	onlyList := splitList(*only)

	exitCode := 0
	for _, spec := range agentSpecs {
		report, err := ops.Doctor(spec, onlyList)
		if err != nil {
			fail(err)
		}
		fmt.Printf("Agent: %s\n", spec.Name)
		if !printDoctorReport(report, *verbose) {
			exitCode = 1
		}
	}
	os.Exit(exitCode)
}

func runList(args []string) {
	fs := flag.NewFlagSet("list", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	agent := fs.String("agent", "codex", "target agent (default: codex)")
	all := fs.Bool("all", false, "operate on all supported agents")
	only := fs.String("only", "", "comma-separated list of skills to list")
	if err := fs.Parse(args); err != nil {
		os.Exit(2)
	}

	agentSpecs, err := resolveAgents(*agent, *all)
	if err != nil {
		fail(err)
	}
	onlyList := splitList(*only)

	for _, spec := range agentSpecs {
		report, err := ops.Doctor(spec, onlyList)
		if err != nil {
			fail(err)
		}
		fmt.Printf("Agent: %s\n", spec.Name)
		for _, skill := range report.Skills {
			fmt.Printf("  %s\t%s\n", skill.Name, skill.Status)
		}
		for _, extra := range report.ExtraSkills {
			fmt.Printf("  %s\textra\n", extra)
		}
	}
}

func runUninstall(args []string) {
	fs := flag.NewFlagSet("uninstall", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	agent := fs.String("agent", "codex", "target agent (default: codex)")
	all := fs.Bool("all", false, "operate on all supported agents")
	only := fs.String("only", "", "comma-separated list of skills to remove")
	force := fs.Bool("force", false, "remove without backup")
	verbose := fs.Bool("verbose", false, "show per-skill details")
	if err := fs.Parse(args); err != nil {
		os.Exit(2)
	}

	agentSpecs, err := resolveAgents(*agent, *all)
	if err != nil {
		fail(err)
	}
	onlyList := splitList(*only)

	for _, spec := range agentSpecs {
		report, err := ops.Uninstall(spec, onlyList, *force)
		if err != nil {
			fail(err)
		}
		fmt.Printf("Agent: %s\n", spec.Name)
		printUninstallReport(report, *verbose)
	}
}

func runCodex(args []string) {
	exitCode, session, err := ops.RunCodex(args)
	if err != nil {
		fail(err)
	}
	fmt.Printf("Transcript: %s\n", session.LogPath)
	fmt.Printf("Metadata:   %s\n", session.MetaPath)
	os.Exit(exitCode)
}

func runVersion() {
	fmt.Printf("loopr %s\n", version.Version)
	if version.Commit != "" {
		fmt.Printf("commit: %s\n", version.Commit)
	}
	if version.Date != "" {
		fmt.Printf("date: %s\n", version.Date)
	}
}

func resolveAgents(agent string, all bool) ([]agents.Spec, error) {
	if all {
		return agents.All(), nil
	}
	spec, err := agents.Resolve(agent)
	if err != nil {
		return nil, err
	}
	return []agents.Spec{spec}, nil
}

func splitList(value string) []string {
	if strings.TrimSpace(value) == "" {
		return nil
	}
	parts := strings.Split(value, ",")
	var out []string
	for _, part := range parts {
		name := strings.TrimSpace(part)
		if name == "" {
			continue
		}
		out = append(out, name)
	}
	return out
}

func printInstallReport(report ops.InstallReport, verbose bool) {
	if report.BackupPath != "" {
		fmt.Printf("  backup: %s\n", report.BackupPath)
	}
	fmt.Printf("  installed: %d, updated: %d, skipped: %d\n", len(report.Installed), len(report.Updated), len(report.Skipped))
	if verbose {
		printList("installed", report.Installed)
		printList("updated", report.Updated)
		printList("skipped", report.Skipped)
	}
}

func printUninstallReport(report ops.UninstallReport, verbose bool) {
	if report.BackupPath != "" {
		fmt.Printf("  backup: %s\n", report.BackupPath)
	}
	fmt.Printf("  removed: %d\n", len(report.Removed))
	if verbose {
		printList("removed", report.Removed)
	}
}

func printDoctorReport(report ops.DoctorReport, verbose bool) bool {
	ok := true
	for _, skill := range report.Skills {
		fmt.Printf("  %s\t%s\n", skill.Name, skill.Status)
		if skill.Status != "installed" {
			ok = false
			if verbose {
				printList("missing", skill.Missing)
				printList("drifted", skill.Drifted)
			}
		}
	}
	if len(report.ExtraSkills) > 0 {
		ok = false
		if verbose {
			printList("extra", report.ExtraSkills)
		} else {
			fmt.Printf("  extra: %d\n", len(report.ExtraSkills))
		}
	}
	return ok
}

func printList(label string, items []string) {
	if len(items) == 0 {
		return
	}
	fmt.Printf("    %s:\n", label)
	for _, item := range items {
		fmt.Printf("      - %s\n", item)
	}
}

func fail(err error) {
	fmt.Fprintf(os.Stderr, "error: %v\n", err)
	os.Exit(1)
}
