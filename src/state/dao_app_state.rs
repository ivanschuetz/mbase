use super::app_state::{
    get_uint_value_or_error, local_state, local_state_from_account, AppStateKey,
    ApplicationGlobalState, ApplicationLocalStateError, ApplicationStateExt,
};
use crate::{
    api::version::{bytes_to_versions, Version},
    models::{
        dao_app_id::DaoAppId,
        funds::{FundsAmount, FundsAssetId},
        hash::GlobalStateHash,
        nft::Nft,
        share_amount::ShareAmount,
        shares_percentage::SharesPercentage,
        timestamp::Timestamp,
    },
};
use algonaut::{
    algod::v2::Algod,
    core::Address,
    model::algod::v2::{Account, ApplicationLocalState, TealKeyValue, TealValue},
};
use anyhow::{anyhow, Result};
use data_encoding::{BASE64, HEXLOWER};
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
};

const GLOBAL_TOTAL_RECEIVED: AppStateKey = AppStateKey("CentralReceivedTotal");
const GLOBAL_WITHDRAWABLE_AMOUNT: AppStateKey = AppStateKey("AvailableAmount");

const GLOBAL_FUNDS_ASSET_ID: AppStateKey = AppStateKey("FundsAssetId");
const GLOBAL_SHARES_ASSET_ID: AppStateKey = AppStateKey("SharesAssetId");

const GLOBAL_DAO_NAME: AppStateKey = AppStateKey("DaoName");
const GLOBAL_DAO_DESC: AppStateKey = AppStateKey("DaoDesc");
const GLOBAL_SHARE_PRICE: AppStateKey = AppStateKey("SharePrice");
const GLOBAL_INVESTORS_SHARE: AppStateKey = AppStateKey("InvestorsPart");

const GLOBAL_LOGO_URL: AppStateKey = AppStateKey("LogoUrl");
const GLOBAL_IMAGE_URL: AppStateKey = AppStateKey("ImageUrl");
const GLOBAL_IMAGE_ASSET_ID: AppStateKey = AppStateKey("ImageAsset");
const GLOBAL_SOCIAL_MEDIA_URL: AppStateKey = AppStateKey("SocialMediaUrl");

const GLOBAL_SHARES_LOCKED: AppStateKey = AppStateKey("LockedShares");

const GLOBAL_VERSIONS: AppStateKey = AppStateKey("Versions");

const GLOBAL_TARGET: AppStateKey = AppStateKey("Target");
const GLOBAL_TARGET_END_DATE: AppStateKey = AppStateKey("TargetEndDate");
const GLOBAL_RAISED: AppStateKey = AppStateKey("Raised");

const LOCAL_CLAIMED_TOTAL: AppStateKey = AppStateKey("ClaimedTotal");
const LOCAL_CLAIMED_INIT: AppStateKey = AppStateKey("ClaimedInit");
const LOCAL_SHARES: AppStateKey = AppStateKey("Shares");

const GLOBAL_SETUP_DATE: AppStateKey = AppStateKey("SetupDate");

pub const GLOBAL_SCHEMA_NUM_BYTE_SLICES: u64 = 6; // dao name, dao descr, logo, social media, versions, image nft url
pub const GLOBAL_SCHEMA_NUM_INTS: u64 = 12; // total received, shares asset id, funds asset id, share price, investors part, shares locked, funds target, funds target date, raised, image nft asset id, setup date

pub const LOCAL_SCHEMA_NUM_BYTE_SLICES: u64 = 0;
pub const LOCAL_SCHEMA_NUM_INTS: u64 = 3; // for investors: "shares", "claimed total", "claimed init"

