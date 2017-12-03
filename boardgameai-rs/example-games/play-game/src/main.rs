extern crate boardgameai_rs;
extern crate nim;
// extern crate rand;

use boardgameai_rs::state::State;
use boardgameai_rs::node::NodeArena;
use nim::NimState;
use std::fmt::Debug;
use std::ops::Deref;

fn UCT<S: State+Clone+Debug>(arena: &mut NodeArena, mut rootstate: S, iterations: u32) -> u32 {
    let mut rootnode = arena.new_node(rootstate.clone());

    for i in 1..2 {
        let mut node = &arena[rootnode];
        let mut state = rootstate.clone();

        // Select
        if ( node.untried_actions.len() == 0 && node.children.len() > 0 ) {
            println!("Select");
        }

        // Expand
        if node.untried_actions.len() > 0 {
            let action = node.untried_actions[0]; // TODO: Random this
            state.do_action(action);
            let new_nodeid = node.add_child(arena, Some(action), state.clone());
            node = &arena[new_nodeid];

            println!("New node: {:?}", node);
        }

        /*
        // Rollout
        while ( state.get_actions().len() > 0 ) {
            let curr_move = state.get_actions()[0];
            state.do_action(curr_move); // TODO: Random this
            println!("Move: {:?} State: {:?}", curr_move, state);
        }

        println!("Node before: {:?}", node);
        {
            // Backpropogate
            loop {
                let result = state.get_result(node.player_just_moved);
                node.update(result);
                match node.parent {
                    Some(parent) => node = &mut arena[parent],
                    None => break,
                }
            }
        }
        println!("Node before: {:?}", node);
        */
    }

    println!("End rootnode: {:?}", rootnode);
    1 as u32
}

fn main() {
    let arena = &mut NodeArena::new();
    let mut state = NimState::new(5);
    println!("{:?}", state);

    UCT(arena, state, 100);
    println!("{:?}", state);
    /*
    loop {
        let actions = state.get_actions();
        if (actions.len() == 0) {
            break;
        }

        state.do_action(actions[0]);
        println!("{:?}", actions);
    }
    println!("{:?}", state);
    */
}
