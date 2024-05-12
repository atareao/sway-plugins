use swayipc_async::{Node, Connection, NodeType};

use super::Workspace;

#[derive(Debug, Clone)]
pub struct Root{
    node: Node,
}

impl Root {
    pub async fn default() -> Root {
        let node = Connection::new()
            .await
            .unwrap()
            .get_tree()
            .await
            .unwrap();
        Self{
            node,
        }
    }
    pub fn get_workspaces(&self) -> Vec<Workspace> {
        get_workspaces(&self.node)
    }

    pub fn get_workspace(&self, node: &Node) -> Option<Workspace> {
        self.get_workspaces()
            .into_iter()
            .find(|workspace| workspace.has_node(node))
    }
}

fn get_workspaces(node: &Node) -> Vec<Workspace> {
    let mut workspaces: Vec<Workspace> = Vec::new();
    for child in node.nodes.as_slice(){
        if child.node_type == NodeType::Workspace && child.num.is_some(){
            workspaces.push(Workspace::new(child.clone()));
        }else{
            let some_workspaces = get_workspaces(child);
            workspaces.extend(some_workspaces);
        }
    }
    workspaces
}