// TODO rename in DaoGlobalState
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CentralAppGlobalState {
    /// Total funds the app has received from customer payments, since it was created
    /// note that it doesn't include capi fees - these are deducated before the amount is added to this
    pub received: FundsAmount,

    /// Funds on app escrow available to be used (i.e. withdrawn or claimed as dividend)
    /// Funds that are not available are not-yet-drained funds. Nobody can do anything with them, aside of draining.
    /// The reason this exists, is to be able to effectively perform actions upon receiving regular payments.
    /// We can't directly do that, but we can make the funds unavailable and perform the actions as part of making them available.
    /// The actions currently are:
    /// - pay the capi fee (charged on customer payments)
    /// - increment `received` state, which is used as basis to calculate dividend.
    /// "Draining" is how we have called the flow (transaction group) that makes the funds available.
    pub available: FundsAmount,

    pub app_approval_version: Version,
    pub app_clear_version: Version,

    pub funds_asset_id: FundsAssetId,
    pub shares_asset_id: u64,

    pub project_name: String,
    pub project_desc_url: Option<String>,
    pub share_price: FundsAmount,
    pub investors_share: SharesPercentage,

    pub image_hash: Option<GlobalStateHash>,
    pub image_nft: Option<Nft>,
    pub social_media_url: String,

    // fetched from the application, not from state, but here for convenience,
    // (the application is fetched when fetching state)
    pub owner: Address,

    pub locked_shares: ShareAmount,

    pub min_funds_target: FundsAmount,
    pub min_funds_target_end_date: Timestamp,
    pub raised: FundsAmount,

    pub setup_date: Timestamp,
}

/// Returns Ok only if called after dao setup (branch_setup_dao), where all the global state is initialized.
pub async fn dao_global_state(algod: &Algod, app_id: DaoAppId) -> Result<CentralAppGlobalState> {
    let app = algod.application_information(app_id.0).await?;
    let gs = ApplicationGlobalState(app.params.global_state);

    let expected_gs_len = GLOBAL_SCHEMA_NUM_BYTE_SLICES + GLOBAL_SCHEMA_NUM_INTS;
    if gs.len() != expected_gs_len as usize {
        log::debug!("DAO global state:");
        print_state(&gs.0)?;
        return Err(anyhow!(
            "Unexpected global state length: {}. Expected: {expected_gs_len}. Was the DAO setup performed already?",
            gs.len(),
        ));
    }

    let total_received = FundsAmount::new(get_int_or_err(&GLOBAL_TOTAL_RECEIVED, &gs)?);
    let available = FundsAmount::new(get_int_or_err(&GLOBAL_WITHDRAWABLE_AMOUNT, &gs)?);

    let funds_asset_id = FundsAssetId(get_int_or_err(&GLOBAL_FUNDS_ASSET_ID, &gs)?);
    let shares_asset_id = get_int_or_err(&GLOBAL_SHARES_ASSET_ID, &gs)?;

    let project_name = String::from_utf8(get_bytes_or_err(&GLOBAL_DAO_NAME, &gs)?)?;
    let project_desc_url = match gs.find_bytes(&GLOBAL_DAO_DESC) {
        Some(bytes) => {
            if bytes.is_empty() {
                None
            } else {
                Some(String::from_utf8(bytes)?)
            }
        }
        None => None,
    };

    let share_price = FundsAmount::new(get_int_or_err(&GLOBAL_SHARE_PRICE, &gs)?);
    let investors_share = get_int_or_err(&GLOBAL_INVESTORS_SHARE, &gs)?.try_into()?;

    let image_hash = match gs.find_bytes(&GLOBAL_LOGO_URL) {
        Some(bytes) => bytes_to_hash(bytes)?,
        None => None,
    };
    let image_asset_id = gs.find_uint(&GLOBAL_IMAGE_ASSET_ID);
    let image_url = gs.find_bytes(&GLOBAL_IMAGE_URL);

    let image_nft = match (image_asset_id, image_url) {
        // default values - meaning we didn't set them (they were just initialized in teal)
        (Some(asset_id), Some(url_bytes)) if asset_id == 0 && url_bytes.is_empty() => None,
        (Some(asset_id), Some(url_bytes)) => Some(Nft {
            asset_id,
            url: String::from_utf8(url_bytes)?,
        }),
        (None, None) => None,
        _ => {
            return Err(anyhow!(
                "Invalid state: nft asset id and url must both be set or not set".to_owned()
            ))
        }
    };

    let social_media_url = String::from_utf8(get_bytes_or_err(&GLOBAL_SOCIAL_MEDIA_URL, &gs)?)?;

    let versions_bytes = get_bytes_or_err(&GLOBAL_VERSIONS, &gs)?;
    let versions = bytes_to_versions(&versions_bytes)?;

    let shares_locked = ShareAmount::new(get_int_or_err(&GLOBAL_SHARES_LOCKED, &gs)?);

    let min_funds_target = FundsAmount::new(get_int_or_err(&GLOBAL_TARGET, &gs)?);
    let min_funds_target_end_date = Timestamp(get_int_or_err(&GLOBAL_TARGET_END_DATE, &gs)?);
    let raised = FundsAmount::new(get_int_or_err(&GLOBAL_RAISED, &gs)?);

    let setup_date = Timestamp(get_int_or_err(&GLOBAL_SETUP_DATE, &gs)?);

    Ok(CentralAppGlobalState {
        received: total_received,
        available,
        app_approval_version: versions.app_approval,
        app_clear_version: versions.app_clear,
        funds_asset_id,
        shares_asset_id,
        project_name,
        project_desc_url,
        share_price,
        investors_share,
        image_hash,
        image_nft,
        social_media_url,
        owner: app.params.creator,
        locked_shares: shares_locked,
        min_funds_target,
        min_funds_target_end_date,
        raised,
        setup_date,
    })
}

