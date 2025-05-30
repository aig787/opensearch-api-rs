use anyhow::Result;
use opensearch_api::{Client, ClientConfig};

pub mod fixture;
use fixture::OpenSearchFixture;

#[tokio::test]
async fn test_client_invalid_credentials() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let base_url = fixture.get_base_url().await?;

    // Test connection with invalid credentials
    let invalid_client = Client::builder()
        .config(
            ClientConfig::builder()
                .base_url(base_url)
                .username("invalid")
                .password("invalid")
                .verify_ssl(false)
                .build()?,
        )
        .build()?;

    // Invalid credentials should fail to authenticate, but cluster
    // should be reachable
    assert_eq!(invalid_client.ping().await?, false);

    Ok(())
}

#[tokio::test]
async fn test_client_ping() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    let ping_result = fixture.client.ping().await?;
    assert!(ping_result, "Ping should succeed on available cluster");

    Ok(())
}

#[tokio::test]
async fn test_client_info() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    let info = fixture.client.info().await?;
    assert!(
        !info.cluster_name.is_none(),
        "Cluster name should not be empty"
    );
    assert!(
        !info.cluster_name.unwrap().is_empty(),
        "Cluster name should not be empty"
    );
    assert_eq!(info.version.number, "2.19.0");
    assert_eq!(
        info.tagline.as_deref(),
        Some("The OpenSearch Project: https://opensearch.org/")
    );

    Ok(())
}
