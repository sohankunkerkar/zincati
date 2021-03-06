//! Strategy for immediate updates.

use failure::{Error, Fallible};
use futures::future;
use futures::prelude::*;
use log::trace;
use serde::Serialize;
use std::pin::Pin;

/// Strategy for immediate updates.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct StrategyImmediate {
    /// Whether to check for and fetch updates.
    check: bool,
    /// Whether to finalize updates.
    finalize: bool,
}

impl StrategyImmediate {
    /// Check if finalization is allowed.
    pub(crate) fn can_finalize(&self) -> Pin<Box<dyn Future<Output = Result<bool, Error>>>> {
        trace!(
            "immediate strategy, can finalize updates: {}",
            self.finalize
        );

        let res = future::ok(self.finalize);
        Box::pin(res)
    }

    pub(crate) fn report_steady(&self) -> Pin<Box<dyn Future<Output = Result<bool, Error>>>> {
        trace!("immediate strategy, report steady: {}", true);

        let immediate = future::ok(true);
        Box::pin(immediate)
    }

    pub(crate) fn can_check_and_fetch(&self) -> Pin<Box<dyn Future<Output = Result<bool, Error>>>> {
        trace!("immediate strategy, can check updates: {}", self.check);

        let immediate = future::ok(self.check);
        Box::pin(immediate)
    }
}

impl Default for StrategyImmediate {
    fn default() -> Self {
        Self {
            check: true,
            finalize: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use tokio::runtime as rt;

    #[test]
    fn default() {
        let default = StrategyImmediate::default();
        assert_eq!(default.check, true);
        assert_eq!(default.finalize, true);
    }

    #[test]
    fn report_steady() {
        let default = StrategyImmediate::default();
        let mut runtime = rt::Runtime::new().unwrap();
        let steady = runtime.block_on(default.report_steady()).unwrap();
        assert_eq!(steady, true);
    }

    proptest! {
        #[test]
        fn proptest_config(check in any::<bool>(), finalize in any::<bool>()){
            let strat = StrategyImmediate{
                check,
                finalize
            };

            assert_eq!(strat.check, check);
            assert_eq!(strat.finalize, finalize);
        }

        #[test]
        fn proptest_can_check(check in any::<bool>(), finalize in any::<bool>()){
            let strat = StrategyImmediate{
                check,
                finalize
            };

            let mut runtime = rt::Runtime::new().unwrap();
            let can_check = runtime.block_on(strat.can_check_and_fetch()).unwrap();
            assert_eq!(can_check, check);
        }

        #[test]
        fn proptest_can_finalize(check in any::<bool>(), finalize in any::<bool>()){
            let strat = StrategyImmediate{
                check,
                finalize
            };

            let mut runtime = rt::Runtime::new().unwrap();
            let can_finalize = runtime.block_on(strat.can_finalize()).unwrap();
            assert_eq!(can_finalize, finalize);
        }
    }
}