fn bytes_to_hash(bytes: Vec<u8>) -> Result<Option<GlobalStateHash>> {
    Ok(if bytes.is_empty() {
        // we always get vectors as teal values (not optionals)
        // we map empty vector to none here - this is meant to be used for values where it makes sense semantically
        None
    } else {
        Some(GlobalStateHash::from_bytes(bytes)?)
    })
}

fn print_state(values: &[TealKeyValue]) -> Result<()> {
    let mut key_values = BTreeMap::new();
    for kv in values {
        let key_bytes = BASE64.decode(kv.key.as_bytes())?;
        key_values.insert(String::from_utf8(key_bytes)?, value_to_str(&kv.value)?);
    }

    // separate step in case we split the fn
    for (k, v) in key_values {
        log::debug!("{k} => {v:?}")
    }

    Ok(())
}

fn value_to_str(value: &TealValue) -> Result<String> {
    match &value.value_type {
        1 => Ok(value
            .bytes
            .clone()
            .try_into()
            // try first to interpret bytes as address
            .map(|array| Address(array).to_string())
            // if not address, display as hex
            .unwrap_or_else(|_| to_hex_str(&value.bytes.clone()))),
        2 => Ok(value.uint.to_string()),
        _ => {
            return Err(anyhow!(
                "Unexpected global value type: {}",
                value.value_type
            ))
        }
    }
}

fn to_hex_str(bytes: &[u8]) -> String {
    format!("0x{}", HEXLOWER.encode(bytes))
}

fn get_int_or_err(key: &AppStateKey, gs: &ApplicationGlobalState) -> Result<u64> {
    gs.find_uint(key).ok_or_else(|| {
        anyhow!(
            "Key: {key:?} (int) not set in global state: {gs:?}, global state len: {}",
            gs.len()
        )
    })
}

