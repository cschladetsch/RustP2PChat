# Shell Scripts Documentation

This directory contains a comprehensive collection of shell scripts for testing, demonstrating, and managing the Rust P2P Chat application. These scripts automate common development workflows and provide various testing scenarios.

## 📁 Script Overview

### 🧪 Testing Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| **`quick_test.sh`** | Rapid two-peer testing setup | `./quick_test.sh [port1] [port2]` |
| **`automated_test.sh`** | Automated test scenarios with predefined inputs | `./automated_test.sh` |
| **`comprehensive_test.sh`** | Full feature test suite | `./comprehensive_test.sh` |
| **`test_encryption.sh`** | Encryption-specific functionality tests | `./test_encryption.sh` |
| **`test_p2p.sh`** | P2P connection and messaging tests | `./test_p2p.sh` |
| **`test_tmux.sh`** | Split-screen terminal testing using tmux | `./test_tmux.sh` |
| **`test_chat.sh`** | Basic chat functionality testing | `./test_chat.sh` |

### 🎬 Demo Scripts

| Script | Purpose | Usage |
|--------|---------|-------|
| **`demo.sh`** | Basic demonstration setup | `./demo.sh` |
| **`demo_chat.sh`** | Interactive chat demonstration | `./demo_chat.sh` |
| **`demo_colors.sh`** | Terminal color and formatting tests | `./demo_colors.sh` |

## 🚀 Script Descriptions

### `quick_test.sh` - Rapid Testing
**Purpose**: Fast two-peer testing with automatic setup

**Features**:
- Automatic port validation (1024-65535)
- Error handling with cleanup
- Color-coded output for better visibility
- Signal handling for graceful shutdown
- Background/foreground peer management

**Usage**:
```bash
# Default ports (8080, 8081)
./quick_test.sh

# Custom ports
./quick_test.sh 3000 3001

# With port names
./quick_test.sh --port1 8080 --port2 8081
```

**Error Handling**:
- Validates port ranges
- Checks for cargo availability
- Automatic cleanup on exit
- Process management with PID tracking

### `automated_test.sh` - Unattended Testing
**Purpose**: Fully automated testing with predefined message sequences

**Features**:
- Named pipe communication for automation
- Predefined message exchange scenarios
- Automatic log analysis and result verification
- Connection success validation
- Cleanup of temporary files

**Test Sequence**:
1. Creates named pipes for input simulation
2. Starts two peers with different nicknames
3. Exchanges predefined messages
4. Analyzes logs for connection success
5. Reports pass/fail status

**Output Analysis**:
- Searches for connection indicators
- Extracts message exchanges
- Validates peer communication
- Provides test result summary

### `comprehensive_test.sh` - Full Feature Testing
**Purpose**: Complete test suite covering all application features

**Test Categories**:
- Basic connection establishment
- Message exchange validation
- File transfer functionality
- Command system testing
- Error handling verification
- Performance and stress testing

### `test_encryption.sh` - Security Testing
**Purpose**: Dedicated encryption and security feature testing

**Test Areas**:
- RSA key generation and exchange
- AES-256-GCM encryption/decryption
- Message integrity verification
- Key rotation and session security
- Error handling for cryptographic failures

### `test_p2p.sh` - Network Testing
**Purpose**: P2P-specific connection and networking tests

**Features**:
- Connection race condition testing
- Network failure simulation
- Timeout handling validation
- Multi-peer connection scenarios
- IPv4/IPv6 compatibility testing

### `test_tmux.sh` - Visual Testing
**Purpose**: Split-screen terminal testing using tmux

**Features**:
- Automatic tmux session creation
- Split-pane configuration
- Side-by-side peer visualization
- Interactive testing environment
- Session cleanup on exit

### `demo.sh` - Feature Showcase
**Purpose**: Demonstrates implemented features and capabilities

**Highlights**:
- Project build verification
- Feature summary display
- Usage instructions
- Command reference
- Quick start guide

### `demo_chat.sh` - Interactive Demo
**Purpose**: Guided interactive demonstration

**Features**:
- Step-by-step chat setup
- Feature explanation during demo
- Interactive command examples
- Real-time demonstration of capabilities

### `demo_colors.sh` - UI Testing
**Purpose**: Terminal color and formatting validation

**Tests**:
- ANSI color code verification
- Message formatting consistency
- Cross-platform color support
- Terminal compatibility testing

## 🛠️ Usage Guidelines

### Making Scripts Executable
```bash
# Make all scripts executable
chmod +x shell/*.sh

# Make specific script executable
chmod +x shell/quick_test.sh
```

### Basic Testing Workflow
```bash
# 1. Quick functionality test
./shell/quick_test.sh

# 2. Automated regression test
./shell/automated_test.sh

# 3. Comprehensive feature test
./shell/comprehensive_test.sh

# 4. Visual verification
./shell/test_tmux.sh
```

### Development Testing
```bash
# Test specific features
./shell/test_encryption.sh    # Security features
./shell/test_p2p.sh          # Network functionality

# Demo for stakeholders
./shell/demo.sh              # Feature overview
./shell/demo_chat.sh         # Interactive demo
```

