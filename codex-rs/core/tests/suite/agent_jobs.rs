use anyhow::Result;
use codex_features::Feature;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_function_call;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::sse;
use core_test_support::responses::sse_response;
use core_test_support::responses::start_mock_server;
use core_test_support::test_codex::test_codex;
use regex_lite::Regex;
use serde_json::Value;
use serde_json::json;
use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use wiremock::Mock;
use wiremock::Respond;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path_regex;

const WORK_GRAPH_SURFACE_AUDIT_OUTPUT_NEEDLES: &[&str] = &[
    "work_graph_surface_audit_packet",
    "work_graph_canonical_projection_receipt",
    "work_graph_canonical_projection_replay_consistency_decision",
    "work_graph_canonical_projection_closeout_receipt",
    "work_graph_canonical_projection_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_audit_chain_closeout_receipt",
    "work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_operator_review_packet",
    "work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
    "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
    "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
    "work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
    "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
    "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
    "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision",
    "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
    "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision",
    "operatorMatrixRows",
    "taskResultContractId",
    "missingTaskResultContractParts",
];

fn run_large_stack_async_test<F>(name: &'static str, future: F)
where
    F: std::future::Future<Output = Result<()>> + Send + 'static,
{
    const TEST_STACK_SIZE_BYTES: usize = 8 * 1024 * 1024;

    let handle = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(TEST_STACK_SIZE_BYTES)
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_stack_size(TEST_STACK_SIZE_BYTES)
                .enable_all()
                .build()
                .expect("large-stack runtime should build");
            runtime.block_on(future)
        })
        .expect("large-stack test thread should spawn");

    handle
        .join()
        .expect("large-stack test thread panicked")
        .expect("large-stack async test should return ok");
}

struct AgentJobsResponder {
    spawn_args_json: String,
    seen_main: AtomicBool,
    call_counter: AtomicUsize,
    saw_spawn_replay_gate_output: Option<Arc<AtomicBool>>,
    saw_spawn_closeout_receipt_output: Option<Arc<AtomicBool>>,
    saw_spawn_closeout_replay_gate_output: Option<Arc<AtomicBool>>,
    saw_spawn_audit_chain_receipt_output: Option<Arc<AtomicBool>>,
    saw_spawn_reviewed_flag_plan_output: Option<Arc<AtomicBool>>,
    saw_spawn_reviewed_flag_plan_replay_output: Option<Arc<AtomicBool>>,
    saw_spawn_reviewed_flag_closeout_output: Option<Arc<AtomicBool>>,
    saw_spawn_reviewed_flag_closeout_replay_output: Option<Arc<AtomicBool>>,
    saw_spawn_reviewed_flag_audit_chain_closeout_output: Option<Arc<AtomicBool>>,
    saw_spawn_work_graph_surface_audit_output: Option<Arc<AtomicBool>>,
}

impl AgentJobsResponder {
    fn new(spawn_args_json: String) -> Self {
        Self {
            spawn_args_json,
            seen_main: AtomicBool::new(false),
            call_counter: AtomicUsize::new(0),
            saw_spawn_replay_gate_output: None,
            saw_spawn_closeout_receipt_output: None,
            saw_spawn_closeout_replay_gate_output: None,
            saw_spawn_audit_chain_receipt_output: None,
            saw_spawn_reviewed_flag_plan_output: None,
            saw_spawn_reviewed_flag_plan_replay_output: None,
            saw_spawn_reviewed_flag_closeout_output: None,
            saw_spawn_reviewed_flag_closeout_replay_output: None,
            saw_spawn_reviewed_flag_audit_chain_closeout_output: None,
            saw_spawn_work_graph_surface_audit_output: None,
        }
    }

