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
		// Internal addresses (for nodes inside Kubernetes)
		"/dns/fennel-bootnode-1.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooWDCZGrnJhsgWJtDcs6eZc1hUBrVj5QqkEWggAkNVowRAi".parse().unwrap(),
		"/dns/fennel-bootnode-2.fennel-staging.svc.cluster.local/tcp/30333/p2p/12D3KooWDbfFv6oepAXmQaiwFaVjD9g7AxowQ8LQdWTcVYpKhnAx".parse().unwrap(),

		// External addresses (for validators outside Kubernetes)
		// TODO: Uncomment and replace with actual LoadBalancer IPs when available
		// "/ip4/EXTERNAL_IP_1/tcp/30333/p2p/12D3KooWDCZGrnJhsgWJtDcs6eZc1hUBrVj5QqkEWggAkNVowRAi".parse().unwrap(),
		// "/ip4/EXTERNAL_IP_2/tcp/30333/p2p/12D3KooWDbfFv6oepAXmQaiwFaVjD9g7AxowQ8LQdWTcVYpKhnAx".parse().unwrap(),
	])
	.build())
}
