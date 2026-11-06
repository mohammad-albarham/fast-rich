"""Progress tracking - matches rich.progress API."""

from __future__ import annotations

import time
from typing import (
    Any,
    Callable,
    Iterable,
    Iterator,
    Optional,
    Sequence,
    Union,
)

from fast_rich.console import Console
from fast_rich.style import Style


class TaskID(int):
    """A task ID."""
    pass


class Progress:
    """A progress display for tracking tasks.
    
    Matches rich.progress.Progress API.
    """

    def __init__(
        self,
        *columns: Any,
        console: Optional[Console] = None,
        auto_refresh: bool = True,
        refresh_per_second: float = 10,
        speed_estimate_period: float = 30.0,
        transient: bool = False,
        redirect_stdout: bool = True,
        redirect_stderr: bool = True,
        get_time: Optional[Callable[[], float]] = None,
        disable: bool = False,
        expand: bool = False,
    ) -> None:
        """Create Progress.
        
        Args:
            *columns: Progress columns.
            console: Console to use.
            auto_refresh: Auto refresh display.
            refresh_per_second: Refresh rate.
            speed_estimate_period: Speed estimation period.
            transient: Remove on completion.
            redirect_stdout: Redirect stdout.
            redirect_stderr: Redirect stderr.
            get_time: Custom time function.
            disable: Disable progress.
            expand: Expand to full width.
        """
        self.console = console or Console()
        self.auto_refresh = auto_refresh
        self.refresh_per_second = refresh_per_second
        self.speed_estimate_period = speed_estimate_period
        self.transient = transient
        self.redirect_stdout = redirect_stdout
        self.redirect_stderr = redirect_stderr
        self.get_time = get_time or time.time
        self.disable = disable
        self.expand = expand
        
        self._tasks: dict[TaskID, dict] = {}
        self._task_id_counter = 0
        self._started = False

    def __enter__(self) -> "Progress":
        """Enter context."""
        self.start()
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Exit context."""
        self.stop()

    def start(self) -> None:
        """Start the progress display."""
        self._started = True

    def stop(self) -> None:
        """Stop the progress display."""
        self._started = False
        if not self.disable:
            self.console.print()

    def add_task(
        self,
        description: str,
        start: bool = True,
        total: Optional[float] = 100.0,
        completed: float = 0,
        visible: bool = True,
        **fields: Any,
    ) -> TaskID:
        """Add a task.
        
        Args:
            description: Task description.
            start: Start immediately.
            total: Total steps.
            completed: Initial completed steps.
            visible: Show task.
            **fields: Additional fields.
            
        Returns:
            Task ID.
        """
        task_id = TaskID(self._task_id_counter)
        self._task_id_counter += 1
        
        self._tasks[task_id] = {
            "description": description,
            "total": total,
            "completed": completed,
            "visible": visible,
            "started": start,
            "start_time": self.get_time() if start else None,
            "fields": fields,
        }
        
        return task_id

    def update(
        self,
        task_id: TaskID,
        *,
        total: Optional[float] = None,
        completed: Optional[float] = None,
        advance: Optional[float] = None,
        description: Optional[str] = None,
        visible: Optional[bool] = None,
        refresh: bool = False,
        **fields: Any,
    ) -> None:
        """Update a task.
        
        Args:
            task_id: Task ID.
            total: New total.
            completed: New completed value.
            advance: Amount to advance.
            description: New description.
            visible: Visibility.
            refresh: Force refresh.
            **fields: Additional fields.
        """
        task = self._tasks.get(task_id)
        if task is None:
            return
            
        if total is not None:
            task["total"] = total
        if completed is not None:
            task["completed"] = completed
        if advance is not None:
            task["completed"] = task["completed"] + advance
        if description is not None:
            task["description"] = description
        if visible is not None:
            task["visible"] = visible
        task["fields"].update(fields)
        
        if not self.disable and refresh:
            self.refresh()

    def advance(self, task_id: TaskID, advance: float = 1) -> None:
        """Advance a task.
        
        Args:
            task_id: Task ID.
            advance: Amount to advance.
        """
        self.update(task_id, advance=advance)

    def reset(
        self,
        task_id: TaskID,
        *,
        start: bool = True,
        total: Optional[float] = None,
        completed: float = 0,
        visible: Optional[bool] = None,
        description: Optional[str] = None,
        **fields: Any,
    ) -> None:
        """Reset a task."""
        self.update(
            task_id,
            total=total,
            completed=completed,
            visible=visible,
            description=description,
            **fields,
        )

    def refresh(self) -> None:
        """Refresh the display."""
        if self.disable:
            return
            
        for task_id, task in self._tasks.items():
            if not task["visible"]:
                continue
                
            total = task["total"] or 100
            completed = task["completed"]
            percentage = (completed / total * 100) if total else 0
            
            bar_width = 40
            filled = int(bar_width * completed / total) if total else 0
            bar = "█" * filled + "░" * (bar_width - filled)
            
            self.console.print(
                f"\r{task['description']} [{bar}] {percentage:.0f}%",
                end="",
            )

    def track(
        self,
        sequence: Iterable[Any],
        total: Optional[float] = None,
        task_id: Optional[TaskID] = None,
        description: str = "Working...",
        update_period: float = 0.1,
        remove: bool = False,
    ) -> Iterator[Any]:
        """Track progress through an iterable."""
        if total is None:
            try:
                total = float(len(sequence))  # type: ignore
            except TypeError:
                total = None
                
        if task_id is None:
            task_id = self.add_task(description, total=total)
        
        for item in sequence:
            yield item
            self.advance(task_id)
            if not self.disable:
                self.refresh()


def track(
    sequence: Iterable[Any],
    description: str = "Working...",
    total: Optional[float] = None,
    auto_refresh: bool = True,
    console: Optional[Console] = None,
    transient: bool = False,
    get_time: Optional[Callable[[], float]] = None,
    refresh_per_second: float = 10,
    style: Optional[Union[str, Style]] = "bar.back",
    complete_style: Optional[Union[str, Style]] = "bar.complete",
    finished_style: Optional[Union[str, Style]] = "bar.finished",
    pulse_style: Optional[Union[str, Style]] = "bar.pulse",
    update_period: float = 0.1,
    disable: bool = False,
    show_speed: bool = True,
) -> Iterator[Any]:
    """Track progress through an iterable.
    
    This is a drop-in replacement for rich.track().
    """
    progress = Progress(
        console=console,
        auto_refresh=auto_refresh,
        refresh_per_second=refresh_per_second,
        transient=transient,
        get_time=get_time,
        disable=disable,
    )
    
    with progress:
        yield from progress.track(
            sequence,
            total=total,
            description=description,
            update_period=update_period,
        )


__all__ = ["Progress", "TaskID", "track"]
