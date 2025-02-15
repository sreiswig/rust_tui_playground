use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct FileTreeNode{
    name: String,
    path: PathBuf,
    children: Vec<FileTreeNode>,
    is_dir: bool,
}

impl FileTreeNode {
    fn new(name: String, path: PathBuf, is_dir: bool) -> Self {
        FileTreeNode {
            name,
            path,
            children: Vec::new(),
            is_dir,
        }
    }

    fn populate_children(&mut self) -> std::io::Result<()>{
        if !self.is_dir {
            return Ok(())
        }

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().into_owned();
            let file_type = entry.file_type()?;

            let mut child_node = FileTreeNode::new(
                file_name_str,
                path.clone(),
                file_type.is_dir(),
            );

            if file_type.is_dir() {
                child_node.populate_children()?;
            }
            self.children.push(child_node);
        }
        self.children.sort_by(|a, b| {
            if a.is_dir && !b.is_dir {
                std::cmp::Ordering::Less
            } else if !a.is_dir && b.is_dir {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });
        Ok(());
    }

    fn build_tree(path: &PathBuf) -> std::io::Result<FileTreeNode> {
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or(".").to_string();
        let mut root_node = FileTreeNode::new(
            file_name,
            path.clone(),
            path.is_dir(),
        );
        if path.is_dir() {
            root_node.populate_children()?;
        }
        Ok(root_node)
    }
}
