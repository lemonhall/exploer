use druid::widget::{Flex, Label};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid_widget_nursery::{Tree, TreeNode};

#[derive(Clone, Data, Lens, Debug, PartialEq)]
struct FileItem {
    name: String,
    is_expanded: bool,
    #[data(same_fn = "PartialEq::eq")]
    children: Vec<FileItem>,
}

impl TreeNode for FileItem {
    fn children_count(&self) -> usize {
        self.children.len()
    }
    
    fn get_child(&self, index: usize) -> &Self {
        &self.children[index]
    }
    
    fn for_child_mut(&mut self, index: usize, mut cb: impl FnMut(&mut Self, usize)) {
        let child = &mut self.children[index];
        cb(child, index);
    }
}

#[derive(Clone, Data, Lens)]
struct AppState {
    root: FileItem,
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("文件管理器")
        .window_size((800.0, 600.0));

    let root = FileItem {
        name: "Root".to_string(),
        children: build_file_tree(std::env::current_dir().unwrap().as_path(), 3),
        is_expanded: true,
    };

    let initial_state = AppState {
        root: root,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_file_tree(path: &std::path::Path, depth: usize) -> Vec<FileItem> {
    if depth == 0 {
        return Vec::new();
    }

    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let children = if path.is_dir() {
                    build_file_tree(&path, depth - 1)
                } else {
                    Vec::new()
                };
                items.push(FileItem { 
                    name, 
                    children,
                    is_expanded: false 
                });
            }
        }
    }
    items
}

fn build_ui() -> impl Widget<AppState> {
    let tree = Tree::new(
        || {
            Flex::column()
                .with_child(Label::dynamic(|item: &FileItem, _| item.name.clone()).padding(5.0))
        },
        FileItem::is_expanded,
    )
    .lens(AppState::root);

    tree
}
