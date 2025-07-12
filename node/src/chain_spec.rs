use sc_service::ChainType;
use fennel_node_runtime::WASM_BINARY;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

pub fn development_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
	.build())
}

pub fn local_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
	.build())
}

pub fn staging_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Staging wasm not available".to_string())?,
		None,
	)
	.with_name("Fennel Staging")
	.with_id("staging")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name("staging")
	.with_boot_nodes(vec![
		// Official Fennel network bootnodes
		"/dns4/bootnode1.fennel.network/tcp/30333/p2p/12D3KooWS84f71ufMQRsm9YWynfK5Zxa6iSooStJECnAT3RBVVxz".parse().unwrap(),
		"/dns4/bootnode2.fennel.network/tcp/30333/p2p/12D3KooWLWzcGVuLycfL1W83yc9S4UmVJ8qBd4Rk5mS6RJ4Bh7Su".parse().unwrap(),
	])
	.build())
}

pub fn production_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Production wasm not available".to_string())?,
		None,
	)
	.with_name("Fennel Production")
	.with_id("fennel_production")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_preset_name("production")
	.with_boot_nodes(vec![
		// Production bootnodes - will be populated dynamically in CI
		// These placeholder addresses will be replaced with derived peer IDs
		"/dns4/bootnode1.fennel.network/tcp/30333/p2p/12D3KooWS84f71ufMQRsm9YWynfK5Zxa6iSooStJECnAT3RBVVxz".parse().unwrap(),
		"/dns4/bootnode2.fennel.network/tcp/30333/p2p/12D3KooWLWzcGVuLycfL1W83yc9S4UmVJ8qBd4Rk5mS6RJ4Bh7Su".parse().unwrap(),
	])
	.build())
}
