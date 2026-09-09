# 19 — Cobertura da produção de sprites

## Rodada atual: reações próprias Python × C++

O piloto acrescenta oito clips `reaction_*` com quatro desenhos por perfil em
cada personagem. Python e C++ passam a **33 clips / 120 quadros cada**, preservando
os 25 clips / 88 quadros anteriores. Os seis manifestos candidatos somam agora
**161 clips / 574 quadros de ator**. Esse total não inclui os atlas de atores/efeitos
cinematográficos separados.

| Personagem | Clips novos | Desenhos novos | Produção | Validação |
|---|---:|---:|---|---|
| Python | 8 | 32 | [Fontes e prompts](../assets/production/python/reactions-2026-09-09/README.md) | [Par, duas direções e rajada](evidence/python-cpp-reactions/README.md) |
| C++ | 8 | 32 | [Fontes e prompts](../assets/production/cpp/reactions-own-2026-09-09/README.md) | [Par, duas direções e recuperação](evidence/python-cpp-reactions/README.md) |

Os novos clips são opcionais para o carregador; a seleção por contato habilita
somente essas duas personagens. O [padrão da rodada 25](25-python-cpp-contact-reactions.md)
substitui as reações genéricas rejeitadas no playtest e orienta a extensão futura
ao resto do elenco. Os panoramas antigos continuam como histórico dos clips
originais; a evidência atual é o vídeo e a revisão por contato do par.

## Histórico: assinatura e arremessos

Os cinco selecionáveis têm **25 clips cada**, incluindo oito poses por especial, seis por arremesso e reações próprias `heavy_hit`, `launched` e `thrown`. Go conserva 20 clips / 60 quadros. São **145 clips / 510 quadros de ator**, mais **30 quadros de efeitos** em cinco atlas separados. A produção desta rodada contém **156 desenhos novos**: 126 poses e 30 efeitos, com fontes, revisões de matte, recortes e pivôs preservados. Ver [critérios e evidências da assinatura](21-signature-spectacle-and-throws.md).

| Personagem | Ator: clips / quadros | Assinatura: poses / duração | FX | Reações novas |
|---|---:|---|---:|---|
| Rust | 25 / 97 | Borrow Fortress: 8 / 1533 ms | 6 | heavy_hit 4, launched 4, thrown 4 |
| Duke/Java | 25 / 88 | System.out.println!: 8 / 1667 ms | 6 | heavy_hit 3, launched 4, thrown 4 |
| C | 25 / 89 | Segmentation Fault: 8 / 1700 ms | 6 | heavy_hit 3, launched 4, thrown 4 |
| Python | 25 / 88 | import antigravity: 8 / 1667 ms | 6 | heavy_hit 3, launched 4, thrown 4 |
| C++ | 25 / 88 | Undefined Bazooka: 8 / 1800 ms | 6 | heavy_hit 3, launched 4, thrown 4 |

O novo arremesso troca os lados e usa a vítima capturada/invertida/girando/descendo. Anti-air e três especiais lançam a vítima; a aterrissagem começa no quadro de chão e KO mantém o derrotado deitado. O flash de impacto dura apenas 4 frames para preservar a leitura dos desenhos. Efeitos têm geometria física própria, conforme a [ADR 0014](adr/0014-throws-launches-and-signature-effects.md).

## Histórico: primeira rodada de MVP

A rodada solicitada em 8 de setembro adiciona `signature_special` e `knockdown` para **Rust, Duke/Java, C, Python e C++**, com novas rasteiras baixas. O conjunto daquela etapa tinha **130 clips / 420 quadros**: 22 clips por personagem desta rodada, enquanto Go conserva seus 20 clips / 60 quadros. As 15 animações novas somam 60 desenhos, incluindo a substituição dos antigos quadros de rasteira. Fontes e ações anteriores ficam preservadas. Os [critérios e evidências do MVP](20-mvp-combat-showcase.md) acompanham a validação do conjunto integrado.

| Personagem | Especial próprio | Queda/recuperação | Rasteira |
|---|---|---|---|
| Rust | Borrow Break — 4 quadros / 633 ms | 4 / 600 ms | 4 / 533 ms |
| Duke/Java | GC Slam — 4 / 867 ms | 4 / 600 ms | 4 / 633 ms |
| C | Pointer Lance — 4 / 767 ms | 4 / 600 ms | 4 / 600 ms |
| Python | Serpent Slide — 4 / 650 ms | 4 / 600 ms | 4 / 483 ms |
| C++ | Template Arc — 4 / 700 ms | 4 / 600 ms | 4 / 517 ms |

`hit`, `block` e `crouch_block` permanecem dedicados a impactos normais e defesas. Quedas após rasteira, agarrão e slide usam a nova sequência de recuperação. O quadro ativo dos especiais e rasteiras precisa corresponder ao volume físico atual; essa rodada inclui alterações de combate autorizadas, descritas na [ADR 0013](adr/0013-contextual-showcase-and-mvp-combat.md).

## Rodada anterior: escopo e estado

**Rust, Duke/Java, Go, C, Python e C++ concluíram o refinamento e a verificação de arte final.** São **120 clips, 378 quadros selecionados e os dez golpes de cada personagem**. Os laudos atuais abaixo registram fontes novas, ações reaproveitadas, revisão visual, execução nativa e ressalvas concretas. O [plano de continuação](../assets/production/FINALIZATION-PLAN.md) conserva o andamento geral.

O **novo Go semirrealista está concluído e verificado** (20 clips / 60 quadros). A [nova referência fixa](../assets/production/go/reference/README.md) substitui a identidade cartoon rejeitada pelo usuário. O [laudo final](../assets/production/go/finalization-review.md) inclui o ajuste anatômico de jump e a recaptura final; a produção anterior permanece arquivada.

