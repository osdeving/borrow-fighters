# Peças do capítulo — Depois do silêncio

Assets da [entrega 33](../../../docs/33-after-the-silence.md), seguindo a
[ADR 0027](../../../docs/adr/0027-chapter-spatial-direction.md). São camadas
independentes para atuação e cenário dentro do jogo. A rua anterior, moradores,
acidente e porta continuam disponíveis nos arquivos existentes da aventura.

| Arquivo | Conteúdo |
|---|---|
| [rust-narrative.png](rust-narrative.png) e [metadados](rust-narrative.json) | Oito poses de Rust para verificar o motorista, conversar e usar um celular separado. |
| [driver.png](driver.png) e [metadados](driver.json) | Motorista em duas poses, conversa e agradecimento, para a esquerda. |
| [lane.png](lane.png) e [metadados](lane.json) | Referência histórica de pintura; substituída no runtime por [fachadas e chão modulares](../world/README.md). |

Os JSONs registram medidas de produção. Não decidem triggers, progressão,
colisão, trajetórias ou câmera. Os retângulos usam pixels absolutos do PNG;
pivôs e sockets usam pixels relativos ao respectivo recorte. Preserve uma única
escala por identidade, mesmo quando a pose muda a altura ou a largura visível.

## Rust e telefone independente

O atlas de Rust tem 1536 × 1024 px. A altura de referência é **504 px**;
para uma figura de 230 px, use escala `230 / 504`, multiplicada pela escala de
profundidade escolhida pela cena. `pivots` acompanha o centro dos apoios dos
calçados em uma linha de chão comum, sem centralizar o sprite pelo braço aberto.
Os pontos foram medidos visualmente, não correspondem a um esqueleto.

| Índice | Pose | Aparelho visível |
|---|---|---|
| 0 | Verificar o motorista, inclinação e mão oferecida. | Não |
| 1 | Conversar com palma aberta. | Não |
| 2 | Alcançar o bolso. | Não |
| 3 | Elevar a mão que receberá o telefone. | Sim |
| 4 | Ler, olhando para a mão. | Sim |
| 5 | Digitar com a outra mão. | Sim |
| 6 | Baixar a mão após a leitura. | Sim |
| 7 | Guardar, com a mão junto ao bolso. | Não |

Nenhuma pose contém celular. `sockets[].phone_grip` identifica o contato da
palma; `hand` oferece o mesmo marco com nome genérico. Os ângulos sugeridos
servem como ponto inicial. Um aparelho de aproximadamente 28 × 62 pixels da
fonte, com apoio em `(0,5; 0,86)` de seu próprio retângulo, cabe junto à pega.
O renderer precisa preservar a oclusão dos dedos e transformar o aparelho
junto com o corpo e a câmera. As poses de bolso/guardar mantêm o aparelho
oculto; a fase da encenação determina quando ele aparece, se move e some.
Conferir o contato em movimento antes de aprovar a sequência final.

`groups` sugere seleções de frames, sem impor duração de conversa. O gesto de
digitar pode alternar 4/5, enquanto mensagens, sons e o painel ampliado seguem
o relógio da encenação. Os pés permanecem no apoio do caminho; não deformar
as pernas para fazer a figura chegar a um ponto de interação.

## Travessa e composição

O fundo tem céu, fachadas, muros, portões arquitetônicos, árvores e uma faixa
horizontal livre de asfalto. A imagem inteira cabe num enquadramento próximo
de 16:9; o chão de atuação sugerido é `y580` numa vista de 720 px de altura.
A guia visual de asfalto começa perto de `y650` da fonte. Essas medidas não
substituem a geometria explícita do capítulo.

A imagem não é um tile contínuo. Reutilize-a como base de trechos separados,
com recorte e câmera limitados à composição local; os portões desenhados ao
fundo não criam automaticamente regiões de interação. Barreira, passagem,
atores e objetos móveis devem continuar em peças independentes.

## Procedência e integridade

