# Editar e reaproveitar as peças da rua

Edite [catalog.json](catalog.json) para trocar a arte e
[scene.json](scene.json) para posicionar objetos. As duas tarefas são
independentes: o ID de uma peça continua igual quando seu PNG, recorte ou
resolução muda. Feche e abra o jogo para carregar alterações de imagens,
catálogo e composição.

O bairro pintado continua como camada de base. Garoto, ciclistas, bicicletas,
veículos, moradores, cachorro, porta e adereços são sobrepostos; não fazem
parte desse bitmap. Os atlas
anteriores do garoto/ciclista e do carro do acidente continuam isolados e
podem ser referenciados pelo mesmo catálogo. A organização permite reutilizar
peças dentro da aventura, sem importar assets ou regras do jogo de luta.

O contrato fica em [scenery.rs](../../../src/adventure/scenery.rs), e o
carregamento em [pieces.rs](../../../src/adventure/engine/pieces.rs). O
renderer carrega cada caminho de textura uma vez: várias peças ou instâncias
podem compartilhar um PNG sem carregar cópias adicionais.

## O que pertence a cada arquivo

| Arquivo | Responsabilidade |
|---|---|
| [catalog.json](catalog.json) | `PieceCatalog`: associa IDs a tamanho, animação e geometria dos PNGs. |
| [scene.json](scene.json) | `StreetLayout`: lista instâncias de adereços, posições, escala e letreiros. |
| [pt-BR.json](../texts/pt-BR.json) | Guarda as palavras mostradas nos letreiros e na interface. |
| [vehicles.json](vehicles.json), [props.json](props.json), [neighbours.json](neighbours.json), [shopkeeper.json](shopkeeper.json), [caramelo.json](caramelo.json), [shutter.json](shutter.json) | Metadados de produção que ajudam a importar os recortes; o runtime usa o catálogo. |

Os dois primeiros documentos usam `"version": 1`. O catálogo possui o objeto
`pieces`; a composição possui a lista `props`. A ordem de `props` é a ordem
de desenho, do fundo para a frente, antes dos atores em movimento. Trajetórias
de trânsito e fuga continuam na encenação da aventura; trocar a arte de um
veículo não exige editar sua trajetória nem a composição.

## Trocar um carro por um PNG individual

Salve, por exemplo, `hatch-novo.png` nesta pasta, com transparência real e
perfil para a direita. Em `catalog.json`, substitua somente o conteúdo de
`vehicle.hatch`. Este exemplo supõe um PNG de 512 × 256, com rodas apoiadas
na linha 250 e centros medidos nas posições indicadas:

```json
"vehicle.hatch": {
  "width": 147,
  "frame_ticks": 1,
  "frames": [
    {
      "image": "street/hatch-novo.png",
      "source": [0, 0, 512, 256],
      "anchor": [256, 250],
      "wheels": [[106, 207, 12], [408, 207, 12]]
    }
  ]
}
```

O exemplo é uma entrada dentro de `pieces`, não um arquivo completo. Preserve
o ID `vehicle.hatch`; as referências existentes continuam funcionando.

| Campo da peça | Como ajustar |
|---|---|
| `width` | Largura do primeiro frame no mundo do jogo; mantenha 147 para conservar o tamanho aparente do exemplo. |
| `frame_ticks` | Updates fixos por frame, maior que zero. Uma imagem parada pode usar 1. |
| `frames` | Lista ordenada com pelo menos um frame; cada frame pode usar um arquivo diferente. |

| Campo do frame | Unidade e significado |
|---|---|
| `image` | Caminho relativo a `assets/adventure/`; use PNG e separadores `/`, sem caminho absoluto ou `..`. |
| `source` | `[x, y, largura, altura]`, em pixels da imagem inteira. |
| `anchor` | `[x, y]` dentro do recorte; é o apoio colocado na posição da instância, normalmente pés ou chão das rodas. |
| `wheels` | Opcional: `[x, y, raio]` por cubo de roda, em pixels relativos ao recorte, para o brilho animado. |

Ao aumentar a resolução do PNG, atualize `source`, `anchor` e `wheels`, mantendo
`width` para preservar o tamanho no jogo. Nas animações, todos os frames usam
a escala do primeiro; frames mais largos não são comprimidos automaticamente.
Use apoios consistentes entre as poses para evitar saltos ao trocar de frame.
Os campos são estritos: no catálogo runtime use `width` e `anchor`, não os
nomes `world_width` e `pivot` dos metadados de produção.

