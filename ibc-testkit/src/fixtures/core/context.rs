use alloc::fmt::Debug;
use core::cmp::min;
use core::ops::Add;
use core::time::Duration;

use basecoin_store::context::ProvableStore;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc::core::primitives::prelude::*;
use ibc::core::primitives::Timestamp;
use typed_builder::TypedBuilder;

use crate::context::MockGenericContext;
use crate::hosts::TestHost;
use crate::testapp::ibc::core::types::{MockIbcStore, DEFAULT_BLOCK_TIME_SECS};
use crate::utils::year_2023;

/// Configuration of the `MockContext` type for generating dummy contexts.
#[derive(Debug, TypedBuilder)]
#[builder(build_method(into))]
#[allow(dead_code)]
pub struct MockContextConfig<H>
where
    H: TestHost,
{
    #[builder(default = ChainId::new("mockgaia-0").expect("Never fails"))]
    host_id: ChainId,

    #[builder(default = Duration::from_secs(DEFAULT_BLOCK_TIME_SECS))]
    block_time: Duration,

    // may panic if validator_set_history size is less than max_history_size + 1
    #[builder(default = 5)]
    max_history_size: u64,

    #[builder(default, setter(strip_option))]
    block_params_history: Option<Vec<H::BlockParams>>,

    #[builder(default = Height::new(0, 5).expect("Never fails"))]
    latest_height: Height,

    #[builder(default = year_2023())]
    latest_timestamp: Timestamp,
}

impl<S, H> From<MockContextConfig<H>> for MockGenericContext<S, H>
where
    S: ProvableStore + Debug + Default,
    H: TestHost,
{
    fn from(params: MockContextConfig<H>) -> Self {
        assert_ne!(
            params.max_history_size, 0,
            "The chain must have a non-zero max_history_size"
        );

        assert_ne!(
            params.latest_height.revision_height(),
            0,
            "The chain must have a non-zero revision_height"
        );

        // Compute the number of blocks to store.
        let _n = min(
            params.max_history_size,
            params.latest_height.revision_height(),
        );

        assert_eq!(
            params.host_id.revision_number(),
            params.latest_height.revision_number(),
            "The version in the chain identifier must match the version in the latest height"
        );

        let _next_block_timestamp = params
            .latest_timestamp
            .add(params.block_time)
            .expect("Never fails");

        let host = H::with_chain_id(params.host_id);

        // let history = if let Some(validator_set_history) = params.block_params_history {
        //     (0..n)
        //         .rev()
        //         .map(|i| {
        //             // generate blocks with timestamps -> N, N - BT, N - 2BT, ...
        //             // where N = now(), BT = block_time
        //             host.generate_block(
        //                 params
        //                     .latest_height
        //                     .sub(i)
        //                     .expect("Never fails")
        //                     .revision_height(),
        //                 next_block_timestamp
        //                     .sub(params.block_time * ((i + 1) as u32))
        //                     .expect("Never fails"),
        //                 &validator_set_history[(n - i) as usize - 1],
        //             )
        //         })
        //         .collect()
        // } else {
        //     (0..n)
        //         .rev()
        //         .map(|i| {
        //             // generate blocks with timestamps -> N, N - BT, N - 2BT, ...
        //             // where N = now(), BT = block_time
        //             host.generate_block(
        //                 params
        //                     .latest_height
        //                     .sub(i)
        //                     .expect("Never fails")
        //                     .revision_height(),
        //                 next_block_timestamp
        //                     .sub(params.block_time * ((i + 1) as u32))
        //                     .expect("Never fails"),
        //                 &H::BlockParams::default(),
        //             )
        //         })
        //         .collect()
        // };

        MockGenericContext {
            host,
            ibc_store: MockIbcStore::default(),
        }
    }
}
