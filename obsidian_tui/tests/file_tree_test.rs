#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::io::Write;

    fn create_test_directory(root: &PathBuf) -> std::io::Result<()> {
        fs::create_dir_all(root.join("test_dir").join("dir1"))?;
        fs::File::create(root.join("test_dir").join("dir1").join("file1.txt"))?.write_all(b"file1 content")?;
        fs::File::create(root.join("test_dir").join("dir1").join("file2.txt"))?.write_all(b"file2 content")?;
        fs::File::create(root.join("test_dir").join("file3.txt"))?.write_all(b"file3 content")?;
        Ok(())
    }

    fn cleanup_test_directory(root: &PathBuf) -> std::io::Result<()> {
        fs::remove_dir_all(root.join("test_dir"))?;
        Ok(())
    }

    #[test]
    fn test_file_tree_structure() -> std::io::Result<()> {
        let root_dir = PathBuf::from("./");
        let test_root = root_dir.join("test_dir");

        create_test_directory(&root_dir)?;

        let tree = FileTreeNode::build_tree(&test_root)?;
        let display_lines = tree.get_display_lines();

        let expected_lines = vec![
        "└── 📁 test_dir",
        "    ├── 📁 dir1",
        "    │   ├── 📄 file1.txt",
        "    │   └── 📄 file2.txt",
        "    └── 📄 file3.txt",
        ];

        assert_eq!(display_lines, expected_lines);

        cleanup_test_directory(&root_dir)?;
        Ok(())
    }
}
