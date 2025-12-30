import time
import rich_rust
from rich.tree import Tree as RichTree
from rich.console import Console as RichConsole

ITERATIONS = 5000 # Tree rendering can be expensive, maybe reduce if too slow? 5k is 5k trees.
NODES = 100

def bench_python_tree():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        tree = RichTree("Root")
        for i in range(NODES):
            tree.add(f"Child {i}")
        console.print(tree)

def bench_rust_tree():
    console = rich_rust.Console()
    for _ in range(ITERATIONS):
        tree = rich_rust.Tree("Root")
        for i in range(NODES):
            tree.add_leaf(f"Child {i}")
        console.print_tree(tree) 

if __name__ == "__main__":
    start = time.time()
    bench_python_tree()
    print(f"Python Tree: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_tree()
    print(f"Rust Tree:   {time.time() - start:.4f}s")
