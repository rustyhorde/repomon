//! branch related operations
use error::Result;
use git2::{BranchType, Oid, Repository};

/// Get an OID for a branch given a name.
pub fn get_oid_by_branch_name(
    repo: &Repository,
    branch_name: &str,
    branch_type: Option<BranchType>,
) -> Result<Oid> {
    let oid: Vec<Oid> = repo.branches(branch_type)?
        .filter_map(|branch_res| {
            if let Ok((branch, _)) = branch_res {
                if let Ok(Some(bn)) = branch.name() {
                    if bn == branch_name {
                        branch.get().target()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if oid.len() == 1 {
        Ok(oid[0])
    } else {
        Err("Multiple OIDs found".into())
    }
}
