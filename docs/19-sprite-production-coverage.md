# 19 — Cobertura da produção de sprites

## Escopo e estado

Cobertura de Rust, Duke/Java, Go, C, Python e C++: **21 ações visuais por personagem, 126 linhas**, incluindo o projétil separado. A defesa agachada representa uma combinação já implementada; vitória e derrota representam o resultado atual da partida. A entrada de C++ preenche o fluxo de apresentação existente, que hoje usa idle para ela. Nenhuma dessas ações requer golpes novos.

Os PNGs e manifests em `assets/placeholder/` são a referência inicial e o fallback preservado. A coluna “uso inicial” registra a situação anterior. Os seis candidatos contêm 20 clips cada, com 373 quadros selecionados; a última coluna registra a entrega. As ressalvas e o alcance exato da verificação ficam no registro abaixo. Novas imagens permanecem **candidatas** até a revisão visual e funcional.

A solicitação explícita de evoluir os seis personagens supera as antigas exclusões de arte final/roster dos documentos iniciais; o combate implementado continua sendo a referência e não recebe novas regras para justificar desenhos.

## Fontes conferidas

- [README](../README.md), [direção de arte](07-art-direction.md), [pipeline de sprites](11-sprite-pipeline.md), [matriz de combate](15-character-combat-matrix.md), [escala](17-visual-scale-and-stage-metrics.md) e [Sprite Studio](18-sprite-studio.md).
- [Estados, controles e timers do Fighter](../src/combat/fighter.rs), [loadouts dos seis personagens](../src/characters/mod.rs), [MoveSpec](../src/combat/move_data.rs) e [ProjectileSpec](../src/combat/projectile.rs).
- [Seleção de clips](../src/engine/sprites/selection.rs), [avanço dos frames](../src/engine/sprites/animation.rs), [desenho](../src/engine/sprites/draw.rs), [seleção de intro/resultado no renderer](../src/engine/render.rs) e [fluxo da partida](../src/game/world.rs).
- Os seis manifests de luta, cinco manifests de entrada, PNGs de referência e atlas de runtime foram lidos; os PNGs listados abaixo foram examinados visualmente.

## Referências principais de identidade

| Personagem | Referência principal e quadros | Identidade a preservar | Atenção no material atual |
|---|---|---|---|
| Rust | [sprinte-rust.png](../assets/references/sprinte-rust.png), primeiro idle; [entrada](../assets/references/rust-start-anim.png), pose inicial e guarda final | Cabelo castanho espetado, goggles bronze na testa, olhos escuros, rosto jovem cartunesco; moletom preto com engrenagem/R laranja, detalhes de caranguejo, luvas sem dedos, calça cargo carvão, tênis pretos/laranja, cinto com acessório marrom; proporção compacta e cabeça grande; contorno escuro, sombras em blocos quentes | O atlas runtime tem halos claros e recortes que atravessam parte do VFX; não usar esses resíduos como contorno aprovado |
| Duke / Java | [duke-sprite.png](../assets/references/duke-sprite.png), primeiro idle; [entrada](../assets/references/duke-start-anim.png), primeira pose | Corpo branco/marfim, cabeça negra triangular alongada, nariz esférico vermelho brilhante, braços/pernas finos pretos, luvas e pés negros; símbolo de vapor/café vermelho e azul no torso; caneca e arma de grãos de café; sem olhos humanos acrescentados | Vapor, dedos e branco do corpo não podem sumir no recorte; halos cinza/branco atuais não são arte final |
| Go | [go-sprite-altas.png](../assets/references/go-sprite-altas.png), primeiro idle; [atlas runtime](../assets/placeholder/go-fighter-atlas.png), `idle_0` para proporção estreita | Gopher azul/ciano, ventre azul muito claro, duas orelhas arredondadas, grandes olhos brancos com pupilas pretas, focinho creme e dois dentes, luvas/estribos escuros e faixa escura na cintura; contorno grosso, sombra azul | Não recuperar a proporção baixa/larga da referência bruta; vários frames runtime têm corpo/topo cortados. `idle_0` também toca a borda inferior e perde parte dos pés: usá-lo só para proporção junto do recorte íntegro de calçados da fonte; nunca usar `idle_4` ou taunt cortado como modelo |
| C | [langc-03.png](../assets/references/langc-03.png), primeiro idle e primeira pose de entrada; [langc-04.png](../assets/references/langc-04.png) para livro e golpes | Homem idoso magro, cabelo branco/cinza longo e despenteado, bigode, rosto expressivo com linhas de idade; jaqueta jeans azul clara, camiseta branca, jeans azul escuro, cinto marrom com fivela dourada, sapatos claros; livro azul/branco com grande C; contorno escuro e sombra em blocos | Remover contaminação magenta, preservar livro/mãos; os frames de victory da fonte são cortados na cintura e não servem como corpo inteiro |
| Python | [python-fighter-raster-source.png](../assets/references/python-fighter-raster-source.png), célula linha 1/coluna 1, com linha 2/coluna 2 para braço e soco | Mulher adulta original, cabelo preto longo ondulado, camisa branca com mangas dobradas, saia preta pregueada, cinto escuro com fivela dourada, sapatos pretos de salto e tira; cobra azul/amarela enrolada na cintura/ombro; proporções humanas e sombreamento pintado | Não usar a versão anterior de roupa como referência principal. Agarrão da fonte contém dummy removido pelo empacotador, com perda visível de parte do corpo; gerar agarrão sem vítima desenhada |
| C++ | [cpp-fighter-raster-source.png](../assets/references/cpp-fighter-raster-source.png), célula linha 1/coluna 1, com linha 2/coluna 2 para luvas e braço | Mulher adulta, cabelo castanho/dourado longo volumoso e ondulado, acessório dourado no cabelo, camisa branca curta com mangas dobradas, calça preta justa, luvas pretas, botas pretas de salto com ornamentos dourados, cinto/corrente e bolsa redonda preta/dourada com operadores; proporções humanas e sombreamento pintado | Preservar bolsa, alça, cabelo e ornamentos em todos os gestos; fontes e atlas têm repetições de uma única pose, não animação completa |

