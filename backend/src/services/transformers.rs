use crate::models::*;
use anyhow::Result;
use serde_json::Value;

pub struct GCPTransformer;

impl GCPTransformer {
    pub fn transform_function_response(response: Value) -> Result<CloudFunction> {
        let name = response["name"].as_str().unwrap();
        let state = response["state"].as_str().unwrap();
        let runtime = response["buildConfig"]["runtime"].as_str().unwrap();
        let region = if name.contains("/locations/") {
            name.split("/locations/").nth(1).unwrap().split("/").next().unwrap()
        } else {
            "us-central1"
        };
        let entry_point = response["buildConfig"]["entryPoint"].as_str().unwrap();
        let project_id = name.split("/projects/").nth(1).unwrap().split("/").next().unwrap();

        Ok(CloudFunction {
            name: name.split("/functions/").last().unwrap().to_string(),
            status: state.to_string(),
            runtime: runtime.to_string(),
            region: region.to_string(),
            entry_point: entry_point.to_string(),
            project_id: project_id.to_string(),
        })
    }
}