Produção em 10/09/2026 pelo `image_gen` integrado, sem CLI/API externa.
Os PNGs selecionados foram copiados byte a byte; scripts leram alpha e
escreveram somente metadados. Não houve recorte, pintura, remoção de fundo,
redimensionamento ou regravação de pixels por scripts.

| Seleção | Fonte gerada | SHA-256 |
|---|---|---|
| Rust | `exec-f804f2e7-0080-4eb3-99dc-26cf897b2133.png` | `60a424907da69377a8bc856dc7a48a4fcca00e96a5e0cbac68aac1c60fce5368` |
| Travessa | `exec-63f0ff26-6eb8-45bb-8426-1d4e67d6436f.png` | `d8173ad8e2d256308a3f5fd1db063f95328bfb5fac647f11acb32d672bb23c18` |

As duas fontes acima permanecem em
`/home/willams/.codex/generated_images/01a08b8b-3f76-7df3-b45e-58333ee12b57/`.
O motorista possui [procedência própria](DRIVER.md), incluindo seu prompt,
fonte e hash.

O Rust selecionado é RGBA, com **868.262 pixels de alpha zero**, **704.602
intermediários** e nenhum de alpha 255. As regiões vazias possuem transparência
real; seus canais RGB podem guardar cores residuais invisíveis. Não confundir
um visualizador que ignora alpha com o resultado de composição no jogo.
O fundo da travessa é RGB opaco de **1672 × 941 px**, intencionalmente.

A identidade de Rust foi conferida em
[rust-actions.png](../rust-actions.png) e [rust-morning.png](../rust-morning.png):
cabelo castanho, goggles, moletom carvão/laranja, luvas sem dedos, calça cargo,
tênis e pequeno adorno no quadril. A seleção final usou somente o
[prompt compacto](prompts/rust-narrative-compact.txt), após inspeção dessas
referências internas. Seu acabamento é mais arredondado; verificar a
continuidade com os sprites anteriores no tamanho de jogo.

A travessa usa o [prompt completo](prompts/lane.txt) e o painel externo de
[prologue-environments.png](../prologue-environments.png) como referência de
pintura e luz. Foi o fundo de nível vazio da primeira entrega; permanece como referência, sem ser esticado ou carregado pelo capítulo atual.
Não foram usadas referências de imagens externas adicionais.

### Tentativas não integradas

Os resultados abaixo ficaram somente no diretório de geração. Os prompts
foram preservados para rastrear a seleção e evitar repetir tentativas inválidas.

| Fonte gerada | Prompt | Motivo da rejeição |
|---|---|---|
| `exec-fe191b46-91af-4bb6-a91e-a561d2daff9f.png` | [Oito poses com referências](prompts/rust-narrative.txt) | RGB com xadrez pintado. |
| `exec-eaf8b91f-09ed-4d39-88af-37e685de54e5.png` | [Correção de alpha](prompts/rust-narrative-alpha.txt) | Continuou RGB com xadrez. |
| `exec-3a210ac7-c07d-4e21-b3c5-290a0e73b189.png` | [Geração transparente por descrição](prompts/rust-narrative-transparent.txt) | RGBA válido, mas corpo alongado em relação à identidade existente. |
| `exec-16dc03e9-587a-4968-9322-5989a69c9b1f.png` | [Atlas quadrado](prompts/rust-narrative-square.txt) | RGB com xadrez pintado. |

A verificação desta pasta cobre integridade dos arquivos, alpha, recortes,
apoios, sockets e links. Aproximações, contato do telefone, profundidade,
pausa e entrega de controle dependem da integração e captura nativa do capítulo.

## Edição no jogo

- [`world.json`](world.json): posições de pés, limites dos três espaços,
  áreas de interação, pontos de aproximação, saída e obstáculo. O chão é y580;
  Rust reduz escala ao caminhar até a profundidade do carro ou da porta.
