use fastnum::{dec128, udec128, D128, UD128};
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=example", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let row = client
        .query_one(
            "SELECT $1::NUMERIC, $2::NUMERIC",
            &[
                &udec128!(3.14159265358979323846264),
                &dec128!(-3.14159265358979323846264),
            ],
        )
        .await?;

    let value1_pos: UD128 = row.get(0);
    let value2_neg: D128 = row.get(1);

    assert_eq!(value1_pos, udec128!(3.14159265358979323846264));
    assert_eq!(value2_neg, dec128!(-3.14159265358979323846264));

    Ok(())
}
