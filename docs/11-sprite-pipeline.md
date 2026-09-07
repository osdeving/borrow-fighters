# 11 — Pipeline de Sprites

## Status

Em implementacao. O runtime ja carrega manifests para Rust, Duke, Go, C, Python, C++, animacoes de entrada, clips de luta, pivots, duracoes por frame, multiplos atlas por personagem e fallback greybox.

## Objetivo

Permitir que artistas entreguem sprites com dados suficientes para o jogo renderizar animacoes sem hardcode de frame no codigo, mantendo pivots e hurtboxes ajustaveis quando a arte exigir.

## Formato candidato

O formato inicial do projeto e `borrow-fighters.sprite.v1`, em JSON.

Exemplo real:

- `assets/placeholder/rust-fighter.sprite.json`
- `assets/placeholder/duke-fighter.sprite.json`
- `assets/placeholder/rust-start.sprite.json`
- `assets/placeholder/duke-start.sprite.json`
- `assets/placeholder/go-start.sprite.json`
- `assets/placeholder/c-fighter.sprite.json`
- `assets/placeholder/c-start.sprite.json`
- `assets/placeholder/python-fighter.sprite.json`
- `assets/placeholder/python-start.sprite.json`
- `assets/placeholder/cpp-fighter.sprite.json`

Campos principais:

- `schema`: versao do formato.
- `image`: PNG do atlas.
- `source`: imagem ou arquivo de origem.
- `cell`: tamanho padrao da celula.
- `default_pivot`: ponto de apoio padrao, normalmente perto do pe no chao.
- `scale`: escala visual runtime do atlas; o jogo e o viewer usam o mesmo valor.
- `frames`: retangulos no atlas, duracao e pivot por frame.
- `frames[].image`: PNG opcional para aquele frame quando o personagem e composto por mais de um atlas; frames sem esse campo usam `image`.
- `clips`: animacoes com lista ordenada de frames e flag `loop`.
- `frames[].combat`: metadata opcional por frame para `hurtboxes`, `hitboxes` e `projectile_origin`.

Exemplo de metadata de combate dentro de um frame:

```json
"combat": {
  "hurtboxes": [
    { "x": 120, "y": 16, "w": 80, "h": 190, "label": "body" }
  ],
  "hitboxes": [
    { "x": 250, "y": 82, "w": 72, "h": 38, "label": "strike" }
  ],
  "projectile_origin": { "x": 286, "y": 92 }
}
```

Esses valores sao medidos em pixels locais do frame do atlas, nao em coordenadas de mundo. A validacao rejeita retangulos vazios, retangulos fora do frame, labels vazias e origem de projectile fora do frame. O schema ainda e experimental, mas ja participa do runtime com fallback: hitboxes/hurtboxes presentes no frame substituem as caixas greybox daquele frame; campos ausentes mantem `MoveSpec`, `Fighter::hurtboxes` e `ProjectileSpec`.

## Convencoes

- O atlas runtime deve ter alpha real.
- Labels, guias, checkerboard e anotacoes nao entram no PNG runtime.
- Todo frame deve caber dentro do retangulo declarado.
- Todo frame deve ter pivot.
- Animacoes de ataque devem ter duracao por frame.
- Efeitos reutilizaveis, como projeteis, devem poder virar assets separados.

## Clips do jogo atual

- `spawn`
- `idle`
- `walk`
- `crouch`
- `jump`
- `block`
- `crouch_block`
- `punch_light`
- `punch_heavy`
- `kick`
- `sweep`
- `overhead`
- `anti_air`
- `air_punch`
- `air_kick`
- `throw`
- `hit`
- `special`
- `victory`
- `defeat`

Estes 20 clips cobrem os estados e golpes implementados e sao exigidos para ativar um atlas candidato completo. `taunt` continua como compatibilidade para vitoria nos placeholders; `projectile` pode existir como material de efeito separado. A matriz de producao por personagem fica em [19 — Cobertura de producao](19-sprite-production-coverage.md).

`spawn` e reservado para entrada cinematografica no inicio da luta. Ele deve ser nao-loopavel e nao deve carregar regra de combate; o jogo pausa os inputs durante a intro e depois durante a contagem `11`, `10`, `01`, `Fight!`.

## Aseprite e ferramentas externas

Se os artistas usarem Aseprite, o caminho ideal e exportar PNG + JSON do Aseprite e converter para `borrow-fighters.sprite.v1`, ou adaptar o motor para aceitar Aseprite JSON diretamente.

Por enquanto, preferimos um formato pequeno do projeto porque:

- facilita revisar no Git;
- evita depender de uma ferramenta especifica;
- deixa claro quais dados o jogo precisa;
- permite gerar metadata a partir de scripts locais.

## Implementacao atual

O primeiro corte vive em `src/engine/sprites/`:

1. `manifest.rs` le e valida JSON;
2. `animation.rs` seleciona frames por duracao;
3. `selection.rs` converte estado de lutador em clip;
4. `combat.rs` projeta `frames[].combat` para coordenadas de mundo;
5. `draw.rs` desenha atlas com pivot via Raylib.

O personagem Rust usa `assets/placeholder/rust-fighter.sprite.json`.
O Player 2/Duke usa `assets/placeholder/duke-fighter.sprite.json`.
Go usa `assets/placeholder/go-fighter.sprite.json`.
C usa `assets/placeholder/c-fighter.sprite.json`, extraido dos atlas de referencia `assets/references/langc-03.png` e `assets/references/langc-04.png`.
Python usa `assets/placeholder/python-fighter.sprite.json`, gerado como placeholder visual e integrado ao roster jogavel como `python.py`. A entrada cinematografica da Python usa `assets/placeholder/python-start.sprite.json`, com cavalete e grafico de barras colorido para reforcar a piada de ciencia de dados.
C++ usa `assets/placeholder/cpp-fighter.sprite.json`, gerado de `assets/references/cpp-fighter-raster-source.png` e dividido entre `assets/placeholder/cpp-fighter-atlas-a.png` e `assets/placeholder/cpp-fighter-atlas-b.png`. O manifest mantem `image` apontando para o atlas A e usa `frames[].image` nos frames do atlas B.

O tamanho em jogo nao deve depender da resolucao do PNG. Ajuste `scale` e `frames[].pivot` no manifesto; o renderer de luta e o Sprite Combat Viewer consomem os mesmos valores. O padrao atual de altura, largura e arena fica em [`docs/17-visual-scale-and-stage-metrics.md`](17-visual-scale-and-stage-metrics.md).

O corpo fisico de gameplay fica em [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json). Esse arquivo controla `width`, `standing_height` e `crouch_height` por personagem. Ele define o retangulo base usado por colisao corpo-corpo, hurtboxes compostas e alinhamento do sprite. `frames[].combat` pode substituir hitbox/hurtbox por frame quando houver metadata revisada.

No corte atual, Rust, Duke, Go, C, Python e C++ ja declaram `frames[].combat.projectile_origin` no primeiro frame do clip `special`, usado pelo runtime para alinhar o nascimento do projectile com a mao do personagem. Rust tambem possui `frames[].combat.hitboxes[]` iniciais para `Borrow Jab`, heavy punch e kick, calibradas para reproduzir o alcance atual do `MoveSpec` antes de qualquer ajuste de balanceamento. Python e C++ desenhadas high-res usam clips visuais dos nove golpes, mas os golpes proximos ainda caem no fallback de hitbox do `MoveSpec` ate serem calibrados no Sprite Studio. Outras hitboxes e hurtboxes por frame ainda devem ser preenchidas pelo Sprite Combat Viewer antes de substituir alcances de soco/chute em producao.

O runtime tambem usa:

- `spawn` durante a entrada inicial: prefere o clip no atlas principal, depois o manifesto de entrada separado, e usa `idle` quando nenhum deles tem entrada;
- clips proprios para os nove golpes proximos: `punch_light`, `punch_heavy`, `kick`, `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick` e `throw`;
- `hit` enquanto o personagem esta em hitstun, avancando desde o impacto;
- `block` em defesa e `crouch_block` quando defesa e agachamento estao ativos;
- `special` por alguns frames quando o personagem dispara projectile;
- `victory` para o vencedor e `defeat` para o derrotado; empate usa `defeat` nos dois;
- fallback greybox quando um atlas nao carrega.

Os relogios de apresentacao comecam em zero ao entrar em defesa, agachamento ou salto; um novo impacto reinicia a reacao. Trocar entre defesa em pe e agachada reinicia o clip da nova postura. `idle` e `walk` continuam usando o tempo global; ataques usam o tempo do golpe e `special` usa o timer visual do projectile. O salto percorre os quadros pelas duracoes do manifesto: revisar subida, apice e queda contra a trajetoria real, sem alterar a fisica para encaixar desenhos. O agachamento nao pula mais diretamente ao ultimo quadro.