    fn new_with_governance_output_probe(
        spawn_args_json: String,
        saw_spawn_replay_gate_output: Arc<AtomicBool>,
        saw_spawn_closeout_receipt_output: Arc<AtomicBool>,
        saw_spawn_closeout_replay_gate_output: Arc<AtomicBool>,
        saw_spawn_audit_chain_receipt_output: Arc<AtomicBool>,
        saw_spawn_reviewed_flag_plan_output: Arc<AtomicBool>,
        saw_spawn_reviewed_flag_plan_replay_output: Arc<AtomicBool>,
        saw_spawn_reviewed_flag_closeout_output: Arc<AtomicBool>,
        saw_spawn_reviewed_flag_closeout_replay_output: Arc<AtomicBool>,
        saw_spawn_reviewed_flag_audit_chain_closeout_output: Arc<AtomicBool>,
        saw_spawn_work_graph_surface_audit_output: Arc<AtomicBool>,
    ) -> Self {
        Self {
            spawn_args_json,
            seen_main: AtomicBool::new(false),
            call_counter: AtomicUsize::new(0),
            saw_spawn_replay_gate_output: Some(saw_spawn_replay_gate_output),
            saw_spawn_closeout_receipt_output: Some(saw_spawn_closeout_receipt_output),
            saw_spawn_closeout_replay_gate_output: Some(saw_spawn_closeout_replay_gate_output),
            saw_spawn_audit_chain_receipt_output: Some(saw_spawn_audit_chain_receipt_output),
            saw_spawn_reviewed_flag_plan_output: Some(saw_spawn_reviewed_flag_plan_output),
            saw_spawn_reviewed_flag_plan_replay_output: Some(
                saw_spawn_reviewed_flag_plan_replay_output,
            ),
            saw_spawn_reviewed_flag_closeout_output: Some(saw_spawn_reviewed_flag_closeout_output),
            saw_spawn_reviewed_flag_closeout_replay_output: Some(
                saw_spawn_reviewed_flag_closeout_replay_output,
            ),
            saw_spawn_reviewed_flag_audit_chain_closeout_output: Some(
                saw_spawn_reviewed_flag_audit_chain_closeout_output,
            ),
            saw_spawn_work_graph_surface_audit_output: Some(
                saw_spawn_work_graph_surface_audit_output,
            ),
        }
    }
}

struct StopAfterFirstResponder {
    spawn_args_json: String,
    seen_main: AtomicBool,
    worker_calls: Arc<AtomicUsize>,
}

impl StopAfterFirstResponder {
    fn new(spawn_args_json: String, worker_calls: Arc<AtomicUsize>) -> Self {
        Self {
            spawn_args_json,
            seen_main: AtomicBool::new(false),
            worker_calls,
        }
    }
}

impl Respond for StopAfterFirstResponder {
    fn respond(&self, request: &wiremock::Request) -> ResponseTemplate {
        let body_bytes = decode_body_bytes(request);
        let body: Value = serde_json::from_slice(&body_bytes).unwrap_or(Value::Null);

        if has_function_call_output(&body) {
            return sse_response(sse(vec![
                ev_response_created("resp-tool"),
                ev_completed("resp-tool"),
            ]));
        }

        if let Some((job_id, item_id)) = extract_job_and_item(&body) {
            let call_index = self.worker_calls.fetch_add(1, Ordering::SeqCst);
            let call_id = format!("call-worker-{call_index}");
            let stop = call_index == 0;
            let args = json!({
                "job_id": job_id,
                "item_id": item_id,
                "result": { "item_id": item_id },
                "stop": stop,
            });
            let args_json = serde_json::to_string(&args).unwrap_or_else(|err| {
                panic!("worker args serialize: {err}");
            });
            return sse_response(sse(vec![
                ev_response_created("resp-worker"),
                ev_function_call(&call_id, "report_agent_job_result", &args_json),
                ev_completed("resp-worker"),
            ]));
        }

        if !self.seen_main.swap(true, Ordering::SeqCst) {
            return sse_response(sse(vec![
                ev_response_created("resp-main"),
                ev_function_call("call-spawn", "spawn_agents_on_csv", &self.spawn_args_json),
                ev_completed("resp-main"),
            ]));
        }

        sse_response(sse(vec![
            ev_response_created("resp-default"),
            ev_completed("resp-default"),
        ]))
    }
}

