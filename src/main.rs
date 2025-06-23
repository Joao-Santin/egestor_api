//eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHAiOiJmZGZhNDZjZTkwNGQ4MTk1MDMwMjYyZWQ4MjhkYjllYiIsInN1YmRvbWluaW8iOiJiaXBsYXMiLCJjbGllbnQiOiI2Nzc4ODZjMTQ3ZGVkYjViNzkyNjNmY2E1M2QzMzVmNTNkNWE0ZjczIiwiY3JlYXRlZCI6MTc0NDgyODA4NH0=.PLJwUmU76hknWQplS1sYZkwZcMUIXx2iuQ11+KtBOyc=
//
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    personal_token: &'a str,
}

#[derive(Deserialize, Debug)]
struct TokenResponse{
    access_token: String,
    token_type: String,
    expires_in: i16,
    refresh_token: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Troque aqui pelo seu token real!
    let token_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHAiOiJmZGZhNDZjZTkwNGQ4MTk1MDMwMjYyZWQ4MjhkYjllYiIsInN1YmRvbWluaW8iOiJiaXBsYXMiLCJjbGllbnQiOiI2Nzc4ODZjMTQ3ZGVkYjViNzkyNjNmY2E1M2QzMzVmNTNkNWE0ZjczIiwiY3JlYXRlZCI6MTc0NDgyODA4NH0=.PLJwUmU76hknWQplS1sYZkwZcMUIXx2iuQ11+KtBOyc=";

    let req_body = TokenRequest {
        grant_type: "personal",
        personal_token: token_str,
    };

    let client = Client::new();

    let res_token = client
        .post("https://api.egestor.com.br/api/oauth/access_token")
        .json(&req_body)
        .send()
        .await?;

    let status = res_token.status();
    //let body = res.text().await?;
    let response: TokenResponse = res_token.json().await?;

    println!("Status: {}", status);
    println!("Access Token: {}", response.access_token);
    println!("Token_type: {}", response.token_type);
    println!("expires_in: {}", response.expires_in);
    println!("refresh_token: {}", response.refresh_token);
    println!("-----------");

    let res = client
        .post("https://api.egestor.com.br/api/v1/empresa")
        .bearer_auth(response.access_token)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;
    println!("Status2: {}", status);
    println!("Resposta: {}", body);

    Ok(())
}
