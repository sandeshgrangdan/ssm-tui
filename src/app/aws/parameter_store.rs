use std::collections::HashMap;

use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_ssm::{
    types::{
        ParameterMetadata,
        ParameterStringFilter
    }, Client, Error
};
use aws_config::profile::ProfileFileRegionProvider;

#[derive(Debug)]
pub enum PsMetadata {
    Data(ParameterMetadata),
    None
}

pub async fn get_aws_client(profile: String,region: String) -> Client {
    let default_region = "us-east-1";
    if profile == String::from("None") {
        Client::new(
            &aws_config::defaults(BehaviorVersion::latest())
            .region(
                if region != String::from("None") {
                    RegionProviderChain::first_try(Region::new(region))
                            .or_default_provider()
                            .or_else(Region::new(default_region))
                    } else {
                        RegionProviderChain::default_provider()
                            .or_else(Region::new(default_region))
                    }
                )
                .load()
                .await
            )
        } else {
            Client::new(
                &aws_config::defaults(BehaviorVersion::latest())
                .region(
                    if region == String::from("None") {
                        RegionProviderChain::first_try(ProfileFileRegionProvider::builder().profile_name(profile.clone()).build())
                            .or_default_provider()
                            .or_else(Region::new(default_region))
                    } else {
                        RegionProviderChain::first_try(Region::new(region))
                            .or_default_provider()
                            .or_else(Region::new(default_region))
                        }
                )
                .profile_name(profile)
                .load()
                .await
            )
        }
}

pub async fn fetch_ps(client: &Client) -> Result<(HashMap<String, ParameterMetadata>, HashMap<String, String>, Vec<String>),Error> {
    println!("🔄 Fetching data from the server...");

    let mut parameters_data: Vec<ParameterMetadata> = vec![];

    let mut next_token: Option<String> = None;

    loop {
        let request = client.
            describe_parameters()
            .max_results(50)
            .set_next_token(next_token)
            .send()
            .await?;

        if let Some(metadata) = request.parameters {
            parameters_data.extend(metadata);
        }

        next_token = request.next_token;
        if next_token.is_none() {
            break;
        }
    }

    let mut items: Vec<String> = vec![];
    let mut parameters: HashMap<String, ParameterMetadata>  = HashMap::new();
    let mut ps_values: HashMap<String, String> = HashMap::new();

    println!("📡 Connecting to the server for {} data pieces, our hamster is running as fast as it can! 🐹",parameters_data.len());
    println!("💨 Please wait...");
    for parameter in parameters_data {
        let ps_name = match &parameter.name {
            Some(name) => name,
            None => &String::new()
        };
        items.push(ps_name.clone());
        let ps_value_res = get_ps_value(&ps_name, client).await;
        match ps_value_res  {
            Ok(ps_value) => {
                ps_values.insert((&ps_name).to_string(), ps_value);
            }
            Err(err) => panic!("Error: {}",err)
        }
        parameters.insert((&ps_name).to_string(), parameter.clone());
    }
    
    Ok((parameters,ps_values,items))
}

pub async fn get_ps_value(name: &String, client : &Client) -> Result<String, Error>{

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

pub async fn edit_ps_value(parameter_name: &str, edited_value: String, client : &Client) -> Result<(),Error>{
    client
        .put_parameter()
        .name(parameter_name)
        .value(edited_value)
        .overwrite(true)
        .send()
        .await?;

    Ok(())
}

pub async fn get_ps_metadata(parameter_name: &str, client : &Client) -> PsMetadata {

    let mut result = PsMetadata::None;

    let filter = ParameterStringFilter::builder()
        .key("Name")
        .values(parameter_name)
        .build();

    let filter= match filter {
        Ok(filter_string) => filter_string,
        _ => panic!("")
    };
    
    let response = client
    .describe_parameters()
    .parameter_filters(filter)
    .send()
    .await
    .unwrap();

    if let Some(metadatas) = response.parameters {
        for data in metadatas{
            if let Some(name) = &data.name {
                if name.to_string() == parameter_name.to_string() {
                    result = PsMetadata::Data(data);
                    break;
                }
            }
        }
    };
    result
}
