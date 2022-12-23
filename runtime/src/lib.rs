#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::Encode;
// A few exports that help ease life for downstream crates.
pub use frame_support::{
  construct_runtime,
  dispatch::{DispatchClass, GetDispatchInfo},
  parameter_types,
  traits::{ConstU8, Get, KeyOwnerProofSystem, Randomness, StorageInfo},
  weights::{
    constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
    IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
  },
  StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
use pallet_grandpa::{fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_core::U256;
use sp_runtime::FixedPointNumber;
use sp_runtime::Perquintill;

pub use pallet_key_registry;
pub use pallet_marketplace;
pub use pallet_meta_registry;
pub use pallet_social_network;
pub use pallet_template;

use frame_support::weights::ConstantMultiplier;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
  create_runtime_str, generic, impl_opaque_keys,
  traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConstU128, IdentifyAccount, NumberFor, Verify},
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature, SaturatedConversion,
};
pub use sp_runtime::{Perbill, Permill};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

pub mod currency {
  use super::Balance;
  pub const SUPPLY_FACTOR: Balance = 1;

  pub const ARIA: Balance = 1_000_000_000_000_000_000;

  pub const UNITS: Balance = 1_000_000_000_000_000_000;

  pub const EUROS: Balance = UNITS;
  pub const CENTS: Balance = EUROS / 100;
  pub const MILLICENTS: Balance = CENTS / 1_000;

  pub const WEI: Balance = 1;
  pub const KILOWEI: Balance = UNITS / 1_000_000_000_000_000;
  pub const MEGAWEI: Balance = UNITS / 1_000_000_000_000;
  pub const GIGAWEI: Balance = UNITS / 1_000_000_000;
  pub const MICROUNIT: Balance = UNITS / 1_000_000;
  pub const MILLIUNIT: Balance = UNITS / 1_000;
  pub const KILOUNIT: Balance = UNITS * 1_000;

  pub const EXISTENSIAL_DEPOSIT: Balance = CENTS;
  pub const TRANSACTION_BYTE_FEE: Balance = 1 * GIGAWEI * SUPPLY_FACTOR;
  pub const STORAGE_BYTE_FEE: Balance = 100 * MICROUNIT * SUPPLY_FACTOR;
  pub const WEIGHT_FEE: Balance = 50 * KILOWEI * SUPPLY_FACTOR;

  pub const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance) * EUROS + (bytes as Balance) * STORAGE_BYTE_FEE
  }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
  use super::*;
  pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

  /// Opaque block header type.
  pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
  /// Opaque block type.
  pub type Block = generic::Block<Header, UncheckedExtrinsic>;
  /// Opaque block identifier type.
  pub type BlockId = generic::BlockId<Block>;

  impl_opaque_keys! {
      pub struct SessionKeys {
          pub aura: Aura,
          pub grandpa: Grandpa,
      }
  }
}

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
  spec_name: create_runtime_str!("meta"),
  impl_name: create_runtime_str!("meta"),
  authoring_version: 1,
  // The version of the runtime specification. A full node will not attempt to use its native
  //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
  //   `spec_version`, and `authoring_version` are the same between Wasm and native.
  // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
  //   the compatible custom types.
  spec_version: 100,
  impl_version: 1,
  apis: RUNTIME_API_VERSIONS,
  transaction_version: 1,
  state_version: 0,
};

/// Maximum weight per block
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_div(2);

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
  NativeVersion {
    runtime_version: VERSION,
    can_author_with: Default::default(),
  }
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.ref_time() / GAS_PER_SECOND;

