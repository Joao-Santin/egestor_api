use std::env;

//eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHAiOiJmZGZhNDZjZTkwNGQ4MTk1MDMwMjYyZWQ4MjhkYjllYiIsInN1YmRvbWluaW8iOiJiaXBsYXMiLCJjbGllbnQiOiI2Nzc4ODZjMTQ3ZGVkYjViNzkyNjNmY2E1M2QzMzVmNTNkNWE0ZjczIiwiY3JlYXRlZCI6MTc0NDgyODA4NH0=.PLJwUmU76hknWQplS1sYZkwZcMUIXx2iuQ11+KtBOyc=
//
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use dotenv;


#[derive(Serialize, Debug)]
struct TokenRequest {
    grant_type: String,
    personal_token: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse{
    access_token: String,
    // não sendo usados, por isso comentei, quando for usar, descomentar
    //token_type: String,
    //expires_in: i16,
    //refresh_token: String
}
struct Token{
    token_str: String,
    token_req: TokenRequest,
    token_res: TokenResponse,
    access_token: String
}
impl Token{
    async fn new(client: Client) -> Result<Self, Box<dyn std::error::Error>>{
        dotenv::dotenv().ok();
        let token_str: String = env::var("TOKENEGESTOR").expect("Variable not found.");
        let token_req: TokenRequest = TokenRequest{
            grant_type: "personal".to_string(),
            personal_token: token_str.clone()
        };
        let full_token_res = client
        .post("https://api.egestor.com.br/api/oauth/access_token")
        .json(&token_req)
        .send()
        .await?;

        let token_res: TokenResponse = full_token_res.json().await?;

        let access_token: String = token_res.access_token.clone();
         
        Ok(Token{
            token_str,
            token_req,
            token_res,
            access_token
        })
    }
}

#[derive(Deserialize, Debug)]
struct ProductResponse{
    codigo: i64,
    descricao: String,
    codigoProprio: String,
    estoque: i64,
    tipoProduto: String,
}


#[derive(Deserialize, Debug)]
struct ItemProducao{
    tipo: String,
#[serde(rename="codProduto")]
    codproduto: String,
    #[serde(rename="IxProd")]
    ixprod: String,
    #[serde(rename="IcProd")]
    icprod: String,
    #[serde(rename="IuCom")]
    iucom: String,
    quant: f64,
    #[serde(rename="pPerda")]
    pperda: f64,
    #[serde(rename="qntPerda")]
    qntperda: f64,
    #[serde(rename="custoInsumo")]
    custoinsumo: f64,
    #[serde(rename="custoUnit")]
    custounit: f64,
    #[serde(rename="custoExtra")]
    custoextra: f64,
    custo: f64,
}
#[derive(Deserialize, Debug)]
struct Producao{
    cod: String,
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
    quant: String,
    #[serde(rename="pPerda")]
    pperda:String
}

#[derive(Deserialize)]
struct Composicao{
    cod: String,
    produto: String,
    insumos: Vec<Insumo>
}
// enum utilizado para especificar os gets
enum Gettypes{
    EstoqueEsp(String),// dia: string
    Producao,
    
}

fn sazonalidade() -> Result<(), ()>{
    Ok(())
}

fn estoque() -> Result<(), ()>{
    Ok(())
}

fn material_reservado() -> Result<(), ()>{
    Ok(())
}

fn levantamento_mrp(producao: Vec<Producao>, produtos: Vec<ProductResponse>, composicoes: Vec<Composicao>) -> Result<(), ()>{
    Ok(())
}
struct AppLogic{
    token: Token

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // aqui esta o token pessoal para poder retirar o token de acesso
    let client = Client::new();

    let token: Token = Token::new(client.clone()).await?;

    let res = client
        .get("https://api.egestor.com.br/api/v1/produtos/16")
        .bearer_auth(&token.access_token)
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
        .bearer_auth(&token.access_token)
        .header("Content-Type", "application/json")
        .json(&values_rel_comp)
        .send()
        .await?;
    
    let rel_comp_status = res_rel_comp.status();
    //let rel_comp_text = res_rel_comp.text().await?;
    let rel_comp_response: Vec<Composicao> = res_rel_comp.json().await?;
    
    println!("{}", rel_comp_status);
    for composicao in rel_comp_response {
        println!("-*-*-*-*-*-*COMPOSICAO*-*-*-*-*-*-");
        println!("cod: {}", composicao.cod);
        println!("produto: {}", composicao.produto);
        for insumo in composicao.insumos {
            println!("---insumo---");
            println!("cod insumo: {}", insumo.codinsumo);
            println!("insumo: {}", insumo.insumo);
            println!("codproprio: {}", insumo.codproprio);
            println!("unidade: {}", insumo.unidade);
            println!("quant: {}", insumo.quant);
            println!("p perda: {}", insumo.pperda);
        }
    }
    //println!("{}", rel_comp_text);

    let values_rel_prod = json!({
        "tipoData": "dtInicio",
        "de": "2019-04-01",
        "ate": "2025-07-20",
        "tags": "",
        "situacao": 0,
        "cods": "",
        "esconderValores": false,
        "mostrarQndPerda": true
    });
    let res_rel_prod = client
        .post("https://api.egestor.com.br/api/v1/relatorios/producoesDetalhadas")
        .bearer_auth(token.access_token)
        .header("Content-Type", "application/json")
        .json(&values_rel_prod)
        .send()
        .await?;
    let rel_prod_status = res_rel_prod.status();
    let rel_prod_response: Vec<Producao> = res_rel_prod.json().await?;
    //let rel_prod_body = res_rel_prod.text().await?;

    println!("{}", rel_prod_status);
    for producao in rel_prod_response{
        let mut contador_insumo: i16 = 0;
        println!("*-*-*-*-PRODUCAO*-*-*-*-*");
        for produto in producao.produto{
            println!("{}", produto.tipo);
            println!("{}", produto.codproduto);
            println!("{}", produto.ixprod);
            println!("{}", produto.icprod);
            println!("{}", produto.iucom);
            println!("{}", produto.quant);
            println!("{}", produto.pperda);
            println!("{}", produto.qntperda);
            println!("{}", produto.custoinsumo);
            println!("{}", produto.custounit);
            println!("{}", produto.custoextra);
            println!("{}", produto.custo);
        }
        for insumo in producao.insumos{
            contador_insumo += 1;
            println!("---insumo: {} ---", contador_insumo);
            println!("tipo: {}", insumo.tipo);
            println!("cod produto: {}", insumo.codproduto);
            println!("ix prod: {}", insumo.ixprod);
            println!("ic prod: {}", insumo.icprod);
            println!("iu com: {}", insumo.iucom);
            println!("quant: {}", insumo.quant);
            println!("pperda: {}", insumo.pperda);
            println!("qnt perda: {}", insumo.qntperda);
            println!("custo insumo: {}", insumo.custoinsumo);
            println!("custo unit: {}", insumo.custounit);
            println!("custo extra: {}", insumo.custoextra);
            println!("custo: {}", insumo.custo);
        }
    }
    

    Ok(())
}
