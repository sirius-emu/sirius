use crate::PacketRouter;
use crate::handlers::*;

pub fn build_router() -> PacketRouter {
    let mut builder = crate::router::PacketRouterBuilder::new();

    builder
        .register(UserCurrencyHandler)
        .register(UserSaveLookHandler)
        .register(UserInfoRetrieveHandler);

    builder.build()
}
