mod base_state;
mod state;

use std::collections::{HashMap, HashSet};
use crate::graph_engine::model::{ProxyNodeId, GraphProxy};
use crate::model::{PropertyGraph, Relationship, Node};

use self::state::State;
use super::super::graph::traits::*;
use super::super::graph::*;

enum IterationStates {
    Process,
    Validate,
    LookForCandidates,
    InitGraph1Loop,
    Graph1Loop,
    Backtrack,
}

pub struct Matcher<'g0: 'g1, 'g1, VCOMP, ECOMP, CALLBACK>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
    CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy) -> Option<bool> {
        state: State<'g0, 'g1, VCOMP, ECOMP>,
        found_match: bool,
        match_continuation: Vec<(NodeIndex, ProxyNodeId)>,
        first_candidate_0: Option<NodeIndex>,
        callback: CALLBACK,
}

impl <'g0, 'g1, VCOMP, ECOMP, CALLBACK> Matcher <'g0, 'g1, VCOMP, ECOMP, CALLBACK>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
    CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy) -> Option<bool> {

        pub fn new(graph_0: &'g0 PropertyGraph, graph_1: &'g1 mut GraphProxy, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Self {
            Matcher {
                state: State::new(graph_0, graph_1, vcomp, ecomp),
                found_match: false,
                match_continuation: Vec::new(),
                first_candidate_0: None,
                callback: callback,
            }
        }

        fn back_track(&mut self) {
            let last =  self.match_continuation.pop();
            if let Some(back) = last {
                self.state.pop(&back.0, &back.1);
            }
        }

        pub fn process(&mut self, ids0: Vec<NodeIndex>, ids1: Vec<ProxyNodeId>) -> Option<bool> {
            let mut it0;
            let mut it1 = ids1.iter();
            let mut state = IterationStates::Process;
            loop {
                match state {
                    IterationStates::Process => {
                        if self.state.success() {
                            self.found_match = true;
                            if !self.state.call_back(&mut self.callback)? {
                                return Some(true);
                            } else {
                                state = IterationStates::Backtrack;
                            }
                        } else {
                            state = IterationStates::Validate;
                        }
                    },
                    IterationStates::Validate => {
                        if !self.state.valid() {
                            state = IterationStates::Backtrack;
                        } else {
                            state = IterationStates::LookForCandidates;
                        }
                    },
                    IterationStates::LookForCandidates => {
                        it0 = ids0.iter();
                        while let Some(id0) = it0.next() {
                            if self.state.possible_candidate_0(id0) {
                                self.first_candidate_0 = Some(*id0);
                                break;
                            }
                        }
                        state = IterationStates::InitGraph1Loop;
                    },
                    IterationStates::InitGraph1Loop => {
                        it1 = ids1.iter();
                        state = IterationStates::Graph1Loop;
                    },
                    IterationStates::Graph1Loop => {
                        let mut backtrack = true;
                        if let Some(id0) = &self.first_candidate_0 {
                            while let Some(id1) = it1.next() {
                                if self.state.possible_candidate_1(id1) && self.state.feasible(id0, id1)? {
                                    self.match_continuation.push((*id0, *id1));
                                    self.state.push(id0, id1);
                                    backtrack = false;
                                    break;
                                }
                            }
                        }
                        if !backtrack {
                            state = IterationStates::Process;
                        } else {
                            state = IterationStates::Backtrack;
                        }
                    },
                    IterationStates::Backtrack => {
                        if self.match_continuation.is_empty() {
                            return Some(self.found_match);
                        } else {
                            self.back_track();
                            state = IterationStates::Graph1Loop;
                        }
                    }
                }
            }
        }
    }

fn sort_nodes<'g>(graph: &'g PropertyGraph) -> Vec<NodeIndex> {
    let mut res = graph.get_nodes_ids();
    res.sort_by(|a, b| (graph.in_degree(b) + graph.out_degree(b)).cmp(&(graph.in_degree(a) + graph.out_degree(a))));
    res
}

pub fn sub_graph_isomorphism<'g0: 'g1, 'g1, VCOMP, ECOMP, CALLBACK>
(graph_0: &'g0 PropertyGraph, graph_1: &'g1 mut GraphProxy, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Option<bool>
where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy)-> Option<bool>  {

    let id0 = sort_nodes(graph_0);
    let id1 = graph_1.get_nodes_ids();
    let mut matcher = Matcher::new(graph_0, graph_1, vcomp, ecomp, callback);
    
    matcher.process(id0, id1)
}