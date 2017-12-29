// Copyright (c) 2017 repomon developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `repomon` runtime
use branch;
use clap::{App, Arg};
use error::Result;
use git2::{self, BranchType, CredentialType, FetchOptions, FetchPrune, Progress, RemoteCallbacks,
           Repository, Status, StatusOptions};
use git2::Cred;
use std::io::{self, Write};

/// Check credentials for connecting to remote.
fn check_creds(
    _url: &str,
    username: Option<&str>,
    cred_type: CredentialType,
) -> ::std::result::Result<Cred, git2::Error> {
    if cred_type.contains(git2::SSH_KEY) {
        Cred::ssh_key_from_agent(username.unwrap_or(""))
    } else {
        Err(git2::Error::from_str("Unable to authenticate"))
    }
}

/// Progress remote callback.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn progress(progress: Progress) -> bool {
    writeln!(io::stdout(), "{}", progress.received_objects()).unwrap_or(());
    true
}

/// Side band remote callback.
fn side_band(text: &[u8]) -> bool {
    writeln!(io::stdout(), "{}", String::from_utf8_lossy(text)).unwrap_or(());
    true
}

/// Convert a status to a composite string.
fn status_out(status: &Status, out: &mut String) -> Result<()> {
    let mut statuses = Vec::new();

    if status.contains(git2::STATUS_INDEX_NEW) {
        statuses.push("idx-new");
    }

    if status.contains(git2::STATUS_INDEX_MODIFIED) {
        statuses.push("idx-modified");
    }

    if status.contains(git2::STATUS_INDEX_DELETED) {
        statuses.push("idx-deleted");
    }

    if status.contains(git2::STATUS_INDEX_TYPECHANGE) {
        statuses.push("idx-typechange");
    }

    if status.contains(git2::STATUS_INDEX_RENAMED) {
        statuses.push("idx-renamed");
    }

    if status.contains(git2::STATUS_WT_NEW) {
        statuses.push("wt-new");
    }

    if status.contains(git2::STATUS_WT_MODIFIED) {
        statuses.push("wt-modified");
    }

    if status.contains(git2::STATUS_WT_DELETED) {
        statuses.push("wt-deleted");
    }

    if status.contains(git2::STATUS_WT_TYPECHANGE) {
        statuses.push("wt-typechange");
    }

    if status.contains(git2::STATUS_WT_RENAMED) {
        statuses.push("wt-renamed");
    }

    // if status.contains(git2::STATUS_WT_UNREADABLE) {
    //     statuses.push("wt-unreadable");
    // }

    if status.contains(git2::STATUS_IGNORED) {
        statuses.push("ignored");
    }

    if status.contains(git2::STATUS_CONFLICTED) {
        statuses.push("conflicted");
    }

    out.push_str(&statuses.join(","));
    Ok(())
}

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Monitors a set of repositories for changes to branches")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true)
                .default_value(".repomon.toml"),
        )
        .arg(Arg::with_name("repo").default_value("."))
        .get_matches();

    let repo = Repository::discover(matches.value_of("repo").ok_or("")?)?;
    let mut status_opts = StatusOptions::new();
    status_opts.include_ignored(false);
    status_opts.include_untracked(true);

    let statuses = repo.statuses(Some(&mut status_opts))?;

    let mut rcb = RemoteCallbacks::new();
    rcb.transfer_progress(progress);
    rcb.sideband_progress(side_band);
    rcb.credentials(check_creds);

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(rcb);
    fetch_opts.prune(FetchPrune::On);

    let master_oid = branch::get_oid_by_branch_name(&repo, "master", Some(BranchType::Local))?;
    let origin_master_oid =
        branch::get_oid_by_branch_name(&repo, "origin/master", Some(BranchType::Remote))?;
    let (ahead, behind) = repo.graph_ahead_behind(master_oid, origin_master_oid)?;

    if ahead > 0 {
        writeln!(
            io::stdout(),
            "Your branch is ahead of '{}' by {} commit(s)",
            "origin/master",
            ahead
        )?;
    } else if behind > 0 {
        writeln!(
            io::stdout(),
            "Your branch is behind '{}' by {} commit(s)",
            "origin/master",
            behind
        )?;
    } else {
        writeln!(
            io::stdout(),
            "Your branch is up to date with '{}'",
            "origin/master"
        )?;
    }

    for branch_res in repo.branches(None)? {
        let (branch, _) = branch_res?;
        writeln!(io::stdout(), "Branch: {}", branch.name()?.ok_or("No name")?)?;
        writeln!(io::stdout(), "Branch is head: {}", branch.is_head())?;
        writeln!(
            io::stdout(),
            "Branch OID: {}",
            branch.get().target().ok_or("No OID")?
        )?;
    }
    for status in statuses.iter() {
        let mut status_str = String::new();
        status_out(&status.status(), &mut status_str)?;
        writeln!(
            io::stdout(),
            "Path: {}, {}",
            status.path().unwrap_or("''"),
            status_str
        )?;
    }

    for remote_opt in repo.remotes()?.iter() {
        if let Some(remote) = remote_opt {
            repo.find_remote(remote)?
                .fetch(&["master"], Some(&mut fetch_opts), None)?;
        }
    }
    Ok(0)
}
