use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::plugin::metadata::TransformPluginMetadataContextKind::Filename;

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let filename = metadata.get_context(&Filename);
    println!("File Name: {:?}", filename);
    program.fold_with(&mut as_folder(TransformVisitor))
}