## 🔧 Configuration Options

### Environment Variables
Most scripts support these environment variables:

```bash
# Default ports
export DEFAULT_PORT1=8080
export DEFAULT_PORT2=8081

# Timeout settings
export CONNECTION_TIMEOUT=10
export MESSAGE_TIMEOUT=5

# Logging levels
export RUST_LOG=debug
export CHAT_LOG_LEVEL=info

# Test behavior
export SKIP_CLEANUP=false
export VERBOSE_OUTPUT=true
```

### Script Parameters
Common parameters across scripts:

```bash
# Port specification
--port1 PORT, --port2 PORT

# Timeout configuration
--timeout SECONDS

# Verbose output
--verbose, -v

# Debug mode
--debug, -d

# Skip cleanup
--no-cleanup
```

## 🧪 Testing Patterns

### Unit Testing
```bash
# Test individual components
./shell/test_encryption.sh   # Crypto functions
./shell/test_p2p.sh         # Network layer
```

### Integration Testing
```bash
# Test complete workflows
./shell/automated_test.sh    # End-to-end scenarios
./shell/comprehensive_test.sh # Full feature testing
```

### Performance Testing
```bash
# Stress testing
./shell/test_p2p.sh --stress
./shell/comprehensive_test.sh --performance
```

### Visual Testing
```bash
# Interactive verification
./shell/test_tmux.sh         # Split-screen testing
./shell/demo_chat.sh         # Guided demonstration
```

## 🐛 Troubleshooting

### Common Issues

1. **Permission Denied**:
   ```bash
   chmod +x shell/*.sh
   ```

2. **Port Already in Use**:
   ```bash
   # Scripts automatically handle cleanup
   pkill -f rust-p2p-chat
   ./shell/quick_test.sh 9000 9001
   ```

3. **tmux Not Available**:
   ```bash
   # Install tmux
   sudo apt install tmux      # Ubuntu/Debian
   brew install tmux          # macOS
   ```

4. **Script Hangs**:
   ```bash
   # Use Ctrl+C to trigger cleanup
   # Or kill processes manually
   pkill -f rust-p2p-chat
   ```

### Debug Mode
Enable debug output in any script:
```bash
RUST_LOG=debug ./shell/quick_test.sh
```

### Log Analysis
Scripts generate logs in the current directory:
```bash
# Check logs after test runs
cat peer1.log peer2.log

# Monitor logs in real-time
tail -f peer1.log peer2.log
```

## 🔍 Script Internals

### Error Handling Pattern
All scripts follow this error handling pattern:
```bash
set -euo pipefail    # Strict error handling

error_exit() {
    echo "Error: $1" >&2
    cleanup
    exit 1
}

cleanup() {
    # Kill processes
    # Remove temporary files
    # Reset terminal state
}

trap cleanup INT TERM EXIT
```

### Color Output
Consistent color scheme across scripts:
```bash
RED='\033[0;31m'      # Errors
GREEN='\033[0;32m'    # Success
YELLOW='\033[1;33m'   # Warnings
BLUE='\033[0;34m'     # Info
NC='\033[0m'          # No Color
```

### Process Management
Scripts use proper process management:
```bash
# Background processes with PID tracking
command &
PROCESS_PID=$!

# Cleanup with timeout
timeout 5 kill $PROCESS_PID || kill -9 $PROCESS_PID
```

## 📈 Continuous Integration

These scripts are designed for CI/CD integration:

### GitHub Actions
```yaml
- name: Run P2P Chat Tests
  run: |
    ./shell/automated_test.sh
    ./shell/comprehensive_test.sh
```

### Local CI
```bash
# Run all tests
for script in shell/test_*.sh; do
    echo "Running $script"
    $script || exit 1
done
```

## 🚀 Future Enhancements

### Planned Script Additions
- **`benchmark.sh`** - Performance benchmarking
- **`security_audit.sh`** - Security vulnerability testing
- **`network_simulation.sh`** - Network condition simulation
- **`cross_platform_test.sh`** - Multi-platform testing

### Automation Improvements
- **Docker integration** - Container-based testing
- **Test result reporting** - JSON/XML output formats
- **Parallel execution** - Concurrent test running
- **Test data generation** - Synthetic test scenarios

## 📚 Best Practices

### Script Development
1. **Use `set -euo pipefail`** for strict error handling
2. **Implement proper cleanup** functions
3. **Use color coding** for better user experience
4. **Add parameter validation** and help text
5. **Document all scripts** with clear comments

### Testing Strategy
1. **Start with quick tests** during development
2. **Run comprehensive tests** before commits
3. **Use automated tests** for regression testing
4. **Employ visual tests** for UI verification
5. **Include performance tests** for optimization

### Maintenance
1. **Keep scripts up-to-date** with application changes
2. **Test scripts on multiple platforms**
3. **Update documentation** when adding features
4. **Monitor script performance** and optimize
5. **Collect user feedback** for improvements

---

These scripts provide a robust testing and demonstration framework for the Rust P2P Chat application, enabling rapid development, reliable testing, and effective demonstration of features.