# Veículos modulares da rua

Arte candidata produzida em 10 de setembro de 2026 para a
[entrega 31](../../../docs/31-brazilian-street-evacuation.md), conforme a
[ADR 0025](../../../docs/adr/0025-replaceable-street-pieces.md).

O [atlas](vehicles.png) contém seis desenhos independentes, em perfil para a
direita, com silhuetas e proporções próprias. Os
[metadados](vehicles.json) associam IDs estáveis a arquivo, recorte, apoio,
largura sugerida e centros das rodas. Um ID pode passar a usar um PNG
individual sem redesenhar o bairro nem alterar os outros veículos.

| ID | Silhueta e paleta | Largura inicial sugerida |
|---|---|---:|
| `vehicle.hatch` | Hatch compacto vermelho, traseira curta e teto arredondado. | 147 px |
| `vehicle.sedan` | Sedã prata, teto baixo, entre-eixos longo e porta-malas separado. | 172 px |
| `vehicle.pickup` | Picape compacta branca, cabine dupla e caçamba curta aberta. | 169 px |
| `vehicle.suv` | SUV grafite, teto alto, trilhos e caixas de roda marcadas. | 160 px |
| `vehicle.bus` | Ônibus urbano branco, azul e amarelo, duas portas e faixa ampla de janelas. | 440 px |
| `vehicle.van` | Furgão branco de entregas, compartimento fechado sem janelas laterais. | 149 px |

Esses valores servem à revisão em cena. O ônibus foi desenhado mais comprido
que os carros e deve usar largura maior no mundo; não normalizar todos os
veículos ao mesmo tamanho. O carro azul do acidente conserva seu atlas próprio
e seu registro entre a pose íntegra e a amassada.

## Referências consultadas

Páginas primárias consultadas em 10 de setembro de 2026 para orientar a
variedade de carrocerias oferecidas no Brasil. As páginas orientam categoria
e proporção geral; nenhum bitmap delas foi passado ao gerador ou incorporado
aos arquivos do jogo. Os desenhos são genéricos e não reproduzem emblemas,
nomes de modelos, placas ou grafismos de operadores reais.

| Referência primária | Papel nesta exploração |
|---|---|
| [Fiat Argo](https://argo.fiat.com.br/) | Hatch compacto contemporâneo. |
| [Toyota Corolla](https://www.toyota.com.br/modelos/corolla) | Sedã com cabine baixa e porta-malas destacado. |
| [Fiat Strada](https://strada.fiat.com.br/) | Picape compacta com cabine dupla e caçamba. |
| [Volkswagen T-Cross](https://www.vw.com.br/pt/carros/t-cross.html) | SUV compacto alto, distinto do hatch. |
| [Marcopolo Torino Low Entry](https://onibus.marcopolo.com.br/produtos/urbanos/torino-low-entry) | Corpo longo de ônibus urbano com acesso baixo. |
| [Fiat Fiorino](https://fiorino.fiat.com.br/) | Utilitário de entregas com volume de carga fechado. |

A seleção representa variedade visual plausível, sem afirmar participação de
mercado ou fidelidade a modelos específicos. O estilo usa a luz matinal e os
contornos já observados nas capturas da rua: superfícies pintadas, janelas
verde-azuladas, pneus escuros e detalhes que sobrevivem ao tamanho de jogo.

## Geração e integridade

- Ferramenta: `image_gen` integrada, uma geração textual, sem imagem de entrada.
- [Prompt integral selecionado](prompts/vehicles.txt).
- Fonte: `exec-543de9de-499e-4410-aeaa-82a39e0fe967.png`, preservada em
  `/home/willams/.codex/generated_images/01a08b8b-3f76-7df3-b45e-58333ee12b57/`.
- PNG RGBA de 1254 × 1254, copiado byte a byte para [vehicles.png](vehicles.png).
- SHA-256: `cd00c1af32f8a2e2e7d98221c98680b12a2247eab9fa39640ed8224eca4cea7c`.
- Alpha: 957.396 pixels transparentes, 613.630 intermediários e 1.490 opacos.
  O fundo é transparente de fato, sem xadrez pintado.

A grade visual possui duas colunas e três linhas, mas as divisões das colunas
variam: o ônibus usa mais espaço horizontal e o furgão usa menos. O runtime
deve ler os recortes explícitos, sem calcular células iguais pela imagem.
As janelas de inspeção estão no JSON para permitir repetir a medição.

`source` usa pixels da imagem inteira. `pivot` e `wheel_centers` usam pixels
relativos ao canto superior esquerdo desse recorte; `wheel_centers_source`
registra os mesmos centros na imagem inteira. O pivô é o apoio inferior do
veículo. As rodas foram estimadas por amostragem dos contornos do alpha e
inspeção visual, suficientes para pequenos brilhos animados dos cubos.

Os recortes usam `alpha > 3` e margem de dois pixels. A análise somente leu
pixels e escreveu metadados: não recortou, redimensionou, pintou, removeu fundo
nem regravou o PNG. Placas, ponto de ônibus, pessoas, bicicletas, poste,
asfalto, partículas e movimentos pertencem a peças ou camadas separadas.

## Verificação desta entrega de arte

Conferidos alpha real, seis recortes sem sobreposição, limites da imagem,
pivôs/rodas internos, consistência das coordenadas e SHA-256. A leitura de
ônibus, picape, sedã, hatch, SUV e furgão é distinta no atlas. Escala final,
sobreposição e evacuação devem ser avaliadas nas capturas do runtime.

Na quarta rodada, a escala do ônibus passou de 280 para 440 px de largura
(≈129 px de altura), preservando a proporção do PNG. Os carros medem 147–172 px
de largura. A revisão anterior foi aprovada com o pedido de aumentar o ônibus.