impl Respond for AgentJobsResponder {
    fn respond(&self, request: &wiremock::Request) -> ResponseTemplate {
        let body_bytes = decode_body_bytes(request);
        let body: Value = serde_json::from_slice(&body_bytes).unwrap_or(Value::Null);

        if has_function_call_output(&body) {
            if function_call_output_contains(&body, "promotion_review_replay_consistency_decision")
                && let Some(saw_spawn_replay_gate_output) = &self.saw_spawn_replay_gate_output
            {
                saw_spawn_replay_gate_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(&body, "promotion_closeout_receipt")
                && let Some(saw_spawn_closeout_receipt_output) =
                    &self.saw_spawn_closeout_receipt_output
            {
                saw_spawn_closeout_receipt_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(
                &body,
                "promotion_closeout_replay_consistency_decision",
            ) && let Some(saw_spawn_closeout_replay_gate_output) =
                &self.saw_spawn_closeout_replay_gate_output
            {
                saw_spawn_closeout_replay_gate_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(&body, "promotion_review_audit_chain_receipt")
                && let Some(saw_spawn_audit_chain_receipt_output) =
                    &self.saw_spawn_audit_chain_receipt_output
            {
                saw_spawn_audit_chain_receipt_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(&body, "reviewed_flag_precondition_plan_packet")
                && let Some(saw_spawn_reviewed_flag_plan_output) =
                    &self.saw_spawn_reviewed_flag_plan_output
            {
                saw_spawn_reviewed_flag_plan_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(
                &body,
                "reviewed_flag_precondition_plan_replay_consistency_decision",
            ) && let Some(saw_spawn_reviewed_flag_plan_replay_output) =
                &self.saw_spawn_reviewed_flag_plan_replay_output
            {
                saw_spawn_reviewed_flag_plan_replay_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(&body, "reviewed_flag_readiness_closeout_receipt")
                && let Some(saw_spawn_reviewed_flag_closeout_output) =
                    &self.saw_spawn_reviewed_flag_closeout_output
            {
                saw_spawn_reviewed_flag_closeout_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(
                &body,
                "reviewed_flag_readiness_closeout_replay_consistency_decision",
            ) && let Some(saw_spawn_reviewed_flag_closeout_replay_output) =
                &self.saw_spawn_reviewed_flag_closeout_replay_output
            {
                saw_spawn_reviewed_flag_closeout_replay_output.store(true, Ordering::SeqCst);
            }
            if function_call_output_contains(&body, "reviewed_flag_audit_chain_closeout_receipt")
                && let Some(saw_spawn_reviewed_flag_audit_chain_closeout_output) =
                    &self.saw_spawn_reviewed_flag_audit_chain_closeout_output
            {
                saw_spawn_reviewed_flag_audit_chain_closeout_output.store(true, Ordering::SeqCst);
            }
            if WORK_GRAPH_SURFACE_AUDIT_OUTPUT_NEEDLES
                .iter()
                .all(|needle| function_call_output_contains(&body, needle))
                && let Some(saw_spawn_work_graph_surface_audit_output) =
                    &self.saw_spawn_work_graph_surface_audit_output
            {
                saw_spawn_work_graph_surface_audit_output.store(true, Ordering::SeqCst);
            }
            return sse_response(sse(vec![
                ev_response_created("resp-tool"),
                ev_completed("resp-tool"),
            ]));
        }

        if let Some((job_id, item_id)) = extract_job_and_item(&body) {
            let call_id = format!(
                "call-worker-{}",
                self.call_counter.fetch_add(1, Ordering::SeqCst)
            );
            let args = json!({
                "job_id": job_id,
                "item_id": item_id,
                "result": { "item_id": item_id }
            });
            let args_json = serde_json::to_string(&args).unwrap_or_else(|err| {
                panic!("worker args serialize: {err}");
            });
            return sse_response(sse(vec![
                ev_response_created("resp-worker"),
                ev_function_call(&call_id, "report_agent_job_result", &args_json),
                ev_completed("resp-worker"),
            ]));
        }

        if !self.seen_main.swap(true, Ordering::SeqCst) {
            return sse_response(sse(vec![
                ev_response_created("resp-main"),
                ev_function_call("call-spawn", "spawn_agents_on_csv", &self.spawn_args_json),
                ev_completed("resp-main"),
            ]));
        }

        sse_response(sse(vec![
            ev_response_created("resp-default"),
            ev_completed("resp-default"),
        ]))
    }
}

fn decode_body_bytes(request: &wiremock::Request) -> Vec<u8> {
    let Some(encoding) = request
        .headers
        .get("content-encoding")
        .and_then(|value| value.to_str().ok())
    else {
        return request.body.clone();
    };
    if encoding
        .split(',')
        .any(|entry| entry.trim().eq_ignore_ascii_case("zstd"))
    {
        zstd::stream::decode_all(std::io::Cursor::new(&request.body))
            .unwrap_or_else(|_| request.body.clone())
    } else {
        request.body.clone()
    }
}

fn has_function_call_output(body: &Value) -> bool {
    body.get("input")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.get("type").and_then(Value::as_str) == Some("function_call_output")
            })
        })
}

fn function_call_output_contains(body: &Value, needle: &str) -> bool {
    body.get("input")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.get("type").and_then(Value::as_str) == Some("function_call_output")
                    && item
                        .get("output")
                        .and_then(Value::as_str)
                        .is_some_and(|output| output.contains(needle))
            })
        })
}

