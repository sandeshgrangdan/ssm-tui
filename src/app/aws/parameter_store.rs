use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
    sync::Arc,
};

use aws_config::{
    meta::region::RegionProviderChain,
    profile::ProfileFileRegionProvider,
    {BehaviorVersion, Region},
};
use aws_sdk_ssm::{
    primitives::DateTime,
    types::{ParameterMetadata, ParameterStringFilter, ParameterType},
    Client, Error,
};

use crate::app::aws;
use crate::app::App;

// ANCHOR: application
#[derive(Debug, Clone)]
pub enum SsmClient {
    Client(Client),
    None,
}

#[derive(Debug)]
pub enum PsMetadata {
    Data(ParameterMetadata),
    None,
}

// ANCHOR_END: application
#[derive(Debug, Default)]
pub enum SelectedPsMetadata<'a, 'b> {
    Data(&'a ParameterMetadata, String, &'b String),
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub struct ParameterStoreMetadata {
    pub name: Option<String>,
    pub arn: Option<String>,
    pub value: Option<String>,
    pub last_modified_date: Option<DateTime>,
    pub version: i64,
    pub store_type: Option<ParameterType>,
}

pub async fn get_aws_client(profile: String, region: String) -> Client {
    let default_region = "us-east-1";
    if profile == *"None" {
        Client::new(
            &aws_config::defaults(BehaviorVersion::latest())
                .region(if region != *"None" {
                    RegionProviderChain::first_try(Region::new(region))
                        .or_default_provider()
                        .or_else(Region::new(default_region))
                } else {
                    RegionProviderChain::default_provider().or_else(Region::new(default_region))
                })
                .load()
                .await,
        )
    } else {
        Client::new(
            &aws_config::defaults(BehaviorVersion::latest())
                .region(if region == *"None" {
                    RegionProviderChain::first_try(
                        ProfileFileRegionProvider::builder()
                            .profile_name(profile.clone())
                            .build(),
                    )
                    .or_default_provider()
                    .or_else(Region::new(default_region))
                } else {
                    RegionProviderChain::first_try(Region::new(region))
                        .or_default_provider()
                        .or_else(Region::new(default_region))
                })
                .profile_name(profile)
                .load()
                .await,
        )
    }
}

pub async fn fetch_ps(
    client: &Client,
) -> Result<
    (
        Vec<ParameterMetadata>,
        Vec<ParameterStoreMetadata>,
        Vec<String>,
    ),
    Error,
> {
    println!("🔄 Fetching data from the server...");

    let mut parameters_data: Vec<ParameterMetadata> = vec![];

    let mut next_token: Option<String> = None;

    loop {
        let request = client
            .describe_parameters()
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

    println!("📡 Connecting to the server for {} data pieces, our hamster is running as fast as it can! 🐹",parameters_data.len());
    println!("💨 Please wait...");
    let mut fn_output: Vec<ParameterStoreMetadata> = vec![];

    let mut handles: Vec<tokio::task::JoinHandle<Vec<ParameterStoreMetadata>>> = Vec::new();
    for chunk in parameters_data.chunks(10) {
        let client = client.clone();
        let chunk = chunk.to_vec();

        let mut names: Vec<String> = Vec::new();

        for item in chunk {
            if let Some(name) = &item.name {
                names.push(name.clone().to_string());
                items.push(name.clone().to_string());
            }
        }

        let handle = tokio::spawn(async move {
            let request = client
                .get_parameters()
                .set_names(Some(names))
                .with_decryption(true);

            match request.send().await {
                Ok(response) => {
                    if let Some(parameters) = response.parameters {
                        parameters
                            .into_iter()
                            .map(|param| ParameterStoreMetadata {
                                name: param.name,
                                arn: param.arn,
                                value: param.value,
                                store_type: param.r#type,
                                version: param.version,
                                last_modified_date: param.last_modified_date,
                            })
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                }
                Err(_) => vec![],
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(result) => fn_output.extend(result),
            Err(_) => eprintln!("A thread failed to execute"),
        }
    }

    Ok((parameters_data, fn_output, items))
}

pub async fn get_ps_value(name: &String, client: &Client) -> Result<String, Error> {
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

pub async fn edit_ps_value(
    parameter_name: &str,
    edited_value: String,
    client: &Client,
) -> Result<(), Error> {
    client
        .put_parameter()
        .name(parameter_name)
        .value(edited_value)
        .overwrite(true)
        .send()
        .await?;

    Ok(())
}

pub async fn get_ps_metadata(parameter_name: &str, client: &Client) -> PsMetadata {
    let mut result = PsMetadata::None;

    let filter = ParameterStringFilter::builder()
        .key("Name")
        .values(parameter_name)
        .build();

    let filter = match filter {
        Ok(filter_string) => filter_string,
        _ => panic!(""),
    };

    let response = client
        .describe_parameters()
        .parameter_filters(filter)
        .send()
        .await
        .unwrap();

    if let Some(metadatas) = response.parameters {
        for data in metadatas {
            if let Some(name) = &data.name {
                if *name == *parameter_name {
                    result = PsMetadata::Data(data);
                    break;
                }
            }
        }
    };
    result
}

impl App {
    pub async fn set_ssm_client(&mut self) {
        self.ssm_client = SsmClient::Client(
            aws::parameter_store::get_aws_client(
                self.args.profile.clone(),
                self.args.region.clone(),
            )
            .await,
        )
    }

    pub async fn fetch_ps_data(&mut self) {
        if let SsmClient::Client(client) = &self.ssm_client {
            match aws::parameter_store::fetch_ps(client).await {
                Ok((ps_metadata, ps_values, items)) => {
                    self.parameter_stores.ps_values = ps_values;
                    self.parameter_stores.ps_metadata = ps_metadata;
                    self.parameter_stores.items = Arc::new(items.clone());
                    self.parameter_stores.display_items = items;
                }
                Err(err) => println!("{:?}", err),
            };
        }
    }

    pub fn get_selected_ps_data(&self) -> SelectedPsMetadata {
        let selected_ps_index = self.parameter_stores.state.selected().unwrap_or_default();

        if !self.parameter_stores.display_items.is_empty() {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            let metadata = self
                .parameter_stores
                .ps_metadata
                .iter()
                .find(|param| param.name.as_deref() == Some(ps_name));

            let metadata = match metadata {
                Some(data) => data,
                _ => panic!(""),
            };

            let value = self
                .parameter_stores
                .ps_values
                .iter()
                .find(|param| param.name.as_deref() == Some(ps_name));

            let value = match value {
                Some(data) => match &data.value {
                    Some(actual_value) => actual_value,
                    None => "",
                },
                None => "",
            };

            return SelectedPsMetadata::Data(metadata, value.to_owned(), ps_name);
        }

        SelectedPsMetadata::None
    }

    pub fn get_selected_value(&mut self) -> String {
        let default_value = "".to_string();
        let selected_ps_index = self.parameter_stores.state.selected().unwrap_or_default();

        if !self.parameter_stores.display_items.is_empty() {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            let value = self
                .parameter_stores
                .ps_values
                .iter()
                .find(|param| param.name.as_deref() == Some(ps_name));

            match value {
                Some(data) => match &data.value {
                    Some(actual_value) => actual_value.to_string(),
                    None => "".to_string(),
                },
                None => default_value.clone(),
            };
        }
        default_value
    }

    pub async fn launch_vim(&mut self) -> io::Result<()> {
        let selected_ps_index = self.parameter_stores.state.selected().unwrap_or_default();

        let ps_name = &self.parameter_stores.display_items[selected_ps_index];

        if let SsmClient::Client(client) = &self.ssm_client {
            if let Ok(ps_value) = aws::parameter_store::get_ps_value(ps_name, client).await {
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
                    if let Some(param) = self
                        .parameter_stores
                        .ps_values
                        .iter_mut()
                        .find(|param| param.name.as_deref() == Some(ps_name))
                    {
                        param.value = Some(edited_value.to_string());
                    }

                    let _ =
                        aws::parameter_store::edit_ps_value(ps_name, edited_value, client)
                            .await;
                    if let aws::parameter_store::PsMetadata::Data(data) = aws::parameter_store::get_ps_metadata(ps_name, client).await {
                        if let Some(index) =
                            self.parameter_stores.ps_metadata.iter().position(|param| {
                                param.name.as_deref() == Some(ps_name)
                            })
                        {
                            self.parameter_stores.ps_metadata[index] = data;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn set_ps_list(&mut self) {
        if self.ps_filter_data.input.is_empty() {
            self.parameter_stores.list_title = "All".to_string();
            self.parameter_stores.display_items = self.parameter_stores.items.to_vec();
        } else {
            self.parameter_stores.list_title = self.ps_filter_data.input.to_string();

            self.parameter_stores.display_items = self
                .parameter_stores
                .items
                .iter()
                .filter(|name| {
                    name.trim()
                        .to_lowercase()
                        .contains(&self.ps_filter_data.input.trim().to_lowercase())
                })
                .cloned()
                .collect();
        }
    }
}