A matriz mantém o escopo alvo de **21 ações visuais por personagem, 126 linhas**, incluindo o projétil separado. As 126 linhas estão cobertas pelos seis conjuntos revisados. A defesa agachada representa uma combinação já implementada; vitória e derrota usam o resultado existente. A entrada de C++ preenche a apresentação existente. Nenhuma ação desta produção requer golpes novos.

A arte revisada em `assets/candidates/` é carregada por padrão. `BORROW_FIGHTERS_SPRITE_CANDIDATES=0` permite comparar os originais preservados em `assets/placeholder/`; `1` continua selecionando os revisados explicitamente. Um conjunto ausente, inválido ou incompleto mantém o fallback; a rodada atual exige 25 clips dos cinco e preserva os 20 de Go. O nome da pasta não substitui o estado artístico dos laudos. `combat_manifest` conserva a metadata baseline; rasteiras e especiais novos usam o combate revisado conforme a ADR 0013.

A coluna “uso inicial” registra a situação anterior; “produção / integração” registra o estado atual dos seis concluídos. O Go antigo está identificado apenas nos registros históricos. A solicitação de evoluir os seis personagens supera as antigas exclusões de arte final/roster dos documentos iniciais, mantendo o combate implementado como referência.

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
| Go — nova identidade | [Master novo](../assets/production/go/reference/master.png) e [contrato de identidade](../assets/production/go/reference/README.md); o [atlas antigo](../assets/references/go-sprite-altas.png) é histórico rejeitado | Gopher antropomórfico adulto e atlético, olhos animais pequenos, focinho modelado, incisivos discretos, pelagem azul-ardósia e peito cinza claro; calça carvão, faixa preta, luvas sem dedos e patas nuas | Concluído e verificado no World/Lab/Studio; preservar o novo master e os registros de apoio/escala por ação. Não reaproveitar poses da identidade rejeitada como arte nova |
| C | [langc-03.png](../assets/references/langc-03.png), primeiro idle e primeira pose de entrada; [langc-04.png](../assets/references/langc-04.png) para livro e golpes | Homem idoso magro, cabelo branco/cinza longo e despenteado, bigode, rosto expressivo com linhas de idade; jaqueta jeans azul clara, camiseta branca, jeans azul escuro, cinto marrom com fivela dourada, sapatos claros; livro azul/branco com grande C; contorno escuro e sombra em blocos | Remover contaminação magenta, preservar livro/mãos; os frames de victory da fonte são cortados na cintura e não servem como corpo inteiro |
| Python | [python-fighter-raster-source.png](../assets/references/python-fighter-raster-source.png), célula linha 1/coluna 1, com linha 2/coluna 2 para braço e soco | Mulher adulta original, cabelo preto longo ondulado, camisa branca com mangas dobradas, saia preta pregueada, cinto escuro com fivela dourada, sapatos pretos de salto e tira; cobra azul/amarela enrolada na cintura/ombro; proporções humanas e sombreamento pintado | Não usar a versão anterior de roupa como referência principal. Agarrão da fonte contém dummy removido pelo empacotador, com perda visível de parte do corpo; gerar agarrão sem vítima desenhada |
| C++ | [cpp-fighter-raster-source.png](../assets/references/cpp-fighter-raster-source.png), célula linha 1/coluna 1, com linha 2/coluna 2 para luvas e braço | Mulher adulta, cabelo castanho/dourado longo volumoso e ondulado, acessório dourado no cabelo, camisa branca curta com mangas dobradas, calça preta justa, luvas pretas, botas pretas de salto com ornamentos dourados, cinto/corrente e bolsa redonda preta/dourada com operadores; proporções humanas e sombreamento pintado | Preservar bolsa, alça, cabelo e ornamentos em todos os gestos; fontes e atlas têm repetições de uma única pose, não animação completa |

Em cada geração, usar a referência principal fixa junto da sequência já revisada quando útil. Não depender somente da geração anterior. Rust/Duke/C usam leitura cartunesca mais compacta e Python/C++ proporções humanas; o novo Go segue o acabamento pintado mais realista de seu master. A compatibilidade vem da escala em tela, contorno legível e luz coerente, preservando cada identidade.

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

