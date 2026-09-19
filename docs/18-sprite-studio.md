# 18 — Sprite Studio

## Status

Corte funcional implementado e validado em `tools/sprite-studio`.

O Sprite Studio e a ferramenta desktop externa para revisar e editar artefatos
de sprite do Borrow Fighters. Apos a paridade operacional documentada, o Sprite
Combat Viewer Raylib foi removido do jogo, menu e CLI. Combat Lab e Actor Lab
continuam disponiveis para seus respectivos fluxos de verificacao no jogo.

## Decisao

A decisao arquitetural fica em [`docs/adr/0008-external-sprite-studio-tooling.md`](adr/0008-external-sprite-studio-tooling.md).

Resumo:

- Tauri 1.8 + React + TypeScript;
- pnpm como package manager;
- backend Rust apenas para ler/salvar arquivos locais;
- sem dependencia de codigo do jogo principal;
- comunicacao por artefatos: `*.sprite.json`, PNGs e manifestos de tuning.

O Studio usa Tauri 1.8.x nesta fase porque o ambiente WSL/Ubuntu 20.04 do projeto possui GLib/GIO 2.64.6. A cadeia Linux do Tauri 2 puxa WebKitGTK/libsoup3 e exige GLib/GIO 2.70 via `glib-sys`/`gio-sys`, causando falha de `pkg-config`. Tauri 1 usa WebKitGTK/libsoup2 e compila com `libwebkit2gtk-4.0-dev`.

## Como Rodar

```bash
cd tools/sprite-studio
pnpm install
pnpm build
pnpm tauri dev
```

Build desktop debug:

```bash
cd tools/sprite-studio
pnpm tauri build --debug
```

No Linux, o Tauri precisa de bibliotecas nativas de WebKitGTK, DBus, SVG e integracao desktop. Em Ubuntu 20.04, o conjunto esperado e:

```bash
sudo apt-get install -y \
  libdbus-1-dev \
  libwebkit2gtk-4.0-dev \
  librsvg2-dev \
  libxdo-dev \
  libayatana-appindicator3-dev \
  libssl-dev
```

Em Ubuntu mais novo, a documentacao oficial do Tauri pode indicar `libwebkit2gtk-4.1-dev` em vez de `libwebkit2gtk-4.0-dev`.

## Corte Atual

O primeiro corte ja permite:

- abrir manifestos por caminho relativo ao repo ou file picker nativo;
- usar presets de Rust, Duke, Go, C e Python;
- inferir `cpp` para body metrics quando um manifest da C++ for aberto manualmente;
- carregar os atlas PNG referenciados pelo manifesto e por `frames[].image`, incluindo preview e export de review PNG/JSON;
- navegar por clips e frames com lista lateral e timeline horizontal;
- usar barra de menu desktop (`File`, `Edit`, `View`, `Help`) e toolbar de acoes frequentes;
- esconder/mostrar browser lateral, inspector e timeline para abrir espaco no canvas;
- visualizar grid, `trimmed_bounds`, `source_crop`, guia de escala alvo, pivot, hitboxes, hurtboxes e origem de projétil;
- editar `scale`, `pivot`, `duration_ms`, `projectile_origin`, `hurtboxes` e `hitboxes`;
- adicionar caixas novas clicando no canvas em modo `Hurtbox` ou `Hitbox`;
- mover pivot/origem de projétil clicando ou arrastando no canvas;
- selecionar, mover e redimensionar hitboxes/hurtboxes com alças no canvas, cursor contextual e snap opcional de 1/2/4/8/16 px;
- controlar zoom pelo slider, por `+`/`-` ou mouse wheel sobre o canvas;
- desfazer/refazer edições com `Ctrl+Z`, `Ctrl+Shift+Z` ou `Ctrl+Y`;
- consultar `Help -> Sprite Studio Guide` ou `F1` para ver tutorial com diagramas sobre vertical slice, target range, pivot, hitbox, hurtbox e origem de projectile;
- validar manifesto antes de salvar, bloqueando erros que o runtime rejeitaria;
- rodar validacao do runtime por botao, usando `cargo test --test sprite_manifest --test sprite_frames --test sprite_selection --test characters`;
- aplicar presets por frame para hurtbox em pe, hurtbox abaixado, strike medio, strike baixo, origem de projectile e limpeza de metadata de combate;
- editar e salvar `assets/tuning/character-body-metrics.json` por personagem;
- salvar automaticamente rascunhos em `target/sprite-studio-autosave/`;
- criar backup do manifesto anterior em `target/sprite-studio-backups/` antes de salvar no arquivo real;
- exportar PNG de review com sprite, pivot, bounds e boxes;
- exportar JSON de review com frame atual, escala, metricas, validacoes, resultado do runtime e notas;
- salvar o manifesto JSON de volta no arquivo original.

