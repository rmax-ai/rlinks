#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")" && pwd)
RESULTS_DIR="$ROOT_DIR/results"
mkdir -p "$RESULTS_DIR"

# Simple smoke: run append mode to generate hits and then run a local BH2 aggregator simulator
REQUESTS=${REQUESTS:-1000}
PORT=8080

# start server in append mode
cargo run -p rlinks-worker -- --http $PORT --mode append --kv-latency-ms 5 &> "$RESULTS_DIR/server-append.log" &
pid=$!

echo "Server PID: $pid"

# wait for server
for i in {1..10}; do
  if curl -s -o /dev/null "http://127.0.0.1:$PORT/ok-code"; then
    break
  fi
  sleep 0.5
done

# run harness to generate hits
cargo run -p bench-harness -- --url "http://127.0.0.1:$PORT/ok-code" --concurrency 10 --requests $REQUESTS --out "$RESULTS_DIR/bh2-append.json"

# run local aggregator prototype (simulate consumption and aggregation)
python3 - <<PY
import json
# Fallback aggregator in pure Python (no Rust import) — suitable for smoke tests
with open('$RESULTS_DIR/bh2-append.json') as f:
    d = json.load(f)
# The bench-harness output contains an array of latencies; we use its length as hit count
count = len(d.get('latencies', []))
# For this smoke test the endpoint is single code 'ok-code'
agg = {'ok-code': count}
with open('$RESULTS_DIR/bh2-agg.json','w') as out:
    json.dump({'agg': agg, 'count': count}, out)
print('Saved $RESULTS_DIR/bh2-agg.json')
PY

# stop server
kill $pid || true
wait $pid 2>/dev/null || true

echo "Done. Results in $RESULTS_DIR"