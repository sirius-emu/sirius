use sirius_types::CurrencyType;

#[derive(Debug)]
pub enum UserCommand {
    GetUserInfo,
    GetCredits,
    GetCurrency,
    UpdateWallet(CurrencyType, i32),
    SendInitialData,
    UpdateLook { gender: String, look: String },
}
