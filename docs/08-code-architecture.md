# 08 — Arquitetura de Código

## Status

Implementado em corte inicial.

O experimento de aventura autorizado em 10 de setembro de 2026 acrescenta
`src/adventure/` e um binário independente, atrás da feature `adventure`.
Os módulos atuais de luta ficam atrás de `fighting`. Ambas as features são
ligadas por padrão, e `cargo run` executa a composição `borrow-story`.
Compilações isoladas usam `--no-default-features` e a feature/binário escolhidos.
Somente `math` e `runtime_paths` são core compartilhado; regras, cenas, input,
renderização, áudio e assets específicos não cruzam entre os jogos.
Veja a [ADR 0021](adr/0021-isolated-adventure-experiment.md) e o
[diário de retomada](worklogs/rust-adventure-prologue.md).

A [ADR 0022](adr/0022-adventure-external-copy-and-opening.md) acrescenta
`adventure/text.rs`, catálogo externo com recarga transacional, e a etapa
`Opening` após o pesar. `engine/morning.rs` calibra apoios anatômicos;
`engine/opening.rs` compõe jornais, histórias e logo; `engine/typography.rs`
ajusta texto editável ao espaço. Esses módulos e seus assets são exclusivos
da aventura, sem ampliar o core compartilhado.

Este documento descreve a arquitetura atual do protótipo Rust + Raylib e mantém algumas intenções futuras. A regra segue sendo evitar transformar o projeto em uma engine antes de provar o combate.

A [ADR 0023](adr/0023-story-to-terminal-menu.md) acrescenta uma composição externa:
`presentation.rs` e o binário `borrow-story`, habilitados somente com ambos os
domínios. A composição possui a janela e conecta APIs de aplicação; os domínios
não a importam nem passam a depender um do outro. A aventura devolve conclusão
confirmada, pedido explícito de pular tudo ou saída. Confirmação/skip seguem ao
menu principal; saída ou limite de frames encerram a sessão.

A [ADR 0024](adr/0024-prologue-background-life.md) acrescenta vida de fundo à
rua do prólogo. [`adventure/ambient.rs`](../src/adventure/ambient.rs) contém o
relógio puro, trajetórias decorativas e a reação do garoto a
`combat.enemy_awake`; [`adventure/story.rs`](../src/adventure/story.rs) avança
esse estado durante encontro/desfecho e o restaura em retry/restart.
Pausa omite o update. [`adventure/engine/street.rs`](../src/adventure/engine/street.rs)
desenha ciclovia, ciclistas, garoto e pipa em planos separados da área jogável;
[`adventure/engine/morning.rs`](../src/adventure/engine/morning.rs) mantém os
apoios de Rust e anima os detalhes do quarto. Esses atores não entram no
estado de corpos, hitboxes ou contatos do combate. [Escopo e verificação](30-prologue-scene-improvements.md).

A extensão de trânsito usa o mesmo `AmbientState` para carros, aproximação,
frenagem e batida no poste. [`engine/traffic.rs`](../src/adventure/engine/traffic.rs)
desenha asfalto, veículos e efeitos locais; o observador de áudio da aventura
consome os marcos compartilhados da buzina, frenagem e impacto uma vez por
passagem, incluindo renderizações que atravessem mais de um update.
Pausa preserva sons em curso, retry rearma o acidente e skip descarta seus cues.
Telemetria de revisão inclui veículos, idade do acidente e sincronização de áudio
após skip; relógio de parede permite alinhar gravação nativa de som e vídeo.

A [ADR 0025](adr/0025-replaceable-street-pieces.md) separa arte e composição:
[`adventure/scenery.rs`](../src/adventure/scenery.rs) valida o catálogo de peças
e as instâncias; [`engine/pieces.rs`](../src/adventure/engine/pieces.rs) carrega
cada PNG uma vez e desenha recortes com apoio e escala próprios.
[`catalog.json`](../assets/adventure/street/catalog.json) pode apontar para
PNGs individuais ou atlas, enquanto [`scene.json`](../assets/adventure/street/scene.json)
mantém posições e textos dos adereços.