fn extract_job_and_item(body: &Value) -> Option<(String, String)> {
    let texts = message_input_texts(body);
    let mut combined = texts.join("\n");
    if let Some(instructions) = body.get("instructions").and_then(Value::as_str) {
        combined.push('\n');
        combined.push_str(instructions);
    }
    if !combined.contains("You are processing one item for a generic agent job.") {
        return None;
    }
    let job_id = Regex::new(r"Job ID:\s*([^\n]+)")
        .ok()?
        .captures(&combined)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())?;
    let item_id = Regex::new(r"Item ID:\s*([^\n]+)")
        .ok()?
        .captures(&combined)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())?;
    Some((job_id, item_id))
}

fn message_input_texts(body: &Value) -> Vec<String> {
    let Some(items) = body.get("input").and_then(Value::as_array) else {
        return Vec::new();
    };
    items
        .iter()
        .filter(|item| item.get("type").and_then(Value::as_str) == Some("message"))
        .filter_map(|item| item.get("content").and_then(Value::as_array))
        .flatten()
        .filter(|span| span.get("type").and_then(Value::as_str) == Some("input_text"))
        .filter_map(|span| span.get("text").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}

fn parse_simple_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(str::to_string).collect()
}

#[test]
fn report_agent_job_result_rejects_wrong_thread() {
    run_large_stack_async_test("report_agent_job_result_rejects_wrong_thread", async {
        let server = start_mock_server().await;
        let mut builder = test_codex().with_config(|config| {
            config
                .features
                .enable(Feature::SpawnCsv)
                .expect("test config should allow feature update");
            config
                .features
                .enable(Feature::Sqlite)
                .expect("test config should allow feature update");
        });
        let test = builder.build(&server).await?;

        let input_path = test.cwd_path().join("agent_jobs_wrong_thread.csv");
        let output_path = test.cwd_path().join("agent_jobs_wrong_thread_out.csv");
        fs::write(&input_path, "path\nfile-1\n")?;

        let args = json!({
            "csv_path": input_path.display().to_string(),
            "instruction": "Return {path}",
            "output_csv_path": output_path.display().to_string(),
        });
        let args_json = serde_json::to_string(&args)?;

        let responder = AgentJobsResponder::new(args_json);
        Mock::given(method("POST"))
            .and(path_regex(".*/responses$"))
            .respond_with(responder)
            .mount(&server)
            .await;

        test.submit_turn("run job").await?;

        let db = test.codex.state_db().expect("state db");
        let output = fs::read_to_string(&output_path)?;
        let rows: Vec<&str> = output.lines().skip(1).collect();
        assert_eq!(rows.len(), 1);
        let job_id = rows
            .first()
            .and_then(|line| {
                parse_simple_csv_line(line)
                    .iter()
                    .find(|value| value.len() == 36)
                    .cloned()
            })
            .expect("job_id from csv");
        let job = db.get_agent_job(job_id.as_str()).await?.expect("job");
        let items = db
            .list_agent_job_items(job.id.as_str(), /*status*/ None, Some(10))
            .await?;
        let item = items.first().expect("item");
        let wrong_thread_id = "00000000-0000-0000-0000-000000000000";
        let accepted = db
            .report_agent_job_item_result(
                job.id.as_str(),
                item.item_id.as_str(),
                wrong_thread_id,
                &json!({ "wrong": true }),
                None,
            )
            .await?;
        assert!(!accepted);
        Ok(())
    });
}

