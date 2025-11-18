# MCP Integration in LogAI

## Overview

LogAI now supports Model Context Protocol (MCP) integration, allowing it to
connect to external MCP servers and utilize their tools during log analysis.

## Features

- ✅ Connect to multiple MCP servers via stdio transport
- ✅ Automatic tool discovery from connected servers
- ✅ Tool invocation with timeout support
- ✅ Parameter validation
- ✅ Graceful error handling and degradation
- ✅ Configuration via TOML file

## Configuration

Create a configuration file at `~/.logai/mcp.toml` or specify a custom path with
`--mcp-config`:

```toml
# Default timeout for all tools (seconds)
default_timeout = 30

# MCP Server configuration
[[servers]]
name = "filesystem"
enabled = true

[servers.connection]
type = "Stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# Per-tool timeout overrides
[tool_timeouts]
read_file = 60
search_files = 45
```

## Usage

### Enable MCP Tools

```bash
# Use default config (~/.logai/mcp.toml)
logai investigate app.log --ai ollama

# Use custom config
logai investigate app.log --ai ollama --mcp-config ./my-mcp-config.toml

# Disable MCP tools
logai investigate app.log --ai ollama --no-mcp
```

### Available MCP Servers

LogAI can connect to any MCP-compatible server. Popular servers include:

- `@modelcontextprotocol/server-filesystem` - File system operations
- `@modelcontextprotocol/server-github` - GitHub API access
- `@modelcontextprotocol/server-postgres` - PostgreSQL database access
- Custom servers implementing the MCP protocol

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         LogAI CLI                            │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Analyzer Module                           │
│  (Groups errors, prepares analysis context)                  │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   AI Provider (Enhanced)                     │
│  • analyze_with_tools() method                               │
│  • Invokes MCP tools during analysis                         │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      MCP Client                              │
│  • Connection Management                                     │
│  • Tool Discovery                                            │
│  • Tool Invocation with Timeout                              │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     MCP Servers                              │
│  (External tools and data sources)                           │
└─────────────────────────────────────────────────────────────┘
```

## Implementation Status

### Completed (Phase 1 & 2)

- ✅ MCP protocol types and configuration
- ✅ MCPClient with connection management
- ✅ Stdio transport layer
- ✅ Connection and tool discovery
- ✅ Tool invocation with timeout
- ✅ CLI integration (--no-mcp, --mcp-config flags)
- ✅ Main analysis flow integration
- ✅ Enhanced AIProvider trait with analyze_with_tools()

### Pending (Phase 3+)

- ⏳ Built-in MCP tools (search_docs, check_metrics, search_code, query_logs)
- ⏳ HTTP and WebSocket transports
- ⏳ Tool result formatting in output
- ⏳ Advanced error recovery strategies

## Testing

Run the integration test:

```bash
# Test with filesystem server
cargo run --example test_mcp

# Test with custom config
cargo run -- investigate examples/test-error.log --ai ollama --mcp-config examples/mcp-config.toml
```

## Next Steps

1. Implement built-in MCP tools for common log analysis tasks
2. Add HTTP and WebSocket transport support
3. Enhance output formatting to show tool invocations
4. Add MCP tool result caching
5. Implement tool prioritization and composition

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP Servers](https://github.com/modelcontextprotocol/servers)
- [LogAI MCP Design Doc](.kiro/specs/mcp-integration/design.md)
