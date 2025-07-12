#[cfg(all(feature = "std", feature = "metadata-hash"))]
fn main() {
	// Pass production genesis environment variables as compile-time constants
	pass_genesis_env_vars();
	
	substrate_wasm_builder::WasmBuilder::init_with_defaults()
		.enable_metadata_hash("UNIT", 12)
		.build();
}

#[cfg(all(feature = "std", not(feature = "metadata-hash")))]
fn main() {
	// Pass production genesis environment variables as compile-time constants
	pass_genesis_env_vars();
	
	substrate_wasm_builder::WasmBuilder::build_using_defaults();
}

/// The wasm builder is deactivated when compiling
/// this crate for wasm to speed up the compilation.
#[cfg(not(feature = "std"))]
fn main() {}

/// Pass production genesis environment variables as rustc-env for compile-time access
/// Variables are MANDATORY for production builds but optional for development builds
fn pass_genesis_env_vars() {
	// All required environment variables for production builds
	// These MUST be set for production builds to prevent accidental use of test keys
	const REQUIRED_VARS: &[&str] = &[
		"SUDO_SS58",           // Production sudo account
		"VAL1_AURA_PUB",       // Validator 1 AURA public key
		"VAL1_GRANDPA_PUB",    // Validator 1 GRANDPA public key  
		"VAL1_STASH_SS58",     // Validator 1 stash account
		"VAL2_AURA_PUB",       // Validator 2 AURA public key
		"VAL2_GRANDPA_PUB",    // Validator 2 GRANDPA public key
		"VAL2_STASH_SS58",     // Validator 2 stash account
	];

	let mut missing_vars = Vec::new();
	let mut found_vars = Vec::new();
	
	// Check all required variables first
	for &var_name in REQUIRED_VARS {
		match std::env::var(var_name) {
			Ok(value) => {
				// Forward to the compiler so `env!(var_name)` works in the runtime
				println!("cargo:rustc-env={}={}", var_name, value);
				println!("cargo:rerun-if-env-changed={}", var_name);
				found_vars.push(var_name);
			}
			Err(_) => {
				missing_vars.push(var_name);
			}
		}
	}
	
	// Determine build mode based on environment variables presence
	if found_vars.is_empty() {
		// Development/staging build - no production variables set
		println!("🧪 Development/staging build detected (no production env vars)");
		println!("📋 Development and staging presets will use Alice/Bob hardcoded keys");
		println!("⚠️  Production preset will NOT be available for this build");
	} else if missing_vars.is_empty() {
		// Production build - all variables set
		println!("✅ Production build: All {} required environment variables are set", REQUIRED_VARS.len());
		println!("🏭 Production preset will use Vault-sourced public keys");
	} else {
		// Partial production build - some variables set, some missing (ERROR)
		eprintln!("🚨 PRODUCTION BUILD ERROR: Partial environment variable configuration detected!");
		eprintln!("   Found {} variables, missing {} variables", found_vars.len(), missing_vars.len());
		eprintln!();
		eprintln!("   ✅ Found:");
		for var in &found_vars {
			eprintln!("      • {}", var);
		}
		eprintln!();
		eprintln!("   ❌ Missing:");
		for var in &missing_vars {
			eprintln!("      • {}", var);
		}
		eprintln!();
		eprintln!("💡 To fix this:");
		eprintln!("   • For production: Set ALL 7 variables");
		eprintln!("   • For development: Set NONE of them (use Alice/Bob presets)");
		eprintln!("   • This prevents accidental mixing of production and test keys");
		eprintln!();
		panic!("Build halted due to partial production environment variable configuration");
	}
}
