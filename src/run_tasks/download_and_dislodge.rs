use crate::{run_tasks::download::run_download, args::{DownloadAndDislodgeParams, DownloadParams, DislodgeParams}};

use super::dislodge::run_dislodge;

pub async fn run_download_and_dislodge(args: DownloadAndDislodgeParams) -> anyhow::Result<()> {
    let path = run_download(DownloadParams { url: args.url }).await?;
    run_dislodge(DislodgeParams {
        in_path: Some(path),
    }).await?;
    Ok(())
}