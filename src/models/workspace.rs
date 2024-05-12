use swayipc_async::{
    Node,
    NodeType,
};
use tracing::debug;

pub struct Workspace{
    node: Node,
}

impl Workspace {
    pub fn new(node: Node) -> Self {
        Self{
            node,
        }
    }

    pub fn get_name(&self) -> String {
        self.node.clone().name.unwrap_or("".to_string())
    }

    pub fn get_num(&self) -> i32 {
        self.node.clone().num.unwrap_or(0)
    }

    pub fn get_id(&self) -> i64 {
        self.node.clone().id
    }

    pub fn get_focused(&self) -> Option<Node> {
        get_focused(self.node.clone())
    }

    pub fn has_node(&self, node: &Node) -> bool {
        has_node(&self.node, node.id)
    }

    pub fn get_apps(&self) -> Vec<String> {
        get_apps(&self.node)
    }
}

pub fn get_focused(node: Node) -> Option<Node> {
    if node.focused{
        Some(node)
    }else{
        for child in node.nodes.as_slice(){
            if let Some(focused) = get_focused(child.clone()){
                return Some(focused);
            }
        }
        None
    }
}

fn has_node(node: &Node, id: i64) -> bool{
    if node.id == id {
        true
    }else{
        for child in node.nodes.as_slice(){
            if has_node(child, id){
                return true;
            }
        }
        false
    }
}

fn get_apps(node: &Node) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    if node.node_type == NodeType::Con || node.node_type == NodeType::FloatingCon{
        if let Some(name) = get_name(node) {
            names.push(name);
        }
    }
    for child in node.nodes.as_slice(){
        let child_names = get_apps(child);
        names.extend(child_names);
    }
    names
}

fn get_name(node: &Node) -> Option<String> {
    if let Some(app_id) = &node.app_id{
        return Some(app_id.to_string());
    }else if let Some(window_properties) = &node.window_properties {
        debug!("Window properties: {:?}", &window_properties);
        if let Some(class) = &window_properties.class {
            return Some(class.to_string());
        }else if let Some(instance) = &window_properties.instance {
            return Some(instance.to_string());
        }
    }
    None
}
