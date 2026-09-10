# Moradores e porta de enrolar

Assets candidatos produzidos em 10 de setembro de 2026 para a
[entrega 32](../../../docs/32-cinematic-neighbourhood-arrival.md), seguindo a
[ADR 0026](../../../docs/adr/0026-cinematic-arrival-and-neighbours.md).
Os moradores e a porta são peças independentes da fachada pintada.

## Conjunto e identidade

| Arquivo | Conteúdo |
|---|---|
| [neighbours.png](neighbours.png) | Mulher de blusa mostarda com sacola e jovem de camiseta roxa com mochila; cada identidade tem uma pose calma e quatro de corrida para a esquerda. |
| [shopkeeper.png](shopkeeper.png) | Senhor de cabelos/bigode grisalhos, camisa clara e avental verde; calmo, alarmado, puxando alto/baixo e quatro poses de corrida para a esquerda. |
| [shutter.png](shutter.png) | Painel metálico corrugado verde-azulado, frontal, com puxador inferior separado visualmente das lâminas. |
| [caramelo.png](caramelo.png) | Cão e poses documentados separadamente em [CARAMELO.md](CARAMELO.md). |

As duas pessoas do primeiro atlas e o lojista formam três identidades
originais. Instâncias podem reutilizar a mesma identidade no ponto e na
mercearia. Sacola, mochila, figurino e fisionomia acompanham suas poses.
O lojista espera com os pés apoiados e dispõe de dois gestos frontais para
puxar a porta; o desenho não inclui porta nem barra em suas mãos.

A fachada em [props.png](props.png) e a captura local da rua aprovada foram
inspecionadas para orientar paleta, luz matinal e contornos. Todos os PNGs
novos desta página foram gerados pelo `image_gen` integrado a partir de
descrições textuais, sem imagem de entrada. Não se usaram pessoas reais,
marcas ou referências externas. A rua aprovada está registrada nas
[evidências da rodada 31](../../../docs/evidence/brazilian-street-evacuation/README.md).

## Recortes, apoios e animação

[neighbours.json](neighbours.json), [shopkeeper.json](shopkeeper.json) e
[shutter.json](shutter.json) registram dimensões, retângulos e pivôs. `rects`
usa coordenadas da imagem inteira; `pivots` é relativo ao canto superior
esquerdo do respectivo recorte. `groups` associa índices às peças planejadas
para o catálogo.

| Grupo | Frames | Altura de referência |
|---|---|---:|
| `resident.0.idle` / `resident.0.run` | 0 / 1–4 em neighbours | 437 px |
| `resident.1.idle` / `resident.1.run` | 5 / 6–9 em neighbours | 448 px |
| `shopkeeper.idle` / `shopkeeper.alert` | 0 / 1 em shopkeeper | 475 px |
| `shopkeeper.pull` | 2–3 em shopkeeper | Mesma escala do lojista calmo |
| `shopkeeper.run` | 4–7 em shopkeeper | Mesma escala do lojista calmo |

Calcular uma escala por identidade a partir da altura em pé e conservá-la
entre poses. Os pivôs horizontais acompanham aproximadamente o quadril;
os verticais usam chão virtual comum por linha. Os recortes incluem margem
transparente até esse chão para preservar suspensão e joelhos flexionados.
Normalizar cada recorte pela própria altura produziria mudanças de tamanho.

As corridas apontam para a esquerda, em direção à mercearia. As poses calmas
têm leve giro de três quartos, enquanto as de puxar a porta são frontais.
Os pontos das mãos nas duas poses de puxada estão no JSON do lojista para
ajudar a alinhar o tirador na captura real. São aproximações visuais, não
um esqueleto de animação.

## Porta de enrolar

A porta é um painel frontal sem parede, moldura externa ou trilhos. Revelar
uma tira da **parte inferior** da imagem faz o puxador acompanhar a borda
descendente, mantendo as lâminas na mesma escala. A altura aparece por
recorte, sem esticar o metal. O centro e a extensão do puxador estão medidos
no JSON para contato com as mãos.

O recorte mede 876 × 1357 px. A composição atual usa 97 px de largura no
mundo, resultando em altura inteira de aproximadamente 150 px; o renderer
limita a parte visível à abertura de 122 px. O centro inferior da entrada é
`(603, 355)`, conforme o [guia da rua](README.md). A porta deve atingir o chão
e ocultar o interior depois que todos entrarem; trajetória, oclusão e ordem
de desenho pertencem ao renderer. A sugestão de 83 px nos metadados de
produção registra a medida inicial; o catálogo runtime guarda a escala
ajustada após a revisão da fachada.

## Fontes e integridade

| PNG | Dimensões | SHA-256 |
|---|---|---|
| neighbours | 1536 × 1024 | `40bdacd3abad00a17d9cdce266c5e381345ae25ef5d7e22c69107519acc3cba2` |
| shopkeeper | 1536 × 1024 | `00cf325015c8ddf3c52b035bbc0dbd1903be35441638d5bfaf1fdde651c6d409` |
| shutter | 1036 × 1519 | `b11cca82fad5f8d03d135c37976e19c593c0f5ddd991c913c1069acb09e1a9e0` |

As fontes permanecem em
`/home/willams/.codex/generated_images/01a08b8b-3f76-7df3-b45e-58333ee12b57/`:

- Neighbours: `exec-60b6a106-f2a1-4e2f-9b4b-c16e9413348d.png`;
  [prompt completo](prompts/neighbours.txt).
- Shopkeeper: `exec-f950aba1-2d18-46e2-95b4-78f03effa76a.png`;
  [prompt completo](prompts/shopkeeper.txt).
- Shutter: `exec-726cc9f1-36f1-4d90-9c5f-043e766a9179.png`;
  [prompt completo](prompts/shutter.txt).

Os três resultados selecionados possuem alpha real. Pixels totalmente
transparentes: 1.075.914 em neighbours, 1.052.236 em shopkeeper e 389.585 em
shutter. A análise usa `alpha > 3`, margem de dois pixels e extensões de chão
virtual, todas registradas nos metadados. PNGs copiados byte a byte: Python
apenas leu pixels e escreveu JSON, sem retoque, recorte, redimensionamento,
remoção de fundo ou regravação raster.

Conferidos alpha, SHA-256, retângulos separados, apoios internos, direção,
identidades, presença do tirador e links locais. A revisão nativa integrada
conferiu escala, corrida até a entrada, oclusão pelo batente, espera pelo
último visitante e contato da porta com o chão, sem fresta lateral.
[Capturas e prévia](../../../docs/evidence/cinematic-neighbourhood/README.md).
