# ADR 0009 — Manifests de sprite com multiplos atlas

## Status

Aceito.

## Contexto

Os primeiros personagens carregavam um unico PNG por `*.sprite.json`, usando `manifest.image` como atlas global. Isso funcionou para Rust, Duke, Go, C e Python, mas personagens com mais poses, mais resolucao ou efeitos embutidos podem ultrapassar um atlas pratico para revisao e geracao.

C++ precisa entrar no jogo com os nove golpes proximos, `hit`, `special`, `taunt` e `victory`, mas o atlas high-res ficaria grande demais como artefato unico. Separar em dois manifests duplicaria clips, personagem e regras de gameplay. Separar personagem em duas entidades tambem quebraria Combat Lab, Move Showcase e match setup.

## Decisão

Manter um unico `borrow-fighters.sprite.v1` por personagem e permitir que frames individuais apontem para outro PNG via `frames[].image`.

Regras:

- `image` continua obrigatorio na raiz do manifest.
- `frames[].image` e opcional.
- frames sem `frames[].image` usam `image`.
- todos os caminhos continuam resolvidos a partir do arquivo `*.sprite.json`.
- clips continuam referenciando somente nomes de frames, nao nomes de atlas.

O runtime carrega todos os PNGs referenciados por `image` e `frames[].image`; o renderer escolhe a textura pelo frame atual. O manifest da C++ usa `cpp-fighter-atlas-a.png` como imagem raiz e `frames[].image: "cpp-fighter-atlas-b.png"` nos frames do segundo atlas.

## Alternativas consideradas

### Um atlas gigante

Prós:

- menor mudança no runtime.
- Sprite Viewer antigo continua simples.

Contras:

- PNGs grandes ficam piores de revisar, regenerar e inspecionar.
- aumenta risco de limites práticos de textura conforme a resolução dos sprites cresce.

### Um manifest por atlas

Prós:

- não altera schema.
- cada arquivo continua pequeno.

Contras:

- duplica clips e metadata.
- exige lógica de composição por personagem fora do manifest.
- aumenta chance de drift entre atlas A e atlas B.

### Criar pipeline de packing mais avançado agora

Prós:

- poderia otimizar espaço e remover células vazias.

Contras:

- aumenta escopo de tooling antes de precisar.
- complica Sprite Studio e testes no Prototype 0.1.

## Consequências

### Positivas

- Um personagem continua sendo um unico manifest e um unico `CharacterId`.
- C++ pode usar dois atlas sem duplicar clips.
- Personagens futuros podem crescer em resolução ou quantidade de frames sem mudar gameplay.
- Manifests antigos continuam validos sem alteração.

### Negativas

- O loader de assets fica um pouco mais complexo.
- Ferramentas precisam saber que um frame pode vir de outro PNG.
- O Sprite Combat Viewer Raylib antigo ainda deve ser tratado como ferramenta temporaria; o runtime de luta e o Combat Lab sao a referencia para multi-atlas.

## Critério de revisão

Revisar esta decisão se:

- o Sprite Studio precisar editar packing/atlas de forma visual;
- o projeto adotar Aseprite JSON ou outro formato externo como fonte direta;
- muitos personagens passarem a usar mais de dois atlas;
- texturas grandes ou limites de GPU exigirem packing compacto em vez de células fixas.
