use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use loopr::ops::init::{InitOptions, init};
use loopr::ops::loop_config::{default_loop_config, load_loop_config};
use loopr::ops::loop_status::parse_loopr_status;
use loopr::ops::loopr_root::resolve_loopr_root;
use loopr::ops::nanoid::{RandomSource, generate_nanoid, repo_id_alphabet, repo_id_length};
use loopr::ops::run::{RunOptions, plan_steps, run_workflow};
use loopr::ops::work_plan::{load_task_order, load_test_order};
use loopr::ops::work_status::{WorkItemState, WorkItemType, ensure_item, load_work_status};
use loopr::{LooprError, LooprResult};

struct FixedRandom {
    data: Vec<u8>,
    pos: usize,
}

impl FixedRandom {
    fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }
}

impl RandomSource for FixedRandom {
    fn fill(&mut self, buf: &mut [u8]) -> LooprResult<()> {
        for byte in buf {
            if self.pos >= self.data.len() {
                return Err(LooprError::new("random exhausted"));
            }
            *byte = self.data[self.pos];
            self.pos += 1;
        }
        Ok(())
    }
}

struct DirGuard {
    prev: PathBuf,
}

impl DirGuard {
    fn new(path: &Path) -> Self {
        let prev = env::current_dir().unwrap();
        env::set_current_dir(path).unwrap();
        Self { prev }
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.prev);
    }
}

fn temp_dir(name: &str) -> PathBuf {
    let mut dir = env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("loopr-test-{}-{}", name, nanos));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_parse_loopr_status() {
    let log = "noise\n---LOOPR_STATUS---\nSTATUS: COMPLETE\nEXIT_SIGNAL: true\nSUMMARY: all tasks done\n---END_LOOPR_STATUS---\n";
    let (status, ok) = parse_loopr_status(log);
    assert!(ok);
    assert!(status.exit_signal);
    assert_eq!(status.status, "COMPLETE");
    assert_eq!(status.summary, "all tasks done");
}

#[test]
fn test_parse_loopr_status_missing() {
    let (_, ok) = parse_loopr_status("no status here");
    assert!(!ok);
}

#[test]
fn test_load_loop_config_defaults() {
    let dir = temp_dir("config-default");
    let cfg = load_loop_config(&dir.join("config")).unwrap();
    assert_eq!(cfg, default_loop_config());
}

#[test]
fn test_load_loop_config_overrides() {
    let dir = temp_dir("config-overrides");
    let path = dir.join("config");
    fs::write(
        &path,
        "CODEX_TIMEOUT_MINUTES=10\nMAX_ITERATIONS=5\nMAX_MISSING_STATUS=4\nTEST_COMMAND=just test --all\n",
    )
    .unwrap();
    let cfg = load_loop_config(&path).unwrap();
    assert_eq!(cfg.codex_timeout_minutes, 10);
    assert_eq!(cfg.max_iterations, 5);
    assert_eq!(cfg.max_missing_status, 4);
    assert_eq!(cfg.test_command, "just test --all");
}

#[test]
fn test_load_loop_config_unknown_key() {
    let dir = temp_dir("config-unknown");
    let path = dir.join("config");
    fs::write(&path, "UNKNOWN_KEY=1\n").unwrap();
    let cfg = load_loop_config(&path).unwrap();
    assert_eq!(cfg, default_loop_config());
}

#[test]
fn test_plan_steps_defaults() {
    let steps = plan_steps(&RunOptions {
        loopr_root: None,
        from: String::new(),
        to: String::new(),
        step: String::new(),
        seed: String::new(),
        confirm: false,
        no_prompt: false,
        codex: false,
        codex_args: Vec::new(),
        progress: None,
    })
    .unwrap();
    assert!(!steps.is_empty());
}

#[test]
fn test_plan_steps_range() {
    let steps = plan_steps(&RunOptions {
        loopr_root: None,
        from: "tasks".to_string(),
        to: "tests".to_string(),
        step: String::new(),
        seed: String::new(),
        confirm: false,
        no_prompt: false,
        codex: false,
        codex_args: Vec::new(),
        progress: None,
    })
    .unwrap();
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].name, "tasks");
    assert_eq!(steps[1].name, "tests");
}