#[test]
fn spawn_agents_on_csv_runs_and_exports() {
    run_large_stack_async_test("spawn_agents_on_csv_runs_and_exports", async {
        let server = start_mock_server().await;
        let mut builder = test_codex().with_config(|config| {
            config
                .features
                .enable(Feature::SpawnCsv)
                .expect("test config should allow feature update");
            config
                .features
                .enable(Feature::Sqlite)
                .expect("test config should allow feature update");
        });
        let test = builder.build(&server).await?;

        let input_path = test.cwd_path().join("agent_jobs_input.csv");
        let output_path = test.cwd_path().join("agent_jobs_output.csv");
        fs::write(&input_path, "path,area\nfile-1,test\nfile-2,test\n")?;

        let args = json!({
            "csv_path": input_path.display().to_string(),
            "instruction": "Return {path}",
            "output_csv_path": output_path.display().to_string(),
        });
        let args_json = serde_json::to_string(&args)?;

        let saw_spawn_replay_gate_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_closeout_receipt_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_closeout_replay_gate_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_audit_chain_receipt_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_reviewed_flag_plan_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_reviewed_flag_plan_replay_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_reviewed_flag_closeout_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_reviewed_flag_closeout_replay_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_reviewed_flag_audit_chain_closeout_output = Arc::new(AtomicBool::new(false));
        let saw_spawn_work_graph_surface_audit_output = Arc::new(AtomicBool::new(false));
        let responder = AgentJobsResponder::new_with_governance_output_probe(
            args_json,
            saw_spawn_replay_gate_output.clone(),
            saw_spawn_closeout_receipt_output.clone(),
            saw_spawn_closeout_replay_gate_output.clone(),
            saw_spawn_audit_chain_receipt_output.clone(),
            saw_spawn_reviewed_flag_plan_output.clone(),
            saw_spawn_reviewed_flag_plan_replay_output.clone(),
            saw_spawn_reviewed_flag_closeout_output.clone(),
            saw_spawn_reviewed_flag_closeout_replay_output.clone(),
            saw_spawn_reviewed_flag_audit_chain_closeout_output.clone(),
            saw_spawn_work_graph_surface_audit_output.clone(),
        );
        Mock::given(method("POST"))
            .and(path_regex(".*/responses$"))
            .respond_with(responder)
            .mount(&server)
            .await;

        test.submit_turn("run batch job").await?;

        assert!(
            saw_spawn_replay_gate_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include replay consistency gate"
        );
        assert!(
            saw_spawn_closeout_receipt_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include promotion closeout receipt"
        );
        assert!(
            saw_spawn_closeout_replay_gate_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include closeout replay consistency gate"
        );
        assert!(
            saw_spawn_audit_chain_receipt_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include promotion review audit chain receipt"
        );
        assert!(
            saw_spawn_reviewed_flag_plan_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include reviewed flag precondition plan packet"
        );
        assert!(
            saw_spawn_reviewed_flag_plan_replay_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include reviewed flag precondition plan replay consistency"
        );
        assert!(
            saw_spawn_reviewed_flag_closeout_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include reviewed flag readiness closeout receipt"
        );
        assert!(
            saw_spawn_reviewed_flag_closeout_replay_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include reviewed flag readiness closeout replay consistency"
        );
        assert!(
            saw_spawn_reviewed_flag_audit_chain_closeout_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include reviewed flag audit-chain closeout receipt"
        );
        assert!(
            saw_spawn_work_graph_surface_audit_output.load(Ordering::SeqCst),
            "spawn_agents_on_csv tool output should include WorkGraph surface audit, projection replay, and closeout receipts"
        );
        let output = fs::read_to_string(&output_path)?;
        assert!(output.contains("result_json"));
        assert!(output.contains("item_id"));
        assert!(output.contains("\"item_id\""));
        Ok(())
    });
}

