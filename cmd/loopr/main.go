package main

import (
	"errors"
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
	case "init":
		runInit(os.Args[2:])
	case "run":
		runRun(os.Args[2:])
	case "install":
		runInstall(os.Args[2:])
	case "doctor":
		runDoctor(os.Args[2:])
	case "list":
		runList(os.Args[2:])
	case "uninstall":
		runUninstall(os.Args[2:])
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
	fmt.Println("  init       Initialize Loopr metadata in a repo")
	fmt.Println("  run        Orchestrate Loopr steps (requires --codex or --dry-run)")
	fmt.Println("  install     Install loopr skills")
	fmt.Println("  doctor      Validate installed skills")
	fmt.Println("  list        List skills and status")
	fmt.Println("  uninstall   Remove loopr skills")
	fmt.Println("  version     Show version info")
}

func runInit(args []string) {
	fs := flag.NewFlagSet("init", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	root := fs.String("root", ".", "repo root to scan")
	specsDir := fs.String("specs-dir", "specs", "specs directory relative to root")
	allowExisting := fs.Bool("allow-existing", false, "allow initialization in non-greenfield repos")
	if err := fs.Parse(args); err != nil {
		os.Exit(2)
	}

	report, err := ops.Init(ops.InitOptions{
		Root:          *root,
		SpecsDir:      *specsDir,
		AllowExisting: *allowExisting,
	})
	if err != nil {
		var ng ops.NonGreenfieldError
		if errors.As(err, &ng) {
			fmt.Fprintln(os.Stderr, "non-greenfield signals detected (rerun with --allow-existing):")
			for _, signal := range ng.Signals {
				fmt.Fprintf(os.Stderr, "- %s\n", signal)
			}
			os.Exit(1)
		}
		fail(err)
	}

	fmt.Printf("Repo root:   %s\n", report.Root)
	fmt.Printf("Repo ID:     %s\n", report.RepoID)
	if report.Mode != "" {
		fmt.Printf("Mode:        %s\n", report.Mode)
	}
	if report.InitStateCreated {
		fmt.Printf("Init state:  %s\n", report.InitStatePath)
	} else {
		fmt.Printf("Init state:  %s (exists)\n", report.InitStatePath)
	}
	fmt.Printf("Decisions:   %s\n", report.DecisionsDir)
	fmt.Printf("Transcripts: %s\n", report.TranscriptsDir)
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

func runRun(args []string) {
	looprArgs, agentArgs := splitOnDoubleDash(args)
	looprArgs, agentArgs = extractCodexPassthroughFlags(looprArgs, agentArgs)
	fs := flag.NewFlagSet("run", flag.ContinueOnError)
	fs.SetOutput(os.Stderr)
	from := fs.String("from", "", "start step (prd|spec|features|tasks|tests|execute)")
	to := fs.String("to", "", "end step (prd|spec|features|tasks|tests|execute)")
	step := fs.String("step", "", "single step to run (overrides --from/--to)")
	seed := fs.String("seed", "", "seed prompt for PRD (required if prd is missing)")
	force := fs.Bool("force", false, "rerun steps even if outputs exist")
	confirm := fs.Bool("confirm", false, "ask before each step")
	codex := fs.Bool("codex", false, "run steps with Codex (pass Codex args after --)")
	dryRun := fs.Bool("dry-run", false, "print workflow steps without running Codex")
	looprRoot := fs.String("loopr-root", "", "loopr workspace root")
	if err := fs.Parse(looprArgs); err != nil {
		os.Exit(2)
	}
	if *dryRun {
		*codex = false
		agentArgs = nil
		*seed = ""
		*confirm = false
		*force = false
	}
	if len(agentArgs) > 0 && !*codex && !*dryRun {
		fail(fmt.Errorf("agent args provided but --codex not set"))
	}
	if !*codex && !*dryRun {
		fail(fmt.Errorf("run requires --codex or --dry-run"))
	}

	opts := ops.RunOptions{
		LooprRoot: *looprRoot,
		From:      *from,
		To:        *to,
		Step:      *step,
		Seed:      *seed,
		Force:     *force,
		Confirm:   *confirm,
		Codex:     *codex,
		CodexArgs: agentArgs,
	}
	if *codex {
		opts.Progress = func(event ops.ProgressEvent) {
			fmt.Printf("Step %d/%d %s: %s\n", event.Index, event.Total, event.Step.Name, event.Status)
		}
	}
	report, err := ops.RunWorkflow(opts)
	if err != nil {
		fail(err)
	}
	if !*codex {
		for _, step := range report.Steps {
			fmt.Printf("Step: %s\n", step.Name)
			fmt.Printf("  skill: %s\n", step.Skill)
			for _, input := range step.Inputs {
				fmt.Printf("  input: %s\n", input)
			}
			for _, output := range step.Outputs {
				fmt.Printf("  output: %s\n", output)
			}
		}
		return
	}
	if report.LastSession != nil {
		fmt.Printf("Transcript: %s\n", report.LastSession.LogPath)
		fmt.Printf("Metadata:   %s\n", report.LastSession.MetaPath)
	}
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

func splitOnDoubleDash(args []string) ([]string, []string) {
	for i, arg := range args {
		if arg == "--" {
			if i == 0 {
				return nil, args[i+1:]
			}
			return args[:i], args[i+1:]
		}
	}
	return args, nil
}

func extractCodexPassthroughFlags(looprArgs, agentArgs []string) ([]string, []string) {
	if len(agentArgs) > 0 || !hasCodexFlag(looprArgs) {
		return looprArgs, agentArgs
	}
	filtered := make([]string, 0, len(looprArgs))
	for _, arg := range looprArgs {
		if isCodexHelpFlag(arg) {
			agentArgs = append(agentArgs, arg)
			continue
		}
		filtered = append(filtered, arg)
	}
	return filtered, agentArgs
}

func hasCodexFlag(args []string) bool {
	for _, arg := range args {
		if arg == "--codex" || strings.HasPrefix(arg, "--codex=") {
			return true
		}
	}
	return false
}

func isCodexHelpFlag(arg string) bool {
	switch arg {
	case "-h", "-help", "--help", "-V", "--version":
		return true
	default:
		return strings.HasPrefix(arg, "--help=") || strings.HasPrefix(arg, "--version=")
	}
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