## Usar um recorte de outro atlas

Para um carro desenhado em `meu-atlas.png` no retângulo `(640, 320, 512, 256)`,
troque `image` por `"street/meu-atlas.png"` e `source` por
`[640, 320, 512, 256]`. Se o desenho dentro do recorte conserva as mesmas
medidas, mantenha `anchor` e `wheels`: ambos são relativos ao recorte, não à
imagem inteira. `scene.json` continua igual.

O atlas pode ter células irregulares. O catálogo não depende de sua ordem,
quantidade de colunas ou resolução anterior. Cada recorte deve caber no PNG;
apoios e centros das rodas devem ficar dentro dele. Imagem ausente, recorte
fora dos limites ou referência quebrada impedem carregar a rua e produzem
erro, em vez de substituir a arte silenciosamente.

## Reutilizar o mesmo objeto em duas posições

Uma instância de `scene.json` possui `id` único e referencia uma peça do
catálogo. Duplique uma entrada da lista `props`, altere o `id` e a posição,
e conserve `piece`. Por exemplo, dois conjuntos de caixas/vasos podem usar
`prop.corner`:

```json
[
  {
    "id": "bar_corner",
    "piece": "prop.corner",
    "position": [820, 350],
    "scale": 1
  },
  {
    "id": "other_corner",
    "piece": "prop.corner",
    "position": [1650, 350],
    "scale": 0.8
  }
]
```

Esse trecho ilustra a instância original e sua cópia em `props`. Se
`bar_corner` já existe, preserve essa entrada e acrescente somente
`other_corner`, escolhendo um ID ainda não usado.
`position` marca o apoio em coordenadas do mundo antes do deslocamento da
câmera. `scale` multiplica o tamanho da peça e deve ser maior que zero e no
máximo 4. A posição horizontal permite acompanhar o objeto ao mover a câmera.
Mantenha os adereços na calçada e confira sobreposição com pedestres e veículos.

As peças de cenário incluem `prop.bar`, `prop.bus_stop` e `prop.corner`.
As instâncias principais são `bar_casa_nossa`, `bus_stop`, `bar_corner` e
`neighbour_corner`; as duas últimas reutilizam a mesma peça. A bicicleta
abandonada usa `bike.fallen` e sua posição é controlada pela encenação de fuga.

## Preservar a entrada da mercearia

A fachada `bar_casa_nossa` está apoiada em `(666, 355)`, com escala 1 e largura
de catálogo de 330. A entrada usada pelos moradores tem centro inferior em
`(603, 355)`, largura **97** e altura **122**, em coordenadas do mundo. Isso
corresponde ao retângulo `x = 554,5–651,5`, `y = 233–355`. O painel
`shop.shutter` usa a mesma largura; o fechamento revela sua parte inferior
até cobrir a abertura, conservando a escala da chapa e o tirador junto à
borda que desce.

Esses valores pertencem à encenação em
[neighborhood.rs](../../../src/adventure/neighborhood.rs), nas constantes
`DOOR_CENTER_X`, `NEIGHBOR_FLOOR_Y`, `DOOR_WIDTH` e `DOOR_HEIGHT`. Mover ou
escalar a fachada em `scene.json` **não move automaticamente a entrada nem
as rotas de abrigo**. Ao substituir a arte, preserve o alinhamento da abertura
com esse retângulo; ao reposicionar a mercearia, ajuste também essas constantes
e confira a trajetória dos moradores, o contato das mãos e a oclusão dentro
da porta. A entrada fechada precisa ocultar pessoas e prateleiras sem frestas.

Os moradores usam duas identidades de clientes: a mulher aparece no ponto e
perto da mercearia, enquanto o jovem de mochila aguarda no ponto. O lojista é
a terceira identidade. Os clientes correm para a esquerda; o lojista espera
os três entrarem e baixa a porta. O cachorro caramelo alterna poses calmas e
foge para a esquerda ao perceber a EP, espelhando as poses autorais. As
instâncias e trajetórias desses atores pertencem à encenação, enquanto
imagens, recortes, apoios e poses pertencem ao
catálogo. A escala atual corresponde a 96 px de altura de referência para
adultos e 50 px para o cão; preserve a escala entre as poses de cada identidade.

