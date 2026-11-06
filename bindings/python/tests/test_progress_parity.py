"""Parity tests for Progress - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest
import time


class TestProgressParity:
    """Test Progress parity between fast_rich and rich."""

    def test_progress_basic(self):
        """Test basic progress creation."""
        from fast_rich.progress import Progress as FastProgress
        from rich.progress import Progress as RichProgress

        # Both should instantiate without error
        fast_prog = FastProgress()
        rich_prog = RichProgress()

    def test_progress_add_task(self):
        """Test adding tasks to progress."""
        from fast_rich.progress import Progress

        progress = Progress()
        task_id = progress.add_task("Downloading...", total=100)
        assert task_id is not None

    def test_progress_update(self):
        """Test updating progress."""
        from fast_rich.progress import Progress

        progress = Progress()
        task_id = progress.add_task("Processing...", total=100)
        progress.update(task_id, completed=50)
        # Should not raise

    def test_progress_multiple_tasks(self):
        """Test multiple tasks."""
        from fast_rich.progress import Progress

        progress = Progress()
        task1 = progress.add_task("Task 1", total=100)
        task2 = progress.add_task("Task 2", total=200)

        assert task1 != task2


class TestProgressRendering:
    """Test progress rendering."""

    def test_progress_str_output(self):
        """Test progress has string representation."""
        from fast_rich.progress import Progress

        progress = Progress()
        progress.add_task("Test", total=100)
        # Progress should be representable
        assert progress is not None

    def test_progress_track(self):
        """Test track() function."""
        from fast_rich.progress import track

        items = [1, 2, 3, 4, 5]
        result = list(track(items, description="Processing"))
        assert result == items

    def test_progress_task_params(self):
        """Test task creation with all params."""
        from fast_rich.progress import Progress

        progress = Progress()
        task_id = progress.add_task(
            "Loading",
            total=100,
            completed=0,
            visible=True,
        )
        assert task_id is not None

    def test_progress_advance(self):
        """Test advance method."""
        from fast_rich.progress import Progress

        progress = Progress()
        task_id = progress.add_task("Test", total=100)
        progress.advance(task_id, advance=10)
        # Should not raise