fn get_bytes_or_err(key: &AppStateKey, gs: &ApplicationGlobalState) -> Result<Vec<u8>> {
    gs.find_bytes(key).ok_or_else(|| {
        anyhow!(
            "Key: {key:?} (bytes) not set in global state: {gs:?}, global state len: {}",
            gs.len()
        )
    })
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CentralAppInvestorState {
    // Locked (by definition since it's in the app state - free shares are just assets in the wallet) shares
    pub shares: ShareAmount,
    pub claimed: FundsAmount,
    /// Value to which "claimed" is initialized when the investor locks the shares
    /// We need this mainly for UX, to subtract it from "claimed", in order to show the user what they actually have claimed.
    /// elaboration: "claimed" is initialized to what the investor would be entitled to receive (based on received global state and held shares),
    /// to prevent double claiming (i.e. we allow to claim dividend only for future income).
    /// So we need to subtract this initial value from it, to show the investor what they actually claimed.
    pub claimed_init: FundsAmount,
}

pub async fn dao_investor_state(
    algod: &Algod,
    investor: &Address,
    app_id: DaoAppId,
) -> Result<CentralAppInvestorState, ApplicationLocalStateError<'static>> {
    let local_state = local_state(algod, investor, app_id.0).await?;
    central_investor_state_from_local_state(&local_state)
}

pub fn central_investor_state_from_acc(
    account: &Account,
    app_id: DaoAppId,
) -> Result<CentralAppInvestorState, ApplicationLocalStateError<'static>> {
    let local_state = local_state_from_account(account, app_id.0)?;
    central_investor_state_from_local_state(&local_state)
        .map_err(|e| ApplicationLocalStateError::Msg(e.to_string()))
}

/// Expects the user to be invested (as the name indicates) - returns error otherwise.
fn central_investor_state_from_local_state(
    state: &ApplicationLocalState,
) -> Result<CentralAppInvestorState, ApplicationLocalStateError<'static>> {
    if state.len() != ((LOCAL_SCHEMA_NUM_BYTE_SLICES + LOCAL_SCHEMA_NUM_INTS) as usize) {
        log::debug!("Investor local state:");
        print_state(&state.key_value).map_err(|e| {
            ApplicationLocalStateError::Msg(format!("Error printing local state: {e}"))
        })?;
        return Err(ApplicationLocalStateError::Msg(format!(
            "Unexpected investor local state length: {}, state: {state:?}",
            state.len(),
        )));
    }

    let shares = get_uint_value_or_error(state, &LOCAL_SHARES)?;
    let claimed = FundsAmount::new(get_uint_value_or_error(state, &LOCAL_CLAIMED_TOTAL)?);
    let claimed_init = FundsAmount::new(get_uint_value_or_error(state, &LOCAL_CLAIMED_INIT)?);

    Ok(CentralAppInvestorState {
        shares: ShareAmount::new(shares),
        claimed,
        claimed_init,
    })
}

/// Determines whether local state belongs to a capi app
///
/// it's not 100% guaranteed that the app belongs to capi - we just check for the same schema and local variable names
/// the likelihood that other app has this accidentally is small (we could name the keys a bit more specifically TODO)
/// they could use the same schema intentionally - this has to be taken into account when using this function.
/// Note also that the user would have to have opted in to one such app (they might be deceived into it)
///
/// TODO possible security issue?: think:
/// e.g. some app could get the user to optin to the app externally somehow and imitate the schema such that it appears under the user's "my apps" in capi
/// this can make the user open this app, thinking that it's trustable, and be more willing to invest? or something along those likes.
/// alternative (if needed) unclear - previously we were storing the dao id in local state, but that can be imitated by other apps too.
/// maybe it's enough to inform the user of these kind of risks with a short disclaimer
pub fn matches_capi_local_state(app_local_state: &ApplicationLocalState) -> bool {
    let schema = &app_local_state.schema;

    if !(schema.num_byte_slice == LOCAL_SCHEMA_NUM_BYTE_SLICES
        && schema.num_uint == LOCAL_SCHEMA_NUM_INTS
        && app_local_state.len() == 3)
    {
        return false;
    }

    let state_map: HashMap<String, TealValue> = app_local_state
        .clone()
        .key_value
        .into_iter()
        .map(|kv| (kv.key, kv.value))
        .collect();

    state_map.contains_key(&LOCAL_CLAIMED_TOTAL.to_teal_encoded_str())
        && state_map.contains_key(&LOCAL_CLAIMED_INIT.to_teal_encoded_str())
        && state_map.contains_key(&LOCAL_SHARES.to_teal_encoded_str())
}
