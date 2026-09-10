# 20 — MVP de combate e showcase contextual

## Escopo autorizado

Goal iniciado em 8 de setembro de 2026. O usuário confirmou **Rust, Duke/Java, C, Python e C++** como elenco desta rodada. Go permanece fora de novos especiais e arte. As correções das regras compartilhadas também valem para lutas iniciadas por CLI com ele. Esta solicitação amplia expressamente o corte inicial do Prototype 0.1 para especiais, revisão visual e balanceamento de MVP.

## Entregas verificadas

| Frente | Entrega verificável | Estado |
|---|---|---|
| Debug | Options desliga todas as caixas retangulares de corpo/guarda/hitbox; impactos e tintas continuam legíveis | Concluído |
| Showcase | Dois lutadores no `World` real; entradas do oponente apropriadas para cada golpe; dano/defesa confirmados pela colisão | Concluído |
| Controles de treino | Autoplay, seleção, replay, pausa, avanço de quadro, repetição e inversão dos lados; CLI direta | Concluído |
| Defesa e reações | Defesa em pé/abaixada coerente durante blockstun; agarrões não atingem alvos no ar; reações adequadas a rasteira/agarrão | Concluído |
| Especiais | Uma nova ação de assinatura por personagem desta rodada, com risco, resposta defensiva e animação própria | Concluído |
| Arte | Auditar os dez golpes anteriores, guarda, dano e transições; gerar e integrar poses novas onde faltarem | Concluído |
| Balanceamento | Corrigir defeitos comprovados e registrar alcance, timing, dano e contra-jogo dos especiais; testes e playtests comparáveis | Concluído |
| Validação | Fmt, Clippy, testes Rust, links, review de sprites e verificação gráfica de contatos, ambos os lados e debug desligado | Concluído |

## Cenários do showcase

- Jab, soco forte e chute: adversário se aproxima até a faixa de contato.
- Rasteira: adversário mantém guarda alta, expondo as pernas.
- Overhead: adversário defende abaixado.
- Anti-air: adversário salta de verdade em direção ao atacante.
- Ataques aéreos: atacante salta e atinge um adversário no chão.
- Agarrão: adversário próximo em guarda; precisa haver contato válido no chão.
- Projétil: distância apropriada para acompanhar emissão, percurso e impacto.
- Especiais: adversário se posiciona e age conforme a assinatura; mesmas regras do jogo normal.
- Defesa: exemplos de guarda em pé, guarda baixa e defesa de projétil mostram bloqueio real e recuperação.

Cada apresentação deve explicar a situação e registrar o resultado. Uma falha de contato não pode ser substituída por dano artificial. O reset reposiciona os atores entre demonstrações; durante a ação, valem movimento, colisão, stun e dano do jogo.

## Especiais implementados

| Personagem | Ação | Intenção visual/mecânica |
|---|---|---|
| Rust | Borrow Break | Estocada precisa com as duas palmas; pressão média bloqueável |
| Duke/Java | GC Slam | Golpe descendente com as duas mãos; overhead pesado e anunciável |
| C | Pointer Lance | Estocada longa e linear, com recuperação punível |
| Python | Serpent Slide | Chute deslizando baixo; deslocamento curto e risco na defesa |
| C++ | Template Arc | Golpe diagonal ascendente; resposta a aproximação aérea |

Dados finais, alcance, risco na defesa e contrajogo estão na [matriz de combate](15-character-combat-matrix.md). A rodada mantém comandos simples, sem medidor, árvore de combos, cancelamentos complexos, online ou personagem adicional.

## Registro de execução

- Estado anterior preservado nos commits de cursor/menu; redução do Linker registrada em `c2f441c`.
- Auditoria inicial: retângulos de reação/guarda eram desenhados mesmo sem debug; bloqueio baixo perdia postura durante stun; agarrões precisavam validar alvo no chão; cast de projétil permitia sobreposição com novos ataques.
- Arte anterior: os cinco personagens já possuem clips de `block`, `crouch_block` e `hit`; a revisão desta rodada verifica sua correspondência com as novas situações e completa reações ausentes.

