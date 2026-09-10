# Retomada — prólogo de Ada e primeira manhã de Rust

## Estado atual

- Fechamento autorizado de `v0.1.0-prototype.4`: todos os arquivos, incluindo
  as edições narrativas do usuário, serão commitados e integrados à `main`.
  O estado da publicação está no [diário da release](release-prototype-4.md).
- Entrada conjunta oficial por pedido do usuário: `cargo run` executa
  `borrow-story`, com `fighting` e `adventure` habilitadas por padrão.
  `cargo run -- --menu` entra no menu e `cargo run -- --start opening` revê
  a apresentação. Domínios isolados continuam disponíveis com feature/binário
  explícitos e `--no-default-features`. ADR 0023 registra a atualização.
  Verificado: Fmt, Clippy, 432 testes com defaults, 395 luta, 38 aventura/core,
  3 core e 56 fixtures de fronteira. Inicialização pelo Cargo sem `--bin` nem
  `--features` confirmou `borrow-story` e `AdaPrologue` no renderer.
- Revisão de navegação concluída após reinício em 2026-09-10: prompt original
  recuperado do histórico local, com confirmação no final, avanço por trechos e
  pular tudo direto ao menu. Alterações anteriores preservadas; cópia do diff
  inicial em `/tmp/borrow-navigation-recovered.patch` nesta sessão.
  Integração completada com oito textos `navigation.*`, validação do catálogo e
  correção do limite da revisão automática para permitir confirmação/fade.
  Todos os 68 valores anteriores do catálogo do usuário continuam intactos.
  Matriz aprovada: 432 ambos, 395 luta, 38 aventura/core e 3 core; Fmt, Clippy,
  fronteiras e 54 fixtures. Aventura isolada: seis checks X11 aprovados para
  skip total, pausa, replay, retry, skip do encontro e saída.
  Entrada conjunta: 81 checks X11 aprovados; confirmação por tecla/clique,
  tecla segurada, skip direto e revisão automática verificados. Capturas,
  vídeo e resultados em [evidências da navegação](../evidence/story-terminal-menu/navigation/README.md).
  Sem etapa técnica pendente. Controle físico e avaliação auditiva não realizados.
  Trabalho local sobre `f90304b`; alterações não commitadas preservadas.
- Rodada 3 concluída: [junção com menu de terminal](../29-story-terminal-menu.md)
  e [ADR 0023](../adr/0023-story-to-terminal-menu.md). Go saiu da apresentação;
  conclusão abre o menu na mesma janela; Modo História inerte, cursor bloco,
  moldura, título coerente e revelação binária implementados.
- Para executar tudo: `cargo run`.
  `--start opening` revê apresentação → menu; `--menu` abre diretamente o menu.
- Checkpoints: `67d6441` (escopo), `5ea92d4` (junção/menu), `0574347` (antecipar
  gráficos para retirar espera escura no final). Evidências e fechamento no
  commit seguinte. Lore / Roster não foi alterado: revisão futura no TODO-016.
- Verificação da rodada 3: 421 testes ambos, 395 luta, 27 aventura/core e 3 core;
  Fmt, Clippy, 54 fixtures de fronteira e 14 checks nativos aprovados. Mesmo PID
  e janela; transição em aproximadamente 1,23 s com fades; saídas e janela antes
  oculta verificadas. [Capturas e vídeo](../evidence/story-terminal-menu/README.md).
- Edições do usuário em `assets/adventure/texts/pt-BR.json` continuam preservadas
  fora dos commits. Não reformatar nem restaurar o arquivo.
- Rodada 2 concluída: [textos editáveis, encaixe na cama e apresentação](../28-adventure-texts-and-opening.md).
- Branch: `feature/rust-adventure-prologue`.
- Base inicial: `ca63b25` (`v0.1.0-prototype.3`); base da continuação: `0046312`.
- Checkpoints da continuação: `7ca6cc8` (escopo), `6f911ec` (implementação/arte),
  `1352fe2` (prova de F5 e controles). O commit seguinte fecha docs/evidências.
- Texto externo: `assets/adventure/texts/pt-BR.json`; F5 recarrega sem rebuild.
  JSON inválido mantém a versão anterior. `--texts PATH` usa catálogo alternativo.
- Manhã corrigida por apoios anatômicos. Apresentação de 48 s entra após o
  combate/pesar; `--start opening` abre diretamente e `T/X` repete na conclusão.
- Lore: C++ tem passado de profissional do sexo e torna-se heroína após acessar
  o Linker; Python é professora universitária e ensina humanos sobre EPs.
