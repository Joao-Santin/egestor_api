use std::env;
use std::fmt;
use chrono::Utc;

//
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use dotenv;


#[derive(Serialize, Debug, Clone)]
struct TokenRequest {
    grant_type: String,
    personal_token: String,
}

#[derive(Deserialize, Debug, Clone)]
struct TokenResponse{
    access_token: String,
    // não sendo usados, por isso comentei, quando for usar, descomentar
    //token_type: String,
    //expires_in: i16,
    //refresh_token: String
}
#[derive(Clone, Debug)]
pub struct ERPToken{
    token_str: String,
    token_req: TokenRequest,
    token_res: TokenResponse,
    pub access_token: String
}

impl ERPToken{
    pub async fn new(client: &Client) -> Result<Self, Box<dyn std::error::Error>>{

        let client = Client::new();
        dotenv::dotenv().ok();
        let token_str: String = env::var("TOKENEGESTOR").expect("Variavel TOKENEGESTOR nao encontrada");
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
         
        Ok(ERPToken{
            token_str,
            token_req,
            token_res,
            access_token
        })
    }
    pub async fn get_access_token() -> Result<String, String>{
        let client = Client::new();
        let token = Self::new(&client).await.map_err(|e| e.to_string())?;
        Ok(token.access_token)
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


#[derive(Deserialize, Debug, Clone)]
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
#[derive(Deserialize, Debug, Clone)]
struct Producao{
    cod: String,
    insumos: Vec<ItemProducao>,
    produto: Vec<ItemProducao>

}
#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
struct Composicao{
    cod: String,
    produto: String,
    insumos: Vec<Insumo>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Estoque{
    pub codigo: u32, //number
    pub produto: String,
    pub estoque: f32, //number
    pub custo: String, //number, custo de fab
    pub total: f32 //number, total de custo de fabricacao
}

//fn sazonalidade() -> Result<(), ()>{
//   Ok(())
//}
//
//fn estoque() -> Result<(), ()>{
//    Ok(())
//}
//
//fn material_reservado() -> Result<(), ()>{
//    Ok(())
//}
//
//fn levantamento_mrp(producao: Vec<Producao>, produtos: Vec<ProductResponse>, composicoes: Vec<Composicao>) -> Result<(), ()>{
//    Ok(())
//}

#[derive(Clone, Debug)]
struct Reqrequirementsrelatorios{
    producoes: serde_json::Value,
    composicoes: serde_json::Value,
    estoques: serde_json::Value,
}
impl Reqrequirementsrelatorios{
    fn standard()->Self{
        let today = Utc::now().date_naive();
        let today_string = today.to_string();
        println!("hoje(em standard reqrequirements): {}", &today_string);
        Self{
            producoes: json!({
            "tipoData": "dtInicio",
            "de": "2019-04-01",
            "ate": &today_string,
            "tags": "",
            "situacao": 0,
            "cods": "",
            "esconderValores": false,
            "mostrarQntPerda": true
        }),
            composicoes: json!({
            "codProds": "",
            "categoria": "",
            "tags": ""
        }),
            estoques: json!({
            "dia": &today_string,
            "categoria": "",
            "tags": "Almoxarifado",
            "semExcluidos": false,
            "semEstNaoControl": false,
            "mostrarEstoqueNegativo": false,
            "mostrarCodProprio": false,
            "apresentarArquivados": false
            })
        }
    }
    fn filter_composicoes(mut self, codprods: &str, categoria: &str, tags: &str)-> Self{
        self.composicoes = json!({
            "codProds": codprods,
            "categoria": categoria,
            "tags": tags
        });
        self
    }
    fn filter_producoes(mut self, tipodata: Tipodataproducao, de: &str, ate: &str, tags: &str, situacao: SituacaoProducao, cods: &str, escondervalores:bool, mostrarandperda: bool)-> Self{
        self.producoes = json!({
            "tipoData": tipodata.codigo(),
            "de": de,
            "ate": ate,
            "tags": tags,
            "situacao": situacao.codigo(),
            "cods": cods,
            "esconderValores": escondervalores,
            "mostrarQndPerda": mostrarandperda
        });
        self
    }
    fn filter_estoques(mut self, dia: &str, categoria: &str, tags: &str, semexcluidos: bool, semestnaocontrol: bool, mostrarestoquenegativo: bool, mostrarcodproprio: bool, apresentararquivados: bool) -> Self{
        self.estoques = json!({
            "dia": dia,
            "categoria": categoria,
            "tags": tags,
            "semExcluidos": semexcluidos,
            "semEstNaoControl": semestnaocontrol,
            "mostrarEstoqueNegativo": mostrarestoquenegativo,
            "mostrarCodProprio": mostrarcodproprio,
            "apresentarArquivados": apresentararquivados
            });
        self
    }
}

// Enum usado para requisicoes nos filtros de producao
enum Tipodataproducao{
    DTInicio,
    DTConclusao
}
impl Tipodataproducao{
    fn codigo(&self) -> &str{
    match self {
        Tipodataproducao::DTInicio => "dtInicio",
        Tipodataproducao::DTConclusao => "dtConclusao"
    }
}
}
enum SituacaoProducao {
    Todas,
    EmAberto,
    Concluidas
}
impl SituacaoProducao{
    fn codigo(&self) -> i32{
        match self{
            SituacaoProducao::Todas => 0,
            SituacaoProducao::EmAberto => 10,
            SituacaoProducao::Concluidas => 50 
        }
    }
}

enum RelatoriosEnum{
    All,
    Producoes,
    Composicoes,
    Estoques
}

#[derive(Debug, Clone)]
pub struct Relatorios{
    pub producoes: Vec<Producao>,
    pub composicoes: Vec<Composicao>,
    pub estoques: Vec<Estoque>
}

impl Relatorios{
    async fn get_all(client: &Client, token: &ERPToken, reqrequi: &Reqrequirementsrelatorios) -> Result<Self, Box<dyn std::error::Error>>{
        
        let res_rel_prod = client
            .post("https://api.egestor.com.br/api/v1/relatorios/producoesDetalhadas")
            .bearer_auth(&token.access_token)
            .header("Content-Type", "application/json")
.json(&reqrequi.producoes)
            .send()
            .await?;

        let rel_prod_status = res_rel_prod.status();
        let rel_prod_text = res_rel_prod.text().await?;

        println!("Relatorio Prod: {}", rel_prod_status);
        println!("Relatorio Prod Text: {}", rel_prod_text);
        let producoes: Vec<Producao> = serde_json::from_str(&rel_prod_text)?;
        let res_rel_comp = client
            .post("https://api.egestor.com.br/api/v1/relatorios/composicoes")
            .bearer_auth(&token.access_token)
            .header("Content-Type", "application/json")
            .json(&reqrequi.composicoes)
            .send()
            .await?;

        let rel_comp_status = res_rel_comp.status();
        println!("Relatorio Comp: {}", rel_comp_status);
        let composicoes: Vec<Composicao> = res_rel_comp.json().await?;
        let res_rel_est = client
            .post("https://api.egestor.com.br/api/v1/relatorios/estoqueDoDia")
            .bearer_auth(&token.access_token)
            .header("Content-Type", "application/json")
            .json(&reqrequi.estoques)
            .send()
            .await?;

        let rel_est_status = res_rel_est.status();
        println!("Relatorio Comp: {}", rel_est_status);
        let estoques: Vec<Estoque> = res_rel_est.json().await?;

        Ok(Relatorios{
            producoes,
            composicoes,
            estoques
        })

    }
    async fn dbg_print(&self, relatorio: RelatoriosEnum) -> (){ // vai somente dar um print
        match relatorio{
            RelatoriosEnum::All => {
                self.print_producoes();
                self.print_composicoes();
                self.print_estoques();
            },
            RelatoriosEnum::Producoes => self.print_producoes(),
            RelatoriosEnum::Composicoes => self.print_composicoes(),
            RelatoriosEnum::Estoques => self.print_estoques(),
        }

    }
    fn print_estoques(&self) -> (){
        if self.estoques.is_empty(){
            println!("Estoques vazio :(")
        }else{
            for estoque in &self.estoques{
                println!("--estoque--");
                println!("{}", estoque.codigo);
                println!("{}", estoque.produto);
                println!("{}", estoque.estoque);
                println!("{}", estoque.custo);
                println!("{}", estoque.total);
            }
        }

    }

    fn print_producoes(&self) -> () {
        if self.producoes.len() != 0{
            for producao in &self.producoes{
                let mut contador_insumo: i16 = 0;
                println!("*-*-*-*-PRODUCAO*-*-*-*-*");
                for produto in &producao.produto{
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
                for insumo in &producao.insumos{
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
        }else{
            println!("Producoes vazio :(")
        }
    }

    fn print_composicoes(&self) -> () {
        if self.composicoes.len() != 0{
            for composicao in &self.composicoes {
            println!("-*-*-*-*-*-*COMPOSICAO*-*-*-*-*-*-");
            println!("cod: {}", composicao.cod);
            println!("produto: {}", composicao.produto);
            for insumo in &composicao.insumos {
                println!("---insumo---");
                println!("cod insumo: {}", insumo.codinsumo);
                println!("insumo: {}", insumo.insumo);
                println!("codproprio: {}", insumo.codproprio);
                println!("unidade: {}", insumo.unidade);
                println!("quant: {}", insumo.quant);
                println!("p perda: {}", insumo.pperda);
            }
            }
        }else{
            println!("Composicoes vazio :(")
        }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoMovimento{
    Entrada,
    Retirada
}
impl fmt::Display for TipoMovimento{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self{
            TipoMovimento::Entrada => "Entrada",
            TipoMovimento::Retirada => "Retirada"
        };
        write!(f, "{}", s)
    }

}

#[derive(Debug, Clone)]
pub struct ItemRetirada{
    pub codigo: u32,
    pub produto: String,
    pub tipo: TipoMovimento,
    pub quantidade: f32,
    pub estoqueatual: f32,
}

#[derive(Serialize, Debug, Clone)]
struct ItemResumo{
    #[serde(rename="codProduto")]
    codproduto: u32,
    #[serde(rename="estoqueFinal")]
    estoquefinal: f32
}

#[derive(Debug, Clone)]
pub struct AjusteEstoque{
    pub estoque: Vec<Estoque>,
    pub carrinhoretirada: Vec<ItemRetirada>,
    pub resumoretirada: Vec<ItemResumo>,
    pub obs: String
    // historico: À FAZER AINDA
}

impl AjusteEstoque{
    pub fn new() -> Self{
        Self{
            estoque: Vec::new(),
            carrinhoretirada: Vec::new(),
            resumoretirada: Vec::new(),
            obs: String::new()
        }
    }

    pub fn get_estoque(&mut self, estoque: Vec<Estoque>)-> &mut Self{
        self.estoque = estoque;
        self
    }

    pub fn add_item_carrinho(&mut self, mut item: ItemRetirada) -> &mut Self{
        if let Some(itemestoque) = self.estoque.iter_mut().find(|i| i.codigo == item.codigo){
            let i_item_estoque = itemestoque.estoque as f32;
            item.estoqueatual = i_item_estoque;

            match item.tipo{
                TipoMovimento::Entrada =>(),
                TipoMovimento::Retirada=>{
                    item.quantidade = -item.quantidade
                },
            };

            if let Some(itemlista) = self.carrinhoretirada.iter_mut().find(|i| i.codigo == item.codigo){
                itemlista.quantidade += item.quantidade;
            }else{
            self.carrinhoretirada.push(item);
            }
        }else{
            println!("Item nao encontrado em estoque")
        }
        self
    }

    pub fn del_item_carrinho(&mut self, codigo: u32){
        self.carrinhoretirada.retain(|item| item.codigo != codigo)
    }

    pub fn resumir(&mut self){
        if self.carrinhoretirada.is_empty(){
            println!("Adicione itens no carrinho, meu querido!")
        }else{
            for item in &self.carrinhoretirada{
                let texto_item = &item.produto.to_lowercase();
                let texto_item = &item.produto.trim();
                if let Some(start) = &texto_item.find("(almoxarifado"){
                    let inicio_codigo = start + "(almoxarifado".len();
                    if let Some(fim_rel) = texto_item[inicio_codigo..].find(")"){
                        let fim_codigo = inicio_codigo + fim_rel;
                        let mut codigo = texto_item[inicio_codigo..fim_codigo].trim().to_string();
                        codigo.retain(|c| c != ':');

                        println!("{}", codigo.trim())
                    };
                }
                self.resumoretirada.push(ItemResumo{
                    codproduto: item.codigo,
                    estoquefinal: (item.estoqueatual+item.quantidade)
                })
            }
        };
    }
    pub async fn realizar_operacao(self, client: Client, token: ERPToken){
        let req: serde_json::Value = json!({
            "codContato": 40,
            "motivo": 10,
            "obs": self.obs,
            "tags": ["Sistema Almoxarifado"],
            "produtos": self.resumoretirada
        });

        let post = client
            .post("https://api.egestor.com.br/api/v1/ajusteEstoque")
            .bearer_auth(token.access_token)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await;
        match post{
            Ok(resp) => {
                println!("Status: {}", resp.status());
                if let Ok(body) = resp.text().await {
                    println!("Resposta: {}", body);
                }
            }
            Err(e) => println!("Erro: {}", e)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppLogic{
    pub token: ERPToken,
    pub reqs: Reqrequirementsrelatorios,
    pub relatorios: Relatorios,
    pub ajuste_estoque: AjusteEstoque
}

impl AppLogic{
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let client = Client::new();
        let token = ERPToken::new(&client).await?;
        let reqs = Reqrequirementsrelatorios::standard();
        let relatorios = Relatorios::get_all(&client, &token, &reqs).await?;
        let ajuste_estoque = AjusteEstoque::new();

        Ok(Self{
            token,
            reqs,
            relatorios,
            ajuste_estoque
        })
    }
}
// impl AppLogic{
//     fn start(client: Client) -> Self{
//         AppLogic {
//             token: ERPToken::new(client),
//             reqs: Reqrequirements::standard(),
//             relatorios: Relatorios::get_all()
//
//         }   
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let today = Utc::now().date_naive();
//     let today_string = today.to_string();
    // let client = Client::new();
//     let token: ERPToken = ERPToken::new(client.clone()).await?; //token acesso
//     let reqrequirements = Reqrequirementsrelatorios::standard().filter_estoques(&today_string, "", "Almoxarifado", false, false, true, false, false);
//     let relatorios = Relatorios::get_all(client.clone(), token.clone(), reqrequirements.clone()).await?;
//     relatorios.print_composicoes();
//     relatorios.print_producoes();
//     relatorios.print_estoques();
//     let mut ajuste_estoque = AjusteEstoque::new();
//     ajuste_estoque.get_estoque(relatorios.estoques)
//         .add_item_carrinho(ItemRetirada { codigo: 669, produto: "".to_string(), tipo: TypoMovimentacao::Retirada, quantidade: 1, estoqueatual: 0 })
//         .resumir();
//     ajuste_estoque.realizar_operacao(client, token).await;
//
//     Ok(())
// }
