import time
import rich_rust
from rich.console import Console as RichConsole
from rich.traceback import Traceback as RichTraceback
import sys

ITERATIONS = 1000

def bench_python_traceback():
    console = RichConsole(file=open('/dev/null', 'w'))
    try:
        1 / 0
    except Exception:
        exc_info = sys.exc_info()
        # Rich creation + printing
        for _ in range(ITERATIONS):
            # Rich creates traceback from exc_info or last exception
            tb = RichTraceback.from_exception(*exc_info)
            console.print(tb)

def bench_rust_traceback():
    console = rich_rust.Console() 
    # Current rust binding only takes a message string, verifying "visual" parity mostly
    # But to be fair, we should include "extraction" overhead if possible, 
    # or just measure the visual rendering part.
    # Rich's Traceback.from_exception does a lot of heavy lifting (parsing StackSummary).
    # rich-rust binding is lighter.
    
    error_message = "ZeroDivisionError: division by zero"
    
    for _ in range(ITERATIONS):
        # We simulate the "result" of an exception info
        tb = rich_rust.Traceback(error_message, show_locals=False)
        console.print_traceback(tb)

if __name__ == "__main__":
    start = time.time()
    bench_python_traceback()
    print(f"Python Traceback: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_traceback()
    print(f"Rust Traceback:   {time.time() - start:.4f}s")
