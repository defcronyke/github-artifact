// #![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::header::{
    HeaderValue, ACCEPT, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, LOCATION, USER_AGENT,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;

use serde::{Deserialize, Serialize};

use base64::encode;

use std::collections::HashMap;

static INDEX1: &[u8] = b"Hello, world!";

#[derive(Serialize, Deserialize, Debug, Default)]
struct ArtifactsRes {
    total_count: u64,
    artifacts: Vec<ArtifactsResInner>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

    let repo = params.get("repo").unwrap_or(&repo_default);

    let repo_parts: Vec<&str> = repo.split("/").collect();

    if repo_parts.len() != 2 {
        eprintln!("route called with invalid query params: {}", repo);
        return Ok(Response::new(Body::default()));
    }

    let repo_user = repo_parts[0];
    let repo_name = repo_parts[1];

    let default_auth_header = HeaderValue::from_static("Basic none");

    let req_headers = req.headers();

    let auth_parts: Vec<&str> = req_headers
        .get(AUTHORIZATION)
        .unwrap_or(&default_auth_header)
        .to_str()
        .unwrap()
        .split(" ")
        .collect();

    if auth_parts.len() != 2 {
        eprintln!(
            "route called with invalid auth: {}",
            &auth_parts.join(" ").to_string()
        );
        return Ok(Response::new(Body::default()));
    }

    let auth_parts2: Vec<&str> = auth_parts[1].split(":").collect();

    if auth_parts2.len() != 2 {
        eprintln!(
            "route called with invalid formatted auth: {}",
            &auth_parts.join(" ").to_string()
        );
        return Ok(Response::new(Body::default()));
    }

    let user = auth_parts2[0];
    let token = auth_parts2[1];

    if user.chars().count() == 0 {
        eprintln!(
            "route called with invalid auth: empty user: {}",
            &auth_parts.join(" ").to_string()
        );
        return Ok(Response::new(Body::default()));
    }

    if token.chars().count() == 0 {
        eprintln!(
            "route called with invalid auth: empty token: {}",
            &auth_parts.join(" ").to_string()
        );
        return Ok(Response::new(Body::default()));
    }

    println!("{}:{}", user, token);

    // 'https://api.github.com/repos/' + repo_user + '/' + repo_name + '/actions/artifacts'

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
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    client_req_headers.append(
        USER_AGENT,
        HeaderValue::from_static("Google Cloud Run Web App"),
    );

    // client_req_headers.append(
    //     AUTHORIZATION,
    //     HeaderValue::from_str(&format!("Basic {}:{}", user, token)).unwrap_or(default_auth_header),
    // );

    let client_res = client.request(client_req).await?;

    println!("status: {}", client_res.status());

    let buf = hyper::body::to_bytes(client_res).await?;

    println!("body: {:?}", buf);

    let artifacts_res: ArtifactsRes = serde_json::from_slice(&buf).unwrap_or_default();

    println!(
        "body parsed:\n{}",
        serde_json::to_string_pretty(&artifacts_res).unwrap_or_default()
    );

    // let download_request_url: Vec<&str> = artifacts_res.artifacts[0]
    //     .archive_download_url
    //     .split("//")
    //     .collect();

    let mut client_req = Request::builder()
        .method("GET")
        .uri(&format!(
            "{}",
            artifacts_res.artifacts[0].archive_download_url
        ))
        .body(Body::default())
        .unwrap_or(Request::new(Body::default()));

    let client_req_headers = client_req.headers_mut();

    client_req_headers.append(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    client_req_headers.append(
        USER_AGENT,
        HeaderValue::from_static("Google Cloud Run Web App"),
    );

    let auth_string = format!("{}:{}", user, token);

    let auth_base64 = encode(auth_string.as_bytes());

    client_req_headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", &auth_base64)).unwrap_or(default_auth_header),
    );

    let client_res = client.request(client_req).await?;

    println!("status: {}", client_res.status());

    let default_location_header = HeaderValue::from_static("none");

    println!(
        "artifact download link (expires in 1 minute): {}",
        client_res
            .headers()
            .get(LOCATION)
            .unwrap_or(&default_location_header)
            .to_str()
            .unwrap()
    );

    let mut body: Response<Body> = Response::new(Body::from(INDEX1));

    *body.status_mut() = StatusCode::FOUND;

    let headers = body.headers_mut();

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
