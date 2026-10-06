pub mod cmd;

use std::{error::Error, io::stdout};

use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    Client,
    config::{self, Credentials},
    primitives::ByteStream,
};
use aws_smithy_types_convert::date_time::DateTimeExt;
use clap::Parser;
use save_point_common::types::{catalog::CatalogItem, envar::Envar};
use sutils::Singleton;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::cmd::Cmd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ident = Envar::One().identity.as_ref();
    if ident.is_empty() {
        panic!("no ident provided")
    }
    let client = client(ident).await;
    match Cmd::parse().action {
        cmd::Action::List => {
            let objects = client.list_objects_v2().bucket(ident).send().await?;
            let objects = objects
                .contents()
                .iter()
                .map(|obj| CatalogItem {
                    unique: obj.key().unwrap().to_owned(),
                    artifact: String::new(),
                    receive_at: obj.last_modified().unwrap().to_chrono_utc().unwrap(),
                })
                .collect::<Vec<_>>();
            serde_yaml::to_writer(stdout(), &objects)?;
        }
        cmd::Action::Sync { key } => {
            let reader = ByteStream::from_path(&key).await?;
            let put = client
                .put_object()
                .bucket(ident)
                .key(&key)
                .body(reader)
                .send()
                .await?;
            println!("{put:?}")
        }
        cmd::Action::Pull { key } => {
            let mut get = client.get_object().bucket(ident).key(&key).send().await?;
            let mut file = File::create(key).await?;

            while let Some(bytes) = get.body.next().await {
                let chunk = bytes?;
                file.write_all(&chunk).await?;
            }
        }
    }
    Ok(())
}

async fn client(ident: &str) -> Client {
    let credentials = Credentials::new(
        ident,
        ident,
        None, // Session token (Optional)
        None, // Expiry time (Optional)
        "env-provider",
    );
    let region = aws_config::Region::new("us-east-1");
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(region)
        .load()
        .await;

    let s3_config_builder = config::Builder::from(&sdk_config)
        .endpoint_url("https://la.seule.sbs/storage.sp/steam/")
        .force_path_style(true);

    Client::from_conf(s3_config_builder.build())
}
