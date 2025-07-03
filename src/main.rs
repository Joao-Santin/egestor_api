use std::env;

//eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHAiOiJmZGZhNDZjZTkwNGQ4MTk1MDMwMjYyZWQ4MjhkYjllYiIsInN1YmRvbWluaW8iOiJiaXBsYXMiLCJjbGllbnQiOiI2Nzc4ODZjMTQ3ZGVkYjViNzkyNjNmY2E1M2QzMzVmNTNkNWE0ZjczIiwiY3JlYXRlZCI6MTc0NDgyODA4NH0=.PLJwUmU76hknWQplS1sYZkwZcMUIXx2iuQ11+KtBOyc=
//
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use dotenv;

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
#[derive(Deserialize)]
struct ItemProducao{
    tipo: String,
    #[serde(rename="codProduto")]
    codproduto: i32,
    #[serde(rename="IxProd")]
    ixprod: String,
    #[serde(rename="IcProd")]
    icprod: String,
    #[serde(rename="IuCom")]
    iucom: String,
    quant: f64,
    #[serde(rename="pPerda")]
    pperda: f32,
    #[serde(rename="qntPerda")]
    qntperda: f32,
    #[serde(rename="custoInsumo")]
    custoinsumo: f32,
    #[serde(rename="custoUnit")]
    custounit: f32,
    #[serde(rename="custoExtra")]
    custoextra: f32,
    custo: f32,
}
#[derive(Deserialize, Debug)]
struct Producao{
    cod: i32,
    insumos: Vec<ItemProducao>,
    produto: Vec<ItemProducao>

}
#[derive(Deserialize, Debug)]
struct Insumo{
    #[serde(rename="codInsumo")]
    codinsumo: String,
    insumo: String,
    #[serde(rename="codProprio")]
    codproprio: String,
    unidade: String,
    quant: f64,
    #[serde(rename="pPerda")]
    pperda:f64
}

#[derive(Deserialize)]
struct Composicao{
    cod: String,
    produto: String,
    insumos: Vec<Insumo>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // aqui esta o token pessoal para poder retirar o token de acesso

    dotenv::dotenv().ok();
    let token_str: String = env::var("TOKENEGESTOR").expect("Variable not found.");

    let req_body = TokenRequest {
        grant_type: "personal",
        personal_token: &token_str,
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
        .bearer_auth(&response.access_token)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let produtos_response: ProductResponse = res.json().await?;
    println!("{}", produtos_response.codigo.to_string());

    //let status = res.status();
    //let body = res.text().await?;
    //println!("Status2: {}", status);
    //println!("Resposta: {}", body);
    
    //relatorio de composicoes retornada
    let values_rel_comp = json!({
        "codProds": "",
        "categoria": "",
        "tags": ""
    });

    let res_rel_comp = client
        .post("https://api.egestor.com.br/api/v1/relatorios/composicoes")
        .bearer_auth(&response.access_token)
        .header("Content-Type", "application/json")
        .json(&values_rel_comp)
        .send()
        .await?;
    
    let rel_comp_status = res_rel_comp.status();
    let rel_comp_response: Vec<Composicao> = res_rel_comp.json().await?;
    
    println!("{}", rel_comp_status);

    let values_rel_prod = json!({
        "tipoData": "dtInicio",
        "de": "2019-04-01",
        "ate": "2025-07-03",
        "tags": "",
        "situacao": 0,
        "cods": "",
        "esconderValores": false,
        "mostrarQndPerda": true
    });
    let res_rel_prod = client
        .post("https://api.egestor.com.br/api/v1/relatorios/producoesDetalhadas")
        .bearer_auth(response.access_token)
        .header("Content-Type", "application/json")
        .json(&values_rel_prod)
        .send()
        .await?;
    let rel_prod_status = res_rel_prod.status();
    let rel_prod_body = res_rel_prod.text().await?;

    println!("{}", rel_prod_status);
    println!("{}", rel_prod_body);

    Ok(())
}
