use super::contract::Contract;
use crate::{
    api::version::{Version, VersionedTealSourceTemplate, Versions},
    teal::load_teal_template,
};
use anyhow::Result;

pub struct TealFileLoader {}

impl TealFileLoader {
    pub fn last_versions(&self) -> Versions {
        Versions {
            app_approval: Version(1),
            app_clear: Version(1),
            customer_escrow: Version(1),
        }
    }

    pub fn template(
        &self,
        contract: Contract,
        version: Version,
    ) -> Result<Option<VersionedTealSourceTemplate>> {
        match contract {
            Contract::DaoCustomer => dao_customer_teal(version),
            Contract::DaoAppApproval => dao_app_approval_teal(version),
            Contract::DaoAppClear => dao_app_clear_teal(version),
        }
    }
}

fn dao_customer_teal(version: Version) -> Result<Option<VersionedTealSourceTemplate>> {
    Ok(match version.0 {
        1 => Some(load_versioned_teal_template(version, "customer_escrow")?),
        _ => None,
    })
}

fn dao_app_approval_teal(version: Version) -> Result<Option<VersionedTealSourceTemplate>> {
    Ok(match version.0 {
        1 => Some(load_versioned_teal_template(version, "dao_app_approval")?),
        _ => None,
    })
}

fn dao_app_clear_teal(version: Version) -> Result<Option<VersionedTealSourceTemplate>> {
    Ok(match version.0 {
        1 => Some(load_versioned_teal_template(version, "dao_app_clear")?),
        _ => None,
    })
}

fn load_versioned_teal_template(
    version: Version,
    file_name: &str,
) -> Result<VersionedTealSourceTemplate> {
    Ok(VersionedTealSourceTemplate {
        version,
        template: load_teal_template(file_name)?,
    })
}
