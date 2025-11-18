# LogAI v0.2.0 - MCP Integration Release

## 🎉 Major New Feature: MCP Integration

LogAI now supports the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/), enabling it to connect to external tools and data sources during log analysis!

### What is MCP?

MCP (Model Context Protocol) is an open protocol that allows AI applications to connect to external tools and data sources. With MCP, LogAI can now:

- Query external documentation (Stack Overflow, GitHub)
- Check system metrics (Prometheus, CloudWatch)
- Search code repositories
- Access historical logs
- Use any custom MCP-compatible tool

### New Features

#### MCP Client
- ✅ Connect to multiple MCP servers via stdio transport
- ✅ Automatic tool discovery from connected servers
- ✅ Tool invocation with parameter validation
- ✅ Configurable timeouts per tool
- ✅ Graceful error handling and degradation

#### CLI Enhancements
- ✅ `--mcp-config <path>` - Specify MCP configuration file
- ✅ `--no-mcp` - Disable MCP tools integration
- ✅ Automatic MCP initialization when AI analysis is enabled

#### AI Provider Integration
- ✅ Enhanced `analyze_with_tools()` method for all AI providers
- ✅ Tool results automatically included in AI analysis context
- ✅ Tool invocation tracking in analysis output

### Configuration

Create `~/.logai/mcp.toml`:

```toml
default_timeout = 30

[[servers]]
name = "filesystem"
enabled = true

[servers.connection]
type = "Stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[tool_timeouts]
# Optional per-tool timeout overrides
read_file = 60
```

### Usage Examples

```bash
# Use with default MCP config
logai investigate app.log --ai ollama

# Use custom MCP config
logai investigate app.log --ai ollama --mcp-config ./my-mcp.toml

# Disable MCP tools
logai investigate app.log --ai ollama --no-mcp
```

### Architecture

```
LogAI CLI
    ↓
Analyzer (groups errors)
    ↓
AI Provider (enhanced with MCP)
    ↓
MCP Client
    ↓
MCP Servers (filesystem, custom tools, etc.)
```

### What's Next?

The MCP integration opens up exciting possibilities:

- **Built-in MCP Tools** (v0.3.0)
  - `search_docs` - Query Stack Overflow and GitHub
  - `check_metrics` - Query Prometheus/CloudWatch
  - `search_code` - Search code repositories
  - `query_logs` - Search historical logs

- **Additional Transports** (v0.3.0)
  - HTTP transport
  - WebSocket transport

- **Enhanced Output** (v0.3.0)
  - Display tool invocations in analysis output
  - Show which tools contributed to suggestions

### Breaking Changes

None! MCP integration is opt-in and fully backward compatible.

### Bug Fixes

- Fixed logger initialization for proper debug output
- Improved error handling in MCP client
- Better cache management for MCP-enhanced analysis

### Documentation

- [MCP Integration Guide](docs/MCP_INTEGRATION.md)
- [Example MCP Configuration](examples/mcp-config.toml)
- [MCP Test Example](examples/test_mcp.rs)

### Contributors

Special thanks to the MCP community and all contributors!

### Installation

```bash
# Cargo
cargo install logai

# Homebrew
brew tap ranjan-mohanty/logai
brew install logai

# From source
git clone https://github.com/ranjan-mohanty/logai.git
cd logai
cargo install --path .
```

### Feedback

We'd love to hear your feedback on the MCP integration! Please:
- 🐛 [Report bugs](https://github.com/ranjan-mohanty/logai/issues/new?labels=bug)
- 💡 [Request features](https://github.com/ranjan-mohanty/logai/issues/new?labels=enhancement)
- 💬 [Join discussions](https://github.com/ranjan-mohanty/logai/discussions)

---

**Full Changelog**: https://github.com/ranjan-mohanty/logai/compare/v0.1.0...v0.2.0
