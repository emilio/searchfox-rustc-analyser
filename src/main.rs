extern crate rls_analysis as analysis;
#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod loader;

use loader::Loader;
use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;

type Host = analysis::AnalysisHost<Loader>;

fn collect_files(id: analysis::Id, host: &Host, files: &mut HashSet<PathBuf>) {
    each_def_from(id, host, &mut |id, def| {
        files.insert(def.span.file.clone());
    });
}

fn each_def_from<F>(id: analysis::Id, host: &Host, f: &mut F)
where
    F: FnMut(analysis::Id, &analysis::Def),
{
    let childs = host.for_each_child_def(id, |child_id, def| {
        f(child_id, def);
        child_id
    });

    if let Ok(childs) = childs {
        for child_id in childs {
            each_def_from(child_id, host, f);
        }
    }
}

fn dump_symbol(
    file: &mut File,
    symbol: &analysis::SymbolResult,
    def: &analysis::Def,
) -> Result<(), io::Error> {
    unimplemented!();
}

fn main() {
    let matches = app_from_crate!()
        .args_from_usage(
            "<src>    'Points to the source root'
             <input>  'Points to the deps/save-analysis directory'
             <output> 'Points to the directory where searchfox metadata should'"
        )
        .get_matches();

    let src_dir = Path::new(matches.value_of("src").unwrap());
    let input_dir = Path::new(matches.value_of("input").unwrap());
    let output_dir = Path::new(matches.value_of("output").unwrap());

    let loader = Loader::new(PathBuf::from(input_dir));


    if false {
        let crates = analysis::read_analysis_from_files(
            &loader,
            Default::default(),
            &[],
        );

        println!("{:?}", crates);
    }

    let host = analysis::AnalysisHost::new_with_loader(loader);
    host.reload(src_dir.clone(), src_dir.clone()).unwrap();

    let roots = host.def_roots().unwrap();
    let mut files = HashSet::new();
    for &(root_id, ref name) in &roots {
        collect_files(root_id, &host, &mut files);
    }

    for file in files {
        let symbols = match host.symbols(&file) {
            Ok(symbols) => symbols,
            Err(..) => {
                eprintln!("Couldn't find symbols for {}", file.display());
                continue;
            }
        };

        let stripped_file = match file.strip_prefix(&src_dir) {
            Ok(stripped) => stripped,
            Err(err) => {
                eprintln!("File wasn't in the source dir: {}", file.display());
                continue;
            }
        };

        let dest = output_dir.join(&stripped_file);
        let mut out = match File::create(dest) {
            Ok(out) => out,
            Err(err) => {
                eprintln!("Couldn't create destination file: {:?}", err);
                continue;
            }
        };

        for symbol in symbols {
            let def =
                host.get_def(symbol.id).expect("Symbol without definition?");

            if dump_symbol(&mut out, &symbol, &def).is_err() {
                eprintln!("Couldn't dump: {:?}, {:?}", symbol, def);
            }
        }
    }
}
