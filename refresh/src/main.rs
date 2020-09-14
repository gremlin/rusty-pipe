use std::collections::HashSet;
use std::path::PathBuf;

use cargo_lock::{Lockfile, Package};

#[derive(Debug)]
struct PackageTracker<'a> {
    name: &'a str,
    package: &'a Package,
}

impl PartialEq for PackageTracker<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PackageTracker<'_> {}

impl std::cmp::Ord for PackageTracker<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(other.name)
    }
}

impl std::cmp::PartialOrd for PackageTracker<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for PackageTracker<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(std::env::var("AGENT")?);

    let mut lock_path = base_path.clone();
    lock_path.push("Cargo.lock");

    let lockfile = Lockfile::load(lock_path).unwrap();
    println!("number of dependencies: {}", lockfile.packages.len());
    let mut newest = HashSet::<PackageTracker>::new();
    for package in &lockfile.packages {
        let candidate = PackageTracker {
            name: package.name.as_str(),
            package,
        };
        if let Some(existing) = newest.get(&candidate) {
            // rmv println!("{} is a duplicate.", package.name);
            if candidate.package.version > existing.package.version {
                // rmv println!("{:?} > {:?}", candidate.package.version, existing.package.version);
                newest.replace(candidate);
            }
        }
        else {
            newest.insert(candidate);
        }
    }
    let mut newest: Vec<PackageTracker> = newest.drain().collect();
    newest.sort();
    for tracker in &newest {
        if !tracker.package.name.as_str().starts_with("gremlin")
            && !tracker.package.name.as_str().starts_with("bright-light")
            && !tracker.package.name.as_str().starts_with("fwpmu")
        {
            println!("{} = \"{}.{}.{}\"",
                tracker.package.name,
                tracker.package.version.major, tracker.package.version.minor, tracker.package.version.patch);
            // cargo-lock = "4.0.1"
        }
    }

/*
    for (key, value) in std::env::vars() {
		println!("{}={}", key, value);
	}
*/
    Ok(())
}
