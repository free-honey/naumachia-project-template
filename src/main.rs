use naumachia::{
    backend::Backend, smart_contract::SmartContract,
    trireme_ledger_client::get_trireme_ledger_client_from_file,
};
use naumachia_project_template::MyLogic;

#[tokio::main]
async fn main() {
    let logic = MyLogic;
    let ledger_client = get_trireme_ledger_client_from_file().await.unwrap();
    let backend = Backend::new(ledger_client);
    let _contract = SmartContract::new(&logic, &backend);
}
