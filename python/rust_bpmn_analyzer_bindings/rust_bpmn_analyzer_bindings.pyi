class PyProperty:
    """
    Represents the result of a BPMN property analysis.
    """
    property_name: str
    fulfilled: bool
    problematic_elements: list[str]
    description: str

    def __init__(
        self,
        property_name: str,
        fulfilled: bool,
        problematic_elements: list[str],
        description: str,
    ) -> None: ...

def analyze_safeness(model: str) -> PyProperty:
    """Analyzes if the given BPMN model properly synchronizes concurrent activities"""
    ...

def analyze_dead_activities(model: str) -> PyProperty:
    """Analyzes if the given BPMN model has dead activities."""
    ...

def analyze_option_to_complete(model: str) -> PyProperty:
    """Analyzes if the given BPMN model has a deadlock state that can be reached."""
    ...

def analyze_proper_completion(model: str) -> PyProperty:
    """Analyzes if the given BPMN model has reachable states where an end event executes multiple times."""
    ...
