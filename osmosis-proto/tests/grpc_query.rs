use std::time::{SystemTime, UNIX_EPOCH};

use osmosis_proto::osmosis::twap::v1beta1::{self as twap, query_client::QueryClient};

const GRPC_ENDPOINT: &str = "https://osmosis-grpc.polkachu.com:12590";

const OSMO_USDC_POOL_ID: u64 = 678;

const OSMO_DENOM: &str = "uosmo";

const USDC_DENOM: &str = "ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858";

// number of seconds in half an hour
const HALF_AN_HOUR_SECS: i64 = 30 * 60;

#[tokio::test]
async fn querying_twap() {
    let current_time_secs: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap();

    let price: u128 = QueryClient::connect(GRPC_ENDPOINT)
        .await
        .unwrap()
        .geometric_twap_to_now(twap::GeometricTwapToNowRequest {
            pool_id: OSMO_USDC_POOL_ID,
            base_asset: OSMO_DENOM.into(),
            quote_asset: USDC_DENOM.into(),
            start_time: Some(prost_types::Timestamp {
                seconds: current_time_secs - HALF_AN_HOUR_SECS,
                nanos: 0,
            }),
        })
        .await
        .unwrap()
        .into_inner()
        .geometric_twap
        .parse()
        .unwrap();

    // OSMO price must be greater than zero, lol
    assert!(price > 0);
}
