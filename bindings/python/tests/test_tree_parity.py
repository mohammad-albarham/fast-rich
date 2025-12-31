"""Parity tests for Tree - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest


def normalize_output(s: str) -> str:
    """Normalize output for comparison."""
    import re
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])')
    s = ansi_escape.sub('', s)
    return s.strip()


class TestTreeParity:
    """Test Tree parity between fast_rich and rich."""

    def test_tree_basic(self):
        """Test basic tree creation."""
        from fast_rich.tree import Tree as FastTree
        from rich.tree import Tree as RichTree

        # Both should instantiate without error
        fast_tree = FastTree("Root")
        rich_tree = RichTree("Root")

        # Both should allow adding children
        fast_tree.add("Child 1")
        fast_tree.add("Child 2")

        rich_tree.add("Child 1")
        rich_tree.add("Child 2")

    def test_tree_nested(self):
        """Test nested tree structure."""
        from fast_rich.tree import Tree

        tree = Tree("Root")
        child = tree.add("Child")
        child.add("Grandchild 1")
        child.add("Grandchild 2")

        output = str(tree)
        assert "Root" in output
        assert "Child" in output

    def test_tree_with_labels(self):
        """Test tree with styled labels."""
        from fast_rich.tree import Tree

        tree = Tree("[bold]Root[/]")
        tree.add("[red]Child[/]")

        output = str(tree)
        assert "Root" in output


class TestTreeRendering:
    """Test tree rendering output."""

    def test_tree_str_output(self):
        """Test tree renders to string."""
        from fast_rich.tree import Tree

        tree = Tree("Root")
        tree.add("Leaf")
        output = str(tree)

        assert "Root" in output
        assert "Leaf" in output

    def test_tree_guide_style(self):
        """Test tree with different guide style."""
        from fast_rich.tree import Tree

        tree = Tree("Root", guide_style="bold bright_blue")
        tree.add("Child")
        output = str(tree)
        assert "Root" in output

    def test_tree_hide_root(self):
        """Test tree with hidden root."""
        from fast_rich.tree import Tree

        tree = Tree("Hidden Root", hide_root=True)
        tree.add("Visible Child")
        output = str(tree)
        # Root might be hidden, child should be visible
        assert "Child" in output

    def test_tree_deep_nesting(self):
        """Test deeply nested tree."""
        from fast_rich.tree import Tree

        tree = Tree("Level 0")
        current = tree
        for i in range(1, 5):
            current = current.add(f"Level {i}")

        output = str(tree)
        assert "Level 0" in output
        assert "Level 4" in output
