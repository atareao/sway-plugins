use tokio_i3ipc::{reply::{Node, NodeType}, I3};

use super::Workspace;

#[derive(Debug, Clone)]
pub struct Root{
    node: Node,
}

impl Root {
    pub async fn default() -> Root {
        let node = I3::connect()
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
        for workspace in self.get_workspaces() {
            if workspace.has_node(node) {
                return Some(workspace);
            }
        }
        None

    }
}

fn get_workspaces(node: &Node) -> Vec<Workspace> {
    let mut workspaces: Vec<Workspace> = Vec::new();
    for child in node.nodes.as_slice(){
        if child.node_type == NodeType::Workspace && child.num.is_some(){
            workspaces.push(Workspace::new(child.clone()));
        }else{
            let some_workspaces = get_workspaces(&child);
            workspaces.extend(some_workspaces);
        }
    }
    workspaces
}
