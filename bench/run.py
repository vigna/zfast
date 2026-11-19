#!/usr/bin/env python3
"""
Comparative benchmarks of z-fast tries.

  python3 bench/run.py bench --method all --dataset urls --n 2000000
  python3 bench/run.py bench --method ZFastTrie,Wormhole --dataset eu-2015.urls \\
      --n 20000000 --pin "taskset -c 8"
  python3 bench/run.py report --dataset urls
  python3 bench/run.py table --dataset eu-urls --n 10000000 --kind dynamic \
      --results paper/results-epyc --out paper/fig/bench-dynamic-10000000.tex

`bench` builds the drivers, generates the workload if necessary (in
bench/data), runs each method and stores its raw output in
bench/results/<method>/<dataset>-<n>.out; existing results are kept unless
--force is given, so a campaign can be interrupted and resumed. A dataset is a
name defined in configs.py or the name of a file in bench/data with one key
per line. `report` prints a table with the time in nanoseconds per operation
of each method, the space, and checks that all methods computed the same
checksums. `table` writes a LaTeX tabular (operations in rows, methods in
columns) for the paper. Both read results from bench/results, or from the
directory given by --results (e.g., the results stored with the paper).
"""

import argparse
import os
import re
import shlex
import subprocess
import sys

from configs import DATASETS, METHODS, OPS, REPS, SIZES

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "data")
RESULTS = os.path.join(HERE, "results")
BUILD = os.path.join(HERE, "build")


def run(cmd, dry_run=False, **kwargs):
    print("+ " + (cmd if isinstance(cmd, str) else shlex.join(cmd)), file=sys.stderr)
    if dry_run:
        return None
    return subprocess.run(cmd, check=True, **kwargs)


def methods(arg):
    names = list(METHODS) if arg == "all" else arg.split(",")
    for name in names:
        if name not in METHODS:
            sys.exit(f"Unknown method {name}; known methods: {', '.join(METHODS)}")
    return names


def spec(dataset):
    """Returns the source of the keys of a dataset."""
    if dataset in DATASETS:
        return DATASETS[dataset]["spec"]
    path = os.path.join(DATA, dataset)
    if not os.path.exists(path):
        sys.exit(f"Unknown dataset {dataset}: not in configs.py, and {path} does not exist")
    return path


def workload(dataset, n, args):
    """Generates the workload files, if necessary, and returns their prefix."""
    os.makedirs(DATA, exist_ok=True)
    prefix = os.path.join(DATA, f"{dataset}-{n}")
    if args.force_gen or not all(os.path.exists(f"{prefix}.{e}") for e in ("keys", "hits", "misses")):
        run([os.path.join(BUILD, "zfast-bench"), "gen", spec(dataset), str(n), prefix], args.dry_run)
    return prefix


def result_file(method, dataset, n, results=None):
    return os.path.join(results or RESULTS, method, f"{dataset}-{n}.out")


def cmd_bench(args):
    names = methods(args.method)
    tools = {METHODS[m]["tool"] for m in names}
    targets = ["build/zfast-bench"] + (["build/wh_bench"] if "wormhole" in tools else [])
    run(["make", "-C", HERE] + targets, args.dry_run)
    pin = shlex.split(args.pin) if args.pin else []
    for dataset in args.dataset.split(","):
        for n in [int(x) for x in args.n.split(",")]:
            prefix = workload(dataset, n, args)
            for name in names:
                out = result_file(name, dataset, n)
                if os.path.exists(out) and not args.force:
                    print(f"{out} exists, skipping", file=sys.stderr)
                    continue
                method = METHODS[name]
                if method["tool"] == "rust":
                    cmd = [os.path.join(BUILD, "zfast-bench"), "run", method["arg"], prefix, str(args.reps)]
                else:
                    cmd = [os.path.join(BUILD, "wh_bench"), prefix, str(args.reps)]
                done = run(pin + cmd, args.dry_run, capture_output=True, text=True)
                if done is None:
                    continue
                os.makedirs(os.path.dirname(out), exist_ok=True)
                with open(out + ".tmp", "w") as f:
                    f.write(done.stdout)
                os.replace(out + ".tmp", out)
                sys.stderr.write(done.stdout)


def parse(path):
    """Returns the times, the checksums and the space of a result file."""
    times, checksums, space = {}, {}, {}
    with open(path) as f:
        for line in f:
            fields = dict(re.findall(r"(\w+)=(\S+)", line))
            if line.startswith("RESULT"):
                times[fields["op"]] = float(fields["ns"])
                checksums[fields["op"]] = int(fields["checksum"])
            elif line.startswith("SPACE"):
                space = {k: float(fields[k]) for k in ("bytes_per_key", "rss_per_key")}
    return times, checksums, space


