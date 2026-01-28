#!/bin/bash
set -euo pipefail

# Orchestration loop for autonomous agent execution
# Implements iteration control, state persistence, process monitoring,
# and structured logging for continuous development workflows.

# Load configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${SCRIPT_DIR}/loop.config.sh"

if [[ -f "$CONFIG_FILE" ]]; then
  # shellcheck source=loop.config.sh
  source "$CONFIG_FILE"
else
  echo "Error: Configuration file not found: $CONFIG_FILE"
  exit 1
fi

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <goal>"
  echo ""
  echo "Example: $0 'Complete integration tests'"
  echo ""
  echo "Configuration can be overridden via environment variables."
  echo "See loop.config.sh for available options."
  exit 1
fi

CURRENT_GOAL="$*"

# Build opencode command with commit reminder
OPENCODE_GOAL="Follow execution protocol in $PROMPT_FILE to achieve: $CURRENT_GOAL. IMPORTANT: You MUST commit all changes at the end of this iteration using git."
OPENCODE_COMMAND="run --print-logs --agent $OPENCODE_AGENT \"$OPENCODE_GOAL\" -f $PROMPT_FILE"

# Colors for TUI-lite experience
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

mkdir -p "$LOG_DIR"

# Resource management: Check disk space
check_disk_space() {
  if command -v df &>/dev/null; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
      # macOS uses -g for gigabytes
      AVAILABLE_GB=$(df -g "$LOG_DIR" | awk 'NR==2 {print int($4)}')
    else
      # Linux uses -BG
      AVAILABLE_GB=$(df -BG "$LOG_DIR" | awk 'NR==2 {gsub(/G/,"",$4); print $4}')
    fi
    if ((AVAILABLE_GB < MIN_DISK_SPACE_GB)); then
      echo -e "${RED}❌ Error: Insufficient disk space (${AVAILABLE_GB}GB < ${MIN_DISK_SPACE_GB}GB required).${NC}"
      exit 1
    fi
  fi
}

# Resource management: Clean old logs
cleanup_old_logs() {
  if [[ "$LOG_RETENTION_DAYS" -gt 0 ]] && command -v find &>/dev/null; then
    echo -e "${BLUE}🧹 Cleaning logs older than ${LOG_RETENTION_DAYS} days...${NC}"
    find "$LOG_DIR" -name "iteration_*.log" -mtime +"$LOG_RETENTION_DAYS" -delete 2>/dev/null || true
  fi
}

# Load or initialize state
load_state() {
  if [[ -f "$STATE_FILE" ]]; then
    LAST_ITERATION=$(jq -r '.last_iteration // 0' "$STATE_FILE" 2>/dev/null || echo 0)
    LAST_TASK=$(jq -r '.last_task // "none"' "$STATE_FILE" 2>/dev/null || echo "none")
    echo -e "${BLUE}📂 Resuming from iteration $((LAST_ITERATION + 1)) (last task: $LAST_TASK)${NC}"
  else
    LAST_ITERATION=0
    LAST_TASK="none"
    echo -e "${BLUE}🆕 Starting fresh (no previous state)${NC}"
  fi
}

# Save state after each iteration
save_state() {
  local iteration=$1
  local task=$2
  mkdir -p "$(dirname "$STATE_FILE")"
  jq -n \
    --arg iter "$iteration" \
    --arg task "$task" \
    --arg timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    '{last_iteration: ($iter | tonumber), last_task: $task, updated_at: $timestamp}' \
    >"$STATE_FILE"
}

# Detect uncommitted changes
check_git_status() {
  if git rev-parse --git-dir >/dev/null 2>&1; then
    if ! git diff --quiet || ! git diff --cached --quiet || [ -n "$(git ls-files --others --exclude-standard)" ]; then
      return 1 # Has changes
    fi
  fi
  return 0 # Clean or not a git repo
}

# Process activity monitoring using ps
monitor_process_activity() {
  local pid=$1
  local timeout=$2
  local last_activity=$(date +%s)

  while kill -0 "$pid" 2>/dev/null; do
    sleep "$ACTIVITY_CHECK_INTERVAL"

    # Check if process is active (using CPU)
    if [[ "$OSTYPE" == "darwin"* ]]; then
      CPU_USAGE=$(ps -p "$pid" -o %cpu= 2>/dev/null | awk '{print int($1)}' || echo 0)
    else
      CPU_USAGE=$(ps -p "$pid" -o %cpu= 2>/dev/null | awk '{print int($1)}' || echo 0)
    fi

    NOW=$(date +%s)

    # If process is using CPU or logs are being written, update activity time
    if ((CPU_USAGE > 0)); then
      last_activity=$NOW
    fi

    # Check timeout
    if ((NOW - last_activity > timeout)); then
      echo -e "\n${RED}❌ Timeout: No process activity for ${timeout}s. Killing process...${NC}"
      kill -9 "$pid" 2>/dev/null || true
      return 1
    fi
  done
  return 0
}