parameter_types! {
  pub BlockGasLimit: U256
    = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
  /// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
  /// than this will decrease the weight and more will increase.
  pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
  /// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
  /// change the fees more rapidly. This low value causes changes to occur slowly over time.
  pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
  /// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
  /// that combined with `AdjustmentVariable`, we can recover from the minimum.
  /// See `multiplier_can_grow_from_zero` in integration_tests.rs.
  /// This value is currently only used by pallet-transaction-payment as an assertion that the
  /// next multiplier is always > min value.
  pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
  /// Maximum multiplier. We pick a value that is expensive but not impossibly so; it should act
  /// as a safety net.
  pub MaximumMultiplier: Multiplier = Multiplier::from(100_000u128);
  pub WeightPerGas: Weight = Weight::from_ref_time(WEIGHT_PER_GAS);
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const NORMAL_WEIGHT: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_mul(3).saturating_div(4);
pub const EXTRINSIC_BASE_WEIGHT: Weight = Weight::from_ref_time(10000 * WEIGHT_PER_GAS);

pub struct RuntimeBlockWeights;
impl Get<frame_system::limits::BlockWeights> for RuntimeBlockWeights {
  fn get() -> frame_system::limits::BlockWeights {
    frame_system::limits::BlockWeights::builder()
      .for_class(DispatchClass::Normal, |weights| {
        weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT;
        weights.max_total = NORMAL_WEIGHT.into();
      })
      .for_class(DispatchClass::Operational, |weights| {
        weights.max_total = MAXIMUM_BLOCK_WEIGHT.into();
        weights.reserved = (MAXIMUM_BLOCK_WEIGHT - NORMAL_WEIGHT).into();
      })
      .avg_block_initialization(Perbill::from_percent(10))
      .build()
      .expect("Provided BlockWeight definitions are valid, qed")
  }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const BlockHashCount: BlockNumber = 2400;
    pub BlockWeights: frame_system::limits::BlockWeights = RuntimeBlockWeights::get();
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
  /// The basic call filter to use in dispatchable.
  type BaseCallFilter = frame_support::traits::Everything;
  /// Block & extrinsics weights: base values and limits.
  type BlockWeights = BlockWeights;
  /// The maximum length of a block (in bytes).
  type BlockLength = BlockLength;
  /// The identifier used to distinguish between accounts.
  type AccountId = AccountId;
  /// The aggregated dispatch type that is available for extrinsics.
  type RuntimeCall = RuntimeCall;
  /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
  type Lookup = AccountIdLookup<AccountId, ()>;
  /// The index type for storing how many extrinsics an account has signed.
  type Index = Index;
  /// The index type for blocks.
  type BlockNumber = BlockNumber;
  /// The type for hashing blocks and tries.
  type Hash = Hash;
  /// The hashing algorithm used.
  type Hashing = BlakeTwo256;
  /// The header type.
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  /// The ubiquitous event type.
  type RuntimeEvent = RuntimeEvent;
  /// The ubiquitous origin type.
  type RuntimeOrigin = RuntimeOrigin;
  /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
  type BlockHashCount = BlockHashCount;
  /// The weight of database operations that the runtime can invoke.
  type DbWeight = RocksDbWeight;
  /// Version of the runtime.
  type Version = Version;
  /// Converts a module to the index of the module in `construct_runtime!`.
  ///
  /// This type is being generated by `construct_runtime!`.
  type PalletInfo = PalletInfo;
  /// What to do if a new account is created.
  type OnNewAccount = ();
  /// What to do if an account is fully reaped from the system.
  type OnKilledAccount = ();
  /// The data to be stored in an account.
  type AccountData = pallet_balances::AccountData<Balance>;
  /// Weight information for the extrinsics of this pallet.
  type SystemWeightInfo = ();
  /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
  type SS58Prefix = SS58Prefix;
  /// The set code logic, just the default since we're not a parachain.
  type OnSetCode = ();
  type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub const MaxAuthorities: u32 = 32;
}

impl pallet_aura::Config for Runtime {
  type AuthorityId = AuraId;
  type DisabledValidators = ();
  type MaxAuthorities = MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type KeyOwnerProofSystem = ();
  type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
  type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;
  type HandleEquivocation = ();
  type WeightInfo = ();
  type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
  /// A timestamp: milliseconds since the unix epoch.
  type Moment = u64;
  type OnTimestampSet = Aura;
  type MinimumPeriod = MinimumPeriod;
  type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
  type MaxLocks = MaxLocks;
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type RuntimeEvent = RuntimeEvent;
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

pub struct LengthToFee;
impl WeightToFeePolynomial for LengthToFee {
  type Balance = Balance;

  fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
    smallvec![
      WeightToFeeCoefficient {
        degree: 1,
        coeff_frac: Perbill::zero(),
        coeff_integer: currency::TRANSACTION_BYTE_FEE,
        negative: false,
      },
      WeightToFeeCoefficient {
        degree: 3,
        coeff_frac: Perbill::zero(),
        coeff_integer: 1 * currency::SUPPLY_FACTOR,
        negative: false,
      },
    ]
  }
}

impl pallet_transaction_payment::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
  type OperationalFeeMultiplier = ConstU8<5>;
  type WeightToFee = ConstantMultiplier<Balance, ConstU128<1u128>>;
  type LengthToFee = ConstantMultiplier<Balance, ConstU128<1u128>>;
  type FeeMultiplierUpdate = ();
}

