//! Counterexample DTOs serialized to JSON for the Python bindings.
//!
//! Adapted from `rust_bpmn_analyzer/wasm/src/dtos/mod.rs` of the
//! bpmn-analyzer-js project (https://github.com/timKraeuter/bpmn-analyzer-js),
//! MIT License, Copyright (c) 2024 Tim Kräuter. The JSON shape produced here is
//! intentionally identical so the existing front-end visualization can consume it.

use rust_bpmn_analyzer::states::state_space::{ProcessSnapshot, State, StateSpace};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct CounterExample {
    start_state: StateDTO,
    transitions: Vec<Transition>,
}

impl CounterExample {
    /// Build a counterexample (the path from the start state to the first
    /// problematic state) from the analyzer's `problematic_state_hashes`.
    pub fn new(
        problematic_state_hashes: Vec<u64>,
        state_space: &StateSpace,
    ) -> Option<CounterExample> {
        match problematic_state_hashes.first() {
            None => None,
            Some(problematic_state) => match state_space.get_path_to_state(*problematic_state) {
                None => None,
                Some(path) => {
                    let transitions = path
                        .into_iter()
                        .map(|(label, state_hash)| Transition {
                            label: label.to_string(),
                            next_state: StateDTO::from(state_space.get_state(&state_hash)),
                        })
                        .collect();
                    let start_state = state_space.get_state(&state_space.start_state_hash);
                    Some(CounterExample {
                        start_state: StateDTO::from(start_state),
                        transitions,
                    })
                }
            },
        }
    }
}

#[derive(Serialize)]
struct Transition {
    // label is the executed flow node id
    label: String,
    next_state: StateDTO,
}

#[derive(Serialize)]
struct StateDTO {
    pub snapshots: Vec<ProcessSnapshotDTO>,
    pub messages: BTreeMap<String, u16>,
    pub executed_end_event_counter: BTreeMap<String, u16>,
}

impl From<&State<'_>> for StateDTO {
    fn from(state: &State) -> StateDTO {
        let snapshots = state
            .snapshots
            .iter()
            .map(ProcessSnapshotDTO::from)
            .collect();

        let mut messages = BTreeMap::new();
        messages.extend(
            state
                .messages
                .iter()
                .map(|(message, count)| (message.to_string(), *count)),
        );

        let mut executed_end_event_counter = BTreeMap::new();
        executed_end_event_counter.extend(
            state
                .executed_end_event_counter
                .iter()
                .map(|(end_event, count)| (end_event.to_string(), *count)),
        );

        StateDTO {
            snapshots,
            messages,
            executed_end_event_counter,
        }
    }
}

#[derive(Serialize)]
struct ProcessSnapshotDTO {
    pub id: String,
    pub tokens: BTreeMap<String, u16>,
}

impl From<&ProcessSnapshot<'_>> for ProcessSnapshotDTO {
    fn from(snapshot: &ProcessSnapshot) -> Self {
        let mut tokens = BTreeMap::new();
        tokens.extend(
            snapshot
                .tokens
                .iter()
                .map(|(token, count)| (token.to_string(), *count)),
        );
        ProcessSnapshotDTO {
            id: snapshot.id.to_string(),
            tokens,
        }
    }
}
