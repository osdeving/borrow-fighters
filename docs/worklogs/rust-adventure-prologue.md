# Retomada — prólogo de Ada e primeira manhã de Rust

## Estado atual

- Goal ativo: duas animações e primeiro combate de aventura, lore corrigida,
  isolamento de implementação/testes/assets e validação gráfica.
- Branch: `feature/rust-adventure-prologue`.
- Base: `ca63b25` (`v0.1.0-prototype.3`).
- Primeiro checkpoint: `ac49bae`, backlog e proposta anterior à correção de lore.
- Etapa: contrato registrado; implementação e produção visual em andamento.
- Entrega ainda não validada. Compilar não basta para concluir o goal.

## Retomar após queda do WSL

1. Ler este arquivo, [episódio](../27-rust-story-adventure.md) e
   [ADR 0021](../adr/0021-isolated-adventure-experiment.md).
2. Rodar `git status --short`, `git branch --show-current`, `git log -8 --oneline`.
   Preservar alterações não commitadas: podem ser trabalho em progresso.
3. Conferir código e assets existentes antes de regenerar/reimplementar.
4. Retomar a primeira etapa pendente. Registrar comandos, resultados e limites;
   fazer commits pequenos por etapa coerente.
5. Continuar o goal ativo quando existir; o diário também permite reconstruir
   a tarefa em outra sessão sem depender da memória do chat.

## Pedido autoritativo

Ada começa humana num mundo normal, aprende Linker, recebe mensagem misteriosa
e desperta Assembly, a primeira EP. Ada torna-se híbrida humana/EP; o prólogo
deixa suas consequências em aberto. Muito tempo depois, Rust, a EP mais recente,
acorda numa manhã cotidiana. Uma errática tenta matá-lo; o jogador se defende.
Após vencer, Rust observa a criatura e balança a cabeça com pesar, mostrando
nobreza e desejo de convivência.

EPs são pessoas criadas, não exatamente humanas, com diversidade emocional/moral.
Mau uso de Linker por frontenzos cria involuntariamente condições cósmicas para
erráticas surgirem. Referências a outras obras são analogias de tom. Vínculo,
plataformas e capítulo em Sirius da proposta anterior ficam fora deste corte.

## Frentes

- Domínio: `src/adventure/story.rs`, `src/adventure/combat.rs`.
- Borda: `src/adventure/engine/`, `src/adventure/app.rs`, `src/bin/borrow-adventure.rs`.
- Isolamento: `Cargo.toml`, `src/lib.rs`, verificação de fronteiras.
- Arte: `assets/adventure/` e `ART-PROVENANCE.md` com prompts/fontes.
- Lore: docs 00/01/02/07/12/27, `assets/lore/story.json` e backlog 03.

## Etapas

- [x] Criar branch e preservar documentação anterior.
- [x] Registrar ADR e instruções de retomada.
- [x] Sincronizar lore e roteiro.
- [x] Implementar features/binários e domínio puro.
- [x] Produzir/revisar arte de Ada, manhã, ambiente e errática.
- [x] Integrar animação, input, áudio, combate e desfecho na janela real.
- [ ] Verificar derrota/retry, pausa, pular cenas e controles.
- [ ] Passar fmt, Clippy, matriz de testes e checks de documentação.
- [ ] Registrar capturas/vídeo e smoke tests de isolamento.
- [ ] Fechar docs, diário e commits de entrega.

## Checkpoints

- 2026-09-10: `b7d6be0` fixa lore. Primeiro loop de aventura compila isolado;
  20 testes adventure/core e Clippy completo passaram. Revisão gráfica inicial
  em `/tmp/borrow-adventure-review-01` percorreu cenas e combate real até Complete.
  Arte com alpha verdadeiro integrada via retângulos revisados. Encontrados e
  corrigidos pausa que reutilizava confirmação e relógio de derrota congelado.
  Revisão pendente: uniformizar Rust entre manhã e combate, contraste do objetivo,
  validar controles enviados à janela, captura com áudio e matriz final. Arte
  adicional `rust-actions.png` em produção para manter a identidade da manhã.

- 2026-09-10: `523e212` registra isolamento e diário. Lore revisada nos docs e no
  livro JSON; proposta antiga substituída pelo episódio pedido. Domínio puro tem
  11 testes aprovados em harness isolado. Arte de Ada e ambientes disponível;
  manhã de Rust está em revisão de transparência. Matriz Cargo ainda pendente.

- 2026-09-10: branch criada; `ac49bae` preserva trabalho anterior. ADR e diário
  preparados antes da implementação. Nenhuma validação do novo jogo concluída.

## Verificação planejada

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --all-targets --no-default-features --features fighting
cargo test --all-targets --no-default-features --features adventure
cargo test --lib --no-default-features
git diff --check
```

Ainda não executada nesta rodada. Registrar resultados aqui conforme terminarem.
