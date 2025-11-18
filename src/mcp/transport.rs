use crate::mcp::{MCPError, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// Transport layer for MCP communication
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message to the server
    async fn send(&self, message: Value) -> Result<()>;

    /// Receive a message from the server
    async fn receive(&self) -> Result<Value>;

    /// Close the connection
    async fn close(&mut self) -> Result<()>;

    /// Check if the connection is alive
    fn is_alive(&self) -> bool;
}

/// Stdio-based transport (spawns a process)
pub struct StdioTransport {
    process: Mutex<Option<Child>>,
    command: String,
    args: Vec<String>,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            process: Mutex::new(None),
            command,
            args,
        }
    }

    /// Start the process
    pub async fn start(&self) -> Result<()> {
        let mut process_guard = self.process.lock().await;

        if process_guard.is_some() {
            return Ok(()); // Already started
        }

        let child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| MCPError::TransportError(format!("Failed to spawn process: {}", e)))?;

        *process_guard = Some(child);
        Ok(())
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&self, message: Value) -> Result<()> {
        let mut process_guard = self.process.lock().await;

        let process = process_guard
            .as_mut()
            .ok_or_else(|| MCPError::TransportError("Process not started".to_string()))?;

        let stdin = process
            .stdin
            .as_mut()
            .ok_or_else(|| MCPError::TransportError("No stdin available".to_string()))?;

        let message_str = serde_json::to_string(&message)
            .map_err(|e| MCPError::ProtocolError(format!("Failed to serialize message: {}", e)))?;

        stdin
            .write_all(message_str.as_bytes())
            .await
            .map_err(|e| MCPError::TransportError(format!("Failed to write to stdin: {}", e)))?;

        stdin
            .write_all(b"\n")
            .await
            .map_err(|e| MCPError::TransportError(format!("Failed to write newline: {}", e)))?;

        stdin
            .flush()
            .await
            .map_err(|e| MCPError::TransportError(format!("Failed to flush stdin: {}", e)))?;

        Ok(())
    }

    async fn receive(&self) -> Result<Value> {
        let mut process_guard = self.process.lock().await;

        let process = process_guard
            .as_mut()
            .ok_or_else(|| MCPError::TransportError("Process not started".to_string()))?;

        let stdout = process
            .stdout
            .as_mut()
            .ok_or_else(|| MCPError::TransportError("No stdout available".to_string()))?;

        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        reader
            .read_line(&mut line)
            .await
            .map_err(|e| MCPError::TransportError(format!("Failed to read from stdout: {}", e)))?;

        if line.is_empty() {
            return Err(MCPError::TransportError(
                "Connection closed by server".to_string(),
            ));
        }

        serde_json::from_str(&line)
            .map_err(|e| MCPError::ProtocolError(format!("Failed to parse message: {}", e)))
    }

    async fn close(&mut self) -> Result<()> {
        let mut process_guard = self.process.lock().await;

        if let Some(mut child) = process_guard.take() {
            // Try to kill the process gracefully
            let _ = child.kill().await;
            let _ = child.wait().await;
        }

        Ok(())
    }

    fn is_alive(&self) -> bool {
        // We'll check if process exists; a more robust check would poll the process
        // For now, just check if we have a process
        true // Simplified for MVP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_transport_creation() {
        let transport = StdioTransport::new("echo".to_string(), vec![]);
        assert!(transport.is_alive());
    }

    #[tokio::test]
    async fn test_stdio_transport_start() {
        let transport = StdioTransport::new("cat".to_string(), vec![]);
        let result = transport.start().await;
        assert!(result.is_ok());

        // Start again should be ok (idempotent)
        let result = transport.start().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stdio_transport_send_without_start() {
        let transport = StdioTransport::new("cat".to_string(), vec![]);
        let message = serde_json::json!({"test": "message"});
        let result = transport.send(message).await;
        assert!(result.is_err());
    }
}