#[test]
fn spawn_agents_on_csv_dedupes_item_ids() {
    run_large_stack_async_test("spawn_agents_on_csv_dedupes_item_ids", async {
        let server = start_mock_server().await;

        let mut builder = test_codex().with_config(|config| {
            config
                .features
                .enable(Feature::SpawnCsv)
                .expect("test config should allow feature update");
            config
                .features
                .enable(Feature::Sqlite)
                .expect("test config should allow feature update");
        });
        let test = builder.build(&server).await?;

        let input_path = test.cwd_path().join("agent_jobs_dupe.csv");
        let output_path = test.cwd_path().join("agent_jobs_dupe_out.csv");
        fs::write(&input_path, "id,path\nfoo,alpha\nfoo,beta\n")?;

        let args = json!({
            "csv_path": input_path.display().to_string(),
            "instruction": "Return {path}",
            "id_column": "id",
            "output_csv_path": output_path.display().to_string(),
        });
        let args_json = serde_json::to_string(&args)?;

        let responder = AgentJobsResponder::new(args_json);
        Mock::given(method("POST"))
            .and(path_regex(".*/responses$"))
            .respond_with(responder)
            .mount(&server)
            .await;

        test.submit_turn("run batch job with duplicate ids").await?;

        let output = fs::read_to_string(&output_path)?;
        let mut lines = output.lines();
        let headers = lines.next().expect("csv headers");
        let header_cols = parse_simple_csv_line(headers);
        let item_id_index = header_cols
            .iter()
            .position(|header| header == "item_id")
            .expect("item_id column");

        let mut item_ids = Vec::new();
        for line in lines {
            let cols = parse_simple_csv_line(line);
            item_ids.push(cols[item_id_index].clone());
        }
        item_ids.sort();
        item_ids.dedup();
        assert_eq!(item_ids.len(), 2);
        assert!(item_ids.contains(&"foo".to_string()));
        assert!(item_ids.contains(&"foo-2".to_string()));
        Ok(())
    });
}

#[test]
fn spawn_agents_on_csv_stop_halts_future_items() {
    run_large_stack_async_test("spawn_agents_on_csv_stop_halts_future_items", async {
        let server = start_mock_server().await;
        let mut builder = test_codex().with_config(|config| {
            config
                .features
                .enable(Feature::SpawnCsv)
                .expect("test config should allow feature update");
            config
                .features
                .enable(Feature::Sqlite)
                .expect("test config should allow feature update");
        });
        let test = builder.build(&server).await?;

        let input_path = test.cwd_path().join("agent_jobs_stop.csv");
        let output_path = test.cwd_path().join("agent_jobs_stop_out.csv");
        fs::write(&input_path, "path\nfile-1\nfile-2\nfile-3\n")?;

        let args = json!({
            "csv_path": input_path.display().to_string(),
            "instruction": "Return {path}",
            "output_csv_path": output_path.display().to_string(),
            "max_concurrency": 1,
        });
        let args_json = serde_json::to_string(&args)?;

        let worker_calls = Arc::new(AtomicUsize::new(0));
        let responder = StopAfterFirstResponder::new(args_json, worker_calls.clone());
        Mock::given(method("POST"))
            .and(path_regex(".*/responses$"))
            .respond_with(responder)
            .mount(&server)
            .await;

        test.submit_turn("run job").await?;

        let output = fs::read_to_string(&output_path)?;
        let rows: Vec<&str> = output.lines().skip(1).collect();
        assert_eq!(rows.len(), 3);
        let job_id = rows
            .first()
            .and_then(|line| {
                parse_simple_csv_line(line)
                    .iter()
                    .find(|value| value.len() == 36)
                    .cloned()
            })
            .expect("job_id from csv");
        let db = test.codex.state_db().expect("state db");
        let job = db.get_agent_job(job_id.as_str()).await?.expect("job");
        assert_eq!(job.status, codex_state::AgentJobStatus::Cancelled);
        let progress = db.get_agent_job_progress(job_id.as_str()).await?;
        assert_eq!(progress.total_items, 3);
        assert_eq!(progress.completed_items, 1);
        assert_eq!(progress.failed_items, 0);
        assert_eq!(progress.running_items, 0);
        assert_eq!(progress.pending_items, 2);
        assert_eq!(worker_calls.load(Ordering::SeqCst), 1);
        Ok(())
    });
}