Em cada geração, usar a referência principal fixa junto da sequência já revisada quando útil. Não depender somente da geração anterior. Rust/Duke/Go/C usam leitura cartunesca mais compacta e Python/C++ proporções humanas; a compatibilidade entre eles deve vir da escala em tela, contorno legível e luz coerente, sem redesenhar identidades para uma anatomia única.

## Lacunas concretas da situação inicial

1. Rust, Duke, Go e C já possuem nomes de clips para `sweep`, `overhead`, `anti_air`, `air_punch`, `air_kick` e `throw`, mas os atlas reaproveitam poses terrestres com faixas, setas e círculos coloridos. O desenho não mostra a mecânica correspondente. Esses marcadores estão incorporados ao PNG, não são o overlay de debug.
2. Rust e Duke usam `punch_0`/`punch_1` no soco fraco e apenas `punch_2` no forte. Esse último desenho é uma pose de guarda/retorno, sem o soco forte visível.
3. Python e C++ têm uma pose específica para cada golpe, mas os scripts [Python](../tools/art/build_python_high_res_fighter_atlas.py) e [C++](../tools/art/build_cpp_fighter_atlas.py) repetem essa pose em vários frames. Idle repete a mesma pose, walk alterna walk/idle e vários golpes encerram em contato, sem recuperação própria.
4. No seletor inicial, hitstun e blockstun retornavam tempo `0`; `crouch` retornava `999`; jump amostrava somente `0`, `0.18` e `0.36` segundo. Assim, ter frames adicionais não garantia reproduzi-los. Python/C++ começavam hit e block em idle, o que ocultava a própria reação/defesa durante stun.
5. O vencedor era forçado a `taunt`, com relógio global da partida. Como o clip não era loopável, começava em geral no último quadro. `victory` de C/Python/C++ não era selecionado. A derrota não possuía clip selecionado: o mundo encerrava os updates do Fighter após `outcome` e mantinha a reação/pose existente.
6. A defesa agachada usava o mesmo `block` da defesa em pé, apesar do corpo físico abaixado. Não há esquiva, parry ou defesa aérea separada para produzir nesta tarefa.
7. `projectile` nos manifests de luta não dirige o projétil em partida; o renderer usa texturas separadas. C++ não tem manifesto de entrada e fica em idle durante a intro.

