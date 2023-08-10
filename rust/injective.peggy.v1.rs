// @generated
/// Attestation is an aggregate of `claims` that eventually becomes `observed` by
/// all orchestrators
/// EVENT_NONCE:
/// EventNonce a nonce provided by the peggy contract that is unique per event
/// fired These event nonces must be relayed in order. This is a correctness
/// issue, if relaying out of order transaction replay attacks become possible
/// OBSERVED:
/// Observed indicates that >67% of validators have attested to the event,
/// and that the event should be executed by the peggy state machine
///
/// The actual content of the claims is passed in with the transaction making the
/// claim and then passed through the call stack alongside the attestation while
/// it is processed the key in which the attestation is stored is keyed on the
/// exact details of the claim but there is no reason to store those exact
/// details becuause the next message sender will kindly provide you with them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestation {
    #[prost(bool, tag="1")]
    pub observed: bool,
    #[prost(string, repeated, tag="2")]
    pub votes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(message, optional, tag="4")]
    pub claim: ::core::option::Option<::prost_types::Any>,
}
/// ERC20Token unique identifier for an Ethereum ERC20 token.
/// CONTRACT:
/// The contract address on ETH of the token, this could be a Cosmos
/// originated token, if so it will be the ERC20 address of the representation
/// (note: developers should look up the token symbol using the address on ETH to
/// display for UI)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
/// ClaimType is the cosmos type of an event from the counterpart chain that can
/// be handled
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClaimType {
    Unknown = 0,
    Deposit = 1,
    Withdraw = 2,
    Erc20Deployed = 3,
    ValsetUpdated = 4,
}
impl ClaimType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClaimType::Unknown => "CLAIM_TYPE_UNKNOWN",
            ClaimType::Deposit => "CLAIM_TYPE_DEPOSIT",
            ClaimType::Withdraw => "CLAIM_TYPE_WITHDRAW",
            ClaimType::Erc20Deployed => "CLAIM_TYPE_ERC20_DEPLOYED",
            ClaimType::ValsetUpdated => "CLAIM_TYPE_VALSET_UPDATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLAIM_TYPE_UNKNOWN" => Some(Self::Unknown),
            "CLAIM_TYPE_DEPOSIT" => Some(Self::Deposit),
            "CLAIM_TYPE_WITHDRAW" => Some(Self::Withdraw),
            "CLAIM_TYPE_ERC20_DEPLOYED" => Some(Self::Erc20Deployed),
            "CLAIM_TYPE_VALSET_UPDATED" => Some(Self::ValsetUpdated),
            _ => None,
        }
    }
}
/// OutgoingTxBatch represents a batch of transactions going from Peggy to ETH
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutgoingTxBatch {
    #[prost(uint64, tag="1")]
    pub batch_nonce: u64,
    #[prost(uint64, tag="2")]
    pub batch_timeout: u64,
    #[prost(message, repeated, tag="3")]
    pub transactions: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub block: u64,
}
/// OutgoingTransferTx represents an individual send from Peggy to ETH
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutgoingTransferTx {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub dest_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub erc20_token: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag="5")]
    pub erc20_fee: ::core::option::Option<Erc20Token>,
}
/// SignType defines messages that have been signed by an orchestrator
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignType {
    Unknown = 0,
    OrchestratorSignedMultiSigUpdate = 1,
    OrchestratorSignedWithdrawBatch = 2,
}
impl SignType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignType::Unknown => "SIGN_TYPE_UNKNOWN",
            SignType::OrchestratorSignedMultiSigUpdate => "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE",
            SignType::OrchestratorSignedWithdrawBatch => "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGN_TYPE_UNKNOWN" => Some(Self::Unknown),
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_MULTI_SIG_UPDATE" => Some(Self::OrchestratorSignedMultiSigUpdate),
            "SIGN_TYPE_ORCHESTRATOR_SIGNED_WITHDRAW_BATCH" => Some(Self::OrchestratorSignedWithdrawBatch),
            _ => None,
        }
    }
}
/// BridgeValidator represents a validator's ETH address and its power
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BridgeValidator {
    #[prost(uint64, tag="1")]
    pub power: u64,
    #[prost(string, tag="2")]
    pub ethereum_address: ::prost::alloc::string::String,
}
/// Valset is the Ethereum Bridge Multsig Set, each peggy validator also
/// maintains an ETH key to sign messages, these are used to check signatures on
/// ETH because of the significant gas savings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Valset {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(string, tag="4")]
    pub reward_amount: ::prost::alloc::string::String,
    /// the reward token in it's Ethereum hex address representation
    #[prost(string, tag="5")]
    pub reward_token: ::prost::alloc::string::String,
}
/// LastObservedEthereumBlockHeight stores the last observed
/// Ethereum block height along with the Cosmos block height that
/// it was observed at. These two numbers can be used to project
/// outward and always produce batches with timeouts in the future
/// even if no Ethereum block height has been relayed for a long time
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastObservedEthereumBlockHeight {
    #[prost(uint64, tag="1")]
    pub cosmos_block_height: u64,
    #[prost(uint64, tag="2")]
    pub ethereum_block_height: u64,
}
/// LastClaimEvent stores last claim event details of validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastClaimEvent {
    #[prost(uint64, tag="1")]
    pub ethereum_event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub ethereum_event_height: u64,
}
/// This records the relationship between an ERC20 token and the denom
/// of the corresponding Cosmos originated asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20ToDenom {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttestationObserved {
    #[prost(enumeration="ClaimType", tag="1")]
    pub attestation_type: i32,
    #[prost(string, tag="2")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub bridge_chain_id: u64,
    #[prost(bytes="vec", tag="4")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBridgeWithdrawCanceled {
    #[prost(string, tag="1")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bridge_chain_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOutgoingBatch {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub batch_nonce: u64,
    #[prost(uint64, tag="4")]
    pub batch_timeout: u64,
    #[prost(uint64, repeated, tag="5")]
    pub batch_tx_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOutgoingBatchCanceled {
    #[prost(string, tag="1")]
    pub bridge_contract: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag="3")]
    pub batch_id: u64,
    #[prost(uint64, tag="4")]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventValsetUpdateRequest {
    #[prost(uint64, tag="1")]
    pub valset_nonce: u64,
    #[prost(uint64, tag="2")]
    pub valset_height: u64,
    #[prost(message, repeated, tag="3")]
    pub valset_members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag="4")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub reward_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetOrchestratorAddresses {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub operator_eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventValsetConfirm {
    #[prost(uint64, tag="1")]
    pub valset_nonce: u64,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSendToEth {
    #[prost(uint64, tag="1")]
    pub outgoing_tx_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bridge_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConfirmBatch {
    #[prost(uint64, tag="1")]
    pub batch_nonce: u64,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttestationVote {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(bytes="vec", tag="2")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub voter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDepositClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub event_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub orchestrator_address: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventWithdrawClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub event_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub batch_nonce: u64,
    #[prost(string, tag="5")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventErc20DeployedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub event_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub decimals: u64,
    #[prost(string, tag="9")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventValsetUpdateClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub event_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub attestation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub valset_nonce: u64,
    #[prost(message, repeated, tag="5")]
    pub valset_members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag="6")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCancelSendToEth {
    #[prost(uint64, tag="1")]
    pub outgoing_tx_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSubmitBadSignatureEvidence {
    #[prost(string, tag="1")]
    pub bad_eth_signature: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bad_eth_signature_subject: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventValidatorSlash {
    #[prost(int64, tag="1")]
    pub power: i64,
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub consensus_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub moniker: ::prost::alloc::string::String,
}
// Params represent the peggy genesis and store parameters
// peggy_id:
// a random 32 byte value to prevent signature reuse, for example if the
// cosmos validators decided to use the same Ethereum keys for another chain
// also running Peggy we would not want it to be possible to play a deposit
// from chain A back on chain B's peggy. This value IS USED ON ETHEREUM so
// it must be set in your genesis.json before launch and not changed after
// deploying Peggy
//
// contract_hash:
// the code hash of a known good version of the Peggy contract
// solidity code. This can be used to verify the correct version
// of the contract has been deployed. This is a reference value for
// goernance action only it is never read by any Peggy code
//
// bridge_ethereum_address:
// is address of the bridge contract on the Ethereum side, this is a
// reference value for governance only and is not actually used by any
// Peggy code
//
// bridge_chain_id:
// the unique identifier of the Ethereum chain, this is a reference value
// only and is not actually used by any Peggy code
//
// These reference values may be used by future Peggy client implemetnations
// to allow for saftey features or convenience features like the peggy address
// in your relayer. A relayer would require a configured peggy address if
// governance had not set the address on the chain it was relaying for.
//
// signed_valsets_window
// signed_batches_window
// signed_claims_window
//
// These values represent the time in blocks that a validator has to submit
// a signature for a batch or valset, or to submit a claim for a particular
// attestation nonce. In the case of attestations this clock starts when the
// attestation is created, but only allows for slashing once the event has
// passed
//
// target_batch_timeout:
//
// This is the 'target' value for when batches time out, this is a target
// becuase Ethereum is a probabalistic chain and you can't say for sure what the
// block frequency is ahead of time.
//
// average_block_time
// average_ethereum_block_time
//
// These values are the average Cosmos block time and Ethereum block time
// repsectively and they are used to copute what the target batch timeout is. It
// is important that governance updates these in case of any major, prolonged
// change in the time it takes to produce a block
//
// slash_fraction_valset
// slash_fraction_batch
// slash_fraction_claim
// slash_fraction_conflicting_claim
//
// The slashing fractions for the various peggy related slashing conditions. The
// first three refer to not submitting a particular message, the third for
// submitting a different claim for the same Ethereum event
//
// unbond_slashing_valsets_window
//
// The unbond slashing valsets window is used to determine how many blocks after
// starting to unbond a validator needs to continue signing blocks. The goal of
// this paramater is that when a validator leaves the set, if their leaving
// creates enough change in the validator set to justify an update they will
// sign a validator set update for the Ethereum bridge that does not include
// themselves. Allowing us to remove them from the Ethereum bridge and replace
// them with the new set gracefully.
//
// valset_reward
//
// Valset rewards are the amount of tokens this chain issues to relayers of
// validator sets. These can be any ERC20 token in the bridge, but it's strongly
// advised that chains use only Cosmos originated tokens, which the bridge
// effectively mints on Ethereum. If you run out of the token you are using for
// validator set rewards valset updates will fail and the bridge will be
// vulnerable to highjacking. For these paramaters the zero values are special
// and indicate not to attempt any reward. This is the default for
// bootstrapping.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub peggy_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contract_source_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bridge_ethereum_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub bridge_chain_id: u64,
    #[prost(uint64, tag="5")]
    pub signed_valsets_window: u64,
    #[prost(uint64, tag="6")]
    pub signed_batches_window: u64,
    #[prost(uint64, tag="7")]
    pub signed_claims_window: u64,
    #[prost(uint64, tag="8")]
    pub target_batch_timeout: u64,
    #[prost(uint64, tag="9")]
    pub average_block_time: u64,
    #[prost(uint64, tag="10")]
    pub average_ethereum_block_time: u64,
    #[prost(bytes="vec", tag="11")]
    pub slash_fraction_valset: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub slash_fraction_batch: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub slash_fraction_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub slash_fraction_conflicting_claim: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="15")]
    pub unbond_slashing_valsets_window: u64,
    #[prost(bytes="vec", tag="16")]
    pub slash_fraction_bad_eth_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="17")]
    pub cosmos_coin_denom: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub cosmos_coin_erc20_contract: ::prost::alloc::string::String,
    #[prost(bool, tag="19")]
    pub claim_slashing_enabled: bool,
    #[prost(uint64, tag="20")]
    pub bridge_contract_start_height: u64,
    #[prost(message, optional, tag="21")]
    pub valset_reward: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetOrchestratorAddresses
/// this message allows validators to delegate their voting responsibilities
/// to a given key. This key is then used as an optional authentication method
/// for sigining oracle claims
/// VALIDATOR
/// The validator field is a cosmosvaloper1... string (i.e. sdk.ValAddress)
/// that references a validator in the active set
/// ORCHESTRATOR
/// The orchestrator field is a cosmos1... string  (i.e. sdk.AccAddress) that
/// references the key that is being delegated to
/// ETH_ADDRESS
/// This is a hex encoded 0x Ethereum public key that will be used by this
/// validator on Ethereum
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddresses {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetOrchestratorAddressesResponse {
}
/// MsgValsetConfirm
/// this is the message sent by the validators when they wish to submit their
/// signatures over the validator set at a given block height. A validator must
/// first call MsgSetEthAddress to set their Ethereum address to be used for
/// signing. Then someone (anyone) must make a ValsetRequest the request is
/// essentially a messaging mechanism to determine which block all validators
/// should submit signatures over. Finally validators sign the validator set,
/// powers, and Ethereum addresses of the entire validator set at the height of a
/// ValsetRequest and submit that signature with this message.
///
/// If a sufficient number of validators (66% of voting power) (A) have set
/// Ethereum addresses and (B) submit ValsetConfirm messages with their
/// signatures it is then possible for anyone to view these signatures in the
/// chain store and submit them to Ethereum to update the validator set
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirm {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetConfirmResponse {
}
/// MsgSendToEth
/// This is the message that a user calls when they want to bridge an asset
/// it will later be removed when it is included in a batch and successfully
/// submitted tokens are removed from the users balance immediately
/// -------------
/// AMOUNT:
/// the coin to send across the bridge, note the restriction that this is a
/// single coin not a set of coins that is normal in other Cosmos messages
/// FEE:
/// the fee paid for the bridge, distinct from the fee paid to the chain to
/// actually send this message in the first place. So a successful send has
/// two layers of fees for the user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEth {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub eth_dest: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag="4")]
    pub bridge_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendToEthResponse {
}
/// MsgRequestBatch
/// this is a message anyone can send that requests a batch of transactions to
/// send across the bridge be created for whatever block height this message is
/// included in. This acts as a coordination point, the handler for this message
/// looks at the AddToOutgoingPool tx's in the store and generates a batch, also
/// available in the store tied to this message. The validators then grab this
/// batch, sign it, submit the signatures with a MsgConfirmBatch before a relayer
/// can finally submit the batch
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatch {
    #[prost(string, tag="1")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestBatchResponse {
}
/// MsgConfirmBatch
/// When validators observe a MsgRequestBatch they form a batch by ordering
/// transactions currently in the txqueue in order of highest to lowest fee,
/// cutting off when the batch either reaches a hardcoded maximum size (to be
/// decided, probably around 100) or when transactions stop being profitable
/// (TODO determine this without nondeterminism) This message includes the batch
/// as well as an Ethereum signature over this batch by the validator
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatch {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub eth_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfirmBatchResponse {
}
/// EthereumBridgeDepositClaim
/// When more than 66% of the active validator set has
/// claimed to have seen the deposit enter the ethereum blockchain coins are
/// issued to the Cosmos address in question
/// -------------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub ethereum_sender: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub cosmos_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositClaimResponse {
}
/// WithdrawClaim claims that a batch of withdrawal
/// operations on the bridge contract was executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(uint64, tag="3")]
    pub batch_nonce: u64,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawClaimResponse {
}
/// ERC20DeployedClaim allows the Cosmos module
/// to learn about an ERC20 that someone deployed
/// to represent a Cosmos asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub cosmos_denom: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
    #[prost(string, tag="8")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgErc20DeployedClaimResponse {
}
/// This call allows the sender (and only the sender)
/// to cancel a given MsgSendToEth and recieve a refund
/// of the tokens
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEth {
    #[prost(uint64, tag="1")]
    pub transaction_id: u64,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelSendToEthResponse {
}
/// This call allows anyone to submit evidence that a
/// validator has signed a valset, batch, or logic call that never
/// existed. Subject contains the batch, valset, or logic call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidence {
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBadSignatureEvidenceResponse {
}
/// This informs the Cosmos module that a validator
/// set has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaim {
    #[prost(uint64, tag="1")]
    pub event_nonce: u64,
    #[prost(uint64, tag="2")]
    pub valset_nonce: u64,
    #[prost(uint64, tag="3")]
    pub block_height: u64,
    #[prost(message, repeated, tag="4")]
    pub members: ::prost::alloc::vec::Vec<BridgeValidator>,
    #[prost(string, tag="5")]
    pub reward_amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reward_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub orchestrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgValsetUpdatedClaimResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the peggy parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// GenesisState struct
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(uint64, tag="2")]
    pub last_observed_nonce: u64,
    #[prost(message, repeated, tag="3")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
    #[prost(message, repeated, tag="4")]
    pub valset_confirms: ::prost::alloc::vec::Vec<MsgValsetConfirm>,
    #[prost(message, repeated, tag="5")]
    pub batches: ::prost::alloc::vec::Vec<OutgoingTxBatch>,
    #[prost(message, repeated, tag="6")]
    pub batch_confirms: ::prost::alloc::vec::Vec<MsgConfirmBatch>,
    #[prost(message, repeated, tag="7")]
    pub attestations: ::prost::alloc::vec::Vec<Attestation>,
    #[prost(message, repeated, tag="8")]
    pub orchestrator_addresses: ::prost::alloc::vec::Vec<MsgSetOrchestratorAddresses>,
    #[prost(message, repeated, tag="9")]
    pub erc20_to_denoms: ::prost::alloc::vec::Vec<Erc20ToDenom>,
    #[prost(message, repeated, tag="10")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(uint64, tag="11")]
    pub last_observed_ethereum_height: u64,
    #[prost(uint64, tag="12")]
    pub last_outgoing_batch_id: u64,
    #[prost(uint64, tag="13")]
    pub last_outgoing_pool_id: u64,
    #[prost(message, optional, tag="14")]
    pub last_observed_valset: ::core::option::Option<Valset>,
    #[prost(string, repeated, tag="15")]
    pub ethereum_blacklist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// IDSet represents a set of IDs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdSet {
    #[prost(uint64, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchFees {
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub total_fees: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlacklistEthereumAddressesProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub blacklist_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeEthereumBlacklistProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub blacklist_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentValsetResponse {
    #[prost(message, optional, tag="1")]
    pub valset: ::core::option::Option<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetRequestResponse {
    #[prost(message, optional, tag="1")]
    pub valset: ::core::option::Option<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmResponse {
    #[prost(message, optional, tag="1")]
    pub confirm: ::core::option::Option<MsgValsetConfirm>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValsetConfirmsByNonceResponse {
    #[prost(message, repeated, tag="1")]
    pub confirms: ::prost::alloc::vec::Vec<MsgValsetConfirm>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastValsetRequestsResponse {
    #[prost(message, repeated, tag="1")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingValsetRequestByAddrResponse {
    #[prost(message, repeated, tag="1")]
    pub valsets: ::prost::alloc::vec::Vec<Valset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchFeeResponse {
    #[prost(message, repeated, tag="1")]
    pub batch_fees: ::prost::alloc::vec::Vec<BatchFees>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastPendingBatchRequestByAddrResponse {
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesResponse {
    #[prost(message, repeated, tag="1")]
    pub batches: ::prost::alloc::vec::Vec<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchRequestByNonceResponse {
    #[prost(message, optional, tag="1")]
    pub batch: ::core::option::Option<OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsRequest {
    #[prost(uint64, tag="1")]
    pub nonce: u64,
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBatchConfirmsResponse {
    #[prost(message, repeated, tag="1")]
    pub confirms: ::prost::alloc::vec::Vec<MsgConfirmBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventByAddrRequest {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLastEventByAddrResponse {
    #[prost(message, optional, tag="1")]
    pub last_claim_event: ::core::option::Option<LastClaimEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomRequest {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErc20ToDenomResponse {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub cosmos_originated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Request {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomToErc20Response {
    #[prost(string, tag="1")]
    pub erc20: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub cosmos_originated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByValidatorAddress {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByValidatorAddressResponse {
    #[prost(string, tag="1")]
    pub eth_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByEthAddress {
    #[prost(string, tag="1")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByEthAddressResponse {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByOrchestratorAddress {
    #[prost(string, tag="1")]
    pub orchestrator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegateKeysByOrchestratorAddressResponse {
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub eth_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEth {
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthResponse {
    #[prost(message, repeated, tag="1")]
    pub transfers_in_batches: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
    #[prost(message, repeated, tag="2")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<OutgoingTransferTx>,
}
/// QueryModuleStateRequest is the request type for the Query/PeggyModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {
}
/// QueryModuleStateResponse is the response type for the Query/PeggyModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<GenesisState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissingNoncesRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissingNoncesResponse {
    #[prost(string, repeated, tag="1")]
    pub operator_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
include!("injective.peggy.v1.tonic.rs");
// @@protoc_insertion_point(module)