## Referências

- [Guia técnico](12-technical-combat-guide.md)
- [Matriz de combate](15-character-combat-matrix.md)
- [Pipeline de sprites](11-sprite-pipeline.md)
- [Cobertura visual anterior](19-sprite-production-coverage.md)

## Resultado integrado

Cada um dos cinco personagens tem **15 demonstrações**: onze golpes e quatro defesas. O oponente se aproxima, salta, protege alto ou baixo conforme a situação. O título mostra o golpe real do loadout; o painel confirma contato/dano/bloqueio. A troca de personagem conserva também o cenário de defesa, pausa, repetição e lado.

Foram geradas e integradas **15 animações novas, com 60 desenhos**: cinco especiais, cinco quedas/recuperações e cinco rasteiras baixas. O total dos seis atlas é 130 clips / 420 quadros, incluindo os 20 clips / 60 quadros de Go preservados. Os clips anteriores de defesa e hit foram revisados no combate; os demais desenhos permanecem preservados. Relatórios:

- [Rust](../assets/production/rust/mvp-review.md)
- [Duke/Java](../assets/production/duke/mvp-review.md)
- [C](../assets/production/c/mvp-review.md)
- [Python](../assets/production/python/mvp-review.md)
- [C++](../assets/production/cpp/mvp-review.md)

Defesa baixa conserva postura durante blockstun. Rasteiras/agarrões/slide provocam recuperação de 36 frames protegida contra novos hits; agarrões erram no ar ou durante stun e respeitam seis frames ao levantar. Chip deixa pelo menos 1 HP. A conjuração bloqueia ações sobrepostas e é interrompida por dano. Os efeitos de impacto usam a interseção real antes do pushback. O input guarda a pressão de botão até o próximo tick e a consome uma única vez, sem introduzir buffer através da recuperação.

A CPU usa as novas ações e respostas defensivas de forma falível. Sessenta lutas espelhadas em três distâncias terminaram sem timeout; média21,98 s e vitórias Rust11, Duke15, C11, Python12, C++11 (24 participações cada). O [registro comparável](evidence/mvp-balance/README.md) explica parâmetros e limites: serve para detectar regressões e orientar tuning, sem representar equilíbrio competitivo ou playtest humano.

## Verificação e uso

A revisão gráfica executou **150 cenários** (15 × 5 × 2 lados) no World/Raylib, com clips exatos, contatos e reações reais. Os testes de showcase cobrem 260 combinações, incluindo metadata baseline e fallback; outros testes verificam defesa, recuperação, interrupção, punição e efeito na altura do contato. Os [20 checks gráficos do aplicativo](evidence/mvp-runtime-controls/README.md) confirmaram navegação, teclas dos especiais, debug off/on/off e fechamento. Gamepad físico e escuta de áudio não foram ensaiados; mappings e eventos de áudio têm validação de código/testes.

```sh
cargo run -- --menu
cargo run --bin borrow-fighters -- --showcase --character rust
cargo run --bin borrow-fighters -- --showcase --character python --move signature_special --repeat
```

No showcase: Tab/Shift+Tab seleciona exemplo, Enter repete, Espaço pausa, ponto avança um frame, Home reinicia, L alterna repetição, X troca os lados, PageUp/PageDown muda personagem e Esc volta ao menu. Na luta, especial de assinatura: T no P1, barra invertida no P2, RT no gamepad. O projétil conserva G no P1, Right Ctrl/KP0 no P2 e RB no gamepad.

Validação final: **270 testes Rust e sete testes do exportador passaram**, além de `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`, testes dos exportadores de arte, links locais e `git diff --check`. O [registro estruturado](evidence/mvp-validation.json) guarda contagens e hashes dos atlas. As fontes e a evidência selecionada ficam no repositório; capturas completas podem ser refeitas com `examples/capture_showcase_review.rs`.
