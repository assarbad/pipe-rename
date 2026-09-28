mod run;

use run::{run_with_env, TestCase, EMPTY};

#[test]
fn test_no_input() -> anyhow::Result<()> {
    let assert = run_with_env(EMPTY, EMPTY, true)?;
    assert
        .failure()
        .stdout("")
        .stderr("Error: No input files on stdin or as args.\n");
    Ok(())
}

#[test]
fn test_no_replacements() -> anyhow::Result<()> {
    let assert = run_with_env(&["test-1"], &["test-1"], true)?;
    assert
        .failure()
        .stdout("")
        .stderr("Error: No replacements found.\n");
    Ok(())
}

#[test]
fn test_unequal_lines() -> anyhow::Result<()> {
    let assert = run_with_env(&["test-1", "test-2"], &["test-3"], true)?;
    assert
        .failure()
        .stdout("")
        .stderr("Error: Unequal number of files.\n");
    Ok(())
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn test_duplicate_input() {
    let _ = run_with_env(&["aaa", "aaa"], &["bbb", "ccc"], true);
}

#[test]
fn test_duplicate_output() -> anyhow::Result<()> {
    let assert = run_with_env(&["aaa", "bbb"], &["ccc", "ccc"], true)?;
    assert
        .failure()
        .stdout("")
        .stderr("Error: Duplicate output files.\n");
    Ok(())
}

#[test]
fn test_rename() -> anyhow::Result<()> {
    let mut test_case = TestCase::new()?;
    test_case.replace("1", "2")?;
    test_case.replace("2", "3")?;

    let assert = test_case.run()?;
    assert
        .failure()
        .stderr("Error: Refusing to overwrite existing files.\n");

    // TODO: assert stdout
    // TODO: assert that nothing has been renamed

    Ok(())
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn test_dot() {
    let _ = run_with_env(&["."], &["."], true);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn test_dotdot() {
    let _ = run_with_env(&[".."], &[".."], true);
}

#[test]
fn test_new_dir() -> anyhow::Result<()> {
    let mut test_case = TestCase::new()?;
    test_case.replace("1", "a/1")?;

    let assert = test_case.run()?;

    // The OS's error text for a missing path component differs across
    // platforms (ENOENT vs ERROR_PATH_NOT_FOUND), so derive the expected
    // message from a real io::Error of the same kind instead of hardcoding
    // a platform-specific string.
    let io_err =
        std::fs::File::open(std::path::Path::new("definitely/does/not/exist")).unwrap_err();
    assert.failure().stderr(format!("Error: {io_err}\n"));

    Ok(())
}