- [`chapter-texts.json`](chapter-texts.json): falas, objetivos e três mensagens.
  F5 recarrega este arquivo durante o capítulo.
- [`phone-style.json`](phone-style.json): cores, contato e rótulos do mensageiro;
  F5 recarrega junto com os textos, apenas se ambos validarem.
- [`catalog.json`](catalog.json): PNG/recorte/pivô por frame, duração em ticks de
  60Hz, `looping` e sockets `phone`/`phone_tip`. Os dois pontos definem posição
  e orientação do aparelho; usam o mesmo espelho/escala/rotação da pose.
  Clips não cíclicos seguram o frame final. A validação rejeita um gesto que
  perder o socket enquanto o telefone deveria estar visível.
- O aparelho é desenhado separadamente em
  [`phone.rs`](../../../src/adventure/engine/chapter/phone.rs), antes do corpo
  para os dedos o ocluírem. A ampliação da conversa é uma camada de interface,
  acionada pelo mesmo relógio; não contém conexão externa.
- O caixote reutiliza somente o recorte da caixa superior em
  [`street/props.png`](../street/props.png), sem gerar ou duplicar bitmap.
  Seu retângulo visível120×60 acompanha o sólido de `world.json`.
- [`audio/`](audio/README.md): sons individuais de bolso, toque, envio e resposta.

Trocar imagem/recorte/duração não exige repintar fundo ou demais personagens.
Mudanças de catálogo/geometria entram na próxima sessão; F3 permite conferir
rotas, limites e contatos. O capítulo usa câmera/translação interpoladas e
clips de poses, sem engine adicional, rig esquelético ou vídeo pré-renderizado.

## Carga quebrável e encontro com duas EPs

`world.json` separa as nove instâncias de carga de seus desenhos: `debris[].id`
é a identidade da caixa; `piece` e `fragment` referenciam o catálogo; `region`
define o volume físico e `hp` a resistência. As caixas intactas reutilizam
`prop.crate` e `prop.crate_teal`; lascas usam `prop.crate_fragment`, um recorte
independente do material da caixa. Troque essas referências para substituir uma peça sem recriar o fundo.
Os recortes reaproveitam `street/props.png` sem regravar pixels. `rotation`
aplica pequenas inclinações autorais; `loose_props[]` posiciona tábuas e lascas
independentes no chão sem criar colisões ocultas.

Cada soco ou chute acerta uma única peça uma vez. Trincas, poeira e fragmentos
seguem o relógio do contato; a peça deixa de colidir ao quebrar. Caixas acima
caem por gravidade e se apoiam nas que restam, sem flutuar no local antigo.
O chute da aventura usa **V / RT**, alcança mais longe que o soco e mantém o
pulo do capítulo em **Espaço / B**. Soco leve, forte e chute causam respectivamente
12, 24 e 18 pontos; caixas têm 24 ou 36 pontos de resistência.

A passagem lê `enemies[]` do mesmo `world.json`: identidade, posição, vida,
velocidade, preparação, recuperação e dano de cada EP são editáveis. As duas
instâncias compartilham o sprite de EP, mantendo corpos, ataques, vida e barras
independentes. Vitória exige derrotar ambas. A configuração revisada usa
120/112 PV, velocidade 175/165, preparação de 30/34 ticks e dano 15, preservando
as oportunidades de defesa e interrupção do prólogo.

O carregamento rejeita identidades duplicadas, regiões inválidas, resistência
fora dos limites, EPs sobrepostas e referências de sprites inexistentes.
Alterações de geometria, resistência, elenco ou catálogo entram na próxima
sessão; os textos e instruções continuam recarregáveis por **F5**.

Checkpoint guarda marcos seguros: retomar `lane_start` repõe a carga inicial;
`lane_cleared` mantém a travessa livre. Derrota na passagem restaura as duas EPs
e Rust no encontro local, sem repetir a demolição nem as conversas.
A revisão automática usa um save próprio e registra todos os objetos e inimigos
em `telemetry.jsonl`.