Ao despertar a EP, `AmbientState` preserva as posições anteriores e inicia
aceleração dos veículos e frenagem/desmontagem/fuga dos ciclistas. O caminho
de evacuação não aplica wrap: em até seis segundos todos saíram, deixando
bicicletas e carro acidentado. Pausa, retry e restart seguem o relógio da história.
Os marcos de áudio incluem fuga e contato das bicicletas com o chão; nenhum
ator decorativo entra nas regras do combate.
[Escopo e verificação](31-brazilian-street-evacuation.md).

A [ADR 0026](adr/0026-cinematic-arrival-and-neighbours.md) acrescenta uma
chegada de seis segundos dentro do encontro. `Story::arrival_active()` retém
o avanço do combate enquanto o ambiente segue vivo; `arrival.rs` descreve
alvo/zoom puros. O renderer transforma o mundo com `Camera2D`, preservando
HUD e comandos em coordenadas de tela. A transformação final é a identidade;
retry acordado dispensa a tomada e avanço de trecho entrega a exploração.

[`adventure/neighborhood.rs`](../src/adventure/neighborhood.rs) deriva pessoas,
cão e porta do mesmo relógio ambiente. [`engine/neighborhood.rs`](../src/adventure/engine/neighborhood.rs)
compõe moradores e porta usando recortes geométricos que acompanham a câmera.
A entrada da fachada é uma abertura de 97×122 pixels, centrada em x603 e no
chão y355; a porta espera os visitantes entrarem e cobre o lojista ao baixar.
Áudio separa ar e tráfego, apaga somente os motores na evacuação e compartilha
os marcos de latido e fechamento com esse estado puro.
[Escopo e validação](32-cinematic-neighbourhood-arrival.md).

A [ADR 0029](adr/0029-modular-running-world.md) acrescenta
`adventure/landscape.rs`, dados do mundo do prólogo e composições por instância,
e `engine/landscape.rs`, renderização por faixa visível, peças de chão repetidas
e parallax distante. `Combat::set_bounds` recebe a extensão física de cada
cena. A corrida em `locomotion/run.rs` usa poses e apoios interpolados por
distância; não acrescenta uma engine ou ECS.
[Arquivos editáveis e distâncias](35-modular-running-world.md).

A [ADR 0031](adr/0031-continuous-run-leg-mesh.md) substitui os recortes
sobrepostos de coxa/canela por tecido contínuo. `locomotion/mesh.rs` calcula
vértices, pesos e correção de volume sem Raylib; `engine/locomotion.rs` aplica
as poses e `engine/pieces.rs` desenha os triângulos texturizados. Botas e braços
continuam rígidos. JSONs separam landmarks, proporções e alvos da arte.
[Pesquisa e protocolo visual](37-run-cycle-rebuild.md).

A [ADR 0030](adr/0030-painted-biography-shots.md) preserva os trilhos e textos
externos das biografias, usando uma pintura completa por tomada de Duke/Old C
para garantir perspectiva e iluminação coerentes. O mundo jogável mantém as
peças independentes. O trilho inicial da pipa também é externo e seu corte
entre vizinhança e Rust ocorre somente sob preto completo.

A [ADR 0032](adr/0032-character-production-and-independent-chapters.md) acrescenta
`adventure/production` com contratos de personagem, clips e combate independentes
de Raylib, `engine/production` com carregamento/renderização comuns ao laboratório
e capítulo, e `lab_app`/`borrow-actor-lab` para produção fora da campanha.
`augusta` dirige a história C++ com checkpoints próprios; `campaign` oferece
rotas implementadas sem acoplar seus saves. `SharedAssets` separa os recursos
Rust reutilizáveis das imagens exclusivas do prólogo; C++ usa outro pacote e
não carrega esse conjunto. A composição carrega o menu de luta sob demanda.
[Formatos, fontes e workflow](38-cpp-augusta-production.md).

