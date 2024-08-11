// #[derive:(derive)]
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ssm::{
    client, types::ParameterMetadata, Client, Error
};


async fn get_aws_client() -> Result<Client,Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    Ok(Client::new(&config))
}

pub async fn fetch_ps() -> Result<(Vec<ParameterMetadata>, Vec<String>),Error> {
    let client = get_aws_client().await?;

    println!("Getting parameters:");
    let resp = client.
        describe_parameters().
        max_results(50).
        send().await?;
    println!("Parameter store fetched:");

    let mut parameters: Vec<ParameterMetadata>  = vec![];
    let mut ps_values: Vec<String> = vec![];
    for parameter in resp.parameters() {
        let ps_name = match &parameter.name {
            Some(name) => name,
            None => &String::from("")
        };
        let ps_value_res = get_ps_value(ps_name, client.clone()).await;
        match ps_value_res  {
            Ok(ps_value) => {
                ps_values.push(ps_value)
            }
            Err(err) => panic!("Error: {}",err)
        }
        parameters.push(parameter.clone());
        // println!(" {:?}", parameter);
    }
    
    Ok((parameters,ps_values))
}

async fn get_ps_value(name: &String, client : Client) -> Result<String, Error>{

    let result = client
            .get_parameter()
            .name(name)
            .with_decryption(true)
            .send()
            .await?
            .parameter()
            .unwrap()
            .value()
            .unwrap()
            .to_string();
    
    Ok(result)

}

pub async fn edit_ps_value(parameter_name: &str, edited_value: String) -> Result<(),Error>{
    let client = get_aws_client().await?;

    client
        .put_parameter()
        .name(parameter_name)
        .value(edited_value)
        .overwrite(true)
        .send()
        .await?;

    Ok(())
}
