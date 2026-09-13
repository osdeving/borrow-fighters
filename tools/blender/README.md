# Autoria 3D da Augusta

Blender 4.5.4 LTS produz malhas e ações; Rust/Raylib reproduz os GLBs nas
câmeras existentes. O [guia da cena](../../docs/40-augusta-blender-actors.md)
explica os arquivos e o limite desta conversão.

## Instalação usada

Executável local: `~/.local/bin/blender`, apontando para
`~/.local/opt/blender-4.5.4-linux-x64/blender`. Há também um lançador desktop.
Download oficial:
<https://download.blender.org/release/Blender4.5/blender-4.5.4-linux-x64.tar.xz>.
SHA-256 verificado antes da instalação:
`2e6ef8e99fc36327270429ddc8e7bad2859dd878a5a137d2e0bf0f02f6792505`.

As fontes `.blend` têm texturas empacotadas e podem ser abertas diretamente.
Regenerar anatomia/roupa usa o MPFB2 externo; revisar movimentos e reexportar
uma fonte pronta usa apenas Blender. A procedência dos recursos está junto
às fontes em `assets/adventure/production-3d`.

## Corrigir somente uma ação

`actor_motion.py` cria poses sobre o esqueleto `game_engine` com 53 ossos.
Joelhos e cotovelos são resolvidos em 3D sem alongar os membros. C++ conserva
as durações e os alvos de apoio do pacote anterior; os braços são adaptados
à anatomia. A orientação e o deslocamento global pertencem ao jogo.

Renderizar quatro fases de corrida, sem abrir o jogo nem alterar o modelo:

```sh
blender --background assets/adventure/production-3d/humans/cpp-pilot.blend --threads 4 --python-exit-code 1 \
  --python tools/blender/render_motion.py -- --actor cpp --output /tmp/cpp-motion \
  --samples run:0,run:0.25,run:0.5,run:0.75 --angle profile
```

Também há vistas `front`, `quarter` e `hands`. Escolha mais fases ao revisar
contato, pernas cruzando, dedos, cabelo e roupa. Uma pose correta não comprova
todo o ciclo. Alterações visuais locais pedem essa revisão focada; mudanças
no carregamento ou renderer também pedem validação nativa.

Depois da revisão, reexportar ações sem reconstruir a geometria:

```sh
blender --background assets/adventure/production-3d/humans/cpp-pilot.blend --threads 4 --python-exit-code 1 \
  --python tools/blender/export_actions.py -- --actor cpp \
  --model assets/adventure/models/cpp.glb \
  --metadata assets/adventure/production-3d/humans/cpp-actions.json --save-source
```

`--save-source` salva as ações novas no `.blend` carregado. O export aplica
modificadores de superfície, reúne as peças para desenhá-las com menos chamadas,
mantém o esqueleto e substitui o GLB somente depois da exportação completa.
Não executar sobre uma fonte contendo outros atores ou o cenário.

Se nomes de ações ou comprimento da passada mudarem, sincronizar `clips` e
`strides_m` da entrada correspondente em `models/humans.json`. Um ajuste de
cotovelo que conserva esses valores exige somente substituir o GLB. O jogo
carrega a nova exportação; ele não reescreve os scripts Blender.

Uma checagem do GLB exportado, também sem abrir o jogo:

```sh
python3 tools/blender/validate_humans.py --actor cpp --output /tmp/cpp-export-check.json
```

Ela confere pesos, ossos, dados geométricos e ações presentes no arquivo,
inclusive timestamps a 60 Hz. Não substitui a inspeção visual das fases.

## Regenerar aparência

`humans.py` constrói C++; `human_base.py` contém anatomia e ajuste de roupa
compartilhados. O checkout MPFB usado está em `/tmp/borrow-fighters-mpfb2`;
commit e licenças estão no JSON de procedência. Exemplo do piloto C++:

```sh
blender --background --factory-startup --threads 6 --python-exit-code 1 --python tools/blender/humans.py -- \
  --mpfb /tmp/borrow-fighters-mpfb2 --review /tmp/cpp-3d-pilot
```

`--skip-export` permite conferir a nova aparência sem substituir o GLB que
estiver sendo capturado pelo laboratório. Fontes editáveis e renders ficam
separados dos modelos distribuídos. Backups `.blend1` são locais.

O restante do elenco usa os mesmos helpers e esqueleto:

```sh
blender --background --factory-startup --threads 4 --python-exit-code 1 \
  --python tools/blender/human_cast.py -- --actor julia
blender --background --factory-startup --threads 4 --python-exit-code 1 \
  --python tools/blender/security.py -- --review /tmp/security-review
blender --background --factory-startup --threads 4 --python-exit-code 1 \
  --python tools/blender/crowd_humans.py -- --wardrobe denim --review /tmp/crowd-review
```

O [entregador](../../assets/adventure/production-3d/humans/delivery-rider.md)
tem um guia específico de apoios e manoplas. Os
[veículos](../../assets/adventure/production-3d/vehicles/README.md) documentam
as fontes, dimensões e validação da rotação das rodas.
