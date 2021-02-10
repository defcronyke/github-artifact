// #![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::header::{
    HeaderValue, ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
    LOCATION, USER_AGENT,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;

use serde::{Deserialize, Serialize};

use base64::encode;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ArtifactsRes {
    total_count: u64,
    artifacts: Vec<ArtifactsResInner>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ArtifactsResInner {
    id: u64,
    node_id: String,
    name: String,
    size_in_bytes: u64,
    url: String,
    archive_download_url: String,
    expired: bool,
    created_at: String,
    updated_at: String,
    expires_at: String,
}

impl ArtifactsResInner {
    fn new() -> Self {
        Self {
            id: 0,
            node_id: "".to_string(),
            name: "".to_string(),
            size_in_bytes: 0,
            url: "".to_string(),
            archive_download_url: "".to_string(),
            expired: false,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            expires_at: "".to_string(),
        }
    }
}

async fn index1(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let repo_default = "none".to_string();
    let file_default = "".to_string();
    let num_default = "".to_string();

    let repo = params.get("repo").unwrap_or(&repo_default);
    let file = params.get("file").unwrap_or(&file_default);
    let num = params.get("num").unwrap_or(&num_default);

    let repo_parts: Vec<&str> = repo.split("@").collect();

    if repo_parts.len() != 2 {
        eprintln!("route called with invalid query params: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    let repo_parts2: Vec<&str> = repo_parts[0].split(":").collect();

    if repo_parts2.len() != 2 {
        eprintln!("route called with invalid auth query params: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    let user = repo_parts2[0];
    let token = repo_parts2[1];

    let repo_parts3: Vec<&str> = repo_parts[1].split("/").collect();

    if repo_parts3.len() != 2 {
        eprintln!("route called with invalid query params: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    let repo_user = repo_parts3[0];
    let repo_name = repo_parts3[1];

    let default_auth_header = HeaderValue::from_static("Basic none");

    if user.chars().count() == 0 {
        eprintln!("route called with invalid auth: empty user: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    if token.chars().count() == 0 {
        eprintln!("route called with invalid auth: empty token: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    let auth_string = format!("{}:{}", user, token);
    let auth_base64 = encode(auth_string.as_bytes());

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut client_req = Request::builder()
        .method("GET")
        .uri(&format!(
            "https://api.github.com/repos/{}/{}/actions/artifacts",
            repo_user, repo_name
        ))
        .body(Body::default())
        .unwrap_or(Request::new(Body::default()));

    let client_req_headers = client_req.headers_mut();

    client_req_headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", &auth_base64))
            .unwrap_or(default_auth_header.clone()),
    );

    client_req_headers.append(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    client_req_headers.append(
        USER_AGENT,
        HeaderValue::from_static("Google Cloud Run Web App"),
    );

    let client_res = client.request(client_req).await?;

    println!("artifact lookup status: {}", client_res.status());

    let buf = hyper::body::to_bytes(client_res).await?;

    println!("artifact lookup response body:\n{:?}", &buf);

    let artifacts_res: ArtifactsRes = serde_json::from_slice(&buf).unwrap_or_default();

    println!(
        "artifacts:\n{}",
        serde_json::to_string_pretty(&artifacts_res).unwrap_or_default()
    );

    let parsed_num: i64 = num.parse().unwrap_or_default();

    let default_artifact = ArtifactsResInner::new();

    let artifacts_filtered: Vec<ArtifactsResInner> = artifacts_res
        .artifacts
        .clone()
        .into_iter()
        .filter(|val| if file == "" { true } else { &val.name == file })
        .collect();

    let artifact: &ArtifactsResInner;

    if num == "0" {
        let mut body: Response<Body> = Response::new(Body::from(format!(
            "{}\n",
            serde_json::to_string_pretty(&artifacts_filtered).unwrap_or_default()
        )));

        let body_headers = body.headers_mut();

        body_headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        *body.status_mut() = StatusCode::OK;

        return Ok(body);
    } else {
        if parsed_num > 0 {
            artifact = artifacts_filtered
                .iter()
                .nth((parsed_num - 1) as usize)
                .unwrap_or(&artifacts_filtered.last().unwrap_or(&default_artifact));
        } else if parsed_num < 0 {
            artifact = artifacts_filtered
                .iter()
                .nth_back((-1 * (parsed_num + 1)) as usize)
                .unwrap_or(&artifacts_filtered.first().unwrap_or(&default_artifact));
        } else {
            artifact = artifacts_filtered
                .iter()
                .nth(parsed_num as usize)
                .unwrap_or(&artifacts_filtered.last().unwrap_or(&default_artifact));
        }
    }

    let artifact_url = artifact.archive_download_url.clone();

    let mut body: Response<Body> = Response::new(Body::from(format!("302 Found\n")));

    if &artifact_url == "" {
        eprintln!("file not found: {}", file);

        *body.status_mut() = StatusCode::NOT_FOUND;
        *body.body_mut() = Body::from("404 Not Found\n");

        return Ok(body);
    }

    let mut client_req = Request::builder()
        .method("GET")
        .uri(&format!("{}", artifact_url))
        .body(Body::default())
        .unwrap_or(Request::new(Body::default()));

    let client_req_headers = client_req.headers_mut();

    client_req_headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", &auth_base64))
            .unwrap_or(default_auth_header.clone()),
    );

    client_req_headers.append(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    client_req_headers.append(
        USER_AGENT,
        HeaderValue::from_static("Google Cloud Run Web App"),
    );

    let client_res = client.request(client_req).await?;

    println!("status: {}", client_res.status());

    let default_location_header = HeaderValue::from_static("none");

    let final_link = client_res
        .headers()
        .get(LOCATION)
        .unwrap_or(&default_location_header)
        .to_str()
        .unwrap();

    println!(
        "artifact download link (expires in 1 minute): {}",
        final_link
    );

    *body.body_mut() = Body::from(format!(
        "artifact download link (expires in 1 minute): {} (id: {}, created: {}): {}\n",
        artifact.name, artifact.id, artifact.created_at, final_link
    ));

    *body.status_mut() = StatusCode::FOUND;

    let headers = body.headers_mut();

    headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

    headers.append(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Accept"),
    );

    headers.append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    headers.append(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("OPTIONS, GET"),
    );

    headers.append(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("https://defcronyke.github.io"),
    );

    headers.append(
        LOCATION,
        client_res
            .headers()
            .get(LOCATION)
            .unwrap_or(&default_location_header)
            .clone(),
    );

    let buf = hyper::body::to_bytes(client_res).await?;

    println!("client res: {:?}", buf);

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match std::env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    let addr1 = ([0, 0, 0, 0], port).into();

    let srv1 = Server::bind(&addr1).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index1))
    }));

    println!("Listening on http://{}", addr1);

    let _ret = srv1.await;

    Ok(())
}