- Verificação da rodada 2: 417 testes ambos, 393 luta, 27 aventura/core e 3 core;
  Fmt/Clippy, fronteiras e 26 checks nativos aprovados. Isolamento de 30 assets,
  cinco músicas/quatro efeitos, vídeos e recarga no mesmo binário verificados.
- Sem etapa técnica pendente. Avaliação humana continua no TODO-013. Contagens
  e vídeos da rodada 2 abaixo são históricos; a rodada 3 está nos links acima.
- Durante o fechamento apareceram edições externas em `assets/adventure/texts/pt-BR.json`.
  O JSON e suas chaves foram conferidos; alterações preservadas no working tree,
  fora dos commits do agente. Não restaurar esse arquivo a partir das capturas:
  os vídeos representam o texto de `6f911ec`, anterior a essas edições.
- [Vídeos, relatórios e reprodução da rodada 2](../evidence/adventure-opening/README.md).
  A [rodada inicial](../evidence/adventure-prologue/README.md) permanece histórica.

## Retomar após queda do WSL

1. Ler este arquivo, [junção atual](../29-story-terminal-menu.md),
   [ADR 0023](../adr/0023-story-to-terminal-menu.md), [continuação](../28-adventure-texts-and-opening.md),
   [ADR 0021](../adr/0021-isolated-adventure-experiment.md) e
   [ADR 0022](../adr/0022-adventure-external-copy-and-opening.md).
2. Rodar `git status --short`, `git branch --show-current`, `git log -8 --oneline`.
   Preservar alterações não commitadas: podem ser trabalho em progresso.
3. Conferir código e assets existentes antes de regenerar/reimplementar.
4. Retomar a primeira etapa pendente. Registrar comandos, resultados e limites;
   fazer commits pequenos por etapa coerente.
5. Continuar o goal ativo quando existir; o diário também permite reconstruir
   a tarefa em outra sessão sem depender da memória do chat.

## Pedido autoritativo

Revisão de navegação recuperada do histórico local após o reinício:

> quando termina toda a história prologue, não vai para o menu? teria que adicionar
> um 'aprte qualquer tecla para continuar' e cair no menu principal... E precisa
> também ter opção de pular tudo... não dá pro jogador ficar vendo tudo isso toda
> hora... precisa ser possível dar skip tanto em cada parte, pra ir pulando mais
> rapidinho e ver/rever a ceninha que ele quiser ver novamente, ou pular realmente
> tudo e ir direto pro menu, manja? E se ele seguir normalmente, aí no final ter
> o aperte qualquer tecla...

Pedido inicial do experimento:

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
- [x] Verificar derrota/retry, pausa, pular cenas e controles.
- [x] Passar fmt, Clippy, matriz de testes e checks de documentação.
- [x] Registrar capturas/vídeo e smoke tests de isolamento.
- [x] Fechar docs, diário e commits de entrega.

## Checkpoints

- 2026-09-10, entrada conjunta promovida a padrão: Cargo, checker de fronteiras,
  exemplos de CLI e ADRs sincronizados. Comandos de luta/ferramentas agora
  explicitam `--bin borrow-fighters`; `cargo run` segue história → menu.
  Matriz/defaults e isolamento aprovados, com logs em `/tmp/borrow-default-entry-checks`.
  Smoke de janela: `cargo run -- --hidden --mute --frames 3 --capture DIR`
  confirmou `AdaPrologue`, três frames e saída por limite em
  `/tmp/borrow-default-run-smoke-6t60rnrm`. Help da composição passou; help da
  luta manteve seu código 2 já existente, com exemplos novos conferidos.
  Links locais e `git diff --check` aprovados. Alterações anteriores e textos
  do usuário preservados; nenhuma publicação ou mudança de pacote de release.

- 2026-09-10, navegação concluída: 81 checks X11 da composição e seis do modo
  isolado aprovados. Prompt final, controles da pausa e destinos de menu
  conferidos visualmente. Confirmação nova → cabeçalho do menu levou cerca de
  1,13 s no host (amostragem de aproximadamente 100 ms). A captura conjunta
  preservou o mesmo binário e catálogo; nenhuma alteração Rust depois da matriz.
  Resultados e capturas arquivados em `docs/evidence/story-terminal-menu/navigation/`.
  Comando para jogar: `cargo run --features adventure --bin borrow-story`;
  `-- --start opening` começa na apresentação, `-- --menu` abre direto o menu.

