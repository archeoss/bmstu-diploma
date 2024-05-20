use std::process::{Command, ExitStatus};

const GIT_HASH_VAR: &str = "BOBFUSION_GIT_HASH";
const BRANCH_TAG_VAR: &str = "BOBFUSION_BUILD_BRANCH_TAG";

fn main() {
    std::env::var(GIT_HASH_VAR).is_err().then(set_commit_hash);
    std::env::var(BRANCH_TAG_VAR).is_err().then(set_branch_tag);
}

fn set_commit_hash() {
    set_env(GIT_HASH_VAR, "git", &["rev-parse", "HEAD"]);
}

fn set_branch_tag() {
    if !set_env(BRANCH_TAG_VAR, "git", &["describe", "--tags", "--abbrev=0"]).success() {
        set_env(
            BRANCH_TAG_VAR,
            "git",
            &["rev-parse", "--abbrev-ref", "HEAD"],
        );
    }
}

#[allow(clippy::unwrap_used)]
fn set_env(env_var: &str, cmd: &str, args: &[&str]) -> ExitStatus {
    let mut command = Command::new(cmd);
    command.args(args);
    let output = command.output().unwrap();
    if command.status().unwrap().success() {
        println!(
            "cargo:rustc-env={env_var}={}",
            String::from_utf8(output.stdout).unwrap()
        );
    };

    command.status().unwrap()
}
