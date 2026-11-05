# RULES FOR EVERY AGENT: customizing an existing open-source project

You are improving a FORK of an existing, working open-source project. The project is the
baseline: change only what the task needs.

1. **Minimal, surgical changes.** Do not rewrite, reformat or reorganize code the task does not
   need. Keep the project's own style, naming, structure and libraries.
2. **Keep it building and its tests passing.** The project's own build and test commands run
   after you. Never delete, skip or weaken existing tests.
3. **Keep license and copyright headers** exactly as they are. Respect the project's license.
4. **Add tests** for new behavior when the project has a test suite.
5. **Do not touch** `forja.toml`, `.forja-projeto/`, `.github/`, CI files or release metadata.
6. **No new dependencies** unless the task explicitly requires one.
7. **Whole files.** Return the complete new content of every file you change.