## Editar letreiros

Cada instância pode ter `labels`. Uma etiqueta aponta para `text_key` no
objeto `text` de [pt-BR.json](../texts/pt-BR.json); as palavras ficam fora dos
PNGs. As chaves usadas atualmente são `street.bar.name`, `street.stop.title`
e `street.stop.route`. A chave anterior `street.bar.kind` permanece no JSON
para comparação, mas não possui etiqueta na composição atual.

Para mudar o letreiro inteiro da mercearia, edite `street.bar.name`, atualmente
`BAR E MERCEARIA CASA NOSSA`. Ele ocupa uma única linha com tamanho uniforme.
Para mover ou redimensionar o letreiro, edite sua etiqueta em `scene.json`:

```json
{
  "text_key": "street.bar.name",
  "offset": [-3, -193],
  "width": 209,
  "font_size": 18,
  "color": [49, 68, 63, 255]
}
```

`offset` é relativo à posição do objeto; `width` e `font_size` controlam o
espaço e tamanho máximo do texto. A escala da instância também escala esses
valores. `color` usa vermelho, verde, azul e alpha, de 0 a 255. Mantenha frases
curtas e confira o resultado com o objeto inteiro visível.

O letreiro completo da mercearia usa tamanho máximo 18. O carregador usa o papel
`Signage`, com Barlow Condensed SemiBold rasterizada separadamente a 64 px e
filtrada para leitura em tamanhos pequenos; o mesmo arquivo de fonte também
serve à apresentação. Consulte o [guia de fontes](../fonts/README.md).

**F5 recarrega somente os textos.** Alterações no PNG, catálogo, posição ou
geometria dos letreiros exigem reiniciar o aplicativo. O
[guia de textos](../texts/README.md) explica UTF-8 e edição segura do JSON.

## IDs necessários e conferência

O carregador exige estes IDs usados pela encenação:

| Grupo | IDs |
|---|---|
| Garoto | `kid.play`, `kid.startled`, `kid.release`, `kid.run` |
| Ciclista | `cyclist.ride`, `cyclist.brake`, `cyclist.dismount`, `cyclist.run` |
| Bicicleta | `bike.upright`, `bike.fallen` |
| Trânsito | `vehicle.hatch`, `vehicle.sedan`, `vehicle.pickup`, `vehicle.suv`, `vehicle.bus` |
| Acidente | `incident.intact`, `incident.crashed` |
| Clientes | `resident.0.idle`, `resident.0.run`, `resident.1.idle`, `resident.1.run` |
| Lojista e porta | `shopkeeper.idle`, `shopkeeper.alert`, `shopkeeper.pull`, `shop.shutter` |
| Caramelo | `dog.idle`, `dog.sit`, `dog.sniff`, `dog.alert`, `dog.run` |

Outras peças, como `vehicle.van` e adereços, podem coexistir. Toda referência
de `scene.json` deve existir no catálogo, e IDs de instâncias não se repetem.

Depois de editar, reinicie com `cargo run -- --start encounter`. Confira
escala, apoio no chão, rodas, letreiros, sobreposições e os dois extremos da
câmera. A chegada começa pela pipa e abre o enquadramento até Rust em cerca de
seis segundos; ao terminar, libera exploração e HUD. O ônibus usa largura 440
para manter proporção distinta dos automóveis. Aproxime Rust da EP para
conferir a troca de poses, a saída dos veículos, as bicicletas abandonadas,
o abrigo dos moradores, a porta fechada e a fuga do caramelo. Arte e composição
são decorativas; não alteram vida, hitboxes ou resultado do combate.

[Procedência dos veículos](VEHICLES.md) ·
[Moradores e porta](NEIGHBOURS.md) · [Caramelo](CARAMELO.md) ·
[Escopo da rua modular](../../../docs/31-brazilian-street-evacuation.md) ·
[Chegada e vizinhança](../../../docs/32-cinematic-neighbourhood-arrival.md) ·
[Decisão do catálogo](../../../docs/adr/0025-replaceable-street-pieces.md) ·
[Decisão da chegada](../../../docs/adr/0026-cinematic-arrival-and-neighbours.md)
