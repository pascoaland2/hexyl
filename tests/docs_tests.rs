use std::fs;
use std::path::Path;

fn read_doc(rel: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e))
}

#[test]
fn readme_documents_resumo_with_example() {
    let readme = read_doc("README.md");
    assert!(readme.contains("--resumo"));
    assert!(readme.contains("-R"));
    assert!(readme.contains("## Resumo de bytes"));
    assert!(readme.contains("hexyl --resumo"));
    assert!(readme.contains("Resumo:"));
    assert!(readme.contains("Total de bytes: 5"));
    assert!(readme.contains("Não-ASCII: 1 (20.0%)"));
    let resumo_pos = readme.find("## Resumo de bytes").unwrap();
    let config_pos = readme.find("## Configuration").unwrap();
    assert!(resumo_pos < config_pos);
}

#[test]
fn man_page_lists_resumo_option() {
    let man = read_doc("doc/hexyl.1.md");
    assert!(man.contains("**-R**, **\\--resumo**"));
    assert!(man.contains("imprime um resumo com o total de bytes"));
}
