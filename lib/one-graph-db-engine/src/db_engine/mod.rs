use one_graph_gremlin::gremlin::*;
use one_graph_core::graph_engine::GraphEngine;
use one_graph_core::model::init::InitContext;

use self::gremlin_state::*;
use self::utils::convert_graph_to_gremlin_response;
use self::utils::is_creation_graph_only;


pub mod gremlin_state;
mod utils;
mod match_out_edge_state;
mod match_vertex_state;
mod alias_vertex_state;
mod add_edge_state;
mod match_state;
mod from_state;
mod add_vertex_state;
mod set_property_state;


pub struct GraphDatabaseEngine<'a> {
    conf: InitContext<'a>,
}

fn iterate_gremlin_steps(steps: &Vec<GStep>, mut gremlin_state: GremlinStateMachine) -> Option<GremlinStateMachine> {
    let mut previous_step = GStep::Empty;
    for step in steps {
        match step {
            GStep::Match(bytecodes) => {
                for bc in bytecodes {
                    gremlin_state = iterate_gremlin_steps(bc, gremlin_state)?;
                }
            }
            _ => {
                gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &previous_step, step)?;
            }
        }
        previous_step = step.clone();
    }
    gremlin_state = GremlinStateMachine::new_step_state(gremlin_state, &previous_step, &GStep::Empty)?;
    Some(gremlin_state)
}

impl <'a> GraphDatabaseEngine<'a> {
    pub fn new(ctx: InitContext<'a>) -> Self {
        GraphDatabaseEngine{conf: ctx}
    }

    pub fn handle_gremlin_request(&mut self, gremlin: &GremlinRequest) -> Option<GremlinResponse> {
        let mut gremlin_state = GremlinStateMachine::new();
        gremlin_state = iterate_gremlin_steps(&gremlin.steps, gremlin_state)?;
        let ctx = gremlin_state.context;
        let mut graph_engine = GraphEngine::new(&self.conf);
        for pattern in ctx.patterns {
            if is_creation_graph_only(&pattern) {
                let created = graph_engine.create_graph(&pattern)?;
                graph_engine.sync();
                return convert_graph_to_gremlin_response(&created, &gremlin.request_id);
            }
        }
        None
    }

}