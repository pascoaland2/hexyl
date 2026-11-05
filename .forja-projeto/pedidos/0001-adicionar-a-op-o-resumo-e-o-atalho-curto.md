# Pedido

> Adicionar a opção --resumo (e o atalho curto -R). Ao final da saída normal, o hexyl imprime um resumo do arquivo: total de bytes e quantos bytes existem de cada categoria que ele já usa nas cores (NULL/zero, ASCII imprimível, espaço em branco ASCII, outros ASCII e não-ASCII), com a contagem e o percentual de cada uma (uma casa decimal). Sem a opção, a saída deve continuar exatamente igual à de hoje. Incluir testes automatizados e documentar a opção no --help e no README.

## Como o projeto é
hexyl is a Rust command-line hex viewer. It colors each byte by category: NULL, ASCII printable, ASCII whitespace, other ASCII and non-ASCII. `src/main.rs` holds the clap `Opt` struct (derive API) and `run()`, which parses the options, wraps the input in a reader (with --skip/--length applied) and drives `hexyl::PrinterBuilder`/`Printer::print_all`. `src/lib.rs` is the library: the public `ByteCategory` enum, the private `Byte::category()` classifier, color handling and the `Printer` with its own unit tests. `src/input.rs` handles input and seeking. Integration tests in `tests/integration_tests.rs` use assert_cmd against files in `tests/examples` (`ascii`, `empty`, `hello_world_elf64`). User docs are the README and the man page source `doc/hexyl.1.md`.

## Mudanças pedidas (especificação)
## New option `--resumo` / `-R`

- New boolean flag `-R, --resumo`, shown in `--help` as: "Ao final da saída, imprime um resumo com o total de bytes e a contagem/percentual de cada categoria de byte."
- **Without the flag**, stdout, stderr and the exit code are byte-for-byte identical to today's in every mode (normal, --plain, -i include, empty input, squeezing).
- **With the flag**, after the normal output is complete (bottom border/footer included, or the C include output when `-i` is used), hexyl prints to stdout a blank line followed by exactly these lines (no colors, no ANSI codes, no padding):
```
Resumo:
Total de bytes: <T>
NULL: <n> (<p>%)
ASCII imprimível: <n> (<p>%)
Espaço em branco ASCII: <n> (<p>%)
Outros ASCII: <n> (<p>%)
Não-ASCII: <n> (<p>%)
```
- Categories use the same rule as `Byte::category()` in src/lib.rs: 0x00 → NULL; `is_ascii_graphic` → ASCII printable; `is_ascii_whitespace` → ASCII whitespace; other `is_ascii` → other ASCII; everything else → non-ASCII. The 5 counts always add up to T.
- `<p>` = `100.0 * n / T` formatted with `{:.1}`. When T = 0, every percentage is `0.0`.
- Only the bytes hexyl actually showed are counted, i.e. after `--skip` and `--length`. Bytes hidden by squeezing (`*` lines) are still counted.
- Example: stdin `"AB \x00\xff"` with `-R` → Total 5, NULL 1 (20.0%), ASCII imprimível 2 (40.0%), Espaço em branco ASCII 1 (20.0%), Outros ASCII 0 (0.0%), Não-ASCII 1 (20.0%).