## Objetivo

A [ADR 0033](adr/0033-augusta-cinematic-stage.md) acrescenta câmera pura em
`augusta/cinema.rs` e vida noturna em `augusta/ambient.rs`. O adaptador
`engine/production/cinema.rs` usa perspectiva 3D e os mesmos assets/mapa do
capítulo; `restraint.rs` compõe a atuação pareada, e `nightlife.rs` apresenta
figurantes em ambos os enquadramentos. Fases, contato, trajetórias e relógio
de evacuação continuam no domínio Augusta. [Escopo](39-augusta-cinematic-direction.md).
`painted_crowd.rs` aplica as poses aos recortes pintados, com registros externos
em `nightlife-cast.json` carregados junto ao world-art do capítulo.

A [ADR 0028](adr/0028-editable-cinematic-tracks-and-destructibles.md) amplia
os catálogos existentes com `adventure/biography.rs` (tomadas e keyframes),
`adventure/locomotion.rs` (distância/apoios) e `chapter/debris.rs` (dano e queda
de peças espaciais). Seus adaptadores Raylib permanecem em `engine`. A chegada
da EP em `arrival.rs` compartilha o relógio físico com impacto e evacuação.
[Mapa de arquivos editáveis](34-cinematic-expansion.md).

Criar uma base simples, testável e extensível para o protótipo 0.1 sem transformar o projeto em uma engine antes de provar o combate.

## Referências de base

