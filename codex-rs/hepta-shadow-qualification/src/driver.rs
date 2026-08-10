use std::path::Path;

use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

use crate::CompletedPreSend;
use crate::DurablePreSendObserver;
use crate::DurablePreSendToken;
use crate::QualificationError;
use crate::request::FIXED_MODEL;
use crate::request::FIXED_PROVIDER;
use crate::request::app_server_sample_request;
use crate::request::json_line;
use crate::request::mcp_sample_request;

const MCP_PROTOCOL_VERSION: &str = "2025-03-26";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunStage {
    NeedAppServer,
    AppServerActive,
    NeedMcp,
    McpActive,
    Complete,
    Failed,
}

#[derive(Debug)]
pub struct QualificationDriverRun {
    expected_work_directory: String,
    observer: DurablePreSendObserver,
    stage: RunStage,
}

impl QualificationDriverRun {
    pub fn create(
        observer_root: impl AsRef<Path>,
        expected_work_directory: impl AsRef<Path>,
    ) -> Result<Self, QualificationError> {
        let expected_work_directory = expected_work_directory.as_ref();
        Ok(Self {
            expected_work_directory: expected_work_directory.to_string_lossy().into_owned(),
            observer: DurablePreSendObserver::create(observer_root, expected_work_directory)?,
            stage: RunStage::NeedAppServer,
        })
    }

    pub fn run_id(&self) -> &str {
        self.observer.run_id()
    }

    pub fn run_root(&self) -> &Path {
        self.observer.run_root()
    }

    pub fn app_server<W>(&mut self, writer: W) -> Result<AppServerDriver<'_, W>, QualificationError>
    where
        W: AsyncWrite + Unpin,
    {
        if self.stage != RunStage::NeedAppServer {
            return Err(state(
                "app-server driver must be created first and exactly once",
            ));
        }
        self.stage = RunStage::AppServerActive;
        Ok(AppServerDriver {
            expected_work_directory: self.expected_work_directory.clone(),
            observer: &mut self.observer,
            run_stage: &mut self.stage,
            stage: AppStage::NeedInitialize,
            writer,
        })
    }

    pub fn mcp<W>(&mut self, writer: W) -> Result<McpDriver<'_, W>, QualificationError>
    where
        W: AsyncWrite + Unpin,
    {
        if self.stage != RunStage::NeedMcp {
            return Err(state(
                "MCP driver requires a completed app-server driver first",
            ));
        }
        self.stage = RunStage::McpActive;
        Ok(McpDriver {
            expected_work_directory: self.expected_work_directory.clone(),
            observer: &mut self.observer,
            run_stage: &mut self.stage,
            stage: McpStage::NeedInitialize,
            writer,
        })
    }

    pub fn finish(self) -> Result<CompletedPreSend, QualificationError> {
        if self.stage != RunStage::Complete {
            return Err(state(
                "driver run requires complete app-server and MCP state machines",
            ));
        }
        self.observer.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AppStage {
    NeedInitialize,
    NeedInitialized,
    NeedThreadStart,
    NeedTurn(u8),
    Complete,
    Failed,
}

pub struct AppServerDriver<'a, W> {
    expected_work_directory: String,
    observer: &'a mut DurablePreSendObserver,
    run_stage: &'a mut RunStage,
    stage: AppStage,
    writer: W,
}

impl<W> AppServerDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    pub async fn initialize(&mut self) -> Result<(), QualificationError> {
        let wire = json_line(&serde_json::json!({
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {"experimentalApi": false, "requestAttestation": false},
                "clientInfo": {
                    "name": "hepta-shadow-qualification",
                    "title": "Hepta Shadow Qualification",
                    "version": "1.0.0",
                },
            },
        }))?;
        self.send_control(AppStage::NeedInitialize, AppStage::NeedInitialized, &wire)
            .await
    }

    pub async fn initialized(&mut self) -> Result<(), QualificationError> {
        let wire = json_line(&serde_json::json!({"method": "initialized"}))?;
        self.send_control(AppStage::NeedInitialized, AppStage::NeedThreadStart, &wire)
            .await
    }

    pub async fn start_thread(&mut self) -> Result<(), QualificationError> {
        let wire = json_line(&serde_json::json!({
            "id": 2,
            "method": "thread/start",
            "params": {
                "approvalPolicy": "never",
                "cwd": self.expected_work_directory,
                "ephemeral": false,
                "model": FIXED_MODEL,
                "modelProvider": FIXED_PROVIDER,
                "sandbox": "workspace-write",
            },
        }))?;
        self.send_control(AppStage::NeedThreadStart, AppStage::NeedTurn(1), &wire)
            .await
    }

    pub async fn start_turn(
        &mut self,
        thread_id: &str,
    ) -> Result<DurablePreSendToken, QualificationError> {
        let ordinal = match self.stage {
            AppStage::NeedTurn(ordinal @ 1..=2) => ordinal,
            _ => return Err(state("app-server turn request is out of order")),
        };
        let wire = app_server_sample_request(ordinal, thread_id)?;
        let token = self.observer.record_app_server(&wire)?;
        self.stage = if ordinal == 1 {
            AppStage::NeedTurn(2)
        } else {
            AppStage::Complete
        };
        if let Err(error) = write_wire(&mut self.writer, &wire).await {
            self.fail();
            return Err(error);
        }
        Ok(token)
    }

    pub fn finish(self) -> Result<W, QualificationError> {
        if self.stage != AppStage::Complete || *self.run_stage != RunStage::AppServerActive {
            return Err(state("app-server driver did not send its exact sequence"));
        }
        *self.run_stage = RunStage::NeedMcp;
        Ok(self.writer)
    }

    async fn send_control(
        &mut self,
        expected: AppStage,
        next: AppStage,
        wire: &[u8],
    ) -> Result<(), QualificationError> {
        if self.stage != expected {
            return Err(state("app-server control message is out of order"));
        }
        self.stage = next;
        if let Err(error) = write_wire(&mut self.writer, wire).await {
            self.fail();
            return Err(error);
        }
        Ok(())
    }

    fn fail(&mut self) {
        self.stage = AppStage::Failed;
        *self.run_stage = RunStage::Failed;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum McpStage {
    NeedInitialize,
    NeedInitialized,
    NeedFirstCall,
    NeedSecondCall,
    Complete,
    Failed,
}

pub struct McpDriver<'a, W> {
    expected_work_directory: String,
    observer: &'a mut DurablePreSendObserver,
    run_stage: &'a mut RunStage,
    stage: McpStage,
    writer: W,
}

