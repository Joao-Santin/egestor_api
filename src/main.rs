//eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHAiOiJmZGZhNDZjZTkwNGQ4MTk1MDMwMjYyZWQ4MjhkYjllYiIsInN1YmRvbWluaW8iOiJiaXBsYXMiLCJjbGllbnQiOiI2Nzc4ODZjMTQ3ZGVkYjViNzkyNjNmY2E1M2QzMzVmNTNkNWE0ZjczIiwiY3JlYXRlZCI6MTc0NDgyODA4NH0=.PLJwUmU76hknWQplS1sYZkwZcMUIXx2iuQ11+KtBOyc=
//
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    personal_token: &'a str,
}

#[derive(Deserialize, Debug)]
struct TokenResponse{
    access_token: String,
    // não sendo usados, por isso comentei, quando for usar, descomentar
    //token_type: String,
    //expires_in: i16,
    //refresh_token: String
}
#[derive(Deserialize, Debug)]
struct ProductResponse{
    codigo: i16,
    descricao: String,
    codigoProprio: String,
    estoque: i64,
    tipoProduto: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // aqui esta o token pessoal para poder retirar o token de acesso
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

    //let status = res_token.status();
    //let body = res.text().await?;

    let response: TokenResponse = res_token.json().await?;

    // Aqui alguns relacionados ao request de token
    //println!("Status: {}", status);
    //println!("Access Token: {}", response.access_token);
    //println!("Token_type: {}", response.token_type);
    //println!("expires_in: {}", response.expires_in);
    //println!("refresh_token: {}", response.refresh_token);
    //println!("-----------");

    let res = client
        .get("https://api.egestor.com.br/api/v1/produtos/16")
        .bearer_auth(response.access_token)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let produtos_response: ProductResponse = res.json().await?;
    println!("{}", produtos_response.codigo.to_string());

    //let status = res.status();
    //let body = res.text().await?;
    //println!("Status2: {}", status);
    //println!("Resposta: {}", body);

    Ok(())
}
