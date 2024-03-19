//! tx log
//! tools for parsing a TX record from api

use std::path::Path;

use diem_json_rpc::views::TransactionView;

fn load_tx_json(path: &Path) -> anyhow::Result<()> {
    let json_string = std::fs::read_to_string(path)?;
    let txs: Vec<TransactionView> = serde_json::from_str(&json_string)?;
    dbg!(&txs);
    Ok(())
}

pub(crate) fn debug_parse_manifest() {
    let this_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let this_path = this_path.join("fixtures/v5_sample_tx_log.json");
    load_tx_json(&this_path).expect("parse json");
}

#[test]
fn test_parse_manifest() {
  debug_parse_manifest()
}
