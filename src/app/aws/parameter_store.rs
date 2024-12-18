use std::{
    collections::HashMap, fs::{self, File}, io::{self, Write}, process::Command, sync::Arc
};

use aws_config::{
    profile::ProfileFileRegionProvider,
    meta::region::RegionProviderChain,
    {BehaviorVersion, Region}
};
use aws_sdk_ssm::{
    types::{
        ParameterMetadata,
        ParameterStringFilter
    }, Client, Error
};

use crate::app::App;
use crate::app::aws;

// ANCHOR: application
#[derive(Debug, Clone)]
pub enum SsmClient{
    Client(Client),
    None
}

#[derive(Debug)]
pub enum PsMetadata {
    Data(ParameterMetadata),
    None
}

// ANCHOR_END: application
#[derive(Debug, Default)]
pub enum SelectedPsMetadata<'a, 'b> {
    Data(&'a ParameterMetadata, String, &'b String),
    #[default]
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

impl App {
    pub async fn set_ssm_client(&mut self){
        self.ssm_client = SsmClient::Client(
                aws::parameter_store::get_aws_client(
                    self.args.profile.clone(), 
                    self.args.region.clone()
                ).await
        )
    }

    pub async fn fetch_ps_data(&mut self){
        match &self.ssm_client {
            SsmClient::Client(client) => {
                match aws::parameter_store::fetch_ps(&client).await {
                    Ok((ps_metadata,ps_values,items)) => {
                        self.parameter_stores.ps_values = ps_values;
                        self.parameter_stores.ps_metadata = ps_metadata;
                        self.parameter_stores.items = Arc::new(items.clone());
                        self.parameter_stores.display_items = items;
                    }
                    Err(err) => println!("{:?}",err)
                };
            }
            _ => {}
        }
    }

    pub fn get_selected_ps_data(&self) -> SelectedPsMetadata{
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        if self.parameter_stores.display_items.len() > 0 {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            let metadata = match self.parameter_stores.ps_metadata.get(ps_name) {
                Some(ps_metadata) => ps_metadata,
                _ => panic!("")
            };
    
            let value = match self.parameter_stores.ps_values.get(ps_name) {
                Some(value) => value.to_string(),
                None => "".to_string()
            };

            return SelectedPsMetadata::Data(metadata,value,ps_name)
        }

       SelectedPsMetadata::None

    }

    pub fn get_selected_value(&mut self) -> String {

        let default_value = "".to_string();
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        if self.parameter_stores.display_items.len() > 0 {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            return match self.parameter_stores.ps_values.get(ps_name) {
                Some(value) => value.to_string(),
                None => default_value
            }
        }
        default_value
    }

    pub async fn launch_vim(&mut self) -> io::Result<()> {
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        let ps_name = &self.parameter_stores.display_items[selected_ps_index];

        match &self.ssm_client {
            SsmClient::Client(client) => {
                match aws::parameter_store::get_ps_value(ps_name, client).await {
                    Ok(ps_value) => {
                        let temp_file_path = &self.generate_random_file_name();
        
                        let mut file = File::create(temp_file_path)?;
                        file.write_all(ps_value.as_bytes())?;
                        drop(file);
                
                        Command::new("vim")
                            .arg(temp_file_path) // Specify the file you want to edit with Vim
                            .status()?;
                
                        let edited_value = fs::read_to_string(temp_file_path)?;
                        let edited_value = edited_value.trim().to_string();
                
                        fs::remove_file(temp_file_path)?;
                        
                        if edited_value != ps_value.trim() {
                            self.parameter_stores.ps_values.insert((ps_name).to_string(), (&edited_value).to_string());
                            let _ = aws::parameter_store::edit_ps_value(ps_name, edited_value, client).await;
                            match aws::parameter_store::get_ps_metadata(ps_name, client).await {
                                aws::parameter_store::PsMetadata::Data(data) => {
                                    self.parameter_stores.ps_metadata.insert((ps_name).to_string(), data);
                                }
                                _ => {}
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_ps_list(&mut self) {
        if self.ps_filter_data.input.is_empty() {
            self.parameter_stores.list_title = "All".to_string();
            self.parameter_stores.display_items = self.parameter_stores.items.to_vec();
        }else{
            self.parameter_stores.list_title = self.ps_filter_data.input.to_string();

            self.parameter_stores.display_items = self.parameter_stores.items
                .iter()
                .filter(|name| name.trim().to_lowercase().contains(&self.ps_filter_data.input.trim().to_lowercase()))
                .cloned()
                .collect();
        }
    }
}