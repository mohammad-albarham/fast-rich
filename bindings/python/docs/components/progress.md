# Progress

Display progress bars for long-running operations.

## Quick Usage

```python
from fast_rich.progress import track
import time

for item in track(range(100), description="Processing..."):
    time.sleep(0.01)
```

## Progress Class

### Basic Usage

```python
from fast_rich.progress import Progress
import time

with Progress() as progress:
    task = progress.add_task("Working...", total=100)
    
    while not progress.finished:
        progress.update(task, advance=1)
        time.sleep(0.02)
```

### Constructor

```python
Progress(
    *columns,                    # Progress columns
    console=None,                # Console instance
    auto_refresh=True,           # Auto-refresh display
    refresh_per_second=10,       # Refresh rate
    speed_estimate_period=30.0,  # Speed estimation window
    transient=False,             # Remove when done
    redirect_stdout=True,        # Capture stdout
    redirect_stderr=True,        # Capture stderr
    get_time=None,               # Time function
    disable=False,               # Disable progress
    expand=False,                # Expand to width
)
```

### Methods

#### add_task()

```python
task_id = progress.add_task(
    description="Task name",  # Task description
    total=100,                # Total steps
    completed=0,              # Initial progress
    visible=True,             # Show task
    start=True,               # Start immediately
)
```

#### update()

```python
progress.update(
    task_id,
    completed=50,      # Set completed value
    advance=1,         # Or advance by amount
    description="New description",
    visible=True,
    refresh=False,
)
```

#### remove_task()

```python
progress.remove_task(task_id)
```

#### reset()

```python
progress.reset(task_id)
```

## track() Function

Simple wrapper for iterating with progress.

```python
from fast_rich.progress import track

# Basic usage
for item in track(range(100)):
    process(item)

# With options
for item in track(
    items,
    description="Processing...",
    total=len(items),
    transient=True,
):
    process(item)
```

## Examples

### Multiple Tasks

```python
from fast_rich.progress import Progress
import time

with Progress() as progress:
    download = progress.add_task("[cyan]Downloading...", total=100)
    process = progress.add_task("[green]Processing...", total=100)
    upload = progress.add_task("[red]Uploading...", total=100)
    
    while not progress.finished:
        progress.update(download, advance=0.9)
        progress.update(process, advance=0.6)
        progress.update(upload, advance=0.3)
        time.sleep(0.02)
```

### File Download Simulation

```python
from fast_rich.progress import Progress
import time

def download_files():
    files = ["file1.zip", "file2.tar", "file3.iso"]
    
    with Progress() as progress:
        overall = progress.add_task("[cyan]Overall", total=len(files))
        
        for filename in files:
            file_task = progress.add_task(f"[green]{filename}", total=100)
            
            for _ in range(100):
                progress.update(file_task, advance=1)
                time.sleep(0.01)
            
            progress.update(overall, advance=1)

download_files()
```

### Indeterminate Progress

```python
from fast_rich.progress import Progress
import time

with Progress() as progress:
    task = progress.add_task("Searching...", total=None)
    
    for _ in range(100):
        progress.update(task)
        time.sleep(0.05)
```

### With Console Status

```python
from fast_rich.console import Console
import time

console = Console()

with console.status("[bold green]Working...") as status:
    time.sleep(1)
    status.update("[bold yellow]Still working...")
    time.sleep(1)
    status.update("[bold cyan]Almost done...")
    time.sleep(1)

console.print("[bold green]Done!")
```
