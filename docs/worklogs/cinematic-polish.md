# Revisão visual do prólogo e da rua

Branch: `main`. Base commitada: `6740d6d`. Esta rodada preserva a revisão
anterior ainda no workspace, descrita em `modular-running-world.md`.

## Pedido e plano

Prioridade: Duke e Old C com acabamento e perspectiva coerentes com Ada,
Python e C++. O usuário autorizou quadros completos quando a montagem por
peças prejudicar a qualidade. Manter dois enquadramentos realmente distintos
por personagem e câmera/legendas editáveis. Conferir capturas do runtime.

Corrigir volume das pernas na corrida e mostrar Rust de costas durante a
travessia para conversar. Dar tempo à pipa, aos moradores e ao trânsito antes
da ameaça. Fechar junções das casas e reduzir a sensação de fundo deslizando.

## Divisão em andamento

- `chapter_combat`: biografias, arte completa e captura nativa.
- `rust_gait`: proporções e travessia, assets e captura nativa.
- `ep_arrival`: ritmo/câmera da pipa, sem alterar queda veloz da EP.
- Root: fundo/junções da rua, revisão visual cruzada, documentação e checks.

## Retomada

Ler este diário, `git status --short` e mensagens dos agentes antes de editar.
Não descartar a revisão anterior. Evidências desta rodada ficam em
`docs/evidence/cinematic-polish/`. Ainda não há verificações finais desta rodada.

## Primeiro checkpoint

Pilares `wall.pillar` com alpha nativo fecham as junções, incluindo a extensão
parcial da última fachada da rua. `distance_scroll=0.90` substitui o deslocamento
forte e o renderer ancora o fundo ao centro da câmera (zoom não cria deriva).
Capturas nativas de quatro posições estão em `world/`; precisam ser atualizadas
com o binário final porque o último trecho ganhou uma fachada após a inspeção.

Old C ganhou primeiro quadro completo, inspecionado junto de Ada: monitor,
mãos, teclado e objetos compartilham perspectiva e luz. Rust já tem comparação
nativa idle/run e poses de costas/frente, inspecionadas pelo root.

Build inicial passou. Teste preliminar de pacote encontrou caminho duplicado
no catálogo de travessia ainda em edição; o agente corrigiu. Uma recaptura com
harness antigo falhou no novo campo `back_walk_clip`; reconstruir o harness
com o Cargo atual, sem usar rlibs escolhidas por data (há builds de features
diferentes). Preferir artefatos do `cargo build --message-format=json`.

ADR 0030 registra quadros completos nas biografias, conforme autorização
explícita do usuário, mantendo as peças do mundo jogável independentes.

## Entrega concluída

Duke e Old C usam quatro PNGs novos, cada um com sua tomada/câmera/legenda.
O modelo `scenes.json` v2 elimina as camadas/oclusões que não eram necessárias;
o catálogo runtime carrega somente as quatro pinturas. O histórico modular
está preservado no guia de assets. Root inspecionou os quatro quadros com a
UI real; o vídeo de 24 segundos foi concluído e decodificado.

Pipa: 18 segundos, corte coberto para Rust e vida cotidiana preservada.
Corrida: coxas/canelas com largura transversal ajustável, botas proporcionais;
travessia de costas na ida e de frente na volta. Rua: pilares/junções,
fachada parcial no fim e parallax 0,90 ancorado ao centro da câmera.
As capturas do mundo foram atualizadas e a extremidade direita conferida.

Verificações finais:

- `cargo fmt --all --check` e `git diff --check`: passaram.
- `cargo clippy --all-targets --all-features -- -D warnings`: passou.
- `cargo test --all-targets --all-features`: 529 passaram, zero falhas,
  dois testes preexistentes de dispositivo de áudio ignorados.
- Pacote: 20 testes; novas pinturas, 16 poses de travessia, pilar e trilho da
  pipa incluídos no contrato de assets.
- Fronteiras de domínio e seus 56 testes: passaram.
- Pipa: 6 checks de prévia e 9 funcionais; EP: 11 checks nativos no binário final.
- Vídeos completos: biografias 24 s/720 frames, travessias 18,733 s/562 frames,
  pipa 18,6 s/558 frames; todos 1280×720.

[Evidências e relatório](../evidence/cinematic-polish/README.md).
Logs temporários: `/tmp/cinematic-polish-{tests-final,clippy,package}.log`;
binário conferido: `/tmp/borrow-cinematic-polish-verified`. Para rever no jogo:
`cargo run -- --start opening`, `--start encounter` ou `--start chapter`.

Nenhum commit adicional nesta rodada; a revisão anterior e esta permanecem
no workspace. O commit anterior às alterações continua sendo `6740d6d`.
