use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Debug)]
struct ApiRequest {
    provider: String,
    action: String,
    params: Params,
}

#[derive(Serialize, Debug)]
struct Params {
    ami_id: u32,
    vm_name: String,
    subnet_id: u32,
    distro: String,
    vcpus: String,
    memory_mb: String,
    disk_gb: String,
    hours: String,
    disk_path: String,
    size_mb: String,
}

// Function to create multiple VMs
async fn create_multiple_vms(count: u32) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    for i in 1..=count {
        let vm_name = format!("OmniVM_{}", i);
        
        // Create the request payload
        let request = ApiRequest {
            provider: "virtualbox_cpi_linux".to_string(),
            action: "create_vm".to_string(),
            params: Params {
                ami_id: 1,
                vm_name,
                subnet_id: 2,
                distro: "ubuntu".to_string(),
                vcpus: "1".to_string(),
                memory_mb: "8000".to_string(),
                disk_gb: "5".to_string(),
                hours: "1".to_string(),
                disk_path: "/isos/ubuntu-server.iso".to_string(),
                size_mb: "8000".to_string(),
            },
        };

        // Make the POST request
        let response = client
            .post("http://192.168.1.240:8081/vms/action")
            .json(&request)
            .send()
            .await?;

        // Check if the request was successful
        match response.status() {
            reqwest::StatusCode::OK => {
                // Use Value to parse any JSON response without requiring specific fields
                let response_body: Value = response.json().await?;
                println!("VM {} created successfully! Response: {:?}", i, response_body);
            }
            other => {
                // Handle error cases
                println!("Request for VM {} failed with status: {}", i, other);
                let error_text = response.text().await?;
                println!("Response body: {}", error_text);
            }
        }
        
        // Optional: add a small delay between requests to not overload the server
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Specify how many VMs to create
    let vm_count = 5; // Change this to the number of VMs you want to create
    
    println!("Creating {} VMs...", vm_count);
    create_multiple_vms(vm_count).await?;
    println!("VM creation process completed.");

    Ok(())
}