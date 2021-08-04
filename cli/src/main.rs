#[async_std::main]
async fn main() -> eyre::Result<()> {
    let problem = solved_ac::api::problem::get_by_id(8481).await?;
    dbg!(problem);

    Ok(())
}
