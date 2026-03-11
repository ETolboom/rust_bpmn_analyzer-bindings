# rust_bpmn_analyzer-bindings

Unofficial Python bindings for the [`rust_bpmn_analyzer`](https://github.com/ETolboom/rust_bpmn_analyzer) library.

This package provides a fast, Rust-powered way to analyze BPMN (Business Process Model and Notation) models directly from Python.

## Installation

```bash
pip install rust_bpmn_analyzer-bindings
```

## Features

This package exposes the following analysis functions:
* **Safeness**: Checks if any reachable state has unsafe token counts.
* **Dead Activities**: Checks if every activity that can execute will eventually be explored in successor states.
* **Option to Complete**: Checks if a deadlock state exists and can be reached.
* **Proper Completion**: Checks if there are reachable states where an end event executes multiple times.

## Usage

```python
import rust_bpmn_analyzer_bindings as bpmn

# Load your BPMN XML string
bpmn_xml = """<?xml version="1.0" encoding="UTF-8"?>
<bpmn:definitions ...>
    ...
</bpmn:definitions>"""

# Analyze safeness
result = bpmn.analyze_safeness(bpmn_xml)

print(f"Property: {result.property_name}")
print(f"Fulfilled: {result.fulfilled}")
if not result.fulfilled:
    print(f"Problematic Elements: {result.problematic_elements}")
print(f"Description: {result.description}")

# Other available analyses:
# bpmn.analyze_dead_activities(bpmn_xml)
# bpmn.analyze_option_to_complete(bpmn_xml)
# bpmn.analyze_proper_completion(bpmn_xml)
```

## Disclaimer

This is an **unofficial** wrapper. It is not maintained or endorsed by the original authors of `rust_bpmn_analyzer`.

## Building from source

To build this package from source, you will need the Rust toolchain installed and [`maturin`](https://github.com/PyO3/maturin).

```bash
pip install maturin
maturin develop --release
```
