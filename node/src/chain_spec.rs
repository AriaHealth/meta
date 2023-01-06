use meta_runtime::AccountId;
use meta_runtime::AuraConfig;
use meta_runtime::BalancesConfig;
use meta_runtime::GenesisConfig;
use meta_runtime::GrandpaConfig;
use meta_runtime::KeyRegistryConfig;
use meta_runtime::MetaRegistryConfig;
use meta_runtime::Signature;
use meta_runtime::SocialNetworkConfig;
use meta_runtime::SudoConfig;
use meta_runtime::SystemConfig;
use meta_runtime::WASM_BINARY;
use pallet_key_registry::types::KeyName;
use pallet_key_registry::types::KeyType;
use pallet_key_registry::types::OracleURI;
use pallet_meta_registry::types::DeliveryNetworkURI;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::sr25519;
use sp_core::Pair;
use sp_core::Public;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::Verify;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
  TPublic::Pair::from_string(&format!("//{}", seed), None)
    .expect("static values are valid; qed")
    .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
  AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
  AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
  (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  Ok(ChainSpec::from_genesis(
    // Name
    "Development",
    // ID
    "dev",
    ChainType::Development,
    move || {
      genesis(
        wasm_binary,
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
          get_account_id_from_seed::<sr25519::Public>("Alice"),
          get_account_id_from_seed::<sr25519::Public>("Bob"),
          get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
          get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        true,
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        "http://0.0.0.0:8888".to_string(),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        [0_u8, 0_u8],
        "SHIELDING".into(),
        "http://0.0.0.0:8888".to_string(),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
      )
    },
    // Bootnodes
    vec![],
    // Telemetry
    None,
    // Protocol ID
    None,
    None,
    // Properties
    None,
    // Extensions
    None,
  ))
}

pub fn local_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  Ok(ChainSpec::from_genesis(
    // Name
    "Local Testnet",
    // ID
    "local_testnet",
    ChainType::Local,
    move || {
      genesis(
        wasm_binary,
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
          get_account_id_from_seed::<sr25519::Public>("Alice"),
          get_account_id_from_seed::<sr25519::Public>("Bob"),
          get_account_id_from_seed::<sr25519::Public>("Charlie"),
          get_account_id_from_seed::<sr25519::Public>("Dave"),
          get_account_id_from_seed::<sr25519::Public>("Eve"),
          get_account_id_from_seed::<sr25519::Public>("Ferdie"),
          get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
          get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
          get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
          get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
          get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
          get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        true,
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        "http://0.0.0.0:8888".to_string(),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        [0_u8, 0_u8],
        "SHIELDING".into(),
        "http://0.0.0.0:8888".to_string(),
        get_account_id_from_seed::<sr25519::Public>("Alice"),
      )
    },
    // Bootnodes
    vec![],
    // Telemetry
    None,
    // Protocol ID
    None,
    // Properties
    None,
    None,
    // Extensions
    None,
  ))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
  wasm_binary: &[u8],
  initial_authorities: Vec<(AuraId, GrandpaId)>,
  root_key: AccountId,
  endowed_accounts: Vec<AccountId>,
  _enable_println: bool,
  delivery_network_id: AccountId,
  delivery_network_uri: String,
  main_custodian_id: AccountId,
  key_type: KeyType,
  key_name: KeyName,
  key_oracle_uri: String,
  key_oracle_id: AccountId,
) -> GenesisConfig {
  GenesisConfig {
    system: SystemConfig {
      // Add Wasm runtime to storage.
      code: wasm_binary.to_vec(),
    },
    balances: BalancesConfig {
      // Configure endowed accounts with initial balance of 1 << 60.
      balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
    },
    aura: AuraConfig {
      authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
    },
    grandpa: GrandpaConfig {
      authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
    },
    sudo: SudoConfig {
      // Assign network admin rights.
      key: Some(root_key),
    },
    transaction_payment: Default::default(),
    meta_registry: MetaRegistryConfig {
      delivery_network_id: Some(delivery_network_id),
      delivery_network_uri: Some(DeliveryNetworkURI::try_from(delivery_network_uri.into_bytes()).unwrap()),
    },
    key_registry: KeyRegistryConfig {
      key_type: Some(key_type),
      key_name: Some(key_name),
      key_oracle_uri: Some(OracleURI::try_from(key_oracle_uri.into_bytes()).unwrap()),
      key_oracle_id: Some(key_oracle_id),
    },
    social_network: SocialNetworkConfig {
      main_custodian_id: Some(main_custodian_id),
    },
  }
}
