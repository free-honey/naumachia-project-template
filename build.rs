use aiken::Terminal;
use aiken_lang::ast::Tracing;
use aiken_project::Project;

const PROJECT: &str = "./my-scripts";

fn main() {
    let mut project = Project::new(PROJECT.into(), Terminal::default())
        .expect(&format!("Project not found: {:?}", PROJECT));
    let build_result = project.build(false, Tracing::KeepTraces);

    if let Err(err) = build_result {
        err.iter().for_each(|e| e.report());
        panic!("🍂 Failed to build Aiken code 🍂");
    }
}
