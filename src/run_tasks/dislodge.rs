use crate::{args::DislodgeParams, etcher};

pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    let (out_data, out_path) = etcher::read(&args.in_path.expect("no in path at run_dislodge"), 1)?;
    println!("Saving file as {}", out_path);
    etcher::write_bytes(
        &out_path,
        out_data,
    )?;
    Ok(())
}