- 2026-09-10, retomada após reinício: integração do trabalho recuperado concluída.
  A ausência dos textos `navigation.*` e o encerramento precoce de `--review`
  hospedado foram corrigidos. Testes e logs em `/tmp/borrow-navigation-validation`;
  checks isolados em `/tmp/borrow-navigation-standalone-5drvvdmp`.
  `python3` do host é 3.8; o checker exige 3.11+, então fronteiras e suas
  fixtures foram executados com `python3.13`, ambos aprovados. Nenhum texto
  narrativo existente foi alterado. Captura conjunta concluída no checkpoint acima.

- 2026-09-10, navegação iniciada: final deixa de avançar automaticamente;
  Enter/RB passa por trechos, Backspace/View abre menu diretamente. O modo
  isolado continua independente. Textos permanecem externos e skip de combate
  não altera HP/outcome. Registrar checks e commit após integrar as frentes.

- 2026-09-10, rodada 3 verificada: `0574347` antecipa texturas da luta, sem
  compartilhar recursos com a aventura. Captura final teve 14 checks aprovados,
  duas sessões de áudio limpas, mesmo PID/janela e 1,226 s do Complete até o
  cabeçalho do menu. Enter/Espaço inertes, Versus/Training/Lore/Options/guia e
  retornos revisados. Close, frame cap e hidden → visible passaram. Todos os
  testes da matriz foram repetidos após o ajuste; Fmt/Clippy/fronteiras limpos.
  Catálogo do usuário, livro de lore e manhã sem alterações. Evidências na
  pasta `docs/evidence/story-terminal-menu/`; TODO-017 concluído, TODO-016 aberto.

- 2026-09-10, revisão da transição: `5ea92d4` registra a implementação. A primeira
  captura revelou 5–7 s de carregamento de atlas depois do fade final; antecipar
  a preparação das texturas da luta para antes do prólogo, num objeto opaco de
  `app`, sem acesso pela aventura. Dispositivos de áudio continuam sequenciais.
  Repetir captura e checks após a correção; tentativa inicial não vale como
  aprovação visual. Catálogo do usuário continua intocado.

- 2026-09-10, rodada 3 integrada: `67d6441` fixa escopo. `borrow-story` recebe
  conclusão explícita da aventura e abre o menu na mesma janela; sair/limitar
  frames não dispara outro modo. Recursos de áudio/GPU encerram entre sessões.
  Menu de terminal e corte do Go implementados e capturados separadamente.
  Matriz aprovada: 421 ambos, 395 luta, 27 aventura/core, 3 core; 54 fixtures de
  fronteira, Fmt e Clippy aprovados. Revisão encontrou menu invisível depois de
  `--hidden`/`--review`; corrigido mostrando a janela no handoff. Falta conferir
  essa correção e a sequência pela janela, arquivar evidências e fechar docs.

- 2026-09-10, rodada 3 iniciada: goal criado na mesma branch. ADR 0023 registra
  composição externa `borrow-story` e APIs que compartilham a janela sem
  compartilhar regras. Frentes independentes: menu principal, corte do Go e
  checker de fronteiras. Root cuida transição, backlog, docs e integração.

- 2026-09-10, ajuste pontual de áudio: usuário relatou ruído na manhã. A faixa
  antiga continha vento aleatório e pássaros agudos; `morning.wav` foi substituída
  por notas suaves sem essas camadas, com gerador sincronizado. PCM16 mono/22050,
  12 s, pico -11,06 dBFS, zero clipping e junção do loop em zero. Binário existente
  abriu a manhã com áudio e encerrou com código 0. Hashes confirmam preservação
  dos outros oito WAVs e das edições do usuário no JSON. BUG-006 concluído.
  Não houve alteração Rust nem necessidade de recompilar; reabrir carrega o WAV.

- 2026-09-10, rodada 2 verificada: `6f911ec` registra implementação, arte,
  catálogo e trilha. 26 checks nativos passaram, incluindo F5 válido/inválido,
  prova visual de texto preservado, hashes do mesmo binário, combate e
  pausa/skip/replay da apresentação. Matriz completa: 417 ambos, 393 luta,
  27 aventura/core e 3 core; Fmt, Clippy, 34 fixtures de fronteira e 28 do mixer
  aprovados. Vídeo completo de 122,467 s terminou em Complete; excertos de
  apresentação (48 s) e manhã (14 s) preparados. Smoke abriu somente os 30 assets
  da aventura, com catálogo, cinco músicas e quatro efeitos. Evidências em
  `docs/evidence/adventure-opening/`; faltam fechamento editorial e links.

