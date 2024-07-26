use serde::{Deserialize, Serialize};
use std::future::pending;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
use zbus::{connection, fdo, interface, SignalContext};

#[derive(Debug, Serialize, Deserialize, EnumString, Display, zbus::zvariant::Type)]
enum IceCreamFalvour {
    Chocolate,
    Vanilla,
}

#[derive(Debug, Serialize, Deserialize, zbus::zvariant::Type)]
struct IceCreamReply {
    flavor: IceCreamFalvour,
    quantity: u32,
    total: f32,
}

#[allow(dead_code)]
struct IceCreamTruck {
    name: String,
}

#[interface(name = "top.stdin.icecream")]
impl IceCreamTruck {
    async fn buy_icecream(
        &self,
        flavor: &str,
        quantity: u32,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
    ) -> fdo::Result<IceCreamReply> {
        let flavor = match IceCreamFalvour::from_str(flavor) {
            Ok(v) => v,
            Err(err) => return Err(fdo::Error::Failed(format!("{:?}", err))),
        };

        println!(
            "received order: flavor = {} quantity = {}",
            flavor, quantity
        );
        let details = IceCreamReply {
            flavor,
            quantity,
            total: 6.99 * quantity as f32,
        };
        Self::orders(&ctx, &details).await.unwrap();
        Ok(details)
    }

    #[zbus(signal)]
    async fn orders(ctxt: &SignalContext<'_>, details: &IceCreamReply) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let truck = IceCreamTruck {
        name: "JK".to_string(),
    };
    let _conn = connection::Builder::session()?
        .name("top.stdin.icecream")?
        .serve_at("/top/stdin/IceCreamTruck", truck)?
        .build()
        .await;

    pending::<()>().await;
    Ok(())
}