Entrada usa o tempo da intro. Vitoria e derrota usam `World::outcome_elapsed_seconds`, iniciado quando o resultado aparece e atualizado enquanto o combate permanece congelado. Seus sprites usam anchor vertical no chao, mesmo quando o KO congela um corpo no ar; posicao, velocidade, boxes e debug fisicos conservam o estado do combate. Tint de ataque/dano, flash de reacao, overlay de guarda e luz de stun no chao deixam de cobrir essas poses finais.

Ao espelhar sprites, o recorte de origem conserva `x` e `y`: `DrawTexturePro` do Raylib interpreta largura negativa como flip dentro desse mesmo recorte. Somar a largura a `source.x` leria a celula seguinte ou pixels vazios. Lutadores, projectile e dummy do Sprite Viewer compartilham essa regra.

Fallbacks visuais de manifestos antigos sao explicitos: `victory → taunt → idle`, `defeat → hit → idle`, `crouch_block → block → idle`, `sweep`/`air_kick → kick`, `overhead`/`anti_air → punch_heavy` e `throw`/`air_punch → punch_light`, com `idle` como ultimo recurso. Esses aliases nao emprestam hitboxes de outro golpe.

## Revisao de candidatos no runtime

As fontes por acao ficam em `assets/production/<personagem>/`; o exportador gera `assets/candidates/<personagem>/<personagem>-fighter.sprite.json`. O fluxo e a proveniencia estao em [ADR 0010](adr/0010-reviewed-action-sprite-production.md). Exportar nao substitui placeholders nem ativa arte automaticamente.

Para revisar um conjunto completo em luta ou Combat Lab:

```bash
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run -- --fight --p1 rust --p2 duke
BORROW_FIGHTERS_SPRITE_CANDIDATES=1 cargo run -- --lab combat --character rust --pose defeat
```

O loader verifica o candidato de cada personagem (`rust`, `duke`, `go`, `c`, `python`, `cpp`) e exige os 20 clips listados acima. Arquivo ausente, invalido, textura ausente ou conjunto incompleto mantem o placeholder daquele personagem; clips faltantes produzem aviso no terminal. Essa verificacao cobre nomes e estrutura, nao aprova os desenhos. Um conjunto parcial deve ser aberto diretamente no Sprite Viewer/Studio, sem mascarar golpes ausentes com `idle` na luta.

O mesmo opt-in permite revisar um efeito separado em `assets/candidates/<key>/<key>-projectile.png`, independentemente do atlas do lutador. Arquivo ausente ou falha de carregamento preservam a textura original do projetil. Specs, origem, colisao e a formula de desenho permanecem iguais: largura e altura do PNG multiplicadas por `0.45 * RESOLUTION_SCALE`. Portanto, o canvas candidato precisa de revisao na escala real; o renderer nao ajusta sua imagem automaticamente a hitbox fisica.

`SpriteAtlasAsset.manifest` define o desenho e suas texturas; `combat_manifest` conserva o manifesto baseline de `assets/placeholder/`. `App` entrega somente `combat_manifest` ao `World`, e o overlay da luta usa os mesmos dados. Assim, escala, pivôs e metadata experimentais do candidato nao mudam alcance, hurtboxes ou origem do projectile. No Sprite Viewer, abrir um candidato mostra os dados desse proprio arquivo; isso nao demonstra que foram ativados na luta.

Combat Lab e Move Showcase recebem o mesmo baseline de `App`. O Lab projeta hitboxes/hurtboxes com o sampling da luta e consulta `special` em tempo zero para a origem do projetil; ausencia de metadata conserva o fallback de `Fighter`/`Projectile`. Estimativas de vantagem e posicionamento automatico do dummy continuam baseados em MoveSpec. O exemplo de captura aceita um terceiro argumento opcional com clips separados por virgula, como `rust target/art/rust-lab-baseline punch_light,punch_heavy,kick,special`; omitir o filtro captura os 19 contextos disponiveis. `walk` continua exigindo ensaio em World.

O sampling de metadata tambem permanece separado: stun usa tempo zero, crouch usa a pose final, jump preserva os tempos `0 / 0.18 / 0.36` por velocidade vertical e `crouch_block` consulta as boxes antigas de `block`. Alterar esses dados ou seu sampling exige revisao de gameplay propria. A limitacao preexistente de agachamento durante blockstun esta descrita no [guia de combate](12-technical-combat-guide.md).

