use crate::PacketRouter;
use crate::handlers::*;

pub fn build_router() -> PacketRouter {
    let mut builder = crate::router::PacketRouterBuilder::new();

    // User
    builder
        .register(UserCurrencyHandler)
        .register(UserSaveLookHandler)
        .register(UserInfoRetrieveHandler);

    // Navigator
    builder.register(NavigatorInitHandler);

    builder.build()
}