Manifesto inicial: [rust-fighter.sprite.json](../assets/placeholder/rust-fighter.sprite.json). Escala inicial: `1.3333`. Estado atual: [arte final verificada](../assets/production/rust/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Revisada e integrada: 4/720 ms; ver laudo atual |
| Caminhada | `velocity.x; grounded` | `walk` — 5 / 450 | `walk`: Passadas legíveis; frente e recuo | Revisada e integrada: 4/400 ms; ver laudo atual |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Revisada e integrada: 3/800 ms; ver laudo atual |
| Agachamento | `crouching` | `crouch` — 2 / 240 | `crouch`: Entrada e sustentação da pose baixa | Revisada e integrada: 3/260 ms; ver laudo atual |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Revisada e integrada: 3/280 ms; ver laudo atual |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Revisada e integrada: 3/270 ms; ver laudo atual |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Revisada e integrada: 3/300 ms; ver laudo atual |
| Soco fraco | `RustBorrowJab` — 4–8 / 16 f | `punch_light` — 2 / 160 | `punch_light`: Preparação, contato próprio e recuperação | Revisada e integrada: 4/266 ms; ver laudo atual |
| Soco forte | `HeavyPunch` — 11–20 / 35 f | `punch_heavy` — 1 / 110 | `punch_heavy`: Soco forte completo; guarda isolada não comunica contato | Revisada e integrada: 4/583 ms; ver laudo atual |
| Chute | `Kick` — 9–16 / 28 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Revisada e integrada: 4/466 ms; ver laudo atual |
| Varredura | `SweepKick` — 10–18 / 32 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/533 ms; ver laudo atual |
| Overhead | `OverheadPunch` — 12–18 / 34 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/566 ms; ver laudo atual |
| Anti-air | `RustLifetimeAntiAir` — 6–12 / 26 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/433 ms; ver laudo atual |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/366 ms; ver laudo atual |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/433 ms; ver laudo atual |
| Agarrão | `RustOwnershipThrow` — 5–7 / 20 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Revisada e integrada: 4/333 ms; ver laudo atual |
| Especial | `RUST_PROJECTILE_SPEC` — emissão 0 f; visual 21 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Revisada e integrada: 3/350 ms; ver laudo atual |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [rust-start](../assets/placeholder/rust-start.sprite.json) — 19 / 2545 | `spawn`: gesto de entrada e chegada à guarda | Revisada e integrada: 4/2545 ms; ver laudo atual |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Revisada e integrada: 2/660 ms; ver laudo atual |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Revisada e integrada: 3/1000 ms; ver laudo atual |
| Projétil separado | `Projectile`; textura no renderer | [rust-gear-projectile.png](../assets/placeholder/rust-gear-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Original reaproveitado com alpha limpo; origem e regras preservadas |

### Duke / Java

Manifesto inicial: [duke-fighter.sprite.json](../assets/placeholder/duke-fighter.sprite.json). Escala inicial: `1.3333`. Estado atual: [arte final verificada](../assets/production/duke/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Revisada e integrada: 4/700 ms; ver laudo atual |
| Caminhada | `velocity.x; grounded` | `walk` — 5 / 450 | `walk`: Passadas legíveis; frente e recuo | Revisada e integrada: 4/450 ms; ver laudo atual |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Revisada e integrada: 3/800 ms; ver laudo atual |
| Agachamento | `crouching` | `crouch` — 2 / 240 | `crouch`: Entrada e sustentação da pose baixa | Revisada e integrada: 2/280 ms; ver laudo atual |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Revisada e integrada: 3/360 ms; ver laudo atual |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Revisada e integrada: 3/360 ms; ver laudo atual |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Revisada e integrada: 3/320 ms; ver laudo atual |
| Soco fraco | `LightPunch` — 5–10 / 18 f | `punch_light` — 2 / 160 | `punch_light`: Preparação, contato próprio e recuperação | Revisada e integrada: 3/300 ms; ver laudo atual |
| Soco forte | `DukeBoilerplatePoke` — 13–22 / 40 f | `punch_heavy` — 1 / 110 | `punch_heavy`: Soco forte completo; guarda isolada não comunica contato | Revisada e integrada: 3/666 ms; ver laudo atual |
| Chute | `Kick` — 9–16 / 28 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Revisada e integrada: 3/466 ms; ver laudo atual |
| Varredura | `DukeGarbageCollectorSweep` — 13–22 / 38 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/633 ms; ver laudo atual |
| Overhead | `DukeAbstractFactoryOverhead` — 15–22 / 40 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/666 ms; ver laudo atual |
| Anti-air | `RisingAntiAir` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/500 ms; ver laudo atual |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/366 ms; ver laudo atual |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/433 ms; ver laudo atual |
| Agarrão | `DukeEnterpriseThrow` — 9–11 / 30 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Revisada e integrada: 3/500 ms; ver laudo atual |
| Especial | `DUKE_PROJECTILE_SPEC` — emissão 0 f; visual 25 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Revisada e integrada: 3/417 ms; ver laudo atual |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [duke-start](../assets/placeholder/duke-start.sprite.json) — 18 / 2640 | `spawn`: gesto de entrada e chegada à guarda | Revisada e integrada: 4/2640 ms; ver laudo atual |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Revisada e integrada: 2/900 ms; ver laudo atual |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Revisada e integrada: 3/1100 ms; ver laudo atual |
| Projétil separado | `Projectile`; textura no renderer | [duke-bean-projectile.png](../assets/placeholder/duke-bean-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Novos grãos RGBA revisados; origem e regras preservadas |

### Go

Manifesto inicial: [go-fighter.sprite.json](../assets/placeholder/go-fighter.sprite.json). Escala inicial: `1.4400`. **Histórico da identidade rejeitada**, preservado no [laudo antigo](../assets/production/go-cartoon-archive/VERIFICATION.md) e [atlas arquivado](../assets/candidates/go-cartoon-archive/go-fighter.sprite.json). O novo Go tem [arte final verificada](../assets/production/go/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 5 / 700 | `idle`: Ciclo de respiração/guarda | Nova identidade revisada e integrada: 3/540 ms; ver laudo final |
| Caminhada | `velocity.x; grounded` | `walk` — 6 / 540 | `walk`: Passadas legíveis; frente e recuo | Nova identidade revisada e integrada: 4/240 ms; ver laudo final |
| Pulo | `!grounded; velocity.y` | `jump` — 3 / 360 | `jump`: Subida, ápice e queda coerentes | Nova identidade revisada e integrada: 3/800 ms; ver laudo final |
| Agachamento | `crouching` | `crouch` — 3 / 360 | `crouch`: Entrada e sustentação da pose baixa | Nova identidade revisada e integrada: 2/360 ms; ver laudo final |
| Defesa em pé | `blocking; blockstun` | `block` — 2 / 240 | `block`: Guarda e reação ao bloqueio | Nova identidade revisada e integrada: 3/300 ms; ver laudo final |
| Defesa agachada | `blocking && crouching` | `block` — 2 / 240 | `crouch_block`: Pose de guarda baixa própria | Nova identidade revisada e integrada: 3/300 ms; ver laudo final |
| Reação a dano | `in_hitstun()` | `hit` — 2 / 240 | `hit`: Impacto e recoil; sem vítima integrada | Nova identidade revisada e integrada: 3/260 ms; ver laudo final |
| Soco fraco | `GoGoroutineJab` — 3–7 / 14 f | `punch_light` — 3 / 240 | `punch_light`: Preparação, contato próprio e recuperação | Nova identidade revisada e integrada: 3/233 ms; ver laudo final |
| Soco forte | `HeavyPunch` — 11–20 / 35 f | `punch_heavy` — 3 / 330 | `punch_heavy`: Preparação, contato próprio e recuperação | Nova identidade revisada e integrada: 3/583 ms; ver laudo final |
| Chute | `GoDeferKick` — 7–13 / 23 f | `kick` — 3 / 285 | `kick`: Preparação, contato próprio e recuperação | Nova identidade revisada e integrada: 3/383 ms; ver laudo final |
| Varredura | `SweepKick` — 10–18 / 32 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Nova identidade revisada e integrada: 3/533 ms; ver laudo final |
| Overhead | `GoChannelOverhead` — 10–15 / 28 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Nova identidade revisada e integrada: 3/466 ms; ver laudo final |
| Anti-air | `RisingAntiAir` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Nova identidade revisada e integrada: 3/500 ms; ver laudo final |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Nova identidade revisada e integrada: 3/366 ms; ver laudo final |
| Chute aéreo | `GoHopkick` — 5–12 / 20 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Nova identidade revisada e integrada: 3/333 ms; ver laudo final |
| Agarrão | `CloseThrow` — 6–8 / 22 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Nova identidade revisada e integrada: 3/366 ms; ver laudo final |
| Especial | `GO_PROJECTILE_SPEC` — emissão 0 f; visual 16 f | `special` — 3 / 300 | `special`: emissão no primeiro quadro e recuperação | Nova identidade revisada e integrada: 3/267 ms; ver laudo final |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [go-start](../assets/placeholder/go-start.sprite.json) — 24 / 2360 | `spawn`: gesto de entrada e chegada à guarda | Nova identidade revisada e integrada: 3/2700 ms; ver laudo final |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 2 / 360; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Nova identidade revisada e integrada: 3/860 ms; ver laudo final |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Nova identidade revisada e integrada: 3/1040 ms; ver laudo final |
| Projétil separado | `Projectile`; textura no renderer | [go-channel-projectile.png](../assets/placeholder/go-channel-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Canal cyan original reaproveitado e verificado no primeiro quadro de emissão, nas duas orientações |

### C

Manifesto inicial: [c-fighter.sprite.json](../assets/placeholder/c-fighter.sprite.json). Escala inicial: `1.5467`. Estado atual: [arte final verificada](../assets/production/c/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Revisada e integrada: 3/720 ms; ver laudo atual |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Revisada e integrada: 4/400 ms; ver laudo atual |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Revisada e integrada: 3/800 ms; ver laudo atual |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Revisada e integrada: 2/260 ms; ver laudo atual |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Revisada e integrada: 3/280 ms; ver laudo atual |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Revisada e integrada: 3/270 ms; ver laudo atual |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Revisada e integrada: 3/300 ms; ver laudo atual |
| Soco fraco | `CPointerJab` — 5–10 / 18 f | `punch_light` — 5 / 395 | `punch_light`: Preparação, contato próprio e recuperação | Revisada e integrada: 3/300 ms; ver laudo atual |
| Soco forte | `CUnsafePoke` — 12–20 / 38 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Preparação, contato próprio e recuperação | Revisada e integrada: 4/633 ms; ver laudo atual |
| Chute | `CNullStepKick` — 9–16 / 30 f | `kick` — 6 / 560 | `kick`: Preparação, contato próprio e recuperação | Revisada e integrada: 3/500 ms; ver laudo atual |
| Varredura | `CSegfaultSweep` — 12–20 / 36 f | `sweep` — 3 / 246 | `sweep`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 4/600 ms; ver laudo atual |
| Overhead | `CStackOverflow` — 13–20 / 36 f | `overhead` — 3 / 276 | `overhead`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/600 ms; ver laudo atual |
| Anti-air | `CInterruptVector` — 7–14 / 30 f | `anti_air` — 3 / 240 | `anti_air`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/500 ms; ver laudo atual |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/366 ms; ver laudo atual |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Substituir pose terrestre com marcador por gesto próprio | Revisada e integrada: 3/433 ms; ver laudo atual |
| Agarrão | `CUndefinedThrow` — 7–9 / 26 f | `throw` — 3 / 276 | `throw`: Substituir pose terrestre com marcador por gesto próprio; sem vítima fixa no PNG | Revisada e integrada: 3/433 ms; ver laudo atual |
| Especial | `C_PROJECTILE_SPEC` — emissão 0 f; visual 20 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Revisada e integrada: 3/333 ms; ver laudo atual |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [c-start](../assets/placeholder/c-start.sprite.json) — 7 / 1015 | `spawn`: gesto de entrada e chegada à guarda | Revisada e integrada: 4/2545 ms; ver laudo atual |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Revisada e integrada: 3/910 ms; ver laudo atual |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Revisada e integrada: 3/1000 ms; ver laudo atual |
| Projétil separado | `Projectile`; textura no renderer | [c-bitstream-projectile.png](../assets/placeholder/c-bitstream-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Reaproveitado original / origem e regras preservadas |

### Python

Manifesto inicial: [python-fighter.sprite.json](../assets/placeholder/python-fighter.sprite.json). Escala inicial: `0.6667`. Estado atual: [arte final verificada](../assets/production/python/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Revisada e integrada: 4/720 ms; ver laudo atual |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Revisada e integrada: 4/400 ms; ver laudo atual |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Revisada e integrada: 3/800 ms; ver laudo atual |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Revisada e integrada: 2/320 ms; ver laudo atual |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Revisada e integrada: 2/320 ms; ver laudo atual |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Revisada e integrada: 2/320 ms; ver laudo atual |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Revisada e integrada: 3/320 ms; ver laudo atual |
| Soco fraco | `PythonSnakeBite` — 4–9 / 17 f | `punch_light` — 5 / 395 | `punch_light`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/283 ms; ver laudo atual |
| Soco forte | `PythonDataStrike` — 10–18 / 31 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/516 ms; ver laudo atual |
| Chute | `PythonHeelKick` — 8–14 / 25 f | `kick` — 6 / 560 | `kick`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 4/416 ms; ver laudo atual |
| Varredura | `PythonIndentSweep` — 9–16 / 29 f | `sweep` — 3 / 246 | `sweep`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 4/483 ms; ver laudo atual |
| Overhead | `PythonTracebackOverhead` — 11–17 / 31 f | `overhead` — 3 / 276 | `overhead`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/516 ms; ver laudo atual |
| Anti-air | `PythonVisionAntiAir` — 6–12 / 25 f | `anti_air` — 3 / 240 | `anti_air`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/416 ms; ver laudo atual |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/366 ms; ver laudo atual |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/433 ms; ver laudo atual |
| Agarrão | `PythonConstrictThrow` — 7–9 / 24 f | `throw` — 3 / 276 | `throw`: Manter a pose característica; desenhar preparação e recuperação; sem vítima fixa no PNG | Revisada e integrada: 3/400 ms; ver laudo atual |
| Especial | `PYTHON_PROJECTILE_SPEC` — emissão 0 f; visual 18 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Revisada e integrada: 3/300 ms; ver laudo atual |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `spawn` em [python-start](../assets/placeholder/python-start.sprite.json) — 12 / 2150 | `spawn`: gesto de entrada e chegada à guarda | Revisada e integrada: 4/2650 ms; ver laudo atual |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Revisada e integrada: 3/960 ms; ver laudo atual |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Revisada e integrada: 3/960 ms; ver laudo atual |
| Projétil separado | `Projectile`; textura no renderer | [python-data-projectile.png](../assets/placeholder/python-data-projectile.png) | Asset separado, sem incorporar um projétil permanente ao corpo | Novo data stream RGBA revisado / regras baseline preservadas |

### C++

Manifesto inicial: [cpp-fighter.sprite.json](../assets/placeholder/cpp-fighter.sprite.json). Escala inicial: `0.6667`. Estado atual: [arte final verificada](../assets/production/cpp/finalization-review.md).

| Ação | Estado/golpe no código | Uso inicial (frames / ms) | Animação própria necessária | Produção / integração |
|---|---|---|---|---|
| Idle | `Fighter parado e grounded` | `idle` — 7 / 840 | `idle`: Ciclo de respiração/guarda | Revisada e integrada: 3/720 ms; ver laudo atual |
| Caminhada | `velocity.x; grounded` | `walk` — 7 / 595 | `walk`: Passadas legíveis; frente e recuo | Revisada e integrada: 4/400 ms; ver laudo atual |
| Pulo | `!grounded; velocity.y` | `jump` — 6 / 645 | `jump`: Subida, ápice e queda coerentes | Revisada e integrada: 3/800 ms; ver laudo atual |
| Agachamento | `crouching` | `crouch` — 4 / 470 | `crouch`: Entrada e sustentação da pose baixa | Revisada e integrada: 3/540 ms; ver laudo atual |
| Defesa em pé | `blocking; blockstun` | `block` — 4 / 435 | `block`: Guarda e reação ao bloqueio | Revisada e integrada: 3/340 ms; ver laudo atual |
| Defesa agachada | `blocking && crouching` | `block` — 4 / 435 | `crouch_block`: Pose de guarda baixa própria | Revisada e integrada: 3/340 ms; ver laudo atual |
| Reação a dano | `in_hitstun()` | `hit` — 4 / 460 | `hit`: Impacto e recoil; sem vítima integrada | Revisada e integrada: 3/320 ms; ver laudo atual |
| Soco fraco | `CppReferenceJab` — 4–8 / 16 f | `punch_light` — 5 / 395 | `punch_light`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/266 ms; ver laudo atual |
| Soco forte | `CppTemplateStrike` — 10–18 / 33 f | `punch_heavy` — 7 / 680 | `punch_heavy`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/550 ms; ver laudo atual |
| Chute | `CppOperatorKick` — 8–15 / 26 f | `kick` — 6 / 560 | `kick`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/433 ms; ver laudo atual |
| Varredura | `CppVectorSweep` — 9–17 / 31 f | `sweep` — 3 / 246 | `sweep`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/516 ms; ver laudo atual |
| Overhead | `CppVirtualOverhead` — 11–18 / 32 f | `overhead` — 3 / 276 | `overhead`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/533 ms; ver laudo atual |
| Anti-air | `CppExceptionAntiAir` — 6–13 / 27 f | `anti_air` — 3 / 240 | `anti_air`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/450 ms; ver laudo atual |
| Soco aéreo | `AirPunch` — 5–13 / 22 f | `air_punch` — 3 / 228 | `air_punch`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/366 ms; ver laudo atual |
| Chute aéreo | `AirKick` — 7–16 / 26 f | `air_kick` — 3 / 240 | `air_kick`: Manter a pose característica; desenhar preparação e recuperação | Revisada e integrada: 3/433 ms; ver laudo atual |
| Agarrão | `CppMoveThrow` — 6–8 / 23 f | `throw` — 3 / 276 | `throw`: Manter a pose característica; desenhar preparação e recuperação; sem vítima fixa no PNG | Revisada e integrada: 3/383 ms; ver laudo atual |
| Especial | `CPP_PROJECTILE_SPEC` — emissão 0 f; visual 18 f | `special` — 6 / 595 | `special`: emissão no primeiro quadro e recuperação | Revisada e integrada: 3/300 ms; ver laudo atual |
| Entrada | `World::spawn_intro_active()` — 2,7 s | `idle`; sem start_atlas | `spawn`: gesto de entrada e chegada à guarda | Revisada e integrada: 3/540 ms; ver laudo atual |
| Vitória | `MatchOutcome::Winner(slot)` | `taunt` — 6 / 930; relógio global | `victory`: reproduzir desde o resultado e sustentar pose | Revisada e integrada: 3/1200 ms; ver laudo atual |
| Derrota | `is_defeated()` / resultado da partida | Sem clip de derrota selecionado | `defeat`: recoil/queda ou derrota legível; sem nova regra knockdown | Revisada e integrada: 3/1000 ms; ver laudo atual |
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

A escala usa como referência as métricas de [escala visual](17-visual-scale-and-stage-metrics.md): altura idle humana aproximadamente `247–280 px`, largura aproximadamente `147–200 px`, sem alterar `assets/tuning/character-body-metrics.json` para compensar desenho. Pivôs devem preservar apoio no chão; movimento amplo precisa de margem no retângulo exportado. O novo Go usa o alvo visual de 264 px do master, com proporções e apoios conferidos por ação; o corpo físico baseline permanece. Python conserva sua silhueta adulta esguia, com largura idle menor que a faixa genérica; alargá-la artificialmente contrariaria sua identidade. A altura foi normalizada sem alterar o corpo físico.

Há metadata que já afeta o combate. Rust possui hitboxes em `punch_0`, `punch_1`, `punch_2`, `kick_1` e `kick_2`; removê-las para cair em MoveSpec muda alcance/posição. Rust, Duke e Go têm `projectile_origin` em seus três frames de especial; C, Python e C++ têm em `special_0`. Os demais golpes usam fallback de MoveSpec. A conversão para novos quadros/pivôs/escalas deve preservar a projeção em mundo dessas caixas e origens, ou registrar explicitamente a alteração para revisão de gameplay. Não preencher hurtboxes estéticas novas como se fossem ajuste neutro.

A simulação usa ticks de 60 Hz e os intervalos ativos incluem ambas as pontas. A arte deve marcar antecipação/ativo/retorno de acordo com os frames efetivamente usados pelo runtime, incluindo o quadro zero e o recovery extra após whiff. Quantidade de desenhos não precisa igualar duração em ticks. A atribuição de poses deve ser explícita por nome/ação, nunca por posição arbitrária na folha gerada.

## Critérios de atualização do estado

- **Gerado**: nova arte raster produzida; referência, prompt e folha por ação preservados. Recorte/reescala/reempacotamento de placeholder é reaproveitamento técnico, não nova geração.
- **Revisado**: sequência examinada em movimento e quadro a quadro; rosto, mãos, pés, roupa, acessórios, apoio e silhueta coerentes.
- **Integrado**: runtime seleciona o clip correto e percorre seus quadros; o asset inclui alpha real, pivôs, escala, duração e mapeamento explícito.
- **Verificado**: inspeção em tamanho real nas duas orientações, transições, contraste com arena e sincronização de combate/projétil, com ferramentas e comandos registrados. Testes de JSON sozinhos não satisfazem a revisão visual.
- **Candidato**: estado de entrega até concluir os critérios visuais e funcionais; qualquer ação faltante continua explicitamente pendente.

## Validação atual — seis personagens concluídos

Os laudos de 07/09/2026 consolidam a arte selecionada e substituem as ressalvas superadas do primeiro lote. Houve geração real por ação com master fixo; fontes, prompts, tentativas rejeitadas, snapshots, alpha, recortes, pivôs e escalas ficam preservados. Os ataques mantêm nomes, duração, fases e comportamento baseline.

| Personagem | Clips / quadros | Refinamentos desta continuação | Evidência atual |
|---|---:|---|---|
| Rust | 20 / 71 | walk, sweep, throw e air_kick novos; pivô ativo de air_punch recalibrado; engrenagem limpa | [Laudo](../assets/production/rust/finalization-review.md): Lab 276 PNGs / 19 contextos, recaptura do projétil 17 PNGs, World 650 PNGs; [vídeo e relatórios](../assets/candidates/rust/review/refinement-2026-09-07/README.md) |
| Duke / Java | 20 / 61 | walk, crouch_block, overhead e grãos do projétil | [Laudo](../assets/production/duke/finalization-review.md): Lab filtrado 42 PNGs e World 636 PNGs; [vídeo](../assets/production/duke/finalization-evidence/runtime-motion.mp4) |
| Go — novo | 20 / 60 | Todas as 20 ações geradas com outra identidade; jump v2 corrige a anatomia entre salto/ataques aéreos | [Laudo](../assets/production/go/finalization-review.md): World completo final, Lab e recaptura jump; [vídeo](../assets/candidates/go/review/refinement-2026-09-07/world/runtime-motion.mp4) |
| C | 20 / 63 | walk, punch_heavy e sweep | [Laudo](../assets/production/c/finalization-review.md): Lab filtrado 33 PNGs e World 625 PNGs; [vídeo](../assets/candidates/c/review/refinement-2026-09-07/world/runtime-motion.mp4) |
| Python | 20 / 62 | walk, kick e sweep | [Laudo](../assets/production/python/finalization-review.md): Lab filtrado 38 PNGs e World v2 completo 650 PNGs; [vídeo](../assets/candidates/python/review/refinement-2026-09-07/world/runtime-motion.mp4) |
| C++ | 20 / 61 | walk, crouch_block e air_kick | [Laudo](../assets/production/cpp/finalization-review.md): Lab filtrado 36 PNGs e World 624 PNGs; [vídeo](../assets/candidates/cpp/review/refinement-2026-09-07/world/runtime-motion.mp4) |

Cada World novo concluiu **1.845 estados**, percorrendo entrada, caminhada avançando/recuando, salto, os dez golpes nas duas orientações e retorno ao idle grounded após os golpes. Os vídeos têm exatamente 1.845 quadros, 60 fps e 30,75 s: mantêm a última imagem GPU entre capturas amostradas, sem interpolação nem pausa artificial. Não contêm um PNG distinto por tick. Os inputs são controlados; a verificação não é descrita como playtest manual de teclado. O Lab complementa as poses ativas com caixas baseline; seus subconjuntos nesta rodada constam dos laudos, sem alegar recaptura completa de todos os contextos dos seis nesta rodada.

Os seis conjuntos também foram exercitados em partidas do aplicativo nativo até o resultado: [Rust × Rust, 9 × 0](../assets/candidates/rust/review/refinement-2026-09-07/native-match-result.png); [Duke × Rust, 14 × 0](../assets/candidates/duke/review/refinement-2026-09-07/native-match-result.png); [C × Python, 2 × 0](../assets/candidates/c/review/refinement-2026-09-07/native-match-result.png); [C++ × Rust, 49 × 0](../assets/candidates/cpp/review/refinement-2026-09-07/native-match-result.png). Go também concluiu [Go × Go, 53 × 0, após jump v2](../assets/candidates/go/review/refinement-2026-09-07/native-match-result.png). As quatro últimas foram abertas sem variável de opt-in, confirmando o fluxo padrão, vitória/derrota apoiadas e corpos inteiros. Os hashes e comandos estão junto às capturas. Isso não cobre todas as combinações do roster.

O **Sprite Studio nativo** carregou o Rust atual pela interface real e percorreu 24 quadros de idle, walk, sweep, throw, air_punch e air_kick em zoom 1.00x. [Capturas e ações de acessibilidade](../assets/candidates/rust/review/refinement-2026-09-07/studio/review-captures.json) confirmam a navegação, com hashes de manifesto/atlas/métricas preservados. Trata-se de inspeção quadro a quadro, não reprodução contínua no Studio. Os demais conjuntos concluídos foram revisados por fontes, previews e execução World/Lab. O novo Go também concluiu a [inspeção nativa no Studio](../assets/candidates/go/review/refinement-2026-09-07/studio/README.md): 20 clips / 60 quadros em zoom 1.00x, com valores da UI e hashes conferidos, sem ajustes salvos. Os três quadros de jump foram [recapturados após a correção de anatomia](../assets/candidates/go/review/refinement-2026-09-07/studio/jump-v2/README.md), completando a revisão do conjunto final.

A primeira captura World de Python foi interrompida por SIGTERM antes do relatório final. Ela foi [excluída como prova de conclusão](../assets/candidates/python/review/refinement-2026-09-07/world/discarded-capture.json); somente a repetição v2 completa sustenta os números acima. Após as exportações de C/Python, `cargo test --test sprite_candidates` passou os dois testes de cobertura, ordem dos desenhos e correspondência das poses com todos os ticks de combate. Os laudos registram também as verificações anteriores específicas de cada integração.

### Ressalvas mantidas no escopo concluído

- O renderer continua espelhando letras/símbolos e usa chaves discretas, com pequenas variações de detalhes entre fontes. Não há folhas independentes para a orientação esquerda.
- C preserva a geometria baseline: a ponta elevada do tênis da rasteira e a borda superior do livro excedem parcialmente suas caixas; os centros de contato ficam dentro e foram examinados no Lab. A sola da rasteira Python fica próxima da borda superior. Nenhuma caixa foi deslocada para ajustar o desenho.
- O cone da guarda baixa de Duke fica aproximadamente 10 px acima do corpo físico agachado; o extremo do salto da bota aérea de C++ fica a cerca de 2 px da borda inferior. Os refinamentos e essas margens estão documentados nos respectivos laudos.
- A aprovação limita-se às fontes, sequências e contextos inspecionados; não promete perfeição de cada pixel, todas as combinações de partida ou alteração de balanceamento.
- Go mantém cerca de 7 px de profundidade entre os apoios de mão/pata no sweep e a ponta de uma garra aproximadamente 1 px junto da borda frontal do air_kick. Os contatos úteis permanecem nas caixas; não houve ajuste de combate para acomodar o desenho.

## Histórico de validação do primeiro lote

> O registro abaixo preserva as exportações, ferramentas, descobertas e limitações anteriores ao refinamento de 07/09/2026. Suas contagens e ressalvas não são o estado atual. Os seis laudos atuais prevalecem; o Go antigo foi rejeitado e a nova identidade foi concluída com evidência própria.

Produção raster real concluída para as 120 ações do lutador, com um data stream novo de Python e cinco projéteis originais reaproveitados. Naquela entrega, os arquivos eram candidatos com ressalvas artísticas; os testes estruturais não os promoviam automaticamente a arte final. A [galeria](../assets/candidates/README.md) reúne imagens, GIFs, vídeos e laudos por personagem.

### Sprite Studio nativo: piloto Rust e escala

O editor desktop foi aberto neste ambiente com `GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1` e `WAYLAND_DISPLAY` removido do processo. A tentativa inicial por Wayland encontrou falha de EGL. O manifesto real `assets/candidates/rust/rust-fighter.sprite.json` foi carregado pela interface do Studio; os quatro frames de `idle` foram inspecionados pelo transporte de frames a zoom `1.00x`, sem salvar alterações no manifesto ou nas métricas físicas. O clip observado tem quatro poses de `180 ms` (`720 ms` no total); suas medidas visíveis variam entre `159–162 px` de largura e `266–275 px` de altura. Essa variação ainda faz parte da revisão artística do candidato.

A inspeção revelou uma inconsistência do editor: validador, texto do painel e guia desenhada ainda usavam a escala anterior de `110–150 × 185–210 px`. Isso marcava incorretamente o baseline e o candidato Rust como fora da faixa. A correção pequena em `src/validation.ts` e `src/App.tsx` do Studio compartilha os limites atuais de [docs/17](17-visual-scale-and-stage-metrics.md), `147–200 × 247–280 px`, e identifica a faixa como referência de **idle**. Nenhum PNG, pivô, escala de personagem ou dado de combate foi modificado para eliminar o aviso.

`pnpm build` passou. O executável nativo foi recompilado com `cargo build --manifest-path tools/sprite-studio/src-tauri/Cargo.toml`, reaberto e inspecionado novamente: o candidato `idle_00`, medido em `162 × 272 px`, aparece como dentro da faixa atual. Capturas locais em `target/sprite-studio-candidate-updated-scale.png` e `target/sprite-studio-updated-idle-2.png` / `3.png` / `4.png` registram o editor real e a navegação. As capturas anteriores `target/sprite-studio-initial-review.png` e `target/sprite-studio-candidate-review.png` preservam o achado antes da correção.

Esta verificação cobre carregamento, desenho, medidas e navegação quadro a quadro no Studio nativo. Não comprova reprodução contínua, espelhamento, contraste com arenas nem sincronização de golpes no Studio; essas verificações pertencem ao Viewer/Combat Lab/partidas e devem ser registradas separadamente. Não foi usado o botão de validação Rust do editor, pois os checks do jogo foram executados pelo fluxo de terminal.

### Exportação, transparência e integração

Foram exportados seis atlas PNG+JSON, 120 GIFs e 120 painéis de quadros. A contagem de desenhos selecionados é Rust 71, Duke 61, Go 59, C 61, Python 60 e C++ 61. Cada ação preserva fontes, prompts, recortes explícitos, escala uniforme, pivôs e durações. Variantes rejeitadas ficam na produção e não entram automaticamente no atlas. Notas de revisão escritas durante a geração são registros históricos; os laudos finais ligados na galeria consolidam o estado atual.

O primeiro idle Rust exigiu remoção local do xadrez pintado, autorizada pelo usuário. As folhas seguintes usam fundo magenta, removido com descontaminação de bordas antes da exportação RGBA. O runtime continua usando alpha normal, sem shader de chroma key. Pequenos resíduos isolados encontrados nas folhas foram removidos ou excluídos pelos recortes explícitos, sem sobrescrever as fontes. Todos os candidatos foram examinados sobre fundo claro/escuro e em capturas do renderer. Isso não equivale a garantir perfeição em todo contorno subpixel.

No primeiro lote, o carregamento era opt-in por `BORROW_FIGHTERS_SPRITE_CANDIDATES=1`, com fallback para os originais se o candidato estiver ausente, inválido ou sem algum dos 20 clips exigidos. A seleção agora percorre hit, guarda, agachamento, salto, entrada, vitória e derrota com relógios visuais apropriados. O relógio do resultado avança mesmo com o combate encerrado; as poses de resultado são apoiadas no chão visualmente, inclusive quando o corpo físico congelou no ar. O espelhamento foi corrigido para o contrato de retângulo negativo do Raylib, preservando a posição da célula no atlas.

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

Ao concluir o primeiro lote, os candidatos ainda não tinham aprovação artística final. As ressalvas registradas naquele momento incluíam:

- Caminhadas ainda curtas ou com contrabalanço limitado, especialmente o ciclo de duas poses do Go; há pequenas variações de rosto, roupa e acessórios entre gerações.
- Mãos/pontas de botas excedem parcialmente algumas caixas preservadas. Rust tem sola da varredura acima da caixa e mãos do agarrão além do alcance; Go e Python têm contatos próximos das bordas; o salto da bota de C++ fica cerca de 15 px abaixo da caixa aérea. Os laudos mostram os casos concretos.
- Duke conserva o cone/vapor acima do corpo físico agachado. C foi redesenhado para poses realmente baixas, mas partes da cabeça/livro ainda excedem a caixa. A transição entre crouch e a defesa muito compacta de C++ merece acabamento.
- O espelhamento também inverte letras e símbolos de livros/bolsa. Não há folhas independentes para cada orientação. O comportamento anterior de blockstun poder sair de crouch permanece.
- O Studio foi verificado quadro a quadro apenas no piloto Rust; movimento e duas orientações dos demais foram verificados por previews e capturas World/Lab. Nenhuma combinação ausente é implicitamente aprovada pelos testes automatizados.

Fontes originais e placeholders foram preservados.

## Fechamento da revisão atual

A validação final do novo Go mantém 57 quadros idênticos à revisão anterior e substitui apenas os três de jump. O Studio recarregou o manifesto final e recapturou esses três; as capturas anteriores dos outros 19 clips permanecem válidas por comparação de pixels e metadados. O World completo e a partida normal foram repetidos com o atlas final. [Relatório geral dos assets e testes](../assets/production/runtime-verification/final-assets-report.json).
