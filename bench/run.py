#!/usr/bin/env python3
"""UI benchmark suite. Runs the benchmark RUNS times in each mode and saves
the full results to bench/<date>-<commit>.json for history tracking."""

import json
import os
import platform
import subprocess
import tempfile
from datetime import date
from pathlib import Path
from statistics import median

RUNS = int(os.environ.get("BENCH_RUNS", "10"))

MODES = [
    ("debug", [], {}),
    ("release", ["--release"], {}),
    ("headless", ["--release"], {"TE_HEADLESS": "1"}),
]

ROOT = Path(__file__).resolve().parent.parent


def main():
    commit = subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"],
        capture_output=True,
        text=True,
        check=True,
        cwd=ROOT,
    ).stdout.strip()

    print("building debug and release")
    subprocess.run(["cargo", "build", "-p", "test-game"], check=True, cwd=ROOT)
    subprocess.run(["cargo", "build", "-p", "test-game", "--release"], check=True, cwd=ROOT)

    fd, result_path = tempfile.mkstemp(suffix=".json")
    os.close(fd)
    result_path = Path(result_path)

    modes = {}

    for name, args, extra_env in MODES:
        runs = []

        for i in range(RUNS):
            env = os.environ.copy()
            env.update(extra_env)
            env["UI_BENCHMARK"] = "1"
            env["UI_BENCHMARK_JSON"] = str(result_path)

            subprocess.run(
                ["cargo", "run", "-p", "test-game", *args],
                check=True,
                cwd=ROOT,
                env=env,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )

            run = json.loads(result_path.read_text())
            runs.append(run)
            print(f"{name} {i + 1}/{RUNS}: {run['panels']} panels, {run['views']} views, {run['ms']:.2f} ms")

        modes[name] = {
            "median_views": int(median(r["views"] for r in runs)),
            "runs": runs,
        }

    result_path.unlink()

    suite = {
        "date": date.today().isoformat(),
        "commit": commit,
        "host": f"{platform.system()} {platform.machine()}",
        "runs_per_mode": RUNS,
        "modes": modes,
    }

    out = ROOT / "bench" / f"{suite['date']}-{commit}.json"
    out.write_text(json.dumps(suite, indent=2) + "\n")

    print()
    for name, mode in modes.items():
        print(f"{name} median views before lag: {mode['median_views']}")
    print(f"saved to {out}")


if __name__ == "__main__":
    main()