#[test]
fn test_run_workflow_dry_run_shows_all_steps() {
    let root = temp_dir("dry-run");
    let _guard = DirGuard::new(&root);

    let report = run_workflow(RunOptions {
        loopr_root: None,
        from: String::new(),
        to: String::new(),
        step: String::new(),
        seed: String::new(),
        confirm: false,
        no_prompt: false,
        codex: false,
        codex_args: Vec::new(),
        progress: None,
    })
    .unwrap();

    assert!(!report.steps.is_empty());
}

#[test]
fn test_run_workflow_dry_run_does_not_require_repo_id() {
    let root = temp_dir("dry-run-norepo");
    let _guard = DirGuard::new(&root);

    let report = run_workflow(RunOptions {
        loopr_root: None,
        from: String::new(),
        to: String::new(),
        step: String::new(),
        seed: String::new(),
        confirm: false,
        no_prompt: false,
        codex: false,
        codex_args: Vec::new(),
        progress: None,
    })
    .unwrap();

    assert!(!report.steps.is_empty());
    assert_eq!(report.steps[0].name, "prd");
    assert!(!root.join("loopr").join("state").join("handoff.md").exists());
}

#[test]
fn test_init_greenfield_creates_repo_id() {
    let root = temp_dir("init-greenfield");
    let report = init(InitOptions {
        root: root.clone(),
        rand: Some(Box::new(FixedRandom::new(vec![0, 1, 2, 3, 4, 5]))),
        no_agents: true,
    })
    .unwrap();

    assert_eq!(report.repo_id, "useand");
    assert!(report.repo_id_created);
    assert!(report.transcripts_dir.exists());

    let gitignore_path = root.join("loopr").join(".gitignore");
    let gitignore_body = fs::read_to_string(gitignore_path).unwrap();
    assert!(gitignore_body.contains("state/"));
}

#[test]
fn test_init_reuses_repo_id_when_present() {
    let root = temp_dir("init-reuse");
    let repo_id_path = root.join("loopr").join("repo-id");
    fs::create_dir_all(repo_id_path.parent().unwrap()).unwrap();
    fs::write(&repo_id_path, "abc123\n").unwrap();

    let report = init(InitOptions {
        root: root.clone(),
        rand: None,
        no_agents: true,
    })
    .unwrap();

    assert_eq!(report.repo_id, "abc123");
    assert!(!report.repo_id_created);
}

#[test]
fn test_init_creates_agents_and_docs_index() {
    let root = temp_dir("init-agents");
    let report = init(InitOptions {
        root: root.clone(),
        rand: None,
        no_agents: false,
    })
    .unwrap();

    assert!(!report.repo_id.is_empty());
    let agents_path = root.join("AGENTS.md");
    assert!(agents_path.exists());
    let agents_body = fs::read_to_string(&agents_path).unwrap();
    assert!(agents_body.contains("[loopr: injected"));

    let docs_index = root.join("loopr").join("state").join("docs-index.txt");
    assert!(docs_index.exists());
}

#[test]
fn test_init_injects_agents_when_present() {
    let root = temp_dir("init-agents-existing");
    let agents_path = root.join("AGENTS.md");
    fs::write(&agents_path, "# AGENTS\n\nCustom instructions.\n").unwrap();

    init(InitOptions {
        root: root.clone(),
        rand: None,
        no_agents: false,
    })
    .unwrap();

    let agents_body = fs::read_to_string(&agents_path).unwrap();
    assert!(agents_body.contains("Custom instructions."));
    assert!(agents_body.contains("[loopr: injected"));

    // Running init again should not duplicate the injected section.
    init(InitOptions {
        root: root.clone(),
        rand: None,
        no_agents: false,
    })
    .unwrap();
    let agents_body_again = fs::read_to_string(&agents_path).unwrap();
    let marker_count = agents_body_again.matches("[loopr: injected").count();
    assert_eq!(marker_count, 1);
}

