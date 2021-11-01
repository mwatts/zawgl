use super::gremlin_state::{State, StateContext};
use super::alias_vertex_state::AliasVertexState;
use one_graph_core::model::*;
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use super::match_out_edge_state::MatchOutEdgeState;
use super::match_state::MatchState;
use super::add_edge_state::AddEdgeState;

pub struct MatchInVertexState {
}

impl MatchInVertexState {
    pub fn new() -> Self {
        MatchInVertexState{}
    }
}

impl State for MatchInVertexState {
    
    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
        let mut n = Node::new();
        n.set_status(Status::Match);

        match &context.previous_step {
            GStep::OutE(labels) => {
                if let Some(node_index) = context.node_index {
                    let mut rel = Relationship::new();
                    rel.set_labels(labels.clone());
                    rel.set_status(Status::Match);
                    let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
                    let nid = pattern.add_node(n);
                    pattern.add_relationship(rel, node_index, nid);
                    context.node_index = Some(nid);
                }
            }
            _ => {

            }
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::OutE(_labels) => {
                Ok(Box::new(MatchOutEdgeState::new()))
            }
            GStep::As(alias) => {
                Ok(Box::new(AliasVertexState::new(alias)))
            }
            GStep::Match(bytecodes) => {
                Ok(Box::new(MatchState::new(bytecodes)))
            }
            GStep::AddE(_label) => {
                Ok(Box::new(AddEdgeState::new()))
            }
            GStep::Empty => {
                Ok(Box::new(EndState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}