## Matriz por personagem e ação

“Frames / ms” é a quantidade de quadros listados e a soma das durações no manifesto inicial; não significa desenhos únicos. Nos golpes, `a–b / d f` informa `active_start` até `active_end` **inclusivo**, seguido da duração base do golpe em ticks de 60 Hz. O recovery adicional de whiff existe no código e deve manter o retorno visual sem alterar os números de combate.

### Rust

Manifesto inicial: [rust-fighter.sprite.json](../assets/placeholder/rust-fighter.sprite.json). Escala inicial: `1.3333`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 4/720 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 5 / 450 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 4/400 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 2 / 240 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 3/260 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 3/280 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 3/270 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Soco fraco | `RustBorrowJab` — 4–8 / 16 f | `punch_light` — 2 / 160 | `punch_light`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 4/266 ms; ver laudo |
| Soco forte | `HeavyPunch` — 11–20 / 35 f | `punch_heavy` — 1 / 110 | `punch_heavy`: Soco forte completo; guarda isolada não comunica contato | Gerada e integrada como candidata: 4/583 ms; ver laudo |
| Chute | `Kick` — 9–16 / 28 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 4/466 ms; ver laudo |
| Varredura | `SweepKick` — 10–18 / 32 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 4/533 ms; ver laudo |
| Overhead | `OverheadPunch` — 12–18 / 34 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 4/566 ms; ver laudo |
| Anti-air | `RustLifetimeAntiAir` — 6–12 / 26 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 4/433 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 4/366 ms; ver laudo |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 4/433 ms; ver laudo |
| Agarrão | `RustOwnershipThrow` — 5–7 / 20 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Gerada e integrada como candidata: 4/333 ms; ver laudo |
| Especial | `RUST_PROJECTILE_SPEC` — emissão 0 f; visual 21 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/350 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [rust-start](../assets/placeholder/rust-start.sprite.json) — 19 / 2545 | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 4/2545 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 2/660 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/1000 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [rust-gear-projectile.png](../assets/placeholder/rust-gear-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

### Duke / Java

Manifesto inicial: [duke-fighter.sprite.json](../assets/placeholder/duke-fighter.sprite.json). Escala inicial: `1.3333`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 4/700 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 5 / 450 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 4/450 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 2 / 240 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 2/280 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 3/360 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 3/360 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/320 ms; ver laudo |
| Soco fraco | `LightPunch` — 5–10 / 18 f | `punch_light` — 2 / 160 | `punch_light`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Soco forte | `DukeBoilerplatePoke` — 13–22 / 40 f | `punch_heavy` — 1 / 110 | `punch_heavy`: Soco forte completo; guarda isolada não comunica contato | Gerada e integrada como candidata: 3/666 ms; ver laudo |
| Chute | `Kick` — 9–16 / 28 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/466 ms; ver laudo |
| Varredura | `DukeGarbageCollectorSweep` — 13–22 / 38 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/633 ms; ver laudo |
| Overhead | `DukeAbstractFactoryOverhead` — 15–22 / 40 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/666 ms; ver laudo |
| Anti-air | `RisingAntiAir` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/500 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Agarrão | `DukeEnterpriseThrow` — 9–11 / 30 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Gerada e integrada como candidata: 3/500 ms; ver laudo |
| Especial | `DUKE_PROJECTILE_SPEC` — emissão 0 f; visual 25 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/417 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [duke-start](../assets/placeholder/duke-start.sprite.json) — 18 / 2640 | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 4/2640 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 2/900 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/1100 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [duke-bean-projectile.png](../assets/placeholder/duke-bean-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

### Go

Manifesto inicial: [go-fighter.sprite.json](../assets/placeholder/go-fighter.sprite.json). Escala inicial: `1.4400`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 3/540 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 6 / 540 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 2/240 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 3 / 360 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 3/360 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/260 ms; ver laudo |
| Soco fraco | `GoGoroutineJab` — 3–7 / 14 f | `punch_light` — 3 / 240 | `punch_light`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/233 ms; ver laudo |
| Soco forte | `HeavyPunch` — 11–20 / 35 f | `punch_heavy` — 3 / 330 | `punch_heavy`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/583 ms; ver laudo |
| Chute | `GoDeferKick` — 7–13 / 23 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/383 ms; ver laudo |
| Varredura | `SweepKick` — 10–18 / 32 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/533 ms; ver laudo |
| Overhead | `GoChannelOverhead` — 10–15 / 28 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/466 ms; ver laudo |
| Anti-air | `RisingAntiAir` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/500 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Chute aéreo | `GoHopkick` — 5–12 / 20 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/333 ms; ver laudo |
| Agarrão | `CloseThrow` — 6–8 / 22 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Especial | `GO_PROJECTILE_SPEC` — emissão 0 f; visual 16 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/267 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [go-start](../assets/placeholder/go-start.sprite.json) — 24 / 2360 | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 3/2700 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 3/860 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/1040 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [go-channel-projectile.png](../assets/placeholder/go-channel-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

### C

Manifesto inicial: [c-fighter.sprite.json](../assets/placeholder/c-fighter.sprite.json). Escala inicial: `1.5467`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 3/720 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 4/400 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 2/260 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 3/280 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 3/270 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Soco fraco | `CPointerJab` — 5–10 / 18 f | `punch_light` — 5 / 395 | `punch_light`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Soco forte | `CUnsafePoke` — 12–20 / 38 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/633 ms; ver laudo |
| Chute | `CNullStepKick` — 9–16 / 30 f | `kick` — 6 / 560 | `kick`: Preparação, contato próprio e recuperação | Gerada e integrada como candidata: 3/500 ms; ver laudo |
| Varredura | `CSegfaultSweep` — 12–20 / 36 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/600 ms; ver laudo |
| Overhead | `CStackOverflow` — 13–20 / 36 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/600 ms; ver laudo |
| Anti-air | `CInterruptVector` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/500 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Agarrão | `CUndefinedThrow` — 7–9 / 26 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Especial | `C_PROJECTILE_SPEC` — emissão 0 f; visual 20 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/333 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [c-start](../assets/placeholder/c-start.sprite.json) — 7 / 1015 | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 4/2545 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 3/910 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/1000 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [c-bitstream-projectile.png](../assets/placeholder/c-bitstream-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

### Python

Manifesto inicial: [python-fighter.sprite.json](../assets/placeholder/python-fighter.sprite.json). Escala inicial: `0.6667`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 4/720 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 4/400 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 2/320 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 2/320 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 2/320 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/320 ms; ver laudo |
| Soco fraco | `PythonSnakeBite` — 4–9 / 17 f | `punch_light` — 5 / 395 | `punch_light`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/283 ms; ver laudo |
| Soco forte | `PythonDataStrike` — 10–18 / 31 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/516 ms; ver laudo |
| Chute | `PythonHeelKick` — 8–14 / 25 f | `kick` — 6 / 560 | `kick`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/416 ms; ver laudo |
| Varredura | `PythonIndentSweep` — 9–16 / 29 f | `sweep` — 3 / 246 | `sweep`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/483 ms; ver laudo |
| Overhead | `PythonTracebackOverhead` — 11–17 / 31 f | `overhead` — 3 / 276 | `overhead`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/516 ms; ver laudo |
| Anti-air | `PythonVisionAntiAir` — 6–12 / 25 f | `anti_air` — 3 / 240 | `anti_air`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/416 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Agarrão | `PythonConstrictThrow` — 7–9 / 24 f | `throw` — 3 / 276 | `throw`: Manter a pose característica; desenhar preparação e recuperação; sem vítima fixa no PNG | Gerada e integrada como candidata: 3/400 ms; ver laudo |
| Especial | `PYTHON_PROJECTILE_SPEC` — emissão 0 f; visual 18 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [python-start](../assets/placeholder/python-start.sprite.json) — 12 / 2150 | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 4/2650 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 3/960 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/960 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [python-data-projectile.png](../assets/placeholder/python-data-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Novo data stream RGBA candidato / regras baseline preservadas |

### C++

Manifesto inicial: [cpp-fighter.sprite.json](../assets/placeholder/cpp-fighter.sprite.json). Escala inicial: `0.6667`.

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Gerada e integrada como candidata: 3/720 ms; ver laudo |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Gerada e integrada como candidata: 4/400 ms; ver laudo |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Gerada e integrada como candidata: 3/800 ms; ver laudo |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Gerada e integrada como candidata: 3/540 ms; ver laudo |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Gerada e integrada como candidata: 3/340 ms; ver laudo |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Gerada e integrada como candidata: 3/340 ms; ver laudo |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Gerada e integrada como candidata: 3/320 ms; ver laudo |
| Soco fraco | `CppReferenceJab` — 4–8 / 16 f | `punch_light` — 5 / 395 | `punch_light`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/266 ms; ver laudo |
| Soco forte | `CppTemplateStrike` — 10–18 / 33 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/550 ms; ver laudo |
| Chute | `CppOperatorKick` — 8–15 / 26 f | `kick` — 6 / 560 | `kick`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Varredura | `CppVectorSweep` — 9–17 / 31 f | `sweep` — 3 / 246 | `sweep`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/516 ms; ver laudo |
| Overhead | `CppVirtualOverhead` — 11–18 / 32 f | `overhead` — 3 / 276 | `overhead`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/533 ms; ver laudo |
| Anti-air | `CppExceptionAntiAir` — 6–13 / 27 f | `anti_air` — 3 / 240 | `anti_air`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/450 ms; ver laudo |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/366 ms; ver laudo |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Manter a pose característica; desenhar preparação e recuperação | Gerada e integrada como candidata: 3/433 ms; ver laudo |
| Agarrão | `CppMoveThrow` — 6–8 / 23 f | `throw` — 3 / 276 | `throw`: Manter a pose característica; desenhar preparação e recuperação; sem vítima fixa no PNG | Gerada e integrada como candidata: 3/383 ms; ver laudo |
| Especial | `CPP_PROJECTILE_SPEC` — emissão 0 f; visual 18 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Gerada e integrada como candidata: 3/300 ms; ver laudo |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `idle`; sem start_atlas | `spawn`: gesto de entrada e chegada à guarda | Gerada e integrada como candidata: 3/540 ms; ver laudo |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Gerada e integrada como candidata: 3/1200 ms; ver laudo |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Gerada e integrada como candidata: 3/1000 ms; ver laudo |
| Projétil separado | `Projectile`; textura no renderer | [cpp-plusplus-projectile.png](../assets/placeholder/cpp-plusplus-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

## Clips adicionais e propostas futuras

| Existência no material inicial | Uso atual | Tratamento |
|---|---|---|
| `punch_medium`, `kick_light`, `kick_heavy` em C/Python/C++ | Sem AttackKind/input próprio e sem seleção na partida | Referência/fallback, sem novos golpes para consumi-los |
| `knockdown` em C/Python/C++ | Sem estado de knockdown em combate | Pode inspirar derrota visual; não acrescentar wakeup, juggle ou invulnerabilidade |
| `victory` em C/Python/C++ | Não era selecionado pelo renderer inicial | Integrar ao resultado existente, verificando corpo inteiro |
| `taunt` nos seis | Usado como vitória inicial; sem comando de provocação | Preservar como referência/fallback, sem nova mecânica taunt |
| `idle_reference` em Go | Sem seleção de luta | Referência de arte apenas |
| `projectile` em Rust/Duke/C/Python/C++ | Sem animação via manifest em partida | Preservar separado; não contar como clip de lutador integrado |
| Dash, parry, counter, combo novo, recuperação de knockdown | Não implementados | Fora desta produção |

## Timing, escala e colisão

O caso especial mais importante é o projectile: todos os seis specs têm `startup = 0` e `spawn_frame = 0`. Desenhar preparação antes da emissão tornaria o especial visualmente atrasado. A emissão precisa aparecer no primeiro quadro, seguida de acomodação/recuperação. Duração visual/cooldown em ticks: Rust `21/57`, Duke `25/72`, Go `16/44`, C `20/56`, Python `18/50`, C++ `18/52`. O cooldown não é duração da animação e não impede toda ação durante o intervalo.

A escala usa como referência as métricas de [escala visual](17-visual-scale-and-stage-metrics.md): altura idle humana aproximadamente `247–280 px`, largura aproximadamente `147–200 px`, sem alterar `assets/tuning/character-body-metrics.json` para compensar desenho. Pivôs devem preservar apoio no chão; movimento amplo precisa de margem no retângulo exportado. Go preserva a proporção estreita já ajustada no runtime. Python conserva sua silhueta adulta esguia, com largura idle menor que a faixa genérica; alargá-la artificialmente contrariaria sua identidade. A altura foi normalizada sem alterar o corpo físico.

Há metadata que já afeta o combate. Rust possui hitboxes em `punch_0`, `punch_1`, `punch_2`, `kick_1` e `kick_2`; removê-las para cair em MoveSpec muda alcance/posição. Rust, Duke e Go têm `projectile_origin` em seus três frames de especial; C, Python e C++ têm em `special_0`. Os demais golpes usam fallback de MoveSpec. A conversão para novos quadros/pivôs/escalas deve preservar a projeção em mundo dessas caixas e origens, ou registrar explicitamente a alteração para revisão de gameplay. Não preencher hurtboxes estéticas novas como se fossem ajuste neutro.

A simulação usa ticks de 60 Hz e os intervalos ativos incluem ambas as pontas. A arte deve marcar antecipação/ativo/retorno de acordo com os frames efetivamente usados pelo runtime, incluindo o quadro zero e o recovery extra após whiff. Quantidade de desenhos não precisa igualar duração em ticks. A atribuição de poses deve ser explícita por nome/ação, nunca por posição arbitrária na folha gerada.

## Critérios de atualização do estado

- **Gerado**: nova arte raster produzida; referência, prompt e folha por ação preservados. Recorte/reescala/reempacotamento de placeholder é reaproveitamento técnico, não nova geração.
- **Revisado**: sequência examinada em movimento e quadro a quadro; rosto, mãos, pés, roupa, acessórios, apoio e silhueta coerentes.
- **Integrado**: runtime seleciona o clip correto e percorre seus quadros; o asset inclui alpha real, pivôs, escala, duração e mapeamento explícito.
- **Verificado**: inspeção em tamanho real nas duas orientações, transições, contraste com arena e sincronização de combate/projétil, com ferramentas e comandos registrados. Testes de JSON sozinhos não satisfazem a revisão visual.
- **Candidato**: estado de entrega até concluir os critérios visuais e funcionais; qualquer ação faltante continua explicitamente pendente.

## Registro de validação desta produção

Produção raster real concluída para as 120 ações do lutador, com um data stream novo de Python e cinco projéteis originais reaproveitados. Os arquivos são candidatos com ressalvas artísticas; testes estruturais não os promovem automaticamente a arte final. A [galeria](../assets/candidates/README.md) reúne imagens, GIFs, vídeos e laudos por personagem.

### Sprite Studio nativo: piloto Rust e escala

O editor desktop foi aberto neste ambiente com `GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1` e `WAYLAND_DISPLAY` removido do processo. A tentativa inicial por Wayland encontrou falha de EGL. O manifesto real `assets/candidates/rust/rust-fighter.sprite.json` foi carregado pela interface do Studio; os quatro frames de `idle` foram inspecionados pelo transporte de frames a zoom `1.00x`, sem salvar alterações no manifesto ou nas métricas físicas. O clip observado tem quatro poses de `180 ms` (`720 ms` no total); suas medidas visíveis variam entre `159–162 px` de largura e `266–275 px` de altura. Essa variação ainda faz parte da revisão artística do candidato.

A inspeção revelou uma inconsistência do editor: validador, texto do painel e guia desenhada ainda usavam a escala anterior de `110–150 × 185–210 px`. Isso marcava incorretamente o baseline e o candidato Rust como fora da faixa. A correção pequena em `src/validation.ts` e `src/App.tsx` do Studio compartilha os limites atuais de [docs/17](17-visual-scale-and-stage-metrics.md), `147–200 × 247–280 px`, e identifica a faixa como referência de **idle**. Nenhum PNG, pivô, escala de personagem ou dado de combate foi modificado para eliminar o aviso.

`pnpm build` passou. O executável nativo foi recompilado com `cargo build --manifest-path tools/sprite-studio/src-tauri/Cargo.toml`, reaberto e inspecionado novamente: o candidato `idle_00`, medido em `162 × 272 px`, aparece como dentro da faixa atual. Capturas locais em `target/sprite-studio-candidate-updated-scale.png` e `target/sprite-studio-updated-idle-2.png` / `3.png` / `4.png` registram o editor real e a navegação. As capturas anteriores `target/sprite-studio-initial-review.png` e `target/sprite-studio-candidate-review.png` preservam o achado antes da correção.

Esta verificação cobre carregamento, desenho, medidas e navegação quadro a quadro no Studio nativo. Não comprova reprodução contínua, espelhamento, contraste com arenas nem sincronização de golpes no Studio; essas verificações pertencem ao Viewer/Combat Lab/partidas e devem ser registradas separadamente. Não foi usado o botão de validação Rust do editor, pois os checks do jogo foram executados pelo fluxo de terminal.

### Exportação, transparência e integração

Foram exportados seis atlas PNG+JSON, 120 GIFs e 120 painéis de quadros. A contagem de desenhos selecionados é Rust 71, Duke 61, Go 59, C 61, Python 60 e C++ 61. Cada ação preserva fontes, prompts, recortes explícitos, escala uniforme, pivôs e durações. Variantes rejeitadas ficam na produção e não entram automaticamente no atlas. Notas de revisão escritas durante a geração são registros históricos; os laudos finais ligados na galeria consolidam o estado atual.

O primeiro idle Rust exigiu remoção local do xadrez pintado, autorizada pelo usuário. As folhas seguintes usam fundo magenta, removido com descontaminação de bordas antes da exportação RGBA. O runtime continua usando alpha normal, sem shader de chroma key. Pequenos resíduos isolados encontrados nas folhas foram removidos ou excluídos pelos recortes explícitos, sem sobrescrever as fontes. Todos os candidatos foram examinados sobre fundo claro/escuro e em capturas do renderer. Isso não equivale a garantir perfeição em todo contorno subpixel.

O carregamento é opt-in por `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`, com fallback para os originais se o candidato estiver ausente, inválido ou sem algum dos 20 clips exigidos. A seleção agora percorre hit, guarda, agachamento, salto, entrada, vitória e derrota com relógios visuais apropriados. O relógio do resultado avança mesmo com o combate encerrado; as poses de resultado são apoiadas no chão visualmente, inclusive quando o corpo físico congelou no ar. O espelhamento foi corrigido para o contrato de retângulo negativo do Raylib, preservando a posição da célula no atlas.

Os manifests candidatos contêm apresentação, enquanto o `combat_manifest` conserva os dados baseline. Isso preserva os cinco frames com hitboxes próprias do Rust e as origens dos seis especiais. MoveSpec, ProjectileSpec e métricas físicas não foram modificados. O projétil novo de Python substitui somente a textura candidata: o PNG antigo tinha a personagem caída. A fórmula existente desenha o novo canvas em 86,4 × 40,8 px; a colisão continua 85,333 × 40 px.

### Combat Lab: poses, contato e emissão

O capturador nativo [capture_sprite_review.rs](../examples/capture_sprite_review.rs) percorreu os 19 contextos oferecidos pelo Lab para cada personagem: nove golpes próximos, especial e nove estados/poses. Walk é verificado no World. As capturas usam a orientação nativa do Lab; ambos os sentidos também foram examinados nos previews e no cenário World.

A revisão detectou que o Lab antigo desenhava caixas de MoveSpec e emitia por `Projectile::from_fighter`, ignorando os metadados baseline usados pela partida. Lab, Showcase e capturador agora recebem esse manifesto e projetam caixas/origens pelo mesmo caminho do World. Os contextos afetados foram recapturados. As estimativas de vantagem e o posicionamento automático do dummy continuam baseados em MoveSpec; essa limitação do Lab está explícita.

| Personagem | Captura de todos os contextos | Conferência final de contato/origem |
|---|---|---|
| Rust | 277 PNGs históricos; antes da calibração final do salto | Seis golpes por MoveSpec continuam válidos. LP/HP/kick recapturados em 60 PNGs com as caixas baseline; especial em 17. Salto final revisto no World |
| Duke | 267 PNGs | Nove contatos por MoveSpec inspecionados; especial baseline recapturado em 16 PNGs |
| Go | 270 PNGs | Nove contatos por MoveSpec inspecionados; especial baseline recapturado em 19 PNGs |
| C | 254 PNGs após compactar agachamento/guarda | Nove contatos por MoveSpec inspecionados; especial baseline recapturado em 15 PNGs |
| Python | 263 PNGs com soco v5 e novo projétil | Nove contatos por MoveSpec inspecionados; especial baseline recapturado em 16 PNGs |
| C++ | 253 PNGs já com o Lab corrigido | Nove contatos e primeiro frame do especial inspecionados com baseline carregado |

Relatórios completos e amostras selecionadas ficam nos diretórios `review/runtime-lab*`, `review/runtime-baseline-special/` e, no Rust, `review/runtime-baseline-attacks/`. Os laudos na galeria apontam o diretório exato de cada execução. Nas seis emissões, a saída visual coincide com mão, arma, livro, boca da cobra ou bolsa no primeiro quadro; testes também comparam a origem numérica do Lab à do World. As imagens históricas de especiais não são usadas como prova da origem corrigida.

### World, movimento e partidas nativas

Cada personagem foi capturado no World com os dois sentidos simultâneos, entrada/contagem reais, avanço/recuo, retorno ao idle e salto físico completo. Foram 585 estados por execução: 129 imagens Rust, 125 Duke, 122 Go, 124 C, 129 Python e 123 C++. Os manifests carregados foram comparados aos candidatos e mantidos estáveis durante cada captura. O cenário C precede apenas a última troca da arte baixa; caminhada e salto já eram os finais. Python usa a execução `python-motion-clean-final`, posterior à remoção dos pixels alpha isolados.

Os seis vídeos em `review/runtime-motion/runtime-motion.mp4` preservam o tempo da simulação, apresentados a 20 fps. Cabeça/cintura e apoio foram inspecionados em frames consecutivos, principalmente na troca entre subida e queda. Todos retornam ao idle grounded no tick 543, sem mudar HP. São inputs determinísticos no World; não constituem playtest manual de teclado. Os GIFs de revisão acrescentam uma pausa final e não devem ser usados para medir o tempo da partida.

Três partidas do aplicativo nativo chegaram ao resultado por combate real entre CPUs: Rust × Rust terminou 9 × 0; Go × C terminou 0 × 76; Python × C++ terminou 17 × 0. As [capturas de resultado](../assets/candidates/runtime-matches/README.md) mostram as poses de vencedor e derrotado ancoradas no chão, espelhamento e contraste na arena. Elas não cobrem todas as combinações do roster nem comprovam fluidez contínua por si só.

### Checks e ressalvas de entrega

Passaram `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test` após a correção do Lab, com os seis candidatos presentes. Os testes exercitam seleção/avanço, espelhamento, transição de resultado, fallback, preservação de combate, emissão baseline no Lab/World e a pose correta em todos os ticks dos nove ataques de cada personagem. Também passaram os sete testes do exportador, a compilação web/nativa do Studio, validação dos links Markdown, parse de YAML e `git diff --check`. O carregador de projétil foi exercitado nativamente com opt-in desligado, candidato ausente, inválido e válido.

Os candidatos não receberam aprovação artística final. As ressalvas por personagem incluem:

- Caminhadas ainda curtas ou com contrabalanço limitado, especialmente o ciclo de duas poses do Go; há pequenas variações de rosto, roupa e acessórios entre gerações.
- Mãos/pontas de botas excedem parcialmente algumas caixas preservadas. Rust tem sola da varredura acima da caixa e mãos do agarrão além do alcance; Go e Python têm contatos próximos das bordas; o salto da bota de C++ fica cerca de 15 px abaixo da caixa aérea. Os laudos mostram os casos concretos.
- Duke conserva o cone/vapor acima do corpo físico agachado. C foi redesenhado para poses realmente baixas, mas partes da cabeça/livro ainda excedem a caixa. A transição entre crouch e a defesa muito compacta de C++ merece acabamento.
- O espelhamento também inverte letras e símbolos de livros/bolsa. Não há folhas independentes para cada orientação. O comportamento anterior de blockstun poder sair de crouch permanece.
- O Studio foi verificado quadro a quadro apenas no piloto Rust; movimento e duas orientações dos demais foram verificados por previews e capturas World/Lab. Nenhuma combinação ausente é implicitamente aprovada pelos testes automatizados.

Fontes originais e placeholders foram preservados.
