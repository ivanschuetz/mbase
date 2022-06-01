use algonaut::transaction::SignedTransaction;
use anyhow::{anyhow, Result};
use std::fs;

const TEAL_PROJECT_PATH: &str = "../../teal";

// not rendered teal template (with placeholders)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TealSourceTemplate(pub Vec<u8>);

// regular teal source (not a template)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TealSource(pub Vec<u8>);

impl ToString for TealSource {
    fn to_string(&self) -> String {
        // unwrap: for now we'll assume that this struct is always initialized with valid utf-8,
        // TODO (low prio) actually ensure it
        String::from_utf8(self.0.clone()).unwrap()
    }
}

/// file_name without .teal
/// use this to debug with debug_teal_rendered
pub fn save_rendered_teal(file_name: &str, teal: TealSource) -> Result<()> {
    Ok(fs::write(
        in_teal_dir(&format!("teal_rendered/{file_name}.teal")),
        teal.0,
    )?)
}

// file_name without .teal
pub fn load_teal_template(file_name: &str) -> Result<TealSourceTemplate> {
    log::debug!("Loading teal template: {file_name}");
    load_file_bytes("teal_template", file_name).map(TealSourceTemplate)
}

// file_name without .teal
pub fn load_teal(file_name: &str) -> Result<TealSource> {
    load_file_bytes("teal", file_name).map(TealSource)
}

fn in_teal_dir(subpath: &str) -> String {
    format!("{TEAL_PROJECT_PATH}/{subpath}")
}

fn load_file_bytes(folder: &str, file_name: &str) -> Result<Vec<u8>> {
    Ok(fs::read(in_teal_dir(&format!(
        "{folder}/{file_name}.teal"
    )))?)
}

/// IMPORTANT: keys must not be contained in other keys (e.g: "precision" and "precision_square")
/// this currently uses normal text replacement so such keys will lead to errors
pub fn render_template_new(
    template: &TealSourceTemplate,
    key_values: &[(&str, &str)],
) -> Result<TealSource> {
    // this has not been tuned at all for performance - probably can be improved
    let mut teal_str = std::str::from_utf8(&template.0)?.to_owned();
    for (key, value) in key_values {
        teal_str = teal_str.replace(key, value);
    }
    Ok(TealSource(teal_str.as_bytes().to_vec()))
}

/// file_name without .teal
#[allow(dead_code)]
pub fn debug_teal(txs: &[SignedTransaction], file_name: &str) -> Result<()> {
    debug_teal_internal(txs, "teal", file_name)
}

/// file_name without .teal
/// separate folder for rendered templates to easily add to .gitignore
#[allow(dead_code)]
pub fn debug_teal_rendered(txs: &[SignedTransaction], file_name: &str) -> Result<()> {
    debug_teal_internal(txs, "teal_rendered", file_name)
}

/// file_name without .teal
#[allow(dead_code)]
fn debug_teal_internal(txs: &[SignedTransaction], folder: &str, file_name: &str) -> Result<()> {
    tealdbg::launch_default(txs, &in_teal_dir(&format!("{folder}/{file_name}.teal")))
        .map_err(|e| anyhow!(e))
}
