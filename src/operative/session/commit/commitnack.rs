use serde::{Deserialize, Serialize};

/// `CSessionCommitNack` is returned by the coordinator to the msg.senders
/// upon receiving `NSessionCommit` if the commitment fails.
#[derive(Clone, Serialize, Deserialize)]
pub enum CSessionCommitNack {
    // Immediate errors upon insertion.
    SessionLocked,
    BlacklistedUntil(u64),
    AuthErr,
    Overlap,
    Allowance,
    InvalidLiftAccountKey,
    InvalidAccountRegisteryIndex,
    InvalidLiftOperatorKey,
    MissingLiftOutpoint,
    InvalidLiftOutpoint,
    InsufficientConnectors,
    // Post commit-pool errors
    CommitPruned,
    SessionNotLocked,
    AccountMismatch,
    PayloadAuthCtxErr,
}
