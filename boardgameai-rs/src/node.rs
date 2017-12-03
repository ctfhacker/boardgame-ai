use action::Action;
use state::State;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct NodeId {
    index: usize
}

pub struct NodeArena {
    nodes: Vec<Node>
}

impl NodeArena {
    pub fn new() -> NodeArena {
        NodeArena { nodes: Vec::new() }
    }

    pub fn new_node<S: State>(&mut self, state: S) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node {
            id: NodeId { index: index },
            action: None,
            parent: None,
            children: Vec::new(),
            wins: 0.0,
            visits: 0,
            untried_actions: state.get_actions(),
            untried_action_strings: state.get_action_strings(),
            player_just_moved: state.get_player_just_moved()
        });

        NodeId{ index: index }
    }

    pub fn new_child_node<S: State>(&mut self, parent: Option<NodeId>, action: Option<u32>, state: S) -> NodeId {
        let index = self.nodes.len();

        self.nodes.push(Node {
            id: NodeId { index: index },
            action: action,
            parent: parent,
            children: Vec::new(),
            wins: 0.0,
            visits: 0,
            untried_actions: state.get_actions(),
            untried_action_strings: state.get_action_strings(),
            player_just_moved: state.get_player_just_moved()
        });

        NodeId{ index: index }
    }
}

impl Index<NodeId> for NodeArena {
    type Output = Node;

    fn index(&self, node: NodeId) -> &Node {
        &self.nodes[node.index]
    }
}

impl IndexMut<NodeId> for NodeArena {
    fn index_mut(&mut self, node: NodeId) -> &mut Node {
        &mut self.nodes[node.index]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// Id of the node itself to find itself in the NodeArena
    pub id: NodeId,
    /// Action that got us to this node - None for root
    pub action: Option<u32>, 
    /// Parent node - None for root
    pub parent: Option<NodeId>,
    /// Children nodes
    pub children: Vec<NodeId>,
    /// Number of current wins for this node
    pub wins: f32,
    /// Number of visits for this node
    pub visits: u32,
    /// Vector of actions left to take 
    pub untried_actions: Vec<u32>,
    /// Vector of the string representation of the actions left to take
    pub untried_action_strings: Vec<String>,
    /// Number of the player who has just played
    pub player_just_moved: u32,
}

impl Node {
    pub fn add_child<S: State>(&self, arena: &mut NodeArena, action: Option<u32>, state: S) -> NodeId {

        let new_node = arena.new_child_node(Some(self.id), action, state);
        let self_node = &mut arena[new_node];

        self_node.untried_actions.iter()
            .position(|&n| n == action.unwrap())
            .map(|e| self_node.untried_actions.remove(e));

        new_node
    }

    pub fn update(&mut self, result: f32) {
        self.visits += 1;
        self.wins += result;
    }
}