As animacoes de entrada atuais vivem em manifests separados para nao misturar frames cinematograficos grandes com o atlas principal de luta:

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/duke-start-atlas.png`
- `assets/placeholder/go-start-atlas.png`
- `assets/placeholder/c-start-atlas.png`
- `assets/placeholder/python-start-atlas.png`

Assets relacionados ao slice atual:

- `assets/placeholder/rust-gear-projectile.png`
- `assets/placeholder/duke-bean-projectile.png`
- `assets/placeholder/go-channel-projectile.png`
- `assets/placeholder/c-bitstream-projectile.png`
- `assets/placeholder/python-fighter-atlas.png`
- `assets/placeholder/python-fighter-atlas-backup.png`
- `assets/references/python-fighter-raster-source.png`
- `assets/placeholder/python-start-atlas.png`
- `assets/placeholder/python-data-projectile.png`
- `assets/placeholder/cpp-fighter-atlas-a.png`
- `assets/placeholder/cpp-fighter-atlas-b.png`
- `assets/placeholder/cpp-plusplus-projectile.png`
- `assets/references/cpp-fighter-raster-source.png`
- `assets/placeholder/arena-sirius.png`
- `assets/placeholder/arena-fortaleza.png`
- `assets/placeholder/arena-java-street.png`
- `assets/placeholder/arena-terminal-compiler-lab.png`

As ferramentas locais ficam em `tools/art/` e `tools/sprite-studio/`. Scripts em `tools/art/` devem ser tratados como utilitarios de prototipo. O `tools/sprite-studio/` e o app desktop externo para editar manifestos e reduzir a dependencia do viewer Raylib embutido no jogo.

O atlas candidato de Python e reconstruido por:

```bash
python3 tools/art/build_python_fighter_atlas.py
python3 tools/art/build_python_high_res_fighter_atlas.py
python3 tools/art/build_python_start_atlas.py
python3 tools/art/build_cpp_fighter_atlas.py
```

O primeiro script repacota `assets/references/python-fighter-atlas-source.png` para a grade base do C (`6x16`, celulas `384x256`) e gera `assets/placeholder/python-fighter.sprite.json` e `assets/placeholder/python-data-projectile.png`.

O segundo script usa a pose sheet raster `assets/references/python-fighter-raster-source.png`, remove o chroma key, limpa componentes pequenos e empacota a versao runtime high-res de Python a partir do contrato de clips salvo em `assets/placeholder/python-fighter-backup.sprite.json`. Ele escreve `assets/placeholder/python-fighter-atlas.png` com celulas `768x512`, camisa branca, saia preta e poses proprias para os nove golpes proximos. O backup conserva a versao anterior com os nove clips de golpe para comparacao no Sprite Viewer.

O atlas de entrada repacota `assets/references/python-start-atlas-source.png` para celulas `512x320`, remove os numeros gerados na folha fonte, gera `assets/placeholder/python-start-atlas.png` e `assets/placeholder/python-start.sprite.json`, e salva uma previa local em `tmp/art/python-start-atlas-preview.png`.

O script de C++ reaproveita a estrutura de clips da Python, remove o fundo chroma key de `assets/references/cpp-fighter-raster-source.png`, limpa as linhas de grade da fonte, gera `cpp-fighter-atlas-a.png` e `cpp-fighter-atlas-b.png`, escreve `frames[].image` nos frames do segundo atlas, cria `assets/placeholder/roster-cpp.png` e cria `assets/placeholder/cpp-plusplus-projectile.png`.

## Sprite Studio

O Sprite Studio vive em [`tools/sprite-studio/`](../tools/sprite-studio) e esta documentado em [`docs/18-sprite-studio.md`](18-sprite-studio.md).

Ele usa Tauri 1.8 + React e nao compartilha codigo com o jogo. O contrato entre ferramenta e runtime e somente o artefato salvo em disco:

- o app edita `*.sprite.json`;
- o app edita `assets/tuning/character-body-metrics.json`;
- o jogo carrega `*.sprite.json`;
- o jogo carrega `assets/tuning/character-body-metrics.json`;
- testes do jogo validam se o manifesto continua aceito.

Comando:

```bash
cd tools/sprite-studio
pnpm install
pnpm build
pnpm tauri dev
```

O Studio oferece file picker nativo, menu desktop, paineis colapsaveis, timeline horizontal, tutorial visual (`F1`), edicao de pivot/scale/boxes/origem, snap, guia de escala visual, presets iniciais de boxes, autosave em `target/sprite-studio-autosave/`, backup em `target/sprite-studio-backups/`, validacao do runtime e export de PNG/JSON para review.

## Sprite Combat Viewer

O viewer Raylib embutido no jogo continua disponivel temporariamente ate a limpeza dedicada que removera a ferramenta antiga. Ele vive em:

- `src/scenes/sprite_viewer.rs`: estado testavel, carregamento de manifesto, clip/frame atual, playback e drag.
- `src/engine/render/sprite_viewer.rs`: grid, pivot, bounds e desenho do atlas via Raylib.
- `tests/sprite_viewer.rs`: contrato do estado sem abrir janela.

Abrir o viewer:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
cargo run -- --tool sprite-viewer --manifest assets/placeholder/duke-fighter.sprite.json --clip special --character duke --move projectile
cargo run -- --tool sprite-viewer --manifest assets/placeholder/c-fighter.sprite.json --clip special --character c --move projectile
```

