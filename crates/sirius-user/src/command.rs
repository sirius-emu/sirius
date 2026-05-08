use sirius_types::Currency;

#[derive(Debug)]
pub enum UserCommand {
    GetUserInfo,
    GetCredits,
    GetCurrency,
    UpdateWallet(Currency),
    Disconnect,
    SendInitialData,
}
