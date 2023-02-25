mod m20230225_134136_add_created_updated_gum;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
