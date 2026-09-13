# 40 — Augusta: personagens e veículos no Blender

Esta tentativa substitui humanos e veículos por malhas articuladas em 3D.
O cenário, a sequência, as câmeras, as posições e as trajetórias continuam
sendo os da [Augusta cinematográfica](39-augusta-cinematic-direction.md).
Blender serve para produzir e inspecionar os modelos; o capítulo continua
rodando em Rust/Raylib, inclusive durante as cinemáticas.

A integração está em revisão visual. Para abrir o candidato:

```sh
BORROW_AUGUSTA_MODELS_3D=1 cargo run --release --bin borrow-story -- --start augusta
BORROW_AUGUSTA_MODELS_3D=1 cargo run --release --bin borrow-actor-lab
```

No laboratório, também é possível escolher o catálogo diretamente com
`--models assets/adventure/models/humans.json`. Um snapshot pode ser reaberto
com `--actor source/player/character.json --models source/models/humans.json`
a partir de seu diretório. A fase escolhida na timeline permanece igual ao
ampliar a personagem.

Durante a conversão, personagens ainda sem entrada 3D usam sua apresentação
anterior. A pintura continua disponível como referência de identidade.

## Onde está cada coisa

| Arquivo/diretório | Uso |
| --- | --- |
| [production-3d/humans](../assets/adventure/production-3d/humans) | Fontes humanas `.blend`, roupas, texturas e procedência |
| [production-3d/vehicles](../assets/adventure/production-3d/vehicles) | Fontes `.blend` dos carros e da moto |
| [models/humans.json](../assets/adventure/models/humans.json) | GLB de cada humano, altura, ações e comprimento das passadas |
| [models/vehicles.json](../assets/adventure/models/vehicles.json) | GLB, dimensões, material de pintura e ação das rodas |
| [world-art.json](../assets/adventure/chapters/cpp-augusta/world-art.json) | Liga os dois catálogos ao capítulo |
| [tools/blender](../tools/blender/README.md) | Scripts de criação, revisão isolada e exportação |
| [models3d.rs](../src/adventure/engine/production/models3d.rs) | Carrega GLBs, sincroniza animação e desenha nas câmeras existentes |
| [models3d_pair.rs](../src/adventure/engine/production/models3d_pair.rs) | Posiciona as mãos de Julia e do cafetão no contato existente |

O jogo final precisa dos GLBs e JSONs declarados, com texturas embutidas no
GLB. O empacotador inclui essas dependências e não distribui o Blender,
arquivos `.blend`, scripts de autoria ou capturas de evidência.

## Movimento e edição

Os modelos usam metros. No Blender, Z aponta para cima e a frente do ator
é -Y; o export glTF converte para Y acima e frente +Z. O pivô permanece
no chão. Ações têm 60 quadros por segundo. Corridas e caminhadas acompanham
a distância percorrida para manter os pés em acordo com o deslocamento.

O `.blend` permite pausar a timeline, inspecionar ossos e modificar pose,
geometria, materiais e luz. O [fluxo de revisão isolada](../tools/blender/README.md)
renderiza só os movimentos escolhidos. Reexportar um GLB não exige recompilar
o jogo. Uma edição manual no Blender não altera automaticamente a receita
Python: regenerar as ações por script substitui as poses geradas anteriormente.

O palco e a atuação compartilham os mesmos modelos no gameplay e nas
cinemáticas. Julia e o cafetão usam um alvo de contato comum; as câmeras
apenas projetam esse contato. Ajustes anatômicos exigem inspeção de quadros,
porque esqueleto e cinemática inversa não impedem roupa ou cabelo de cruzar
outro objeto automaticamente.

## Continuidade

`tools/review/check_augusta_3d_continuity.py` compara onze arquivos fixos e,
quando recebe `--telemetry`, todos os campos originais de cada quadro com
o filme anterior. `--prefix` permite uma captura parcial explicitamente.
Essa checagem confirma estado e cenário; aprovação de aparência é visual.

Decisão: [ADR 0034](adr/0034-augusta-blender-actors.md).
Resultados e retomada: [diário](worklogs/augusta-blender-actors.md).
