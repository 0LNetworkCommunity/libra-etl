//! tx log
//! tools for parsing a TX record from api

use std::path::Path;

use diem_json_rpc::views::TransactionView;

pub fn load_tx_json_from_file(path: &Path) -> anyhow::Result<Vec<TransactionView>> {
    let json_string = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json_string)?)
}

pub fn split_tx_vertex_edge(tx_view: &TransactionView) -> anyhow::Result<()> {
  match &tx_view.transaction {
    diem_json_rpc::views::TransactionDataView::UserTransaction { sender, script, .. } => {

      // TODO: parse the script
      dbg!(&sender);
      dbg!(&script);

      // TODO: make a vertex

      // TODO: make an edge

    },
    _ => {
      println!("{:#?}", &tx_view.transaction);
      println!("...not a user tx");
    },
  };

  Ok(())
}

pub(crate) fn debug_parse_manifest() {
    let this_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let this_path = this_path.join("fixtures/v5_sample_tx_log.json");
    let tx_vec = load_tx_json_from_file(&this_path).expect("parse json");
    split_tx_vertex_edge(&tx_vec.get(0).unwrap()).unwrap();
}

#[test]
fn test_parse_manifest() {
  debug_parse_manifest()
}
