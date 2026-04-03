"""Test that the example notebook executes without errors."""

import nbformat
from nbclient import NotebookClient
from jupyter_client.kernelspec import KernelSpecManager
from pathlib import Path
import pytest


NOTEBOOK_PATH = Path(__file__).parent.parent / "examples" / "usage.ipynb"


def _select_python_kernel() -> str:
    """Return a usable Python kernel name across local and CI environments."""
    kernels = KernelSpecManager().find_kernel_specs()
    preferred = ("python3", "python")
    for name in preferred:
        if name in kernels:
            return name

    # Fallback: pick the first kernel that looks like Python.
    for name in kernels:
        if "python" in name.lower():
            return name

    pytest.fail("No Python Jupyter kernel found. Install/register ipykernel before running notebook tests.")
    raise AssertionError("unreachable")


@pytest.fixture()
def notebook():
    with open(NOTEBOOK_PATH) as f:
        return nbformat.read(f, as_version=4)


def test_usage_notebook_executes(notebook):
    """All cells in examples/usage.ipynb should execute without raising."""
    kernel_name = _select_python_kernel()
    client = NotebookClient(
        notebook,
        timeout=60,
        kernel_name=kernel_name,
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
