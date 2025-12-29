use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=RUME_COMMIT_HASH");

    if let Ok(commit_hash) = env::var("RUME_COMMIT_HASH") {
        let commit_hash = commit_hash.trim();
        if !commit_hash.is_empty() {
            let commit_hash_short = &commit_hash[..8.min(commit_hash.len())];
            println!("cargo:rustc-env=RUME_COMMIT_HASH={commit_hash_short}");
            return;
        }
    }

    println!("cargo:rustc-env=RUME_COMMIT_HASH=unknown");
}