def cmd_report(args):
    names = methods(args.method)
    base = args.results or RESULTS
    for dataset in args.dataset.split(","):
        sizes = sorted({int(m.group(1)) for name in names if os.path.isdir(os.path.join(base, name))
                        for f in os.listdir(os.path.join(base, name))
                        if (m := re.fullmatch(re.escape(dataset) + r"-(\d+)\.out", f))})
        for n in sizes:
            results = {name: parse(result_file(name, dataset, n, base)) for name in names
                       if os.path.exists(result_file(name, dataset, n, base))}
            if not results:
                continue
            print(f"\n# {dataset}, {n // 2} keys (time in ns per operation)")
            width = max(len(METHODS[name]["label"]) for name in results)
            print(f"{'':{width}}" + "".join(f"{op:>15}" for op in OPS) + f"{'B/key':>10}{'RSS/key':>10}")
            for name, (times, _, space) in results.items():
                row = "".join(f"{times[op]:15.1f}" if op in times else f"{'':15}" for op in OPS)
                extra = "".join(f"{space[k]:10.1f}" if space.get(k) else f"{'':10}"
                                for k in ("bytes_per_key", "rss_per_key"))
                print(f"{METHODS[name]['label']:{width}}{row}{extra}")
            # All methods must agree on the checksums (insert and build
            # return different things for different kinds of structures)
            for op in OPS:
                if op in ("build",):
                    continue
                values = {name: checksums[op] for name, (_, checksums, _) in results.items() if op in checksums}
                if len(set(values.values())) > 1:
                    print(f"WARNING: checksums of {op} differ: {values}")


# The rows of the tables of the paper: the label, the operation, and the
# number of decimal digits.
TABLE_ROWS = {
    "dynamic": [("Add", "insert", 0), ("Contains$^+$", "contains-hit", 0),
                ("Contains$^-$", "contains-miss", 0), ("Successor$^+$", "succ-hit", 0),
                ("Successor$^-$", "succ-miss", 0), ("Scan", "iterate", 1),
                ("Delete", "remove", 0)],
    "static": [("Build", "build", 0), ("Contains$^+$", "contains-hit", 0),
               ("Contains$^-$", "contains-miss", 0), ("Successor$^+$", "succ-hit", 0),
               ("Successor$^-$", "succ-miss", 0)],
}


def cmd_table(args):
    """Writes a LaTeX tabular with the results of the methods of a kind."""
    names = [name for name in methods(args.method) if METHODS[name]["kind"] == args.kind]
    n = int(args.n)
    results = {name: parse(result_file(name, args.dataset, n, args.results)) for name in names
               if os.path.exists(result_file(name, args.dataset, n, args.results))}
    if not results:
        sys.exit("No results found")
    lines = [f"% Generated by bench/run.py table --dataset {args.dataset} --n {n} --kind {args.kind}",
             "\\begin{tabular}{l" + "r" * len(results) + "}",
             "".join(f"&\\multicolumn{{1}}{{c}}{{{METHODS[name].get('tex', METHODS[name]['label'])}}}"
                     for name in results) + "\\\\",
             "\\hline"]
    for label, op, digits in TABLE_ROWS[args.kind]:
        cells = [f"{times[op]:.{digits}f}" if op in times else "" for times, _, _ in results.values()]
        if any(cells):
            lines.append(label + " & " + " & ".join(cells) + "\\\\")
    # Space: the resident set size for dynamic structures (it includes the
    # keys), the size of the index for static ones
    key = "rss_per_key" if args.kind == "dynamic" else "bytes_per_key"
    cells = [f"{space[key]:.1f}" if space.get(key) else "" for _, _, space in results.values()]
    lines.append("\\hline")
    lines.append("Bytes per key & " + " & ".join(cells) + "\\\\")
    lines.append("\\end{tabular}")
    text = "\n".join(lines) + "\n"
    if args.out:
        with open(args.out, "w") as f:
            f.write(text)
    else:
        sys.stdout.write(text)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="command", required=True)
    for name, func in (("bench", cmd_bench), ("report", cmd_report), ("table", cmd_table)):
        p = sub.add_parser(name)
        p.set_defaults(func=func)
        p.add_argument("--method", default="all", help="comma-separated methods, or all")
        p.add_argument("--dataset", default="urls", help="comma-separated datasets")
        if name != "bench":
            p.add_argument("--results", default=None, help="directory of the results (default: bench/results)")
        if name == "table":
            p.add_argument("--n", required=True, help="number of keys of the workload")
            p.add_argument("--kind", choices=("dynamic", "static"), default="dynamic")
            p.add_argument("--out", default=None, help="output file (default: standard output)")
        if name == "bench":
            p.add_argument("--n", default=",".join(map(str, SIZES)), help="comma-separated numbers of keys")
            p.add_argument("--reps", type=int, default=REPS, help="repetitions of the query phases")
            p.add_argument("--pin", default="", help='a command prefix pinning the CPU, e.g. "taskset -c 8"')
            p.add_argument("--force", action="store_true", help="rerun existing results")
            p.add_argument("--force-gen", action="store_true", help="regenerate existing workloads")
            p.add_argument("--dry-run", action="store_true", help="show the commands without running them")
    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
