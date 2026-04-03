"""Test that the example notebook executes without errors."""

import nbformat
from nbclient import NotebookClient
from pathlib import Path
import pytest


NOTEBOOK_PATH = Path(__file__).parent.parent / "examples" / "usage.ipynb"


@pytest.fixture()
def notebook():
    with open(NOTEBOOK_PATH) as f:
        return nbformat.read(f, as_version=4)


def test_usage_notebook_executes(notebook):
    """All cells in examples/usage.ipynb should execute without raising."""
    client = NotebookClient(
        notebook,
        timeout=60,
        kernel_name="python3",
        resources={"metadata": {"path": str(NOTEBOOK_PATH.parent)}},
    )
    client.execute()

    # Verify every code cell produced no error outputs
    for i, cell in enumerate(notebook.cells):
        if cell.cell_type != "code":
            continue
        for output in cell.get("outputs", []):
            assert output.output_type != "error", (
                f"Cell {i} raised {output.get('ename', '?')}: "
                f"{output.get('evalue', '?')}"
            )
