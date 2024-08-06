// #[derive:(derive)]
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ssm::{
    types::ParameterMetadata,
    Client, Error
};

pub async fn fetch_ps() -> Result<Vec<ParameterMetadata>,Error> {

    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&config);

    println!("Getting parameters:");
    let resp = client.describe_parameters().max_results(50).send().await?;
    println!("Parameter store fetched:");

    let mut parameters  = vec![];
    for parameter in resp.parameters() {
        parameters.push(parameter.clone());
        println!(" {:?}", parameter);
    }
    
    Ok(parameters)
}