#[test]
fn test_init_no_agents_skips_injection() {
    let root = temp_dir("init-no-agents");
    let agents_path = root.join("AGENTS.md");
    fs::write(&agents_path, "# AGENTS\n\nCustom instructions.\n").unwrap();

    init(InitOptions {
        root: root.clone(),
        rand: None,
        no_agents: true,
    })
    .unwrap();

    let agents_body = fs::read_to_string(&agents_path).unwrap();
    assert!(!agents_body.contains("[loopr: injected"));

    let docs_index = root.join("loopr").join("state").join("docs-index.txt");
    assert!(docs_index.exists());
}

#[test]
fn test_resolve_loopr_root_prefers_override() {
    let root_a = temp_dir("root-a");
    let root_b = temp_dir("root-b");
    write_repo_id(&root_a, "aaaaaa");
    write_repo_id(&root_b, "bbbbbb");

    let (resolved_root, repo_id) = resolve_loopr_root(&temp_dir("root-c"), Some(&root_b)).unwrap();
    assert_eq!(resolved_root, root_b);
    assert_eq!(repo_id, "bbbbbb");
}

#[test]
fn test_resolve_loopr_root_searches_upwards() {
    let root = temp_dir("root-up");
    write_repo_id(&root, "cccccc");
    let nested = root.join("a").join("b");
    fs::create_dir_all(&nested).unwrap();

    let (resolved_root, repo_id) = resolve_loopr_root(&nested, None).unwrap();
    assert_eq!(resolved_root, root);
    assert_eq!(repo_id, "cccccc");
}

#[test]
fn test_resolve_loopr_root_missing() {
    let err = resolve_loopr_root(&temp_dir("missing"), None).unwrap_err();
    assert!(err.message.contains("loopr/repo-id"));
    assert!(err.message.contains("loopr init"));
}

#[test]
fn test_generate_nanoid_deterministic() {
    let data = vec![0u8; repo_id_length()];
    let mut rng = FixedRandom::new(data);
    let id = generate_nanoid(&mut rng, repo_id_length()).unwrap();
    let expected = repo_id_alphabet()
        .chars()
        .next()
        .unwrap()
        .to_string()
        .repeat(repo_id_length());
    assert_eq!(id, expected);
}

#[test]
fn test_load_task_order_parses_yaml() {
    let dir = temp_dir("task-order");
    let path = dir.join("task-order.yaml");
    fs::write(
        &path,
        "version: 1\ntasks:\n  - id: 1\n    key: foundation\n    title: \"Foundation\"\n    file: specs/feature-001-task-001.md\n    depends_on: []\n",
    )
    .unwrap();
    let order = load_task_order(&path).unwrap();
    assert_eq!(order.version, 1);
    assert_eq!(order.tasks.len(), 1);
    assert_eq!(order.tasks[0].key, "foundation");
}

#[test]
fn test_load_test_order_parses_yaml() {
    let dir = temp_dir("test-order");
    let path = dir.join("test-order.yaml");
    fs::write(
        &path,
        "version: 1\ntests:\n  - id: 1\n    key: foundation-test\n    title: \"Test: Foundation\"\n    task_id: 1\n    file: specs/feature-001-task-001-test-001.md\n    depends_on: []\n    kind: pbt\n",
    )
    .unwrap();
    let order = load_test_order(&path).unwrap();
    assert_eq!(order.version, 1);
    assert_eq!(order.tests.len(), 1);
    assert_eq!(order.tests[0].kind.as_deref(), Some("pbt"));
}

#[test]
fn test_work_status_ensure_item() {
    let dir = temp_dir("work-status");
    let path = dir.join("work-status.json");
    let now = "2026-01-31T00:00:00Z";
    let mut status = load_work_status(&path, now).unwrap();
    ensure_item(&mut status, "task-one", WorkItemType::Task, now);
    let item = status.items.get("task-one").unwrap();
    assert_eq!(item.state, WorkItemState::NotStarted);
}

fn write_repo_id(root: &Path, repo_id: &str) {
    let path = root.join("loopr");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("repo-id"), repo_id).unwrap();
}