# Pre-flight checks
check_disk_space
cleanup_old_logs
if ! command -v opencode &>/dev/null; then
  echo -e "${RED}❌ Error: 'opencode' command not found.${NC}"
  exit 1
fi

if [[ ! -f "$PROMPT_FILE" ]]; then
  echo -e "${RED}❌ Error: Prompt file '$PROMPT_FILE' not found.${NC}"
  exit 1
fi

echo -e "${BLUE}🔄 Starting orchestration loop (max iterations: $MAX_ITERATIONS)${NC}"
echo -e "${BLUE}🎯 Goal: $CURRENT_GOAL${NC}"
echo -e "${BLUE}📝 Prompt: $PROMPT_FILE${NC}"
echo -e "${BLUE}📂 Logs: $LOG_DIR/${NC}"

# Load state and determine starting iteration
load_state
START_ITERATION=$((LAST_ITERATION + 1))

# Cleanup on interrupt
trap 'cleanup_on_interrupt' SIGINT

cleanup_on_interrupt() {
  echo -e "\n${YELLOW}🛑 Loop interrupted by user. Cleaning up...${NC}"
  if [ -n "$CURRENT_PID" ] && kill -0 "$CURRENT_PID" 2>/dev/null; then
    echo -e "${RED}Killing opencode process (PID: $CURRENT_PID)...${NC}"
    kill -9 "$CURRENT_PID" 2>/dev/null || true
  fi
  END_TIME=$(date +%s)
  TOTAL_DURATION=$((END_TIME - START_TIME))
  output_stats
  exit 1
}

output_stats() {
  if ((COMPLETED_ITERS > 0)); then
    AVG_DURATION=$((SUM_DURATION / COMPLETED_ITERS))
    echo -e "${BLUE}📊 Stats:${NC}"
    echo -e "   - Iterations: $COMPLETED_ITERS"
    echo -e "   - Total time: ${TOTAL_DURATION}s"
    echo -e "   - Avg time:   ${AVG_DURATION}s"
    echo -e "   - Min time:   ${MIN_DURATION}s"
    echo -e "   - Max time:   ${MAX_DURATION}s"
  fi
}

START_TIME=$(date +%s)
SUM_DURATION=0
MIN_DURATION=999999
MAX_DURATION=0
COMPLETED_ITERS=0
CURRENT_PID=""
TASK_REPEAT_DETECTOR=()

