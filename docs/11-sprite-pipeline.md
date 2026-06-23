# 11 — Pipeline de Sprites

## Status

Em implementacao. O runtime ja carrega manifests para Rust, Duke, Go, C, animacoes de entrada, clips de luta, pivots, duracoes por frame e fallback greybox.

## Objetivo

Permitir que artistas entreguem sprites com dados suficientes para o jogo renderizar animacoes sem hardcode de frame no codigo, mantendo pivots e hurtboxes ajustaveis quando a arte exigir.

## Formato candidato

O formato inicial do projeto e `borrow-fighters.sprite.v1`, em JSON.

Exemplo real:

- `assets/placeholder/rust-fighter.sprite.json`
- `assets/placeholder/duke-fighter.sprite.json`
- `assets/placeholder/rust-start.sprite.json`
- `assets/placeholder/duke-start.sprite.json`
- `assets/placeholder/c-fighter.sprite.json`
- `assets/placeholder/c-start.sprite.json`
- `assets/placeholder/python-fighter.sprite.json`

Campos principais:

- `schema`: versao do formato.
- `image`: PNG do atlas.
- `source`: imagem ou arquivo de origem.
- `cell`: tamanho padrao da celula.
- `default_pivot`: ponto de apoio padrao, normalmente perto do pe no chao.
- `scale`: escala visual runtime do atlas; o jogo e o viewer usam o mesmo valor.
- `frames`: retangulos no atlas, duracao e pivot por frame.
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

## Clips recomendados

- `spawn`
- `idle`
- `walk`
- `crouch`
- `jump`
- `block`
- `punch_light`
- `punch_heavy`
- `kick`
- `hit`
- `special`

Clips extras como `taunt`, `victory`, `defeat` e `projectile` podem existir, mas nao devem bloquear o prototipo.

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
Python possui um atlas candidato em `assets/placeholder/python-fighter.sprite.json`, gerado como placeholder visual e ainda nao integrado ao roster jogavel.

O tamanho em jogo nao deve depender da resolucao do PNG. Ajuste `scale` e `frames[].pivot` no manifesto; o renderer de luta e o Sprite Combat Viewer consomem os mesmos valores. O padrao atual de altura, largura e arena fica em [`docs/17-visual-scale-and-stage-metrics.md`](17-visual-scale-and-stage-metrics.md).

O corpo fisico de gameplay fica em [`assets/tuning/character-body-metrics.json`](../assets/tuning/character-body-metrics.json). Esse arquivo controla `width`, `standing_height` e `crouch_height` por personagem. Ele define o retangulo base usado por colisao corpo-corpo, hurtboxes compostas e alinhamento do sprite. `frames[].combat` pode substituir hitbox/hurtbox por frame quando houver metadata revisada.

No corte atual, Rust, Duke, Go e C ja declaram `frames[].combat.projectile_origin` no primeiro frame do clip `special`, usado pelo runtime para alinhar o nascimento do projectile com a mao do personagem. Rust tambem possui `frames[].combat.hitboxes[]` iniciais para `Borrow Jab`, heavy punch e kick, calibradas para reproduzir o alcance atual do `MoveSpec` antes de qualquer ajuste de balanceamento. Outras hitboxes e hurtboxes por frame ainda devem ser preenchidas pelo Sprite Combat Viewer antes de substituir alcances de soco/chute em producao.

O runtime tambem usa:

- `spawn` durante a entrada inicial de Rust e Duke;
- `special` por alguns frames quando o personagem dispara projectile;
- `taunt` quando o personagem vence a luta;
- fallback greybox quando um atlas nao carrega.

As animacoes de entrada atuais vivem em manifests separados para nao misturar frames cinematograficos grandes com o atlas principal de luta:

- `assets/placeholder/rust-start-atlas.png`
- `assets/placeholder/duke-start-atlas.png`
- `assets/placeholder/go-start-atlas.png`
- `assets/placeholder/c-start-atlas.png`

Assets relacionados ao slice atual:

- `assets/placeholder/rust-gear-projectile.png`
- `assets/placeholder/duke-bean-projectile.png`
- `assets/placeholder/go-channel-projectile.png`
- `assets/placeholder/c-bitstream-projectile.png`
- `assets/placeholder/python-fighter-atlas.png`
- `assets/placeholder/arena-sirius.png`
- `assets/placeholder/arena-fortaleza.png`
- `assets/placeholder/arena-java-street.png`
- `assets/placeholder/arena-terminal-compiler-lab.png`

As ferramentas locais ficam em `tools/art/` e `tools/sprite-studio/`. Scripts em `tools/art/` devem ser tratados como utilitarios de prototipo. O `tools/sprite-studio/` e o app desktop externo para editar manifestos e reduzir a dependencia do viewer Raylib embutido no jogo.

O atlas candidato de Python e reconstruido por:

```bash
python3 tools/art/build_python_fighter_atlas.py
```

Ele repacota `assets/references/python-fighter-atlas-source.png` para a grade runtime do C (`6x16`, celulas `384x256`) e gera `assets/placeholder/python-fighter.sprite.json`.

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

O corte atual e viewer com ajuste controlado de escala, pivot, corpo fisico e metadata visual de `frames[].combat`. Ele mostra frame bounds, pivot, dummy espelhado, distancia entre anchors, coordenada local/atlas do cursor, `trimmed_bounds`, `source_crop`, hurtboxes atuais do corpo, hitbox do golpe selecionado, origem/caixa de projectile, trajetoria prevista de projectile, timeline visual e metadata opcional de `frames[].combat`. A camada runtime de combate usa `--character` e `--move`; quando `--character` nao e passado, o viewer tenta inferir Rust/Duke/Go/C pelo nome do manifesto e tambem permite alternar personagem/golpe sem reiniciar a ferramenta. `N` substitui a metadata do frame atual por um rascunho baseado no overlay runtime; depois o artista/dev cria boxes com `H`/`J`, remove com `Delete`, ajusta as boxes e a origem com mouse e salva com `Ctrl+S`. `Enter` tenta sincronizar o clip visual com o golpe atual quando o manifesto possui um clip conhecido como `punch_light`, `punch_heavy` ou `special`. Screenshots de review sao salvas em `target/sprite-viewer-capture.png`. O roadmap completo fica em [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md).

## Pontos ainda em aberto

- Definir se o formato v1 vira padrao permanente ou ponte para Aseprite JSON.
- Decidir se hurtbox/hitbox por frame ficam definitivamente no manifesto ou migram para dados externos quando a arte estabilizar.
- Validar em playtest o padrao inicial de escala definido em [`docs/17-visual-scale-and-stage-metrics.md`](17-visual-scale-and-stage-metrics.md).
- Criar criterio visual para aceitar atlas de personagem como "candidato" em vez de placeholder.
- Definir criterio de review para `projectile_origin`, hitbox e hurtbox antes de aceitar metadata como balanceamento confiavel.
