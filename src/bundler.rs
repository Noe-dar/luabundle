use std::{
    collections::HashMap,
    fmt::{self, Debug},
    fs::read_to_string,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::bail;
use full_moon::{
    ast::Ast,
    parse, print,
    visitors::VisitorMut,
    ShortString,
};
use petgraph::{
    algo::is_cyclic_directed,
    graph::NodeIndex,
    visit::Bfs,
    Graph,
};

use crate::{
    resolver::Resolver, utils::WriteExt,
    visitors::require_patcher::RequirePatcher,
};

#[derive(Clone)]
pub struct Module {
    virtual_file_name: ShortString,
    ast: Ast,
}

impl Module {
    pub fn new(virtual_file_name: ShortString, ast: Ast) -> Self {
        Self {
            virtual_file_name,
            ast,
        }
    }
}

impl Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.virtual_file_name)
    }
}

pub struct Bundler<W: Write> {
    bundle: BufWriter<W>,
    dependency_graph: Graph<Module, ()>,
    cache: HashMap<String, NodeIndex>,
    resolver: Resolver,
}

impl<W: Write> Bundler<W> {
    pub fn new(bundle: W, resolver: Resolver) -> Self {
        Self {
            bundle: BufWriter::new(bundle),
            dependency_graph: Graph::default(),
            cache: HashMap::new(),
            resolver,
        }
    }

    fn append_prelude(&mut self) -> anyhow::Result<()> {
        self.bundle
            .writeln_str(include_str!("prelude/comment.lua"))?;
        self.bundle.writeln_str(include_str!("prelude/cache.lua"))?;

        self.bundle.flush()?;
        Ok(())
    }

    fn append_module(&mut self, virtual_file_name: &str, ast: Ast) -> anyhow::Result<()> {
        let code = print(&ast);
        self.bundle.writeln_str(&format!(
            "ADD_MODULE(\"{}\", function()\n {}\n end)\n",
            virtual_file_name, code
        ))?;
        Ok(())
    }

    // TODO: Reduce the number of allocations
    pub fn bundle_module<P: AsRef<Path>>(
        &mut self,
        path: P,
        virtual_file_name: ShortString,
    ) -> anyhow::Result<NodeIndex> {
        if let Some(node) = self.cache.get(&virtual_file_name.to_string()) {
            return Ok(*node);
        }
        let path = path.as_ref();
        let base_path = path.parent().expect("empty path");

        let module_source = read_to_string(path)?;
        let module_ast = parse(&module_source)?;

        let mut require_patcher = RequirePatcher::default();
        let module_ast = require_patcher.visit_ast(module_ast);

        let node = self
            .dependency_graph
            .add_node(Module::new(virtual_file_name.clone(), module_ast));

        self.cache.insert(virtual_file_name.to_string(), node);

        for require in require_patcher.requires {
            let resolved_path = self.resolver.resolve(require.as_str(), base_path)?;

            let module_node = self.bundle_module(resolved_path, require)?;

            self.dependency_graph.add_edge(node, module_node, ());
        }

        Ok(node)
    }

    // TODO: Verryyyy bad code, rewrite
    pub fn bundle(&mut self, input: PathBuf) -> anyhow::Result<()> {
        self.append_prelude()?;

        let root = self.bundle_module(input, ShortString::new("root"))?;

        if is_cyclic_directed(&self.dependency_graph) {
            // TODO: Better errors about circular dependencies
            bail!("circular dependency")
        }

        let mut bfs = Bfs::new(&self.dependency_graph, root);

        while let Some(node_index) = bfs.next(&self.dependency_graph) {
            if node_index == root { continue; }
            let module = (&self.dependency_graph[node_index]).to_owned();

            self.append_module(&module.virtual_file_name, module.ast.clone())?;
        }

        let root_module = self.dependency_graph[root].clone();
        let code = print(&root_module.ast);

        self.bundle.writeln_str(&code)?;

        self.bundle.flush()?;
        Ok(())
    }
}
