use anyhow::Result;

fn main() -> Result<()> {
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .compile(&["../protos/crm/crm.proto"], &["../protos"])?;
    Ok(())
}
