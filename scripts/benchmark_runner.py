import os
import sys
import subprocess
import json
import time
import re
from datetime import datetime

BENCH_DIR = "benchmarks/python"
RESULTS_DIR = "benchmarks/results/python"
VERSION = "v0.2.0"

def get_git_revision_hash():
    return subprocess.check_output(['git', 'rev-parse', 'HEAD']).strip().decode('ascii')

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = os.path.join(RESULTS_DIR, VERSION)
    os.makedirs(output_dir, exist_ok=True)
    output_file = os.path.join(output_dir, f"{timestamp}.json")
    
    results = {
        "metadata": {
            "version": VERSION,
            "timestamp": timestamp,
            "commit": get_git_revision_hash(),
            "platform": sys.platform,
        },
        "benchmarks": {}
    }
    
    bench_files = [f for f in os.listdir(BENCH_DIR) if f.startswith("bench_") and f.endswith(".py")]
    
    print(f"Running {len(bench_files)} benchmarks...")
    
    for bench_file in bench_files:
        name = bench_file.replace("bench_", "").replace(".py", "")
        path = os.path.join(BENCH_DIR, bench_file)
        
        print(f"Benchmarking {name}...")
        try:
            # Run in venv
            cmd = f"source benchmarks/venv/bin/activate && python {path}"
            output = subprocess.check_output(cmd, shell=True, executable="/bin/zsh").decode("utf-8")
            
            # Parse output: expecting "Python X: 1.23s\nRust X: 0.12s"
            # Flexible parsing
            lines = output.strip().split('\n')
            bench_data = {}
            for line in lines:
                match = re.search(r"(Python|Rust)\s+.*:\s+([\d\.]+)", line)
                if match:
                    lang = match.group(1)
                    duration = float(match.group(2))
                    bench_data[lang] = duration
            
            if bench_data:
                results["benchmarks"][name] = bench_data
                
        except Exception as e:
            print(f"Error running {name}: {e}")
            
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
        
    print(f"Results saved to {output_file}")
    
if __name__ == "__main__":
    main()
