use crate::cache::cache_operations::RequestCached;
use crate::config::{base_transaction_service_url, transaction_request_timeout};
use crate::models::backend::transactions::{MultisigTransaction, Transaction};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::Page;
use crate::models::service::safes::{SafeLastChanges, SafeState};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use chrono::Utc;

// We use Utc::now().timestamp() as the fallback value so that we don't block clients from reloading
// as returning always 0, and the clients invalidating on value changes, would prevent reloading
pub fn get_safe_info_ex(context: &Context, safe_address: &String) -> ApiResult<SafeState> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let safe_info = info_provider.safe_info(safe_address)?;
    let safe_info_ex = safe_info.to_safe_info_ex(&mut info_provider);

    let safe_state = SafeState {
        safe_config: safe_info_ex,
        safe_state: SafeLastChanges {
            collectibles_tag: get_last_collectible(context, safe_address)
                .unwrap_or(Utc::now().timestamp())
                .to_string(),
            tx_queued_tag: get_last_queued_tx(context, safe_address)
                .unwrap_or(Utc::now().timestamp())
                .to_string(),
            tx_history_tag: get_last_history_tx(context, safe_address)
                .unwrap_or(Utc::now().timestamp())
                .to_string(),
        },
    };

    Ok(safe_state)
}

fn get_last_collectible(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let url = format!(
        "{}/v1/safes/{}/transfers/?\
        &erc721=true\
        &limit=1",
        base_transaction_service_url(),
        safe_address,
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<Transfer> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|transfer| match transfer {
            Transfer::Erc721(transfer) => transfer.execution_date.timestamp(),
            Transfer::Erc20(transfer) => transfer.execution_date.timestamp(),
            Transfer::Ether(transfer) => transfer.execution_date.timestamp(),
            Transfer::Unknown => Utc::now().timestamp(),
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

fn get_last_queued_tx(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/?\
        &ordering=-modified\
        &executed=false\
        &trusted=true\
        &limit=1",
        base_transaction_service_url(),
        safe_address,
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<MultisigTransaction> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|tx| tx.modified.as_ref().map(|it| it.timestamp()))
        .flatten()
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

fn get_last_history_tx(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?\
        &ordering=executionDate
        &queued=false\
        &executed=true",
        base_transaction_service_url(),
        safe_address
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<Transaction> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|tx| match tx {
            Transaction::Multisig(tx) => tx
                .modified
                .as_ref()
                .map(|it| it.timestamp())
                .unwrap_or(tx.submission_date.timestamp()),
            Transaction::Ethereum(tx) => tx.execution_date.timestamp(),
            Transaction::Module(tx) => tx.execution_date.timestamp(),
            Transaction::Unknown => Utc::now().timestamp(),
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}