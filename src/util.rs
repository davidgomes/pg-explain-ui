use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use crate::guc;

pub async fn get_pg_conn() -> Result<Pool<Postgres>> {
    let mut cfg = Config::default();

    if let Some(host) = guc::get_guc(guc::VectorizeGuc::Host) {
        info!("Using socket url from GUC: {:?}", host);
        cfg.vectorize_socket_url = Some(host);
    };

    let mut opts = get_pg_options(cfg)?;

    if let Some(dbname) = guc::get_guc(guc::VectorizeGuc::DatabaseName) {
        opts = opts.database(&dbname)
    };

    let pgp = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(4))
        .max_connections(4)
        .connect_with(opts)
        .await?;
    Ok(pgp)
}
