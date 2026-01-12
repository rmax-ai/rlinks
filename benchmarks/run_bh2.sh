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
python3 - <<'PY'
import json,sys
from crates.rlinks_bh2.src.lib import aggregate_hits
# Note: this is a placeholder; real BH2 will consume append log format
with open('$RESULTS_DIR/bh2-append.json') as f:
    d = json.load(f)
# Extract a simple list of codes (this harness uses /ok-code endpoint; simulate codes)
hits = [("ok-code", i) for i in range(len(d.get('latencies',[])))]

agg = {}
for code,_ in hits:
    agg[code] = agg.get(code,0)+1

with open('$RESULTS_DIR/bh2-agg.json','w') as out:
    json.dump({'agg':agg,'count':len(hits)}, out)
print('Saved $RESULTS_DIR/bh2-agg.json')
PY

# stop server
kill $pid || true
wait $pid 2>/dev/null || true

echo "Done. Results in $RESULTS_DIR"