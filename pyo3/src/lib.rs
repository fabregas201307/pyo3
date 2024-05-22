
use log::{debug, error, info, warn};
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_log;

use polars::prelude::*;
use futures;
use std::env;
use tokio;


// include crates used for aws s3 connections
// use aws_sdk_s3::model::PutObjectRequest;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
// use aws_sdk_s3::{Client, Config, Region};
use aws_sdk_s3::{Client, Config};
// use aws_types::credentials::SharedCredentialsProvider;
use std::error::Error;


// define function to list all s3 buckets and print out names
// async fn list_buckets(s3_client: &Client) -> Result<(), Box<dyn Error>> {
//     let buckets = s3_client.list_buckets().send().await?;
//     println!("Buckets:");
//     for bucket in buckets.buckets().unwrap_or_default() {
//         println!("  {}", bucket.name().unwrap_or_default());
//     }
//     Ok(())
// }

async fn list_buckets(s3_client: &Client) -> Result<(), Box<dyn Error>> {
    let result = s3_client.list_buckets().send().await?;
    println!("Buckets:");
    for bucket in result.buckets() {
        println!("  {}", bucket.name().unwrap_or_default());
    }
    Ok(())
}

// // define function to upload a file to s3
// async fn upload_object(s3_client: &Client, bucket: &str, key: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
//     let mut file = File::open(file_path).await?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).await?;

//     s3_client.put_object(PutObjectRequest::builder()
//         .bucket(bucket)
//         .key(key)
//         .body(buffer.into())
//         .build())
//         .send()
//         .await?;

//     Ok(())
// }

// // define function to download a file from s3
// async fn download_object(s3_client: &Client, bucket: &str, key: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
//     let resp = s3_client.get_object().bucket(bucket).key(key).send().await?;
//     let mut stream = resp.body.unwrap().into_async_read();
//     let mut file = File::create(file_path).await?;
//     tokio::io::copy(&mut stream, &mut file).await?;
//     Ok(())
// }

// define function to create s3_client from config, list buckets
async fn altdata_s3() -> Result<(), Box<dyn Error>> {
    // Load the AWS credentials from the environment
    let shared_config = aws_config::from_env().load().await;
    
    // Create an S3 client
    let s3_client = Client::new(&shared_config);
    
    // List buckets
    list_buckets(&s3_client).await?;
    
    Ok(())
}

// define function exposed to python, and call the altdata_s3 function
#[pyfunction]
fn list_s3_buckets() -> PyResult<()> {
    tokio::runtime::Runtime::new().unwrap().block_on(altdata_s3()).unwrap();
    Ok(())
}
