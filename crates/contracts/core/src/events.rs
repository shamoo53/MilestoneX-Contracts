#![no_std]
use soroban_sdk::{Address, Env, String};

/// Event emitted when a donation is received
/// 
/// # Fields
/// * `donor` - The address of the donor
/// * `amount` - The amount donated
/// * `asset` - The asset type donated
/// * `project_id` - The project ID this donation is mapped to
/// * `timestamp` - The timestamp of the donation
#[derive(Clone)]
pub struct DonationReceived {
    pub donor: Address,
    pub amount: i128,
    pub asset: String,
    pub project_id: String,
    pub timestamp: u64,
}

/// Event emitted when a withdrawal is processed
/// 
/// # Fields
/// * `recipient` - The address receiving the withdrawal
/// * `amount` - The amount withdrawn
/// * `asset` - The asset type withdrawn
/// * `timestamp` - The timestamp of the withdrawal
#[derive(Clone)]
pub struct WithdrawalProcessed {
    pub recipient: Address,
    pub amount: i128,
    pub asset: String,
    pub timestamp: u64,
}

/// Event emitted when a donation is rejected due to duplicate transaction
/// 
/// # Fields
/// * `tx_hash` - The duplicate transaction hash
/// * `reason` - The rejection reason
/// * `timestamp` - When the duplicate was detected
#[derive(Clone)]
pub struct DonationRejected {
    pub tx_hash: String,
    pub reason: String,
    pub timestamp: u64,
}

/// Event emitted when a campaign is created
#[derive(Clone)]
pub struct CampaignCreated {
    pub project_id: String,
    pub beneficiary: Address,
    pub goal_amount: i128,
    pub goal_asset: String,
    pub timestamp: u64,
}

/// Event emitted when campaign metadata is updated
#[derive(Clone)]
pub struct CampaignUpdated {
    pub project_id: String,
    pub timestamp: u64,
}

/// Event emitted when campaign status changes
#[derive(Clone)]
pub struct CampaignStatusChanged {
    pub project_id: String,
    pub previous_status: u32,
    pub new_status: u32,
    pub timestamp: u64,
}

impl DonationReceived {
    /// Emit the DonationReceived event to the ledger
    /// 
    /// # Topics (indexed for querying)
    /// - donor: Address of the donor
    /// - project_id: Project ID for grouping donations
    /// 
    /// # Data (full event payload)
    /// - donor: Address of the donor
    /// - amount: Amount donated  
    /// - asset: Asset type donated
    /// - project_id: Project ID this donation is mapped to
    /// - timestamp: When the donation was received
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.donor.clone(), self.project_id.clone()),
            (self.donor.clone(), self.amount, self.asset.clone(), self.project_id.clone(), self.timestamp),
        );
    }
}

impl WithdrawalProcessed {
    /// Emit the WithdrawalProcessed event to the ledger
    /// 
    /// # Topics (indexed for querying)
    /// - recipient: Address of the recipient
    /// - amount: Amount withdrawn
    /// 
    /// # Data (full event payload)
    /// - recipient: Address of the recipient
    /// - amount: Amount withdrawn
    /// - asset: Asset type withdrawn
    /// - timestamp: When the withdrawal was processed
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.recipient.clone(), self.amount),
            (self.recipient.clone(), self.amount, self.asset.clone(), self.timestamp),
        );
    }
}

impl DonationRejected {
    /// Emit the DonationRejected event to the ledger
    /// 
    /// # Topics (indexed for querying)
    /// - tx_hash: The rejected transaction hash
    /// 
    /// # Data (full event payload)
    /// - tx_hash: The duplicate transaction hash
    /// - reason: Rejection reason
    /// - timestamp: When the rejection occurred
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.tx_hash.clone(),),
            (self.tx_hash.clone(), self.reason.clone(), self.timestamp),
        );
    }
}

impl CampaignCreated {
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.project_id.clone(), String::from_str(env, "campaign_created")),
            (
                self.project_id.clone(),
                self.beneficiary.clone(),
                self.goal_amount,
                self.goal_asset.clone(),
                self.timestamp,
            ),
        );
    }
}

impl CampaignUpdated {
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.project_id.clone(), String::from_str(env, "campaign_updated")),
            (self.project_id.clone(), self.timestamp),
        );
    }
}

impl CampaignStatusChanged {
    pub fn emit(&self, env: &Env) {
        env.events().publish(
            (self.project_id.clone(), String::from_str(env, "campaign_status_changed")),
            (
                self.project_id.clone(),
                self.previous_status,
                self.new_status,
                self.timestamp,
            ),
        );
    }
}

/// Event type identifier for DonationReceived
/// Used by indexers to identify this event type
pub const EVENT_DONATION_RECEIVED: &[u8] = b"donation_received";

/// Event type identifier for WithdrawalProcessed  
/// Used by indexers to identify this event type
pub const EVENT_WITHDRAWAL_PROCESSED: &[u8] = b"withdrawal_processed";

/// Event type identifier for DonationRejected
/// Used by indexers to identify this event type
pub const EVENT_DONATION_REJECTED: &[u8] = b"donation_rejected";

/// Event type identifier for CampaignCreated
pub const EVENT_CAMPAIGN_CREATED: &[u8] = b"campaign_created";

/// Event type identifier for CampaignUpdated
pub const EVENT_CAMPAIGN_UPDATED: &[u8] = b"campaign_updated";

/// Event type identifier for CampaignStatusChanged
pub const EVENT_CAMPAIGN_STATUS_CHANGED: &[u8] = b"campaign_status_changed";
