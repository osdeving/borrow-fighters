# Movimento de Rust

O prólogo, a caminhada pelo quarto, o pesar após o encontro e o capítulo usam
o mesmo conjunto de oito poses de caminhada. Quatro poses independentes formam
o chute: preparação, extensão, recolhimento e retorno à guarda.

- `catalog.json`: cada frame aponta para seu próprio PNG; `source`, `anchor`,
  escala e sockets ficam nos dados. O apoio acompanha o quadril e a sola,
  evitando recentralizar o corpo pelo recorte variável dos braços e das botas.
- `motion.json`: nomes dos clips e distância de uma passada completa. O
  cursor acompanha distância realmente percorrida no chão, inclusive nas
  aproximações e no quarto. Pausa, obstáculo e velocidade menor preservam o
  apoio. A escala em profundidade ajusta também o comprimento da passada.
- `waking.json`: poses existentes do despertar, apoios no colchão/assento/chão,
  assentamento interpolado em torno desses apoios e caminho até a porta.
  Trocar um ponto do caminho não exige modificar a imagem do quarto.

O chute mantém a extensão nos ticks 10–16 e termina sua recuperação no tick 34,
em concordância com o domínio de combate. As poses não definem dano ou hitbox.
Os passos do capítulo seguem o mesmo comprimento de passada dos dados.
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