- [Cargo Book — Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html): `Cargo.toml` e `Cargo.lock` na raiz, código em `src/`, binário padrão em `src/main.rs`, biblioteca em `src/lib.rs`, exemplos em `examples/` e testes de integração em `tests/`.
- [The rustdoc book — How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html): documentação de crate/módulo com `//!`, documentação pública com `///`, e primeira frase curta e clara.
- [raylib-rs](https://github.com/deltaphc/raylib-rs): binding Rust para Raylib que mantém proximidade com a API C, mas com ajustes idiomáticos de Rust.
- [Game Programming Patterns — Game Loop](https://gameprogrammingpatterns.com/game-loop.html): separação entre input, update e render; fixed timestep como candidato para previsibilidade de combate.
- [Game Programming Patterns — Update Method](https://gameprogrammingpatterns.com/update-method.html): objetos/sistemas atualizados por frame quando houver simulação concorrente simples.

## Estrutura atual

```text
borrow-fighters/
├── Cargo.toml                  # Manifesto do pacote Rust
├── Cargo.lock                  # Deve subir: jogo/aplicação precisa de build reproduzível
├── src/
│   ├── main.rs                 # Binário fino: inicializa janela, cria App e roda loop
│   ├── lib.rs                  # Módulos testáveis e API interna do jogo
│   ├── app.rs                  # Orquestra estado global, loop e transições de alto nível
│   ├── app/hosted.rs           # Menu preparado e pedidos opacos ao dono da janela
│   ├── presentation.rs        # Host opcional conecta prólogo, menu e capítulo
│   ├── adventure/
│   │   ├── app.rs             # API de sessões e loop do prólogo
│   │   ├── chapter/           # Modelo puro do capítulo, sem Raylib ou acesso a disco
│   │   │   ├── mod.rs         # Estado, comandos e fases
│   │   │   ├── world.rs       # Geometria validada, obstáculos, POIs e rotas
│   │   │   ├── progression.rs # Exploração, contatos e transições
│   │   │   ├── direction.rs   # Caminhos, moradores, diálogo e câmera
│   │   │   ├── phone.rs       # Relógio único dos gestos e mensagens
│   │   │   ├── checkpoint.rs  # Marcos versionados e restauração segura
│   │   │   └── texts.rs       # Contrato do texto externo
│   │   ├── chapter_store.rs   # Perfil em disco e substituição de save completo
│   │   ├── chapter_app/       # Loop fixo, menus, tradução de input e revisão
│   │   └── engine/
│   │       ├── chapter/       # Assets, mundo, atores, sockets e painel do celular
│   │       ├── chapter_audio.rs # Ar, passos, telefone, porta e contatos
│   │       └── capture.rs     # Framebuffer e captura usados pelas duas sessões
│   ├── cli.rs                  # Parser pequeno de argumentos de inicialização
│   ├── config.rs               # Constantes de janela, arena, escala e timestep
│   ├── audio/
│   │   └── mod.rs              # Eventos, manifesto e roteamento data-driven de áudio
│   ├── lore/
│   │   └── mod.rs              # Livro de história e fichas de roster carregados de JSON
│   ├── game/
│   │   ├── mod.rs              # Estado de partida e regras de fluxo
│   │   ├── arena.rs            # Identidade e rotação das arenas do protótipo
│   │   ├── ai.rs               # CPU simples para playtest
│   │   ├── feature_flags.rs    # Flags runtime para experimentos e menu Options
│   │   ├── world.rs            # Estado jogável, intro/contagem e regras de partida
│   │   └── world/
│   │       ├── throws.rs       # Captura pareada e lançamento balístico
│   │       ├── supers.rs       # Captura autoral, fases, áudio e contatos de supers
│   │       └── signatures.rs   # Emissão e contato dos cinco especiais
│   ├── engine/
│   │   ├── mod.rs              # Adaptadores finos em volta de Raylib
│   │   ├── audio.rs            # Boundary Raylib para carregar e tocar clips de áudio
│   │   ├── assets.rs           # Caminhos e carregamento de texturas
│   │   ├── gamepad.rs          # Mapeamento básico de gamepad
│   │   ├── input.rs            # Raylib keyboard/gamepad -> comandos do jogo
│   │   ├── render.rs           # Desenho de arena, HUD, debug, menu e lutadores
│   │   ├── video_capture.rs    # Captura local do framebuffer via ffmpeg
│   │   ├── render/
│   │   │   ├── combat_lab.rs   # Desenho da cena isolada de Combat Lab
│   │   │   ├── move_showcase.rs # Desenho de combate contextual no showcase
│   │   │   ├── signature_effects.rs # Atlas de efeitos nas posições físicas
│   │   │   ├── authored_supers.rs # Eclipse, mutação, terminais, tela azul e BIOS
│   │   │   ├── authored_actors.rs # Clones, lixo, notebook e Footgun
│   │   │   ├── python_super.rs  # Transformação/deglutição e poses de celebração
│   │   │   └── sprite_viewer.rs # Desenho da ferramenta isolada de sprites
│   │   └── sprites/
│   │       ├── animation.rs    # Seleção de frame por duração
│   │       ├── combat.rs       # Projeção de frames[].combat para coordenadas de mundo
│   │       ├── draw.rs         # Desenho de atlas com pivot
│   │       ├── manifest.rs     # Leitura/validação de JSON de sprite
│   │       ├── mod.rs          # API do módulo de sprites
│   │       └── selection.rs    # Estado de lutador -> clip
│   ├── combat/
│   │   ├── mod.rs              # Contratos do sistema de combate
│   │   ├── fighter.rs          # Estado comum de lutador
│   │   ├── fighter/reactions.rs # Captura, voo e aterrissagem
│   │   ├── signature.rs        # Entidades e geometria dos especiais
│   │   ├── super_sequence.rs   # Dados puros dos cinco roteiros autorais
│   │   ├── frame.rs            # Timing de combate em frames inteiros
│   │   ├── collision.rs        # Resolução hitbox x hurtbox
│   │   ├── move_data.rs        # Tabela MoveSpec dos golpes atuais
│   │   ├── move_set.rs         # Tipos runtime e compatibilidade com AttackKind
│   │   └── projectile.rs       # Estado de projéteis
│   ├── characters/
│   │   ├── mod.rs              # CharacterSpec e registro inicial de personagens
│   │   └── body_metrics.rs     # Manifesto de corpo fisico por personagem
│   ├── scenes/
│   │   ├── mod.rs              # Estados de tela
│   │   ├── combat_lab.rs       # Laboratório isolado para timing e boxes
│   │   ├── move_showcase.rs    # Cenários testáveis com dois atores no World
│   │   ├── preferences.rs      # Cursor e navegação do menu principal/submenus
│   │   ├── sprite_viewer.rs    # Viewer testável de atlas, pivot e frame bounds
│   │   └── sprite_viewer/
│   │       └── combat_edit.rs  # Helpers puros para editar boxes do viewer
│   ├── ui/
│   │   ├── mod.rs              # API dos overlays de UI/debug
│   │   ├── menu_layout.rs      # Retângulos compartilhados pelo desenho e cliques dos menus
│   │   └── combat_debug.rs     # Overlay de boxes, pivot e timing do Combat Lab
│   └── math/
│       ├── mod.rs              # Tipos geométricos pequenos do jogo
│       ├── rect.rs             # Retângulos de colisão/hitbox
│       └── vec2.rs             # Vetores 2D se Raylib Vector2 não bastar
├── tests/
│   ├── cli.rs                  # Contrato de argumentos de inicialização
│   ├── characters.rs           # Contrato do registro de personagens
│   ├── combat_lab.rs           # Estado testável do Combat Lab
│   ├── move_showcase.rs        # Contrato do autoplay de golpes do Training
│   ├── attack_frame_data.rs    # Timing de golpes em frames
│   ├── move_data.rs            # Contrato da tabela MoveSpec
│   ├── character_identity_tuning.rs # Intenção mecânica de Rust/Duke/Go/C por dados
│   ├── combat_rules.rs         # Regras puras de combate e IA
│   ├── traditional_moves.rs    # High/low/throw e ataques aéreos tradicionais
│   ├── cpu_traditional_moves.rs # Cobertura da CPU para golpes tradicionais
│   ├── feature_flags.rs        # Contrato de flags runtime
│   ├── audio_manifest.rs       # Contrato do manifesto e roteamento de áudio
│   ├── lore_book.rs            # Contrato do JSON de história/roster
│   ├── sprite_manifest.rs      # Validação do formato JSON de sprites
│   └── sprite_selection.rs     # Clip escolhido a partir do estado do lutador
└── tools/
    ├── art/                    # Utilitarios locais de extracao/ajuste de assets
    └── sprite-studio/          # App Tauri 1.8 + React isolado para editar manifestos de sprite
```

O diretório `scenes/` ainda deve permanecer simples, sem framework de telas. `ui/` já abriga o overlay de debug do Combat Lab, mas ainda não deve virar um sistema genérico antes de haver HUD e menus suficientes para justificar isso. `characters/` já possui o registro mínimo de personagens, mas ainda deve permanecer simples e orientado a dados. Novos módulos só devem entrar quando reduzirem responsabilidade real dos arquivos atuais.

## Regras de fronteira

A [ADR 0015](adr/0015-cinematic-presentation-and-stage-life.md) acrescenta dois
limites de apresentação: `combat/cinematic.rs` expõe um snapshot do ataque vivo,
consumido por `engine/render/cinematic_effects.rs`; `engine/render/stage_life.rs`
anima atores decorativos sem estado de combate. Fontes incorporadas e filtragem
continuam em `engine/assets.rs`. Nenhum desses desenhos amplia a hitbox física.

### `main.rs`

Deve ser fino. Responsabilidades:

- inicializar Raylib;
- carregar config inicial;
- criar `App`;
- rodar o loop;
- encaminhar encerramento.

Não deve concentrar regras de combate, input detalhado, colisão ou UI.

### `lib.rs`

Deve expor os módulos internos para testes e exemplos. Regras puras de jogo devem viver sob `lib.rs` sempre que possível para permitir `cargo test` sem abrir janela.

### `engine/*`

É a camada de adaptação com Raylib. Ela pode conhecer Raylib. O core de combate deve depender pouco ou nada de Raylib para ficar testável.

`src/engine/audio.rs` segue essa regra: ele carrega `Sound` e chama Raylib, mas recebe eventos e bindings já modelados pelo domínio de áudio.

`src/engine/video_capture.rs` tambem segue essa fronteira: ele pode conhecer ferramentas do host (`ffmpeg` e PulseAudio) e Raylib para leitura de framebuffer, mas gameplay e cenas só enxergam start/stop/status e envio do frame renderizado.

`src/engine/sprites/combat.rs` tambem fica em `engine` porque depende do formato de sprite, clip, pivot e escala visual. Ele nao depende de Raylib; apenas projeta metadata local do atlas para `Rect`/`Vec2` em coordenadas de mundo para que `game::World` possa usar com fallback.

### Capítulo da aventura e composição

O [capítulo Depois do silêncio](33-after-the-silence.md) segue a
[ADR 0027](adr/0027-chapter-spatial-direction.md). A divisão de responsabilidades
permite alterar geometria, encenação ou arte sem ampliar o `Story` do prólogo:

| Fronteira | Responsabilidade |
| --- | --- |
| [`adventure/chapter/`](../src/adventure/chapter/mod.rs) | Estado puro em ticks de 60 Hz. Geometria, caminhos, obstáculos e regiões de interação alimentam exploração, câmera e snapshots dos atores. Fases determinam diálogo, telefone e checkpoints; não carregam texturas nem gravam arquivos. |
| [`adventure/chapter_store.rs`](../src/adventure/chapter_store.rs) | Lê o perfil versionado em `runtime_paths::data_dir()/adventure/campaign-v1.json`. Valida antes de escrever, sincroniza um temporário na mesma pasta e substitui o destino; erros de leitura ou formato são devolvidos ao host. Vitória jogada no prólogo é um fato separado do estado canônico do capítulo. |
| [`adventure/chapter_app/`](../src/adventure/chapter_app/mod.rs) | Recebe a janela emprestada, possui recursos da sessão, traduz controles e dirige o loop fixo. Menus suspendem a simulação e descartam comandos pendentes. Mudanças de checkpoint acionam persistência; skip, retry e continue sincronizam o observador de áudio. A revisão usa comandos públicos e registra o estado observado. |
| [`adventure/engine/chapter/`](../src/adventure/engine/chapter/mod.rs) | Carrega arte e dados externos, aplica a câmera uma vez ao mundo e mantém legendas e mensageiro em coordenadas de tela. O catálogo fornece recortes, apoios e sockets; o aparelho é desenhado separado do corpo. Textos e aparência do mensageiro recarregam juntos após validação. |
| [`adventure/engine/chapter_audio.rs`](../src/adventure/engine/chapter_audio.rs) | Observa o mesmo relógio de gestos, porta, passos e contatos. Pausa suspende sons em curso; sincronização abandona efeitos antigos. Reutiliza ar e efeitos da aventura, sem carregar trânsito ou música no capítulo evacuado. |

Os dados próprios ficam em [`assets/adventure/chapter/`](../assets/adventure/chapter/README.md):
`world.json`, `chapter-texts.json`, `phone-style.json`, `catalog.json`, PNGs e
efeitos. O renderer reutiliza peças da rua por meio do catálogo validado;
imagens e metadata de produção não definem triggers. O empacotador segue os
PNGs de todos os frames dos catálogos e os caminhos concretos do runtime.

[`presentation.rs`](../src/presentation.rs) possui a janela e mantém
[`PreparedMenu`](../src/app/hosted.rs) entre visitas à aventura. O menu devolve
`MenuExit::Story`; somente o host interpreta o pedido e chama
`adventure::app::run_campaign_in_window`. A aventura devolve `CampaignExit`
para menu, revisão do prólogo, saída ou limite de frames. Essas APIs transportam
intenções de navegação, sem expor estado narrativo ao domínio de luta.
`fighting` continua compilável sem `adventure`, e o core compartilhado permanece
restrito a `math` e `runtime_paths`.

### `tools/sprite-studio`

O Sprite Studio e uma ferramenta externa em Tauri 1.8 + React. Ele nao faz parte do pacote Rust do jogo, nao importa structs do runtime e nao deve virar dependencia do `src/`.

Responsabilidades:

- abrir atlas e `*.sprite.json`;
- editar dados de manifesto;
- rodar validacao do runtime por comando externo (`cargo test`) sem linkar codigo do jogo;
- salvar artefatos consumidos pelo jogo;
- dar UI melhor para artistas e devs ajustarem pivot, escala, hitbox, hurtbox e origem de projectile.

O contrato entre jogo e ferramenta e apenas o arquivo em disco. A decisao esta registrada em [`docs/adr/0008-external-sprite-studio-tooling.md`](adr/0008-external-sprite-studio-tooling.md).

### `combat/*`

Deve ser o núcleo mais estável do protótipo. Prioridade:

- dados simples;
- estados explícitos;
- colisão legível;
- efeitos previsíveis.

Evitar callback/event bus cedo demais.

Quando combate precisar de feedback sonoro, deve emitir ou encaminhar `AudioEvent` pelo match runtime em vez de tocar arquivo diretamente.

### `characters/*`

No começo, personagens devem ser dados e pequenas funções. `CharacterSpec` já alimenta `World`, `Combat Lab` e `Fighter` com nome, vida máxima e loadout de golpes próximos. Não criar sistema de plugins, scripting ou data-driven avançado antes de existir gameplay divertido.

Chaves estáveis para assets, como `CharacterId::audio_key`, devem ficar próximas do registro de personagem para evitar strings soltas.

`src/characters/body_metrics.rs` carrega [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json). Esse manifesto e o primeiro corte data-driven de corpo fisico por personagem, separado de arte e de hitbox de golpe.

### `audio/*`

É o domínio testável de áudio. Pode conhecer personagens e golpes por chave estável, mas não Raylib.

Responsabilidades:

- nomear cues de gameplay;
- carregar schema do manifesto;
- escolher o binding mais específico para um evento;
- manter clips ausentes como opção de pipeline, não como erro fatal de runtime.

Detalhes ficam em [`docs/14-audio-pipeline.md`](14-audio-pipeline.md) e [`docs/adr/0005-data-driven-audio-events.md`](adr/0005-data-driven-audio-events.md).

### `lore/*`

É o domínio testável de história e perfis de roster. Ele carrega [`assets/lore/story.json`](../assets/lore/story.json), fornece fallback do slice e não conhece Raylib, input, combate ou áudio.

Responsabilidades:

- parsear capítulos e fichas de personagem;
- manter texto editável sem rebuild;
- expor índices seguros para o menu;
- validar que ids de personagem continuam compatíveis com `CharacterId`.

### `scenes/*`

Usar cenas simples para separar fluxo de tela sem criar framework pesado.

Ferramentas temporárias e plugáveis também podem entrar em `scenes/*` quando tiverem estado testável sem Raylib. O corte atual é `src/scenes/sprite_viewer.rs`, acionado por `--tool sprite-viewer`, enquanto o desenho fica em `src/engine/render/sprite_viewer.rs`. Esse modo não deve carregar `World`, áudio ou loop de luta normal.

O `MoveShowcase` é uma cena de treino com dois atores no `World` real. Ele controla apenas preparação, entradas e repetição; as regras de combate e o log determinam o resultado. A [ADR 0013](adr/0013-contextual-showcase-and-mvp-combat.md) registra a base do showcase. A evolução na [ADR 0014](adr/0014-throws-launches-and-signature-effects.md) adiciona captura pareada, reações aéreas e entidades físicas dos especiais. `World` mantém a autoridade sobre trajetória, contato e KO; o renderer apenas anima os atlas separados no mesmo ponto. O resultado do showcase soma todos os pulsos, incluindo as três folhas de Java.

### Feature flags runtime

Opções experimentais de gameplay, UI e input devem entrar por `src/game/feature_flags.rs`.

Regras:

- criar um novo `FeatureFlag`;
- definir label, descrição e default no mesmo módulo;
- consumir com `FeatureFlags::enabled`, `set` ou `toggle`;
- evitar booleans soltos em `App`, `World`, render ou IA;
- registrar ADR quando a flag virar decisão estrutural.

A [ADR0017](adr/0017-reaction-clocks-and-arena-mutation.md) separa o relógio de
reação do relógio global/ataque e acrescenta `World::effective_arena(base)`.
O override de arena pertence ao World e se perde ao reconstruí-lo; seleção de
menu e rotação do próximo round continuam independentes. Renderer e áudio
consultam o mesmo estado, inclusive em Lab e showcase. Python conserva o alvo
no domínio durante a ocultação; escala, rotação e alpha são apenas apresentação.

## Loop de jogo atual

A [ADR 0018](adr/0018-contact-reaction-profiles.md) acrescenta o piloto de reações
por contato em Python/C++. `combat/fighter/contact_reactions.rs` expõe perfil,
idade e janela visual; `engine/sprites/reaction.rs` escolhe os clips opcionais
`reaction_*`. A rajada compartilha sua agenda entre ataque e resposta, com
recuperação visual de nove frames entre contatos a cada dez frames. O stun
físico e as caixas permanecem separados. `examples/capture_pair_reactions.rs`
registra o par inteiro e os frames de cada janela, não apenas poses isoladas.

Fluxo conceitual:

```text
read platform input
translate input to game commands
run fixed update step while needed
render current state
```

No protótipo 0.1:

- usar fixed update simples;
- limitar número máximo de updates por frame para evitar spiral of death;
- renderizar placeholders sem interpolação se isso reduzir risco;
- só adicionar interpolação quando movimento visual exigir.

## Convenções por arquivo

Todo arquivo Rust novo deve começar com uma descrição curta e declarar a qual sistema pertence:

```rust
//! Responsabilidade do módulo em uma frase.
//!
//! System: Nome do sistema maior. Explica qual motor/módulo possui este arquivo
//! e o que não pertence aqui.
```

Regras:

- primeira frase deve dizer o que o arquivo faz;
- a linha `System:` deve ajudar devs e IAs a localizar o módulo maior;
- evitar comentário óbvio em cada função;
- documentar itens públicos com `///`;
- se o arquivo tiver regra de domínio importante, registrar o porquê perto da regra;
- se uma decisão afetar vários arquivos, abrir ou atualizar ADR.
- se uma mudança alterar comandos, hotkeys, hitbox/hurtbox, frame data, personagens ou Combat Lab, atualizar [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md).

## O que evitar no começo

- ECS antes de haver necessidade real.
- Sistema de eventos genérico antes de uma dor concreta.
- Asset pipeline complexo.
- Scripting de personagens.
- Editor de fases.
- Múltiplos crates/workspace.
- Abstração própria de render que esconda Raylib cedo demais.

## Quando revisar esta arquitetura

Revisar se:

- o protótipo 0.1 provar o combate básico;
- hitbox/hurtbox exigirem ferramenta visual;
- personagens passarem a precisar de dados externos;
- testes de combate ficarem difíceis de escrever;
- Raylib começar a vazar para todo o domínio de gameplay.

## Distribuição e primeira abertura

A [ADR 0019](adr/0019-playtest-distribution.md) define os pacotes de playtest.
`runtime_paths` localiza assets na instalação e mantém capturas/marcador do guia
nos dados do usuário, sem trocar o diretório de trabalho nem alterar caminhos
explicitamente passados ao CLI. `App` organiza a primeira abertura; o submenu
`Como jogar` reutiliza a navegação de `PreferencesMenu` e a geometria compartilhada.
O desenho do guia fica em `engine/render/onboarding.rs`. Os modos só configuram
as flags de CPU, sem introduzir novas regras no domínio de combate.
