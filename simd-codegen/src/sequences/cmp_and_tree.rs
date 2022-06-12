use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::VecDeque;

pub(crate) struct CmpAndTree {
    instructions: Vec<TokenStream>,
    root_node_ident: Option<Ident>,
    next_node_id: usize,
    nodes: VecDeque<Ident>,
}

impl CmpAndTree {
    pub(crate) fn build_tree(leaves: Vec<Ident>) -> CmpAndTree {
        assert!(!leaves.is_empty());

        let mut tree = CmpAndTree {
            instructions: vec![],
            root_node_ident: None,
            next_node_id: 1,
            nodes: leaves.into(),
        };

        while tree.nodes.len() > 1 {
            tree.combine_nodes_once();
        }

        tree.root_node_ident = Some(tree.nodes[0].clone());

        tree
    }

    pub(crate) fn root_node_ident(&self) -> Ident {
        self.root_node_ident.clone().unwrap()
    }

    pub(crate) fn instructions(&self) -> &[TokenStream] {
        &self.instructions
    }

    fn combine_nodes_once(&mut self) {
        debug_assert!(self.nodes.len() > 1);

        let new_node = format_ident!("cmp{}", self.next_node_id);
        self.next_node_id += 1;

        let node1 = self.nodes.pop_front();
        let node2 = self.nodes.pop_front();

        let instruction = quote! {
            let #new_node = #node1 & #node2;
        };

        self.instructions.push(instruction);
        self.nodes.push_back(new_node);
    }
}
