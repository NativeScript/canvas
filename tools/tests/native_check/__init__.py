"""The parts of tools/tests/check-native-inputs.py (jobs, paths, wiring) and the key module they hold
the keys of, as `ni`."""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location(
    "native_inputs", os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "scripts", "native-inputs.py"))
ni = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ni)