## Fluxo Recomendado

1. Abrir o manifesto pelo preset ou por `Open Manifest`.
2. Escolher clip/frame pela lista lateral ou pela timeline.
3. Ligar `Scale guide` e ajustar `scale`/pivot ate o frame caber no range visual alvo.
4. Usar `Combat Presets` como rascunho inicial de hurtbox/hitbox/origem.
5. Refinar no canvas com snap ligado.
6. Rodar `Run Runtime Validation`.
7. Exportar PNG e JSON de review para anexar no PR ou na issue de arte.

## Comandos De UI

| Acao | Comando |
|---|---|
| Abrir ajuda | `F1` ou `Help -> Sprite Studio Guide` |
| Salvar manifesto | `Ctrl+S` ou `File -> Save JSON` |
| Desfazer | `Ctrl+Z` |
| Refazer | `Ctrl+Shift+Z` ou `Ctrl+Y` |
| Frame anterior/proximo | `Seta esquerda` / `Seta direita` |
| Zoom in/out | `+` / `-`, slider de zoom ou mouse wheel sobre o canvas |
| Criar hurtbox/hitbox | Selecionar modo `Hurtbox`/`Hitbox` e clicar no canvas |
| Mover pivot/origem/box | Arrastar no canvas |
| Redimensionar box | Arrastar as alcas da box selecionada |
| Esconder paineis | `View -> Hide Browser Panel`, `View -> Hide Inspector`, `View -> Hide Timeline` |

## Autosave E Backup

O autosave nao altera o manifesto real. Ele grava o estado sujo atual em:

```text
target/sprite-studio-autosave/
```

Ao salvar manualmente o manifesto real, o backend cria primeiro uma copia do arquivo anterior em:

```text
target/sprite-studio-backups/
```

Essas pastas ficam dentro de `target/` e nao devem ser versionadas.

## Regra De Isolamento

Os structs em [`src/engine/sprites/manifest.rs`](../src/engine/sprites/manifest.rs)
sao a fonte do formato de sprites. O gerador publica
[`schemas/sprite.schema.json`](../schemas/sprite.schema.json) e
[`sprite.generated.ts`](../tools/sprite-studio/src/sprite.generated.ts), sem
importar o crate do jogo no Studio. A validacao semantica de nomes, clips e
geometria permanece no Rust; o schema formaliza o formato dos artefatos.

Ao mudar o formato, atualize os structs e regenere os contratos:

```sh
./dev schema build
./dev schema check
```

Revise tambem este pipeline, os testes de manifesto pertinentes e um manifesto
real quando o formato exigir migracao. Backend de arquivos, UI e runtime
continuam separados; o contrato gerado evita editar tipos duplicados a mao.

## Remocao Do Viewer Do Jogo

A paridade e o smoke test desktop ja estavam registrados antes da remocao.
O Sprite Studio oferece edicao de escala, pivot, boxes, origem de projectile
e metricas fisicas, guia de escala, review e salvamento dos manifestos.

Foram removidos os modulos `src/scenes/sprite_viewer*` e
`src/engine/render/sprite_viewer.rs`, a rota `--tool sprite-viewer`, a entrada
`Training -> Sprite Viewer` e os testes exclusivos do viewer Raylib. O formato
de sprites e os testes de assets usados pelo combate continuam no jogo.

## Proximos Incrementos

- Adicionar comparacao lado a lado com um personagem de referencia real, se a guia de escala nao for suficiente.
- Criar presets mais especificos por personagem/golpe quando a identidade de combate estabilizar.
- Permitir exportar um pacote de review com PNG + JSON juntos.
