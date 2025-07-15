OBJETIVOS:
    [x] Aprender usar a API.(Fiz primeiras requisições.)
    [] Passar codigos feitos para funcao "levantamento_mrp"
    [] Fazer o levantamento de MRP com base nos arquivos levantados
    [] Salvar conteúdos coletados em arquivo .csv ou json


 - - - - - - - -  - - - -  - - - -  - - - -  - - -  

LIXO:
--

---
*producao detalhada(exemplo):*

{"cod":"84","insumos":
    [
        {"tipo":"Insumo","codProduto":"36","IxProd":"MO - HORA MAQUINA (INJETORAS: 3-4-5-6)","IcProd":"5000036","IuCom":"H","quant":8.311096,"pPerda":0,"qntPerda":0,"custoInsumo":831.1095999999999,"custoUnit":100,"custoExtra":0,"custo":831.1095999999999},
        {"tipo":"Insumo","codProduto":"62","IxProd":"0041 ABS NATURAL","IcProd":"3100062","IuCom":"","quant":71.3048,"pPerda":0,"qntPerda":0,"custoInsumo":0,"custoUnit":0,"custoExtra":0,"custo":0},
        {"tipo":"Insumo","codProduto":"115","IxProd":"MASTER PP PRETO","IcProd":"3000115","IuCom":"KG","quant":1.4552,"pPerda":0,"qntPerda":0,"custoInsumo":0,"custoUnit":0,"custoExtra":0,"custo":0}
    ],
    "produto":
        [
            {"tipo":"Produto","codProduto":"222","IxProd":"PALA P\/ VISEIRA CAP. PROTORK - SPORT MOTO ( SM ) - AUTOMATICO","IcProd":"4100222","IuCom":"UN","quant":680,"pPerda":0,"qntPerda":0,"custoInsumo":831.1096,"custoUnit":1.22222,"custoExtra":0,"custo":831.1096}
        ]
    }

---
*composicao(exemplo)*

{"cod":"68","produto":"TAMPINHA DO CORPO COMPARADOR","insumos":
    [
        {"codInsumo":"69","insumo":"POLIETILENO BAIXA DENSIDADE","codProprio":"3100069","unidade":"KG","quant":"0.0007330400","pPerda":"0.0000"},
        {"codInsumo":"70","insumo":"MASTER PEBD AZUL BIC","codProprio":"3000070","unidade":"KG","quant":"0.0000149600","pPerda":"0.0000"},
        {"codInsumo":"36","insumo":"MO - HORA MAQUINA (INJETORAS: 3-4-5-6)","codProprio":"5000036","unidade":"H","quant":"0.0010463000","pPerda":"0.0000"}
    ]
}

---

"codigo":16,"descricao":"BOMBA DE SUCCAO DE 1\" PARA BANHEIRO QU\u00cdMICO","codigoProprio":"2000016","estoque":25,"estoqueMinimo":0,"controlarEstoque":true,"margemLucro":0.01,"precoCusto":28.13,"precoVenda":28,"origemFiscal":0,"unidadeTributada":"UN","refEanGtin":"","ncm":"","excecaoIPI":0,"codigoCEST":"","pesoBruto":0,"pesoLiquido":0,"codigoGrupoTributos":1,"anotacoesNFE":"","anotacoesInternas":"","tipoProduto":"produto","tags":["Biplas","Bomba"],"dtCad":"2025-04-03 14:45:55","codCategoria":12,"updatedAt":"2025-06-25 11:15:59","listaImagens":[{"foto":{"link":"https:\/\/v4.egestor.com.br\/files\/?key=37975.500.9ad7.91fc.ee0"},"thumb":{"link":"https:\/\/v4.egestor.com.br\/files\/?key=37975.501.075e.a71e.9e1"}},{"foto":{"link":"https:\/\/v4.egestor.com.br\/files\/?key=37975.501.17af.902b.b03"},"thumb":{"link":"https:\/\/v4.egestor.com.br\/files\/?key=37975.501.2b5f.0521.e37"}}]]


14/07/2025
    duvida: input para levantamento mrp(producao x venda).

