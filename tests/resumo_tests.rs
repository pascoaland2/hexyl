use assert_cmd::Command;

fn hexyl() -> Command {
    let mut cmd = Command::new(assert_cmd::cargo_bin!("hexyl"));
    cmd.current_dir("tests/examples");
    cmd
}

fn stdout_of(cmd: &mut Command) -> String {
    let output = cmd.assert().success().get_output().stdout.clone();
    String::from_utf8(output).unwrap()
}

#[test]
fn help_lists_resumo_option() {
    let out = stdout_of(hexyl().arg("--help"));
    assert!(out.contains("-R, --resumo"), "help output: {out}");
}

#[test]
fn resumo_prints_summary_after_bottom_border() {
    let out = stdout_of(
        hexyl()
            .arg("-R")
            .arg("--color=never")
            .write_stdin(&b"AB \x00\xff"[..]),
    );
    let expected = "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
                    │00000000│ 41 42 20 00 ff          ┊                         │AB ⋄×   ┊        │\n\
                    └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n\
                    \n\
                    Resumo:\n\
                    Total de bytes: 5\n\
                    NULL: 1 (20.0%)\n\
                    ASCII imprimível: 2 (40.0%)\n\
                    Espaço em branco ASCII: 1 (20.0%)\n\
                    Outros ASCII: 0 (0.0%)\n\
                    Não-ASCII: 1 (20.0%)\n";
    pretty_assertions::assert_eq!(out, expected);
}

#[test]
fn resumo_with_length_counts_only_shown_bytes() {
    let out = stdout_of(
        hexyl()
            .arg("ascii")
            .arg("-R")
            .arg("--length")
            .arg("4")
            .arg("--color=never"),
    );
    assert!(
        out.ends_with(
            "\nResumo:\n\
             Total de bytes: 4\n\
             NULL: 0 (0.0%)\n\
             ASCII imprimível: 4 (100.0%)\n\
             Espaço em branco ASCII: 0 (0.0%)\n\
             Outros ASCII: 0 (0.0%)\n\
             Não-ASCII: 0 (0.0%)\n"
        ),
        "output: {out}"
    );
}

#[test]
fn resumo_on_empty_input() {
    let out = stdout_of(hexyl().arg("empty").arg("--resumo").arg("--color=never"));
    assert!(
        out.ends_with(
            "\nResumo:\n\
             Total de bytes: 0\n\
             NULL: 0 (0.0%)\n\
             ASCII imprimível: 0 (0.0%)\n\
             Espaço em branco ASCII: 0 (0.0%)\n\
             Outros ASCII: 0 (0.0%)\n\
             Não-ASCII: 0 (0.0%)\n"
        ),
        "output: {out}"
    );
}

#[test]
fn without_resumo_output_is_unchanged() {
    let out = stdout_of(hexyl().arg("ascii").arg("--color=never"));
    assert_eq!(
        out,
        "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
         │00000000│ 30 31 32 33 34 35 36 37 ┊ 38 39 61 62 63 64 65 0a │01234567┊89abcde_│\n\
         └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n"
    );
    assert!(!out.contains("Resumo:"));
}

#[test]
fn resumo_with_plain_output() {
    let out = stdout_of(hexyl().arg("ascii").arg("--plain").arg("-R"));
    assert!(out.starts_with("  30 31 32 33"), "output: {out}");
    assert!(
        out.contains("\n\nResumo:\nTotal de bytes: 16\n"),
        "output: {out}"
    );
    assert!(
        out.contains("Espaço em branco ASCII: 1 (6.2%)\n"),
        "output: {out}"
    );
}