Atalhos:

| Acao | Tecla |
|---|---|
| Inspecionar coordenada local/atlas | Mouse sobre o sprite |
| Arrastar personagem | Mouse esquerdo |
| Proximo clip | `Tab` |
| Clip anterior | `Shift+Tab` |
| Sincronizar clip com golpe | `Enter` |
| Proximo personagem de combate | `C` |
| Personagem de combate anterior | `Shift+C` |
| Proximo golpe | `]` |
| Golpe anterior | `[` |
| Proximo frame | `.` |
| Frame anterior | `,` |
| Pausar/continuar | `Espaco` |
| Zoom | Mouse wheel |
| Resetar zoom | `0` |
| Aumentar `scale` do manifesto | `=` |
| Diminuir `scale` do manifesto | `-` |
| Mover `pivot` do frame atual | `Setas` |
| Mover `pivot` em passos maiores | `Shift+Setas` |
| Ajustar largura/altura do corpo fisico | `Ctrl+Setas` |
| Ajustar altura abaixada do corpo fisico | `Ctrl+Shift+Setas` |
| Gerar rascunho de `frames[].combat` pelo overlay runtime | `N` |
| Adicionar hurtbox no frame atual | `H` |
| Adicionar hitbox no frame atual | `J` |
| Remover box/origem sob o mouse ou ultimo item | `Delete` |
| Mover hurtbox/hitbox/origem de projectile do frame | Mouse esquerdo nas boxes/alcas |
| Redimensionar hurtbox/hitbox do frame | Mouse esquerdo nos cantos da box |
| Salvar manifestos de tuning | `Ctrl+S` |
| Mostrar/esconder dummy | `O` |
| Mostrar/esconder boxes de combate | `M` |
| Mostrar/esconder trajetoria de projectile | `T` |
| Recarregar manifesto e atlas | `F5` |
| Salvar screenshot | `F12` |
| Alternar grade | `G` |
| Alternar pivot | `P` |
| Alternar bounds | `B` |
| Resetar posicao | `R` |

O corte atual e viewer com ajuste controlado de escala, pivot, corpo fisico e metadata visual de `frames[].combat`. Ele mostra frame bounds, pivot, dummy espelhado, distancia entre anchors, coordenada local/atlas do cursor, `trimmed_bounds`, `source_crop`, hurtboxes atuais do corpo, hitbox do golpe selecionado, origem/caixa de projectile, trajetoria prevista de projectile, timeline visual e metadata opcional de `frames[].combat`. A camada runtime de combate usa `--character` e `--move`; quando `--character` nao e passado, o viewer tenta inferir Rust/Duke/Go/C pelo nome do manifesto e tambem permite alternar personagem/golpe sem reiniciar a ferramenta. `N` substitui a metadata do frame atual por um rascunho baseado no overlay runtime; depois o artista/dev cria boxes com `H`/`J`, remove com `Delete`, ajusta as boxes e a origem com mouse e salva com `Ctrl+S`. `Enter` tenta sincronizar o clip visual com o golpe atual quando o manifesto possui um clip conhecido para aquele `--move`, incluindo os nove golpes proximos e `special`. Screenshots de review sao salvas em `target/sprite-viewer-capture.png`. O roadmap completo fica em [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md).

## Pontos ainda em aberto

- Definir se o formato v1 vira padrao permanente ou ponte para Aseprite JSON.
- Decidir se hurtbox/hitbox por frame ficam definitivamente no manifesto ou migram para dados externos quando a arte estabilizar.
- Validar em playtest o padrao inicial de escala definido em [`docs/17-visual-scale-and-stage-metrics.md`](17-visual-scale-and-stage-metrics.md).
- Criar criterio visual para aceitar atlas de personagem como "candidato" em vez de placeholder.
- Definir criterio de review para `projectile_origin`, hitbox e hurtbox antes de aceitar metadata como balanceamento confiavel.
