#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")" && pwd)
RESULTS_DIR="$ROOT_DIR/results"
mkdir -p "$RESULTS_DIR"

CONCURRENCIES=(1 10 50 100 500)
REPEATS=3
REQUESTS_DEFAULT=5000
PORT=8080

# allow override of requests via env or arg
REQUESTS=${REQUESTS:-$REQUESTS_DEFAULT}

run_mode() {
  mode=$1
  echo "Starting server in mode=$mode"
  cargo run -p rlinks-worker -- --http $PORT --mode $mode --kv-latency-ms 5 &> "$RESULTS_DIR/server-$mode.log" &
  pid=$!
  echo "Server PID: $pid"

  # wait for server up
  for i in {1..10}; do
    if curl -s -o /dev/null "http://127.0.0.1:$PORT/ok-code"; then
      break
    fi
    sleep 0.5
  done

  if ! kill -0 $pid 2>/dev/null; then
    echo "Server failed to start, check logs" >&2
    cat "$RESULTS_DIR/server-$mode.log" >&2
    exit 1
  fi

  for c in "${CONCURRENCIES[@]}"; do
    for r in $(seq 1 $REPEATS); do
      out="$RESULTS_DIR/${mode}-${c}-${r}.json"
      echo "Running harness: mode=$mode concurrency=$c repeat=$r"
      cargo run -p bench-harness -- --url "http://127.0.0.1:$PORT/ok-code" --concurrency $c --requests $REQUESTS --out "$out"
      echo "Saved $out"
    done
  done

  echo "Stopping server PID $pid"
  kill $pid || true
  wait $pid 2>/dev/null || true
}

# Run append
run_mode append
# Run kv
run_mode kv

# summarize
python3 "$ROOT_DIR/analysis.py" "$RESULTS_DIR" > "$RESULTS_DIR/summary.csv"

echo "Done. Results in $RESULTS_DIR"
