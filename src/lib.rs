use async_trait::async_trait;
use naumachia::{
    ledger_client::LedgerClient,
    logic::{SCLogic, SCLogicResult},
    transaction::TxActions,
};

pub mod scripts;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MyLogic;

#[async_trait]
impl SCLogic for MyLogic {
    type Endpoints = ();
    type Lookups = ();
    type LookupResponses = ();
    type Datums = ();
    type Redeemers = ();

    async fn handle_endpoint<Record: LedgerClient<Self::Datums, Self::Redeemers>>(
        _endpoint: Self::Endpoints,
        _ledger_client: &Record,
    ) -> SCLogicResult<TxActions<Self::Datums, Self::Redeemers>> {
        let empty_actions = TxActions::v2();
        Ok(empty_actions)
    }

    async fn lookup<Record: LedgerClient<Self::Datums, Self::Redeemers>>(
        _query: Self::Lookups,
        _ledger_client: &Record,
    ) -> SCLogicResult<Self::LookupResponses> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use naumachia::{
        address::PolicyId,
        ledger_client::test_ledger_client::TestBackendsBuilder,
        smart_contract::{SmartContract, SmartContractTrait},
        Address,
    };

    #[tokio::test]
    async fn nothing_test() {
        let me = Address::from_bech32("addr_test1qrksjmprvgcedgdt6rhg40590vr6exdzdc2hm5wc6pyl9ymkyskmqs55usm57gflrumk9kd63f3ty6r0l2tdfwfm28qs0rurdr").unwrap();
        let start_amount = 100_000_000;
        let backend = TestBackendsBuilder::new(&me)
            .start_output(&me)
            .with_value(PolicyId::Lovelace, start_amount)
            .finish_output()
            .build_in_memory();

        let endpoint = ();
        let contract = SmartContract::new(&MyLogic, &backend);

        contract.hit_endpoint(endpoint).await.unwrap();
    }
}
