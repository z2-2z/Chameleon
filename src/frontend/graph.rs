use std::collections::BTreeMap;
use petgraph::{
    graph::{Graph, NodeIndex},
    algo::has_path_connecting,
    Direction,
};
use crate::grammar::{ContainerId, Grammar, ContainerType};

pub struct GrammarGraph<'a> {
    grammar: &'a Grammar,
    graph: Graph<ContainerId, ()>,
    container_map: BTreeMap<ContainerId, NodeIndex>,
}

impl<'a> GrammarGraph<'a> {
    pub fn full_graph(grammar: &'a Grammar) -> Self {
        let mut graph = Graph::new();
        let mut container_map = BTreeMap::<ContainerId, NodeIndex>::new();
        
        for id in grammar.container_ids() {
            if grammar.container(*id).unwrap().typ() == ContainerType::Struct {
                let idx = graph.add_node(*id);
                assert!( container_map.insert(*id, idx).is_none() );
            }
        }
        
        for (start_id, start_idx) in container_map.iter() {
            for dest_id in grammar.container_callees(*start_id, true) {
                let dest_idx = container_map.get(&dest_id).unwrap();
                graph.add_edge(*start_idx, *dest_idx, ());
            }
        }
        
        assert!(graph.is_directed());
        
        Self {
            grammar,
            graph,
            container_map,
        }
    }
    
    pub fn minimal_graph(grammar: &'a Grammar) -> Self {
        let mut graph = Graph::new();
        let mut container_map = BTreeMap::<ContainerId, NodeIndex>::new();
        
        for id in grammar.container_ids() {
            if grammar.container(*id).unwrap().typ() == ContainerType::Struct {
                let idx = graph.add_node(*id);
                assert!( container_map.insert(*id, idx).is_none() );
            }
        }
        
        for (start_id, start_idx) in container_map.iter() {
            for dest_id in grammar.container_callees(*start_id, false) {
                let dest_idx = container_map.get(&dest_id).unwrap();
                graph.add_edge(*start_idx, *dest_idx, ());
            }
        }
        
        assert!(graph.is_directed());
        
        Self {
            grammar,
            graph,
            container_map,
        }
    }
    
    pub fn unreachable_containers(&self) -> Vec<ContainerId> {
        let mut ret = Vec::new();
        
        let root = self.grammar.root().unwrap();
        let root_idx = self.container_map.get(root).unwrap();
        
        for id in self.grammar.container_ids() {
            if self.grammar.container(*id).unwrap().typ() == ContainerType::Struct {
                let target_idx = self.container_map.get(id).unwrap();
                
                if !has_path_connecting(&self.graph, *root_idx, *target_idx, None) {
                    ret.push(id.clone());
                }
            }
        }
        
        ret
    }
    
    pub fn cycle(&self) -> Option<(ContainerId, ContainerId)> {        
        for (id, idx) in self.container_map.iter() {
            for nb in self.graph.neighbors_directed(*idx, Direction::Outgoing) {
                let target = self.graph.node_weight(nb).unwrap();
                
                if has_path_connecting(&self.graph, nb, *idx, None) {
                    return Some( (*id, *target) );
                }
            }
        }
        
        None
    }
}
