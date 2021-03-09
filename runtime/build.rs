use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .import_memory()
        .export_heap_base()
        .build()
}

// 使用wasm-builder-runner将当前的runtime项目编译为Wasm二进制文件，该文件位于target/release/wbuild/node-template-runtimenode_template_runtime.compact.wasm