impl<W> McpDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    pub async fn initialize(&mut self) -> Result<(), QualificationError> {
        let wire = json_line(&serde_json::json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities": {},
                "clientInfo": {
                    "name": "hepta-shadow-qualification",
                    "title": "Hepta Shadow Qualification",
                    "version": "1.0.0",
                },
                "protocolVersion": MCP_PROTOCOL_VERSION,
            },
        }))?;
        self.send_control(McpStage::NeedInitialize, McpStage::NeedInitialized, &wire)
            .await
    }

    pub async fn initialized(&mut self) -> Result<(), QualificationError> {
        let wire = json_line(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
        }))?;
        self.send_control(McpStage::NeedInitialized, McpStage::NeedFirstCall, &wire)
            .await
    }

    pub async fn start_thread(&mut self) -> Result<DurablePreSendToken, QualificationError> {
        if self.stage != McpStage::NeedFirstCall {
            return Err(state("first MCP tools/call request is out of order"));
        }
        let wire = mcp_sample_request(1, &self.expected_work_directory, None)?;
        let token = self.observer.record_mcp(&wire)?;
        self.stage = McpStage::NeedSecondCall;
        if let Err(error) = write_wire(&mut self.writer, &wire).await {
            self.fail();
            return Err(error);
        }
        Ok(token)
    }

    pub async fn continue_thread(
        &mut self,
        thread_id: &str,
    ) -> Result<DurablePreSendToken, QualificationError> {
        if self.stage != McpStage::NeedSecondCall {
            return Err(state("second MCP tools/call request is out of order"));
        }
        let wire = mcp_sample_request(2, &self.expected_work_directory, Some(thread_id))?;
        let token = self.observer.record_mcp(&wire)?;
        self.stage = McpStage::Complete;
        if let Err(error) = write_wire(&mut self.writer, &wire).await {
            self.fail();
            return Err(error);
        }
        Ok(token)
    }

    pub fn finish(self) -> Result<W, QualificationError> {
        if self.stage != McpStage::Complete || *self.run_stage != RunStage::McpActive {
            return Err(state("MCP driver did not send its exact sequence"));
        }
        *self.run_stage = RunStage::Complete;
        Ok(self.writer)
    }

    async fn send_control(
        &mut self,
        expected: McpStage,
        next: McpStage,
        wire: &[u8],
    ) -> Result<(), QualificationError> {
        if self.stage != expected {
            return Err(state("MCP control message is out of order"));
        }
        self.stage = next;
        if let Err(error) = write_wire(&mut self.writer, wire).await {
            self.fail();
            return Err(error);
        }
        Ok(())
    }

    fn fail(&mut self) {
        self.stage = McpStage::Failed;
        *self.run_stage = RunStage::Failed;
    }
}

async fn write_wire<W>(writer: &mut W, wire: &[u8]) -> Result<(), QualificationError>
where
    W: AsyncWrite + Unpin,
{
    writer.write_all(wire).await?;
    writer.flush().await?;
    Ok(())
}

fn state(message: impl Into<String>) -> QualificationError {
    QualificationError::State(message.into())
}
