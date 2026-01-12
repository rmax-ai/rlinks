#!/usr/bin/env python3
import sys
import json
import os
import glob
import csv
from collections import defaultdict

if len(sys.argv) < 2:
    print("Usage: analysis.py <results_dir>")
    sys.exit(2)

results_dir = sys.argv[1]
json_files = glob.glob(os.path.join(results_dir, "*.json"))

rows = []
for p in json_files:
    fname = os.path.basename(p)
    # expect pattern mode-concurrency-repeat.json
    parts = fname.split(".")[0].split("-")
    if len(parts) < 3:
        continue
    mode, concurrency, repeat = parts[0], parts[1], parts[2]
    # skip aggregated or helper files like bh2-agg-... where concurrency isn't numeric
    try:
        concurrency_i = int(concurrency)
        repeat_i = int(repeat)
    except ValueError:
        continue
    with open(p) as f:
        d = json.load(f)
    rows.append({
        'mode': mode,
        'concurrency': concurrency_i,
        'repeat': repeat_i,
        'p50_ms': d.get('p50_ms'),
        'p95_ms': d.get('p95_ms'),
        'p99_ms': d.get('p99_ms'),
        'rps': d.get('rps'),
        'errors': d.get('errors'),
        'count': d.get('count'),
    })

# print detailed CSV to stdout
writer = csv.writer(sys.stdout)
writer.writerow(['mode','concurrency','repeat','p50_ms','p95_ms','p99_ms','rps','errors','count'])
for r in sorted(rows, key=lambda x: (x['mode'], x['concurrency'], x['repeat'])):
    writer.writerow([r['mode'], r['concurrency'], r['repeat'], r['p50_ms'], r['p95_ms'], r['p99_ms'], r['rps'], r['errors'], r['count']])

# also write an aggregated summary for convenience
summary = defaultdict(list)
for r in rows:
    summary[(r['mode'], r['concurrency'])].append(r)

summary_rows = []
for (mode, c), group in sorted(summary.items()):
    avg_p50 = sum(g['p50_ms'] for g in group)/len(group)
    avg_p95 = sum(g['p95_ms'] for g in group)/len(group)
    avg_p99 = sum(g['p99_ms'] for g in group)/len(group)
    avg_rps = sum(g['rps'] for g in group)/len(group)
    total_errors = sum(g['errors'] for g in group)
    summary_rows.append({'mode': mode, 'concurrency': c, 'p50_ms': avg_p50, 'p95_ms': avg_p95, 'p99_ms': avg_p99, 'rps': avg_rps, 'errors': total_errors})

# write aggregated CSV next to results_dir
agg_path = os.path.join(results_dir, 'summary-agg.csv')
with open(agg_path, 'w') as f:
    w = csv.writer(f)
    w.writerow(['mode','concurrency','p50_ms','p95_ms','p99_ms','rps','errors'])
    for r in summary_rows:
        w.writerow([r['mode'], r['concurrency'], r['p50_ms'], r['p95_ms'], r['p99_ms'], r['rps'], r['errors']])

# try to plot if matplotlib is available
try:
    import matplotlib.pyplot as plt
    modes = sorted(set(r['mode'] for r in summary_rows))
    for mode in modes:
        xs = [r['concurrency'] for r in summary_rows if r['mode'] == mode]
        ys = [r['p95_ms'] for r in summary_rows if r['mode'] == mode]
        plt.plot(xs, ys, marker='o', label=mode)
    plt.xlabel('concurrency')
    plt.ylabel('p95_ms')
    plt.legend()
    plt.xscale('log')
    plt.savefig(os.path.join(results_dir, 'p95_vs_concurrency.png'))
except Exception:
    pass
