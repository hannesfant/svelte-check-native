//! Language-server diagnostic-fixture parity suite.
//!
//! Drives `node run.cjs` against
//! `language-tools/packages/language-server/test/plugins/typescript/features/
//! diagnostics/fixtures/`, asserting that our binary's diagnostics match
//! upstream's `expectedv2.json` (or `expected_svelte_5.json` when present)
//! on `(file, line, character, code)` — the lossy-compare gate per
//! `notes/PARITY_TESTING_PLAN.md` P1.
//!
//! Same harness shape as `bug_fixtures.rs`: the runner does the work, we
//! just assert clean exit + "0 failed" tail. Skip-list (with reasons) is
//! enforced inside the runner so the count stays explicit.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::path::PathBuf;
use std::process::Command;

#[test]
fn ls_diagnostics_suite() {
    let bin = env!("CARGO_BIN_EXE_svelte-check-native");
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let runner = crate_dir.join("tests/ls_diagnostics/run.cjs");
    assert!(runner.exists(), "runner not found at {}", runner.display());

    let fixtures_root = crate_dir
        .join("../../language-tools/packages/language-server/test/plugins/typescript/features/diagnostics/fixtures");
    let fixtures = match fixtures_root.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            eprintln!(
                "SKIP: language-tools submodule fixtures not found at {}. \
                 Run `git submodule update --init --recursive` first.",
                fixtures_root.display()
            );
            return;
        }
    };

    let output = match Command::new("node")
        .arg(runner.to_str().expect("runner path is utf-8"))
        .env("SVELTE_CHECK_BIN", bin)
        .env(
            "FIXTURES_DIR",
            fixtures.to_str().expect("fixtures dir is utf-8"),
        )
        .output()
    {
        Ok(output) => output,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            panic!("`node` must be on PATH to run LS diagnostic fixtures ({err})");
        }
        Err(err) => panic!("failed to spawn node: {err}"),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprintln!("----- node stdout -----\n{stdout}");
    eprintln!("----- node stderr -----\n{stderr}");

    let tail = stdout.lines().last().unwrap_or("<no output>");

    assert!(
        output.status.success() && tail.contains("0 failed"),
        "LS diagnostic fixtures suite did not pass cleanly.\n\
         exit: {:?}\n\
         tail: {tail}",
        output.status.code()
    );
}
