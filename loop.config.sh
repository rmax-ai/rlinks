#!/bin/bash
# Configuration for loop.sh orchestration
# Override these values by setting environment variables before running loop.sh

# Maximum number of iterations before stopping
: "${MAX_ITERATIONS:=50}"

# Timeout in seconds for process inactivity (5 minutes default)
: "${TIMEOUT:=300}"

# Directory for log output
: "${LOG_DIR:=logs}"

# Prompt file for the orchestrator
: "${PROMPT_FILE:=LOOP_PROMPT.md}"

# OpenCode binary path (auto-detected if not set)
: "${OPENCODE_BIN:=$(command -v opencode || echo "opencode")}"

# OpenCode agent type
: "${OPENCODE_AGENT:=orchestrator}"

# State file for persistence
: "${STATE_FILE:=$LOG_DIR/loop_state.json}"

# Log retention in days (for cleanup)
: "${LOG_RETENTION_DAYS:=7}"

# Minimum free disk space in GB before stopping
: "${MIN_DISK_SPACE_GB:=1}"

# CPU limit percentage (requires cpulimit tool, 0 = no limit)
: "${CPU_LIMIT_PERCENT:=0}"

# Memory limit in MB (requires systemd-run or similar, 0 = no limit)
: "${MEMORY_LIMIT_MB:=0}"

# Check interval for process activity monitoring (seconds)
: "${ACTIVITY_CHECK_INTERVAL:=10}"

# Export all configuration
export MAX_ITERATIONS TIMEOUT LOG_DIR PROMPT_FILE OPENCODE_BIN OPENCODE_AGENT
export STATE_FILE LOG_RETENTION_DAYS MIN_DISK_SPACE_GB CPU_LIMIT_PERCENT
export MEMORY_LIMIT_MB ACTIVITY_CHECK_INTERVAL
