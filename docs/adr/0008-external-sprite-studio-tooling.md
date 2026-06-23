# ADR 0008 — Sprite Studio externo em Tauri + React

## Status

Aceito.

## Contexto

O Sprite Combat Viewer nasceu dentro do jogo em Rust + Raylib para validar rapidamente atlas, pivot, escala, origem de projétil, hitbox e hurtbox. Esse corte foi útil para provar o formato `*.sprite.json`, mas a ferramenta começou a exigir UI de editor: painéis, timeline, seleção de frames, handles de resize, formulários, zoom, grid, undo/redo e salvamento de artefatos.

Raylib continua adequada para o jogo, Combat Lab e debug visual rápido, mas não é uma boa base para uma ferramenta de produção voltada a artistas.

## Decisão

Criar um aplicativo separado chamado **Sprite Studio** em `tools/sprite-studio`, usando:

- Tauri 1.8 para empacotar uma aplicação desktop leve com backend Rust;
- React + TypeScript para UI rica, painéis, formulários e canvas/SVG;
- pnpm como gerenciador de pacotes JavaScript;
- contratos de JSON duplicados dentro do Sprite Studio, sem dependência de código do jogo principal.

Tauri 1.8 foi escolhido para o corte inicial por compatibilidade com o ambiente WSL/Ubuntu 20.04 do projeto. Esse ambiente fornece GLib/GIO 2.64.6 e `libwebkit2gtk-4.0-dev`; a cadeia Linux do Tauri 2 usa WebKitGTK/libsoup3 e exige GLib/GIO 2.70, quebrando em `pkg-config` via `glib-sys`/`gio-sys`.

Revisitar Tauri 2 quando o baseline de desenvolvimento migrar para uma distro com GLib/GIO >= 2.70 ou quando o projeto decidir empacotar o Studio em um ambiente controlado.

O jogo e o Sprite Studio se comunicam apenas por artefatos de disco:

- `*.sprite.json`;
- atlas PNG;
- imagens de projétil;
- manifestos de tuning quando aplicável.

Não haverá crate compartilhado, import de structs do jogo, workspace comum nem dependência de runtime entre os dois projetos nesta fase.

## Consequências

### Positivas

- A UI de editor deixa de disputar espaço com o game loop.
- Artistas podem usar uma ferramenta dedicada para inspecionar atlas, grid, pivot, hitbox, hurtbox e origem de projétil.
- O jogo permanece focado em runtime e combate.
- O formato de artefatos continua simples e auditável.
- O isolamento reduz acoplamento entre protótipo jogável e tooling.

### Negativas

- Alguns contratos de manifesto serão duplicados.
- Mudanças no schema exigem atualização explícita no jogo e no Sprite Studio.
- O repositório passa a ter uma segunda stack: Rust/Raylib no jogo e Tauri/React no tooling.
- Validações precisam garantir que o Sprite Studio não gere JSON que o jogo rejeita.

## Regra De Isolamento

O Sprite Studio pode ler e escrever arquivos do repositório, mas não pode depender de módulos Rust, structs, crates internos ou lógica compilada do jogo. Se o schema evoluir, a sincronização deve acontecer por documentação, testes e exemplos de manifesto.

## Migração Do Viewer Antigo

O Sprite Combat Viewer embutido no jogo fica temporariamente disponível enquanto o Sprite Studio ainda não tiver paridade operacional.

Critério mínimo para remover o viewer do jogo:

- abrir um `*.sprite.json` existente;
- carregar o atlas referenciado;
- navegar por clips e frames;
- visualizar grid, frame bounds, pivot, escala, hitboxes, hurtboxes e origem de projétil;
- editar e salvar `scale`, `pivot`, `frames[].combat.hitboxes`, `frames[].combat.hurtboxes` e `frames[].combat.projectile_origin`;
- validar que o jogo carrega o manifesto salvo;
- documentar o fluxo para artistas.

Quando esses critérios forem atendidos, remover o código de Sprite Viewer de `src/scenes`, `src/engine/render` e rotas de menu/CLI relacionadas, mantendo o Combat Lab no jogo.

## Alternativas Consideradas

### Manter tudo em Raylib

Prós:

- uma stack só;
- acesso direto ao rendering do jogo.

Contras:

- UI complexa demais para Raylib;
- painéis e formulários viram código manual;
- ferramenta de artista fica presa ao loop do jogo.

### egui/eframe

Prós:

- Rust puro;
- boa UI imediata para ferramentas.

Contras:

- menos familiar para colaboradores de frontend;
- layout e edição visual rica tendem a exigir mais código customizado.

### Electron

Prós:

- ecossistema maduro;
- React muito bem suportado.

Contras:

- runtime mais pesado;
- menos alinhado ao desejo de manter Rust no backend local.

### C# + Windows Forms

Prós:

- rápido para UI desktop simples no Windows.

Contras:

- amarra a ferramenta a Windows;
- cria stack distante do restante do projeto;
- pior para colaboração cross-platform.

## Critério De Revisão

Revisar esta decisão se:

- Tauri dificultar o acesso a arquivos locais no fluxo real de artistas;
- o baseline Linux do projeto deixar de precisar de Ubuntu 20.04/GLib 2.64;
- o Sprite Studio exigir rendering idêntico ao jogo em nível de shader/runtime;
- a duplicação de schema virar fonte frequente de bugs;
- o projeto decidir migrar para uma engine com editor visual próprio.
