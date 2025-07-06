use godot::classes::{EditorPlugin, IEditorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct BuildEditorPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl BuildEditorPlugin {
    #[func]
    fn build() {
        // 获取当前工作目录
        let mut current_dir =
            std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

        current_dir.push("../rust");
        godot_print!("当前工作目录: {}", current_dir.display());

        // 在当前目录执行 cargo build
        std::process::Command::new("cargo")
            .arg("build")
            .current_dir(&current_dir)
            .spawn()
            .unwrap();
    }
}

#[godot_api]
impl IEditorPlugin for BuildEditorPlugin {
    fn enter_tree(&mut self) {
        self.base_mut().add_tool_menu_item(
            "Build",
            &Callable::from_local_static("BuildEditorPlugin", "build"),
        );
    }

    fn exit_tree(&mut self) {
        self.base_mut().remove_tool_menu_item("Build");
    }
}
