use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State(usize);

#[derive(Debug)]
struct NFA {
    states: HashSet<State>,
    transitions: HashMap<(State, Option<char>), HashSet<State>>,
    start_state: State,
    accept_states: HashSet<State>,
}

impl NFA {
    fn new(start_state: State, accept_states: HashSet<State>) -> Self {
        NFA {
            states: HashSet::new(),
            transitions: HashMap::new(),
            start_state,
            accept_states,
        }
    }

    fn add_state(&mut self, state: State) {
        self.states.insert(state);
    }

    fn add_transition(&mut self, from: State, input: Option<char>, to: State) {
        self.transitions
            .entry((from, input))
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    fn is_accepting(&self, input: &str) -> bool {
        let mut current_states = HashSet::new();
        current_states.insert(self.start_state.clone());

        for c in input.chars() {
            let mut next_states = HashSet::new();
            for state in &current_states {
                if let Some(states) = self.transitions.get(&(state.clone(), Some(c))) {
                    next_states.extend(states.clone());
                }
                if let Some(states) = self.transitions.get(&(state.clone(), None)) {
                    next_states.extend(states.clone());
                }
            }
            current_states = next_states;
        }

        current_states
            .iter()
            .any(|state| self.accept_states.contains(state))
    }
}

pub fn test() {

    let start_state = State(0);
    let accept_states = vec![State(1)].into_iter().collect();

    let mut nfa = NFA::new(start_state.clone(), accept_states);
    nfa.add_state(start_state.clone());
    nfa.add_state(State(1));

    nfa.add_transition(start_state.clone(), Some('a'), State(0));
    nfa.add_transition(start_state.clone(), Some('a'), State(1));
    nfa.add_transition(State(1), Some('b'), State(1));

    let input = "aab";
    println!("Does the NFA accept '{}'? {}", input, nfa.is_accepting(input));

}