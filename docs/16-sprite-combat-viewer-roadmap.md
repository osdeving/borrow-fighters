# 16 — Roadmap do Sprite Combat Viewer

## Status

Em implementacao inicial na branch `tooling/sprite-combat-viewer`.

Issue de rastreio: [#15](https://github.com/osdeving/borrow-fighters/issues/15).

## Objetivo

Criar uma ferramenta leve para artistas e devs verificarem sprites, pivots, alinhamento visual, escala e, nas proximas fases, hitbox, hurtbox, origem de projectile e dummy de contato sem entrar no fluxo normal de luta.

O viewer nao deve ser tratado como editor final. Ele e uma ferramenta plugavel de producao para reduzir tentativa e erro ao criar atlas e ajustar manifestos.

## Por Que Nao Basta o Combat Lab

O Combat Lab valida golpes dentro de uma cena de combate. Ele e bom para frame data, vantagem, pushback e dummy de contato, mas nao resolve bem o trabalho do artista quando a pergunta e:

- o pivot do frame esta no lugar correto?
- o personagem cabe dentro do retangulo declarado?
- o atlas carrega em runtime sem recompilar?
- a escala do personagem faz sentido contra a grade?
- o frame atual tem sujeira, corte ruim ou continuidade quebrada?
- o hit/hurt box futuro precisa ser ajustado por frame?

O Sprite Combat Viewer fica um nivel antes: ele inspeciona o asset e seus dados.

## Pesquisa de GUI

Para o primeiro corte, Raylib puro e suficiente: grade, linhas, mouse drag, texto e atalhos de teclado cobrem a inspecao basica.

Se a ferramenta passar a precisar de painel lateral com checkboxes, sliders, listas e inputs editaveis, a opcao candidata e `raygui`, biblioteca immediate-mode criada para o ecossistema raylib. O crate `raylib` 6.0 tambem expoe uma feature `raygui`, entao o caminho tecnico existe sem trocar stack.

Decisao atual:

- usar Raylib puro no corte inicial;
- evitar `egui`/`imgui` enquanto a ferramenta couber em primitives;
- avaliar `raygui` na Fase 2 ou 3 se os controles por tecla ficarem insuficientes;
- nao salvar arquivos automaticamente antes de termos consenso sobre schema de boxes.

Referencias:

- [raygui](https://github.com/raysan5/raygui): GUI immediate-mode para raylib.
- [`raylib` crate](https://crates.io/crates/raylib): binding Rust usado pelo projeto; a versao 6.0 possui feature `raygui`.

## Corte Implementado Agora

Comando:

```bash
cargo run -- --tool sprite-viewer --manifest assets/placeholder/rust-fighter.sprite.json --clip idle
```

Responsabilidades atuais:

- carregar manifesto `borrow-fighters.sprite.v1` em runtime;
- resolver o PNG do atlas relativo ao manifesto;
- abrir uma cena isolada do jogo normal;
- mostrar grade de tela inteira e linha de chao;
- desenhar o frame atual do atlas com pivot e escala do manifesto;
- exibir retangulo do frame, `trimmed_bounds` e `source_crop` quando existirem;
- permitir arrastar o personagem com mouse para testar encaixe;
- permitir arrastar um dummy espelhado para comparar escala, distancia e continuidade contra um oponente;
- aplicar zoom visual com mouse wheel sem alterar o manifesto;
- recarregar manifesto e atlas com `F5`;
- salvar screenshot de review em `target/sprite-viewer-capture.png` com `F12`;
- navegar clips e frames;
- mostrar path do manifesto, path da imagem, frame atual, pivot, anchor e escala.

Atalhos:

| Acao | Tecla |
|---|---|
| Arrastar personagem | Mouse esquerdo |
| Proximo clip | `Tab` |
| Clip anterior | `Shift+Tab` |
| Proximo frame | `.` |
| Frame anterior | `,` |
| Pausar/continuar | `Espaco` |
| Zoom | Mouse wheel |
| Resetar zoom | `0` |
| Mostrar/esconder dummy | `O` |
| Recarregar manifesto e atlas | `F5` |
| Salvar screenshot | `F12` |
| Alternar grade | `G` |
| Alternar pivot | `P` |
| Alternar bounds | `B` |
| Resetar posicao | `R` |

Codigo principal:

- [`src/scenes/sprite_viewer.rs`](../src/scenes/sprite_viewer.rs): estado testavel, manifesto, frame atual, playback e drag.
- [`src/engine/render/sprite_viewer.rs`](../src/engine/render/sprite_viewer.rs): desenho Raylib da ferramenta.
- [`src/cli.rs`](../src/cli.rs): modo `--tool sprite-viewer`.
- [`src/app.rs`](../src/app.rs): desvio para o loop isolado da ferramenta.
- [`tests/sprite_viewer.rs`](../tests/sprite_viewer.rs): testes do estado sem abrir janela.

## Roadmap

### Fase 1 — Viewer de Sprite

Status: em andamento.

Entregas:

- abrir manifesto e atlas em runtime;
- grid, chao, pivot e bounds;
- drag do personagem e dummy com mouse;
- dummy espelhado com distancia entre anchors;
- zoom visual por mouse wheel;
- hot reload manual de manifesto e atlas;
- screenshot de review;
- navegacao de clips/frames;
- testes de estado.

Falta:

- legenda visual mais clara para cada guia.
- pan de camera quando o personagem ficar maior que a tela;
- UI clicavel se a lista de comandos ficar grande demais.

### Fase 2 — Viewer de Combate

Objetivo: enxergar boxes reais, nao apenas bounds de sprite.

Entregas planejadas:

- carregar dois personagens na mesma tela;
- mover personagem e dummy com mouse;
- alternar personagem/clip/golpe;
- desenhar hurtbox e hitbox reais do golpe selecionado;
- mostrar origem de projectile e trajetoria prevista;
- disparar golpe em step frame, sem depender da luta completa.

Dependencia tecnica:

- decidir onde os boxes por frame vivem: manifesto de sprite, dados de personagem, ou um arquivo lateral.

### Fase 3 — Ajuste Data-Driven

Objetivo: permitir que arte e combate sejam ajustados por dados revisaveis no Git.

Entregas planejadas:

- schema de `hurtboxes`, `hitboxes`, `projectile_origin` e talvez `root_motion` por frame;
- validacao de manifesto para boxes fora do frame;
- import/export manual de offsets;
- checklist visual de aceitacao de atlas.

Regra importante: a ferramenta pode sugerir valores, mas nao deve reescrever manifestos automaticamente enquanto o schema nao estiver estabilizado.

### Fase 4 — Produtividade Para Artistas

Entregas planejadas:

- painel com controles clicaveis se `raygui` for adotado;
- preset de escala por personagem;
- overlay comparativo contra greybox de altura alvo;
- marca de chao, centro, alcance e margem segura;
- export de screenshot para PR/review de arte.

### Fase 5 — Integracao Com Gameplay

Entregas planejadas:

- abrir diretamente um `MoveId`/golpe;
- reproduzir startup/active/recovery com frame data real;
- comparar visual do sprite contra hitbox/hurtbox de cada frame;
- validar se projectile nasce da mao correta;
- salvar notas tecnicas para a issue/PR.

## Decisoes Em Aberto

- O schema de hitbox/hurtbox por frame entra no `borrow-fighters.sprite.v1` ou vira arquivo separado?
- O viewer deve permitir editar dados ou apenas visualizar e gerar sugestoes?
- O Combat Lab deve reaproveitar o mesmo estado do viewer ou apenas os mesmos dados?
- Go deve ganhar atlas placeholder antes de boxes por frame?
- `raygui` entra como feature opcional ou evitamos dependency feature ate a ferramenta exigir?

## Criterios de Aceite Para Contribuidores

Um atlas candidato deve:

- carregar sem erro pelo Sprite Combat Viewer;
- manter pivot consistente entre idle, crouch, jump e golpes;
- nao cortar partes importantes do personagem dentro do frame declarado;
- manter o pe/anchor estavel quando a pose nao implica deslocamento;
- indicar visualmente de onde sairia projectile ou golpe ativo, mesmo antes do dado final existir;
- vir acompanhado do manifesto JSON e, quando possivel, screenshot do viewer no PR.
