use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;

fn main() -> Result<()> {
    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        .with_field_attributes(
            &["WelcomeRequest.content_ids"],
            &[r#"#[builder(setter(each(name="content_id", into)))]"#],
        )
        .compile(
            &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )
        .unwrap();
    Ok(())
}