- 2026-09-10, rodada 2: `7ca6cc8` registra escopo e ADR 0022. Catálogo
  `assets/adventure/texts/pt-BR.json` externo com F5 e `--texts`; reload inválido
  preserva o catálogo. Novo `morning.rs` fixa apoios anatômicos em nove capturas
  revisadas (`/tmp/borrow-morning-integrated-chxve2tm`). Stage `Opening` entra
  após pesar, com 48 s de jornais, C++/Python, elenco e logo. PNGs próprios,
  cópias visuais independentes do elenco e trilha original integrados.
  27 testes aventura/core, Clippy completo e checker de fronteiras passaram.
  Revisão completa gravando em `/tmp/borrow-adventure-opening-review-01`;
  harness nativo em atualização para provar F5 sem rebuild e skip/replay da
  apresentação. Pendentes: revisão final, matriz restante, evidências/docs.

- 2026-09-10, encerramento: `bda28d7` preserva ferramentas de revisão e seus
  testes. Evidências selecionadas copiadas para `docs/evidence/adventure-prologue/`,
  incluindo vídeos, capturas, telemetria comprimida e relatórios. Smoke de assets
  isolados aprovado: aventura pelo binário, luta pelo example `capture_ui_review`,
  ambos compilados com feature exclusiva e sem acessos cruzados observados.
  Aventura com áudio inicializou quatro streams/quatro efeitos e encerrou limpa.
  Links Markdown (103 arquivos) e 19 YAMLs aprovados; `git diff --check` limpo.
  README/entrega/backlog/CHANGELOG atualizados. TODO-012 concluído; TODO-013
  guarda avaliação humana de animação, controle, áudio, emoção e continuidade.
  Sem push, merge ou publicação de release nesta rodada.
  `82bd16a` registra a entrega e as evidências; uma revisão final normaliza
  espaços finais dos logs capturados, preservando seu conteúdo.

- 2026-09-10: `3985a4b` registra as ações próprias de Rust e a correção do
  gravador. Revisão completa válida em `/tmp/borrow-adventure-review-02`:
  74,467 s, Ada → manhã → encontro → pesar → Complete. Áudio do vídeo
  reconstruído da telemetria com os WAVs originais, sem gravação do dispositivo.
  Rodada nativa E (`/tmp/borrow-adventure-native-20260910-e`) passou 14 checks
  via teclado sintético dirigido à janela: movimento, pulo, pausa, derrota/retry,
  J/K, defesa, vitória e pesar. MP4 válido 1280×720/30 fps, 40,867 s.
  Matriz final em `/tmp/borrow-adventure-checks-final`: Fmt/Clippy, 412 testes
  com ambos, 22 aventura/core, 34 fixtures de isolamento e 24 testes do mixer.
  Luta (393) e core (3) permanecem aprovados na matriz anterior, sem mudanças
  nesses domínios desde a execução. Pendente: copiar evidências ao repo,
  registrar smoke tests de assets e fechar docs/links.

- 2026-09-10: `4bfc5c6` registra jogo isolado; `4a133ac` adiciona checker de
  fronteiras (34 testes). A matriz inicial passou: 411 testes com ambos os modos,
  393 só luta, 21 só aventura/core e 3 só core. Rust recebeu 16 ações próprias
  consistentes com a manhã; o catálogo copiado da luta foi removido.
  Testes nativos de teclado, derrota/retry e pesar passaram, mas revelaram erro
  no tamanho do buffer do encoder. Corrigido com slice RGBA explícito e regressão;
  vídeos inválidos anteriores não são evidência de gravação aprovada.
  Revisão atual válida em produção: `/tmp/borrow-adventure-review-02`.

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

## Verificação da rodada inicial

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --all-targets --no-default-features --features fighting
cargo test --all-targets --no-default-features --features adventure
cargo test --lib --no-default-features
git diff --check
```

Todos os comandos passaram. Resultados: ambos 412, luta 393, aventura/core 22,
core 3. Também passaram checker de fronteiras (34 fixtures), mixer (24 testes),
14 verificações pela janela e smoke tests de assets/dispositivo de áudio.
Detalhes e limites estão nas evidências versionadas; os diretórios `/tmp`
nos checkpoints anteriores são intermediários e não são necessários à retomada.

## Continuidade

A implementação pedida não tem etapa técnica pendente. Para jogar, usar o
comando da aventura no README; para revisar, abrir o vídeo versionado.
A continuação (rodada 2) também foi implementada e verificada; os dados atuais
estão no topo deste diário. Uma próxima sessão deve partir do feedback do TODO-013. Manter as fronteiras
da ADR 0021 e preservar os itens do backlog da luta ao decidir seguir, descartar
ou desenvolver a aventura em paralelo.