for ((i = START_ITERATION; i <= MAX_ITERATIONS; i++)); do
  ITER_START=$(date +%s)
  TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
  LOG_FILE="$LOG_DIR/iteration_${i}_${TIMESTAMP}.log"

  echo -e "\n${YELLOW}--- Iteration $i / $MAX_ITERATIONS ---${NC}" | tee -a "$LOG_FILE"

  # Check for uncommitted changes from previous iteration
  if ! check_git_status; then
    echo -e "${YELLOW}⚠️  Warning: Uncommitted changes detected from previous iteration${NC}" | tee -a "$LOG_FILE"
  fi

  # Run opencode with stdbuf for line buffering
  (
    stdbuf -oL -eL "$OPENCODE_BIN" $OPENCODE_COMMAND 2>&1
  ) | tee -a "$LOG_FILE" | grep -v INFO &

  PID=$!
  CURRENT_PID=$PID

  # Monitor process activity instead of log file modification
  set +e
  monitor_process_activity "$PID" "$TIMEOUT"
  MONITOR_EXIT=$?
  set -e

  if [ $MONITOR_EXIT -ne 0 ]; then
    echo -e "${RED}❌ Error: Iteration timed out (no activity for ${TIMEOUT}s).${NC}" | tee -a "$LOG_FILE"
    save_state "$i" "TIMEOUT"
    exit 1
  fi

  set +e
  wait $PID
  EXIT_CODE=$?
  set -e
  CURRENT_PID=""

  if [ $EXIT_CODE -eq 0 ]; then
    # Extract task identifier for loop detection
    CURRENT_TASK=$(grep -o "NEXT_STEPS\.md.*" "$LOG_FILE" | head -1 || echo "unknown")

    # Check for completion signals
    if grep -q "<promise>DONE</promise>" "$LOG_FILE"; then
      echo -e "${GREEN}✅ Success: ALL TASKS DONE signal received.${NC}" | tee -a "$LOG_FILE"
      save_state "$i" "DONE"
      break
    elif grep -q "<promise>BLOCKED</promise>" "$LOG_FILE"; then
      BLOCKER_MSG=$(grep -A 3 "<promise>BLOCKED</promise>" "$LOG_FILE" | tail -2 || echo "No details provided")
      echo -e "${RED}🚫 BLOCKED: Human intervention required.${NC}" | tee -a "$LOG_FILE"
      echo -e "${YELLOW}Blocker: $BLOCKER_MSG${NC}" | tee -a "$LOG_FILE"
      save_state "$i" "BLOCKED"
      exit 2
    elif grep -q "<promise>NEXT_TASK</promise>" "$LOG_FILE"; then
      echo -e "${BLUE}⏭️  Task complete. NEXT_TASK signal received. Continuing...${NC}" | tee -a "$LOG_FILE"

      # Detect infinite loops (same task repeated 3+ times)
      TASK_REPEAT_DETECTOR+=("$CURRENT_TASK")
      if [ ${#TASK_REPEAT_DETECTOR[@]} -ge 3 ]; then
        RECENT_TASKS="${TASK_REPEAT_DETECTOR[@]: -3}"
        if [[ "$RECENT_TASKS" == *"$CURRENT_TASK"*"$CURRENT_TASK"*"$CURRENT_TASK"* ]]; then
          echo -e "${RED}❌ Error: Infinite loop detected (same task repeated 3+ times).${NC}" | tee -a "$LOG_FILE"
          echo -e "${YELLOW}Task: $CURRENT_TASK${NC}" | tee -a "$LOG_FILE"
          save_state "$i" "INFINITE_LOOP"
          exit 3
        fi
      fi
    else
      echo -e "${YELLOW}⚠️  Iteration finished without explicit signal. Continuing...${NC}" | tee -a "$LOG_FILE"
    fi

    # Verify files were updated
    if check_git_status; then
      echo -e "${YELLOW}⚠️  Warning: No changes detected in working tree${NC}" | tee -a "$LOG_FILE"
    fi

    # Verify commit happened
    if ! check_git_status; then
      echo -e "${MAGENTA}⚠️  Uncommitted changes remain - agent should commit before next iteration${NC}" | tee -a "$LOG_FILE"
    fi
  else
    echo -e "${RED}❌ Error: opencode failed with exit code $EXIT_CODE. Check $LOG_FILE${NC}" | tee -a "$LOG_FILE"
    save_state "$i" "ERROR_$EXIT_CODE"
    exit 1
  fi

  # Safety check: if the log file is just the header we added
  if [[ $(wc -l <"$LOG_FILE") -le 1 ]]; then
    echo -e "${RED}❌ Error: Empty log file (or only header). opencode may have crashed.${NC}" | tee -a "$LOG_FILE"
    save_state "$i" "CRASH"
    exit 1
  fi

  ITER_END=$(date +%s)
  DURATION=$((ITER_END - ITER_START))

  SUM_DURATION=$((SUM_DURATION + DURATION))
  ((COMPLETED_ITERS++))
  ((DURATION < MIN_DURATION)) && MIN_DURATION=$DURATION
  ((DURATION > MAX_DURATION)) && MAX_DURATION=$DURATION

  save_state "$i" "$CURRENT_TASK"
  echo -e "${GREEN}Iteration $i complete (${DURATION}s). Continuing...${NC}"
done

END_TIME=$(date +%s)
TOTAL_DURATION=$((END_TIME - START_TIME))

echo -e "\n${BLUE}🏁 Loop finished in ${TOTAL_DURATION}s.${NC}"

output_stats

if ((i > MAX_ITERATIONS)); then
  echo -e "${YELLOW}🛑 Reached maximum iterations ($MAX_ITERATIONS).${NC}" | tee -a "$LOG_DIR/final_status.log"
else
  echo -e "${GREEN}✅ Project signaled completion or was stopped manually.${NC}"
fi
