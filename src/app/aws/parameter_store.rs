use std::collections::HashMap;

// #[derive:(derive)]
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ssm::{
    types::ParameterMetadata, types::Parameter, Client, Error
};


async fn get_aws_client() -> Result<Client,Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    Ok(Client::new(&config))
}

pub async fn fetch_ps() -> Result<(HashMap<String, ParameterMetadata>, HashMap<String, String>, Vec<String>),Error> {
    let client = get_aws_client().await?;

    let mut parameters_data: Vec<Parameter> = vec![];

    let mut next_token: Option<String> = None;

    loop {
        // let request = client.
        //     describe_parameters()
        //     .max_results(50)
        //     .set_next_token(next_token)
        //     .send()
        //     .await?;

        // if let Some(metadata) = request.parameters {
        //     parameters_data.extend(metadata);
        // }

        // next_token = request.next_token;
        // if next_token.is_none() {
        //     break;
        // }

        let mut request = client.get_parameters_by_path()
            .path('/')
            .recursive(true)
            .with_decryption(true)
            .set_next_token(next_token)
            .max_results(50)
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
    for parameter in parameters_data {
        // let ps_name = match &parameter.name {
        //     Some(name) => name,
        //     None => &String::new()
        // };
        // items.push(ps_name.clone());
        // let ps_value_res = get_ps_value(&ps_name, client.clone()).await;
        // match ps_value_res  {
        //     Ok(ps_value) => {
        //         ps_values.insert((&ps_name).to_string(), ps_value);
        //     }
        //     Err(err) => panic!("Error: {}",err)
        // }
        // parameters.insert((&ps_name).to_string(), parameter.clone());
        println!(" {:?}", parameter);
    }
    
    Ok((parameters,ps_values,items))
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