/// Parameterized slow adjusting fee updated based on
/// https://w3f-research.readthedocs.io/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism // editorconfig-checker-disable-line
///
/// The adjustment algorithm boils down to:
///
/// diff = (previous_block_weight - target) / maximum_block_weight
/// next_multiplier = prev_multiplier * (1 + (v * diff) + ((v * diff)^2 / 2))
/// assert(next_multiplier > min)
///     where: v is AdjustmentVariable
///            target is TargetBlockFullness
///            min is MinimumMultiplier
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier, MaximumMultiplier>;

impl pallet_sudo::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
}

// Aria pallets configuration

impl pallet_social_network::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type ConnectionRules = ();
}

impl pallet_marketplace::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
}

impl pallet_meta_registry::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type IssuerRules = ();
}

impl pallet_key_registry::Config for Runtime {
  type AuthorityId = pallet_key_registry::crypto::AuthorityId;
  type Call = RuntimeCall;
  type RuntimeEvent = RuntimeEvent;
}

/// Delete later
impl pallet_template::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
  RuntimeCall: From<LocalCall>,
{
  fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
    call: RuntimeCall,
    public: <Signature as sp_runtime::traits::Verify>::Signer,
    account: AccountId,
    index: Index,
  ) -> Option<(RuntimeCall, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
    let period = BlockHashCount::get() as u64;
    let current_block = System::block_number().saturated_into::<u64>().saturating_sub(1);
    let tip = 0;
    let extra: SignedExtra = (
      frame_system::CheckSpecVersion::<Runtime>::new(),
      frame_system::CheckTxVersion::<Runtime>::new(),
      frame_system::CheckGenesis::<Runtime>::new(),
      frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
      frame_system::CheckNonce::<Runtime>::from(index),
      frame_system::CheckWeight::<Runtime>::new(),
      pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
    );

    let raw_payload = SignedPayload::new(call, extra)
      .map_err(|e| {
        log::warn!("Unable to create signed payload: {:?}", e);
      })
      .ok()?;
    let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
    let address = account;
    let (call, extra, _) = raw_payload.deconstruct();
    Some((call, (sp_runtime::MultiAddress::Id(address), signature.into(), extra)))
  }
}

impl frame_system::offchain::SigningTypes for Runtime {
  type Public = <Signature as sp_runtime::traits::Verify>::Signer;
  type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
  RuntimeCall: From<C>,
{
  type OverarchingCall = RuntimeCall;
  type Extrinsic = UncheckedExtrinsic;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Config<T>},
        Grandpa: pallet_grandpa::{Pallet, Call, Storage, Config, Event},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
        SocialNetwork: pallet_social_network::{Pallet, Call, Event<T>},
        MetaRegistry: pallet_meta_registry::{Pallet,Event<T>},
        Marketplace: pallet_marketplace::{Pallet,Event<T>},
        TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
        KeyRegistry: pallet_key_registry::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
  frame_system::CheckSpecVersion<Runtime>,
  frame_system::CheckTxVersion<Runtime>,
  frame_system::CheckGenesis<Runtime>,
  frame_system::CheckEra<Runtime>,
  frame_system::CheckNonce<Runtime>,
  frame_system::CheckWeight<Runtime>,
  pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> fg_primitives::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;

            let mut list = Vec::<BenchmarkList>::new();

            list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
            list_benchmark!(list, extra, pallet_balances, Balances);
            list_benchmark!(list, extra, pallet_timestamp, Timestamp);
            list_benchmark!(list, extra, pallet_template, TemplateModule);

            let storage_info = AllPalletsWithSystem::storage_info();

            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // RuntimeEvent Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_template, TemplateModule);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
