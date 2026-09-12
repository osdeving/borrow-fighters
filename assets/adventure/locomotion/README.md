# Movimento de Rust

A/D produz uma corridinha nas ruas do prólogo e do capítulo, a 455 px/s,
com aceleração e freio. O quarto, as aproximações de NPCs e o pesar após o
encontro mantêm as oito poses anteriores de caminhada cuidadosa. As quatro
poses anteriores de chute também permanecem independentes e intactas.

- `catalog.json`: cada frame aponta para seu próprio PNG; `source`, `anchor`,
  escala e sockets ficam nos dados. O apoio acompanha o quadril e a sola,
  evitando recentralizar o corpo pelo recorte variável dos braços e das botas.
- `motion.json`: nomes dos clips e distância de uma passada completa. O
  cursor acompanha distância realmente percorrida no chão, inclusive nas
  aproximações e no quarto. Pausa, obstáculo e velocidade menor preservam o
  apoio. A escala em profundidade ajusta também o comprimento da passada.
- `run-rig.json`: poses e interpolação da corrida, com comprimentos anatômicos,
  recuperação dos pés, compressão do quadril, inclinação e balanço dos braços.
  O apoio move-se para trás à velocidade do chão; o solver de duas articulações
  encontra os joelhos sem modificar colisão ou posição física. Sockets `sole`
  e dos ombros prendem as botas rígidas e os braços aos pontos anatômicos.
  `air_ankle` alinha o tênis à canela durante a recuperação, com flexão relativa
  e entrada/saída graduais. A rotação ocorre depois de resolver o tornozelo;
  posições das juntas e apoio no chão permanecem independentes desse ajuste.
- `run-mesh.json`: landmarks, subdivisões e pesos da textura contínua
  `run-mesh/leg.png`, compartilhada pelas duas pernas. A mistura entre os ossos
  e a correção localizada conservam o volume na dobra do joelho. Faixas do
  tronco ajustam a relação tronco/pernas, preservando cabeça e emblema em escala
  1:1. Arte, proporções, pesos e alvos de apoio podem ser corrigidos separadamente.
- `crossing/`: oito poses de costas e oito de frente para atravessar a rua.
  `back_walk_clip` e `front_walk_clip` em `motion.json` selecionam os clips.
  A direção real do próximo segmento escolhe costas na ida ao passeio distante,
  frente na volta e a caminhada lateral em segmentos horizontais. As rotas,
  posições e regiões de conversa não dependem da arte escolhida.
- `waking.json`: poses existentes do despertar, apoios no colchão/assento/chão,
  assentamento interpolado em torno desses apoios e caminho até a porta.
  Trocar um ponto do caminho não exige modificar a imagem do quarto.

O chute mantém a extensão nos ticks 10–16 e termina sua recuperação no tick 34,
em concordância com o domínio de combate. As poses não definem dano ou hitbox.
Os passos do capítulo seguem o mesmo comprimento de passada dos dados: 128 px
na caminhada e 272 px na corrida, com dois contatos por ciclo. A pausa e uma
retomada durante suspensão não disparam um novo passo até o próximo contato.
F5 recarrega PNGs, clips e estes dados durante o prólogo e o capítulo.
Um conjunto inválido conserva todos os recursos de animação anteriores; os
relógios da cena, combate e distância percorrida permanecem em andamento.

## Procedência

Assets gerados com a ferramenta integrada `image_gen` em 12/09/2026, usando
`assets/adventure/rust-actions.png` como referência de identidade e estilo.
A fonte preservada em `source/rust-walk-kick.png` é a revisão com fundo de
extração magenta. O primeiro resultado tinha xadrez RGB, portanto não foi usado.
O [prompt inicial](source/prompt.txt) pede os oito apoios alternados e quatro
poses de chute. O [prompt da revisão](source/revision-prompt.txt) preservou a identidade, baixou o pé de passagem,
inverteu o balanço dos braços na segunda metade e substituiu o xadrez por
magenta uniforme para extração.

Separação determinística das células, sem redesenhar o personagem:

```sh
bash tools/art/extract_rust_locomotion.sh
```

A chave cromática remove também a borda magenta e os recortes excluem pontas
da linha seguinte da fonte. PNGs runtime possuem alpha; a RGB oculta em pixels transparentes pode continuar
magenta. Conferir as bordas compondo sobre um fundo cinza, e não apenas olhando
um visualizador que ignore alpha. A fonte e os prompts não pertencem ao pacote
runtime. Cada pose pode ser corrigida ou substituída isoladamente.

O rig da corrida foi gerado separadamente, com a mesma referência original de
Rust. A [fonte preservada](source/run-rig.png) e seu [prompt](source/run-rig-prompt.txt)
documentam suas peças originais. Corpo, braços e botas continuam em uso. Os
quatro recortes antigos de coxa/canela ficam preservados como fonte histórica,
fora do catálogo e do pacote runtime: a sobreposição de suas pregas provocava
um efeito de sanfona nos joelhos.

```sh
bash tools/art/extract_rust_run.sh
```

Este script só remove o matte e recorta os componentes originais. Não gera
poses nem reescreve os sockets ajustados em `catalog.json`.

A nova [fonte da calça e seus prompts](source/run-mesh/prompts.md) preservam o
desenho contínuo, com poucas dobras. Sua extração determinística usa:

```sh
bash tools/art/extract_rust_run_mesh.sh
```

A corrida combina essa malha com as cinco peças rígidas; caminhada/chute
continuam sendo desenhos discretos. A
[reconstrução da corrida](../../../docs/37-run-cycle-rebuild.md) documenta a
pesquisa, os contatos e o protocolo visual de 16 fases nos dois sentidos,
incluindo partida, frenagem e reversão na física real.

A [fonte da travessia](source/crossing-walk.png) e seu [prompt](source/crossing-walk-prompt.txt)
foram gerados com `image_gen` usando `rust-actions.png` como referência de
identidade. As costas mostram capuz, cabelo e bolsos traseiros; a volta mostra
rosto e emblema frontal. Extrair os 16 PNGs independentes:

```sh
bash tools/art/extract_rust_crossing.sh
```

[Comparação e travessias nativas](../../../docs/evidence/cinematic-polish/gait/README.md).
