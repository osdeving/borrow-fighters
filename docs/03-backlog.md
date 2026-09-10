# 03 — Backlog: to-do e bugfix

## Fonte de verdade

Este documento é a fonte de verdade para **o que vem agora**, com filas separadas
de **to-do** (investigações e melhorias) e **bugfix** (defeitos observados).

Prioridade atual: entregar e revisar o primeiro capítulo **Depois do silêncio**,
autorizado em10/09/2026 na mesma branch, preservando `81c50e6` para retorno.
[Escopo33](33-after-the-silence.md), [ADR0027](adr/0027-chapter-spatial-direction.md)
e [diário](worklogs/after-the-silence.md) acompanham execução e evidência.

Rodada anterior, em 10 de setembro de 2026: experimentar melhorias das cenas
do prólogo na branch `feature/prologue-scene-improvements`, a partir da
**release `v0.1.0-prototype.4`**. O TODO-018 começa pelo quarto tecnológico de
Rust e pela rua com ciclistas e garoto soltando pipa. A segunda rodada acrescenta
automóveis e acidente no poste ao despertar da EP. A terceira traz rua brasileira
com catálogo de peças reutilizáveis, ônibus, boteco e evacuação coletiva
sem retorno de trânsito durante a luta. [Escopo 31](31-brazilian-street-evacuation.md). [Escopo das rodadas](30-prologue-scene-improvements.md),
[ADR 0024](adr/0024-prologue-background-life.md) e
[diário](worklogs/prologue-scene-improvements.md). As demais cenas podem receber
rodadas próprias nessa branch; esta rodada não publica outra release.
O playtest humano do episódio continua pendente no TODO-013.

A prototype.4 foi publicada com aventura e luta na mesma execução, concluindo
a frente de distribuição. O episódio de aventura 2D narrativa solo
protagonizada por Rust foi implementado na branch
`feature/rust-adventure-prologue`. O [episódio autorizado](27-rust-story-adventure.md)
mostra Ada, a mensagem misteriosa e Assembly; muito tempo depois, Rust acorda,
enfrenta uma errática e reage com pesar à vitória. A lore aceita foi sincronizada
entre [worldbuilding](12-worldbuilding.md) e [livro do jogo](../assets/lore/story.json).
A [ADR 0021](adr/0021-isolated-adventure-experiment.md) registra features/binários
separados e a base compartilhada de `math`/`runtime_paths`; o
[diário](worklogs/rust-adventure-prologue.md) acompanha execução e evidências.
[Vídeo e validação da entrega](evidence/adventure-prologue/README.md).
A [continuação entregue](28-adventure-texts-and-opening.md) acrescenta catálogo
externo com F5, corrige o apoio na cama e apresenta o universo após o pesar.
[Vídeos atuais e prova da recarga](evidence/adventure-opening/README.md).
A [junção e navegação](29-story-terminal-menu.md) passa a ser padrão em
`cargo run` e nos pacotes; [notas da prototype.4](releases/v0.1.0-prototype.4.md).
A entrega foi integrada à `main` pelo [PR #21](https://github.com/osdeving/borrow-fighters/pull/21)
e publicada como [pré-release](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.4),
com cinco downloads e checksums. [Registro da release](worklogs/release-prototype-4.md).
As pendências do Prototype 3 permanecem registradas abaixo. Estar no backlog,
mesmo com prioridade alta, **não significa execução ativa**.

Rodada entregue: [apresentação e fluxo do playtest](26-playtest-visual-completion.md),
com reações de todo o elenco, seleção Linker, pausa/revanche, energia,
transições e voz provisória de Old C. Implementação e revisão funcional em
9 de setembro de 2026, integrada à `main` pelo
[PR #19](https://github.com/osdeving/borrow-fighters/pull/19).
O corte anterior de distribuição foi `v0.1.0-prototype.3`; [notas e instruções para jogadores](releases/v0.1.0-prototype.3.md).
Playtest humano, balanceamento e acabamento pontual aguardam retomada nas filas
de to-do e bugfix; deixaram de ser a frente imediata a pedido do usuário.

A correção do carregamento em caminhos Windows com acentos já foi publicada
na versão `v0.1.0-prototype.2` e integrada à main pelo PR #18.
[Processo](06-release-process.md), [notas](releases/v0.1.0-prototype.2.md) e
[ADR 0019](adr/0019-playtest-distribution.md).

Rodada concluída: [reações próprias Python × C++](25-python-cpp-contact-reactions.md).
64 desenhos novos e resposta sincronizada a cada contato, incluindo os oito da
rajada. 346 testes, Fmt/Clippy e 64 cenários renderizados nos dois sentidos.
[Vídeo e evidências](evidence/python-cpp-reactions/README.md).
O padrão de reações dos demais personagens integra a rodada 26 entregue. Custo de
energia recebeu seu primeiro corte funcional nessa rodada; balanceamento fino,
contra-jogo e vantagens por arena permanecem para depois.

Rodada anterior: [reações e transformações](24-reactions-and-transformations.md).
Python gigante, expansão de C++ e mutação persistente de Rust continuam
integradas. O playtest rejeitou a leitura das reações genéricas, e o piloto acima
substitui essa apresentação para Python/C++. A validação anterior está preservada
como [registro histórico](evidence/reactions-transformations/README.md).

Rodada anterior concluída: [supers autorais e identidade sonora](23-authored-super-sequences.md).
Garbage Collector, Ownership Eclipse, General Protection Fault e Footgun
integrados, com 321 testes Rust e 41 verificações de controles aprovados.
[Evidência visual e sonora](evidence/authored-supers/README.md).
Pendências preservadas: balancear custo/raridade,
janela de captura e contrajogo após o playtest destas animações.

Rodada anterior concluída: [polimento da apresentação, Brasil cotidiano e seis
especiais cinematográficos](22-presentation-and-brazilian-stage-life.md), solicitado
em 8 de setembro de 2026. Implementação integrada; 300 testes Rust, formatação,
Clippy estrito e links/YAML aprovados. [Evidência gráfica e controles](evidence/presentation-polish/README.md).
O fechamento posterior do fluxo incluiu pausa na rodada 26. Playtest humano,
resolução e remapeamento de controles permanecem nas filas abaixo.

Roadmaps especializados continuam existindo, mas devem apontar para este backlog quando uma frente virar trabalho ativo:

- combate e balanceamento: [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md);
- sprites, atlas e ferramenta de inspecao: [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md);
- arte e mood: [`docs/07-art-direction.md`](07-art-direction.md);
- audio: [`docs/14-audio-pipeline.md`](14-audio-pipeline.md);
- processo, PRs e GitHub: [`docs/05-governance.md`](05-governance.md).

Regra operacional:

1. toda frente ativa deve aparecer na tabela **Agora / Proximo / Depois** abaixo;
2. toda tarefa aceita deve ter issue ou PR relacionado quando sair de ideia para execucao;
3. toda mudanca que altera comandos, processo, formato de dados, roadmap ou contribuicao deve atualizar este backlog ou o roadmap especializado correspondente;
4. se uma decisao criar padrao duradouro, registrar ADR.

## Agora / Próximo / Depois

| Janela | Frente | Status | Registro | Próxima ação |
|---|---|---|---|---|
| Agora | Melhorias das cenas do prólogo: quarto e rua de Rust | TODO-018 em experimentação na branch `feature/prologue-scene-improvements` | [Entrega 30](30-prologue-scene-improvements.md), [ADR 0024](adr/0024-prologue-background-life.md), [diário](worklogs/prologue-scene-improvements.md) | Conferir diversidade de veículos, ponto/boteco, substituição de peças, evacuação coletiva e persistência de bicicletas/carro batido sem tráfego novo; preservar as rodadas anteriores. |
| Próximo | Avaliar episódio e definir continuidade da aventura | Playtest humano pendente no TODO-013 | [Entrega 29](29-story-terminal-menu.md), [critérios do episódio](27-rust-story-adventure.md#critérios-de-aceite) | Jogar de Ada até o menu, avaliar ritmo e navegação e decidir ajustes por evidência; ideias de Vínculo/Sirius continuam estacionadas. |
| Depois | Pendências do Prototype 3 | Backlog; sem execução ativa | [To-do](#backlog-de-to-do), [bugfix](#backlog-de-bugfix) | Retomar itens por prioridade e evidência, em entregas pequenas, quando esta frente for reaberta. |

## Backlog de to-do

Prioridade ordena o trabalho quando a respectiva frente for retomada; não
antecipa o **Agora**. TODO-001 a TODO-010 estão **abertos no backlog**;
TODO-011, TODO-012, TODO-014, TODO-015 e TODO-017 estão **concluídos**;
TODO-013 e TODO-016 estão **abertos**; TODO-018 está **em experimentação**. IDs permanecem
estáveis ao mudar prioridade ou vincular issue/PR.

| ID | Item | Prioridade | Critério de conclusão / evidência esperada |
|---|---|---|---|
| TODO-001 | Roteiro e rodada de playtest humano | Alta | Preparar roteiro no [guia de playtest](10-greybox-playtest.md); observar 4–6 pessoas em três partidas por pessoa, incluindo CPU e duelo local na rodada. Registrar compreensão da seleção, defesa, energia e diferenças entre os cinco personagens, além de frustrações. Consolidar os cinco principais problemas com exemplos reproduzíveis e prioridade, vinculando-os a esta fila ou à de bugfix. |
| TODO-002 | Controles físicos, GPU e percepção humana de áudio | Alta | Testar teclado, mouse e gamepad físicos no fluxo de seleção, partida, pausa e revanche. Registrar plataforma, GPU, dispositivo e resultado; ouvir volumes, clareza, timbres e pausa/retomada com áudio real. Manter explícitas plataformas/dispositivos não testados. A [revisão atual](26-playtest-visual-completion.md#estado-da-entrega) não oferece essas aprovações humanas. |
| TODO-003 | Medir frequência e impacto dos cinematográficos | Alta | Medir tempo até carregar 100 de energia, usos por partida, duração de cada sequência e tempo total em que jogadores ficam sem agir; relacionar os dados à percepção de ritmo no TODO-001. O [contrato atual](10-greybox-playtest.md#defesa-e-recuperação) tem carga inicial 50, custo 100 e sequências de aproximadamente 5–10,5 segundos. Registrar uma linha de base antes de ajustar ganhos de energia e comparar a mesma situação após cada ajuste. |
| TODO-004 | Investigar risco/recompensa e contrajogo dos cinematográficos | Média | Com os dados do TODO-003, avaliar se captura a qualquer distância e defesa exigida no instante da ativação deixam resposta compreensível e suficiente. Registrar decisão e evidência de playtest; testar uma hipótese de alcance, antecipação ou captura por vez se necessário. São regras atuais e hipóteses de design, sem defeito confirmado. Incluir vantagens da mudança de arena de Rust se afetarem os confrontos. |
| TODO-005 | Balanceamento inicial dos cinco selecionáveis | Alta | Reproduzir situações de Rust, Duke/Java, Old C, Python e C++ no Combat Lab: punição após golpe forte bloqueado ou errado, aproximação contra projéteis e vantagens distintas de Python/C++. Usar [roadmap de combate](13-combat-design-roadmap.md) e [matriz de personagens](15-character-combat-matrix.md); registrar confronto, lado, ações e resultado antes/depois, alterando poucos parâmetros por rodada. Go continua fora da seleção pública. |
| TODO-006 | Calibração de sprite/hitbox e hurtbox por pose | Média, condicionada ao playtest | Se surgir divergência percebida entre desenho e contato, preservar reprodução no Combat Lab com caixas visíveis e revisar apenas o caso demonstrado. Conferir os dois lados e registrar se exige arte ou caixa; os desenhos atuais preservam a referência por `combat_manifest`. Sem evidência, manter pendente em vez de alterar combate por aparência. [Pipeline](11-sprite-pipeline.md), [guia técnico](12-technical-combat-guide.md). |
| TODO-007 | Remapeamento de controles | Média | Definir comandos remapeáveis e comportamento para conflitos/restauração; permitir configurar e persistir teclado/gamepad e conferir seleção, luta e pausa com um mapeamento alterado. Melhoria futura de uso. |
| TODO-008 | Opções de janela e resolução | Média | Definir modos suportados; permitir ajustar janela/resolução, preservar preferências e conferir legibilidade do HUD, menus, personagens e navegação nos modos escolhidos. Melhoria futura de uso. |
| TODO-009 | Voz própria de Old C | Baixa | Selecionar voz redistribuível, registrar procedência/licença e obter avaliação humana de identidade e consistência. O [fallback de Rust](../assets/audio/review/old-c-fallback-2026-09-09/README.md) é provisório e autorizado; substituí-lo não deve alterar as vozes dos demais. |
| TODO-010 | Ferramenta visual clicável | Baixa, condicionada à necessidade | Retomar o [roadmap do viewer](16-sprite-combat-viewer-roadmap.md) somente se atalhos e texto forem insuficientes; registrar o problema de uso antes de avaliar `raygui`. Pendência anterior preservada. |
| TODO-011 | Conciliar prólogo e fontes da lore para a aventura — concluído | Concluído em 10/09/2026 | [Worldbuilding](12-worldbuilding.md), [livro do jogo](../assets/lore/story.json) e [entrega 27](27-rust-story-adventure.md) sincronizados e revisados: EPs como pessoas, Rust como mais recente EP pura, Ada humana/híbrida, mensagem misteriosa antes de Assembly e origem involuntária das erráticas. JSON válido; grafias `Liker` e `presente no desde` removidas dos textos. A autoria da mensagem e a relação completa entre os acontecimentos permanecem em aberto. |
| TODO-012 | Implementar prólogo e primeiro encontro da aventura — concluído | Concluído em 10/09/2026 | [Episódio](27-rust-story-adventure.md) implementado na branch própria: cenas, gameplay, derrota/retry e pesar; features, binários, assets e regras próprios. [Evidências](evidence/adventure-prologue/README.md): vídeo completo, 14 checks na janela, 412 testes com ambos os modos e matriz de isolamento. A avaliação humana segue no TODO-013. |
| TODO-013 | Playtest humano do episódio e apresentação | Alta, próxima avaliação | Jogar a sequência e registrar compreensão de Ada/Assembly, continuidade das poses, leitura da ameaça, comandos e pesar. Avaliar ritmo dos jornais, introdução de C++/Python/elenco e vontade de continuar. Ouvir áudio ao vivo e testar gamepad físico; usar o catálogo editável para revisões de texto. Registrar defeitos reproduzíveis em bugfix e hipóteses em to-do. [Vídeos e limites atuais](evidence/adventure-opening/README.md). |
| TODO-014 | Textos da aventura editáveis sem recompilar — concluído | Concluído em 10/09/2026 | JSON externo, `--texts`, F5 transacional e mensagens de recarga. [Prova no mesmo binário](evidence/adventure-opening/native/native-checks.json): texto alterado, JSON inválido conserva revisão/pixels e restauração funciona, sem mudar o catálogo original. [Guia](../assets/adventure/texts/README.md). |
| TODO-015 | Apresentação após o primeiro combate — concluído | Concluído em 10/09/2026 | [Vídeo](evidence/adventure-opening/opening.mp4): 48 s com jornais, biografias C++/Python, elenco e logo/subtítulo; a entrega 29 remove Go da apresentação e mantém cinco personagens. Trilha própria, entrada direta, pausa, skip e replay verificados. [Entrega 28](28-adventure-texts-and-opening.md). |
| TODO-016 | Atualizar a lore as-is no menu Lore / Roster | Média, backlog; não executar nesta rodada | Conciliar livro, imagens e fichas com o que a aventura já apresenta: Ada/Assembly, Rust, origem de C++ e Python professora, respeitando os mistérios ainda abertos. Incorporar ilustrações pertinentes e conferir leitura no menu. A junção atual preserva o livro como está; revisão editorial e visual será uma entrega própria. |
| TODO-017 | Unir apresentação ao menu principal de terminal — concluído | Concluído em 10/09/2026 | [Entrega 29](29-story-terminal-menu.md): Go fora da apresentação, menu na mesma janela, isolamento e demais destinos preservados; Modo História sem ação, logo coerente, moldura, cursor bloco e revelação binária. [Evidências](evidence/story-terminal-menu/README.md): revisão de navegação com 432 testes com ambos, matriz separada, 81 checks da entrada conjunta e seis da aventura isolada; avanço por trecho, skip total e confirmação final. `cargo run` inicia a sequência conjunta. Gráficos preparados antes do prólogo evitam espera escura no final. |
| TODO-018 | Melhorar cenas do prólogo, começando pela manhã de Rust | Alta, experimento em andamento | [Entrega 30](30-prologue-scene-improvements.md) na branch `feature/prologue-scene-improvements`: quarto com setup e pôsteres Rust; ciclistas em ciclovia inacessível ao jogador; garoto que larga a pipa e foge ao despertar da errática; automóveis e sequência de buzina/frenagem/batida no poste, com carro amassado persistente. Conferir animação, áudio, câmera, pausa, retry/restart e ausência de interferência no combate. [Diário e verificações](worklogs/prologue-scene-improvements.md). Terceira rodada: [rua brasileira modular e evacuação](31-brazilian-street-evacuation.md), com ônibus, ponto, boteco e bicicletas abandonadas. Quarta rodada: [chegada cinematográfica e vizinhança](32-cinematic-neighbourhood-arrival.md), com câmera da pipa até Rust, moradores se abrigando, porta de enrolar e caramelo; som da manhã/trânsito por ambiente. Avaliar as quatro rodadas antes de escolher as demais cenas; publicação não faz parte deste pedido. |

## Backlog de bugfix

BUG-001 a BUG-004 estão **abertos no backlog**, sem correção iniciada. São limites
visuais observados na [revisão dos contatos](evidence/roster-contact-reactions/README.md#revisão-visual-e-limites),
também registrados na [entrega 26](26-playtest-visual-completion.md#estado-da-entrega).
Os testes mecânicos existentes não comprovam que estes problemas de apresentação
foram resolvidos.
BUG-005 foi corrigido na entrega 28; BUG-006 substitui o som da manhã após
relato de ruído pelo usuário.

| ID | Defeito observado | Prioridade | Critério de conclusão / evidência esperada |
|---|---|---|---|
| BUG-001 | HUD encobre cabeça/tronco no ápice dos arremessos | Média | Reproduzir o ápice com os dois lados; ajustar apresentação para manter o personagem legível e os indicadores essenciais do HUD disponíveis. Preservar capturas e vídeo antes/depois no renderer real. |
| BUG-002 | Primeiro recuo de Duke na rajada deixa espaço entre cabeça e faísca | Média | Reproduzir o primeiro contato da rajada de C++ contra Duke; alinhar a reação visual ao ponto de impacto e conferir a sequência completa nos dois sentidos, com frames e vídeo antes/depois. |
| BUG-003 | Go mantém as mãos altas na guarda em pé quando o soco atinge o abdômen | Baixa | Revisar a pose/contato documentado e conferir a defesa nos dois sentidos, preservando as regras de bloqueio. Registrar comparação no renderer. Go está fora da seleção pública; esta correção não prevê incluí-lo no menu. |
| BUG-004 | Focinho/volume facial de Go varia nos desenhos iniciais da recuperação | Baixa | Uniformizar a identidade facial entre os desenhos afetados e conferir a transição de recuperação em movimento nos dois sentidos, com evidência visual. Go permanece fora da seleção pública. |
| BUG-005 | Rust deitado na beirada e sentado acima da cama — corrigido | Concluído em 10/09/2026 | Apoios por pose alinham antebraço/quadril ao colchão, assento à borda e bota ao chão; respiração mantém o apoio. [Manhã corrigida](evidence/adventure-opening/morning.mp4) e quadros revisados na entrega 28. |
| BUG-006 | Ruído incômodo na faixa da manhã — corrigido | Concluído em 10/09/2026 | Removidos vento aleatório, pássaros agudos e acorde contínuo; [morning.wav](../assets/adventure/audio/morning.wav) usa notas suaves, pico menor, sem clipping e junção do loop em zero. Carregamento no binário existente verificado; demais WAVs preservados. [Gerador e revisão](../assets/adventure/audio/README.md). |

Uma observação nova só entra como bug confirmado com situação reproduzível ou
evidência identificada. Questões de ritmo, balanceamento e preferência visual
ficam em to-do até a investigação demonstrar um defeito.

## Histórico de entregas

As verificações abaixo pertencem às respectivas rodadas; não são uma nova
execução nem aprovação humana das pendências listadas acima.

| Janela | Frente | Status | Registro | Proxima acao |
|---|---|---|---|---|
| Concluído | Release prototype.4: aventura, apresentação e menu | Publicada em 10/09/2026; integrada à main pelo PR #21, tag em `20ef404` | [Downloads](https://github.com/osdeving/borrow-fighters/releases/tag/v0.1.0-prototype.4), [notas](releases/v0.1.0-prototype.4.md), [diário e checks](worklogs/release-prototype-4.md) | Cinco pacotes Windows/Linux e SHA256SUMS disponíveis; builds/publicação e checks Rust/Docs aprovados. Playtest humano permanece nos TODO-002/013. |
| Concluído | Textos editáveis, manhã e apresentação da aventura | Implementado na branch; TODO-014/015 e BUG-005 | [Entrega 28](28-adventure-texts-and-opening.md), [ADR 0022](adr/0022-adventure-external-copy-and-opening.md), [evidências](evidence/adventure-opening/README.md) | 417 testes e 26 checks nativos; avaliação humana no TODO-013. |
| Concluído | Aventura: prólogo de Ada e primeiro encontro de Rust | Implementado e verificado na branch `feature/rust-adventure-prologue`; TODO-012 | [Entrega 27](27-rust-story-adventure.md), [ADR 0021](adr/0021-isolated-adventure-experiment.md), [evidências](evidence/adventure-prologue/README.md) | Avaliação humana no TODO-013; regras específicas isoladas da luta, 412 testes e sequência completa gravada. |
| Concluído | Apresentação e fluxo do playtest | Integrado à main; PR #19 | [Entrega 26](26-playtest-visual-completion.md), [ADR 0020](adr/0020-match-flow-selection-and-energy.md), [UI e fluxo](evidence/playtest-visual-flow/README.md), [reações](evidence/roster-contact-reactions/README.md) | 128 desenhos, 120 cenários renderizados, três partidas e duas revanches, 393 testes, Windows/Linux verificados. Próximo: playtest humano e balanceamento fino. |
| Concluído | Reações próprias Python × C++ | Implementado e verificado | [Entrega 25](25-python-cpp-contact-reactions.md), [evidências](evidence/python-cpp-reactions/README.md), [ADR 0018](adr/0018-contact-reaction-profiles.md) | 64 desenhos novos, reação individual à rajada, 64 cenários nos dois sentidos e vídeo do par. |
| Concluído | Reações e transformações | Cinco supers autorais e reações dos seis defensores integrados e verificados | [Plano 24](24-reactions-and-transformations.md), [ADR 0017](adr/0017-reaction-clocks-and-arena-mutation.md), [evidências](evidence/reactions-transformations/README.md) | Playtest humano das animações e dos sons. 335 testes, Clippy estrito, formatação e teste separado de áudio ao vivo aprovados; custo, raridade e vantagens por arena ficam para outra rodada. |
| Concluído | Supers autorais e vozes distintas | Quatro roteiros integrados, novos atlas e áudio por fase | [Plano23](23-authored-super-sequences.md), [ADR0016](adr/0016-authored-super-sequences.md) | Playtest humano das animações e timbres; depois calibrar captura, custo e raridade. 321 testes e 41 verificações de controles aprovados. |
| Concluído | Tipografia, cenários vivos e segundo especial | Fontes incorporadas, memes animados e seis cinematográficas com contato local | [Plano e entrega](22-presentation-and-brazilian-stage-life.md), [ADR 0015](adr/0015-cinematic-presentation-and-stage-life.md) | Playtest humano do acabamento e do risco/recompensa dos novos golpes; 300 testes e revisão gráfica registrados. |
| Concluído | Arremessos e especiais extraordinários | Cinco assinaturas, arremessos e reações integrados e verificados | [Goal local e critérios](21-signature-spectacle-and-throws.md), [ADR 0014](adr/0014-throws-launches-and-signature-effects.md) | 290 testes Rust; 40 cenas gráficas nos dois lados, 25 verificações de controles e 60 lutas de CPU. |
| Concluído | MVP: showcase contextual, especiais e coerência de combate/arte | Cinco selecionáveis verificados; sem conteúdo novo para Go | [Plano e critérios](20-mvp-combat-showcase.md) | Corrigir debug, demonstrar contato real com oponente contextual, completar ações/reações e validar balanceamento/arte. |
| Feito local | Cursor livre e menus por mouse | Corrigido e verificado | [Playtest](10-greybox-playtest.md), [guia técnico](12-technical-combat-guide.md#mouse-e-fechamento-da-janela), [ADR 0012](adr/0012-shared-menu-pointer-layout.md) | Cursor sem centralização por quadro; hover, cliques, saída pelo menu e fechamento nativo verificados em janela isolada. 242 testes Rust aprovados. |
| Feito local | Runtime de `frames[].combat` | Feito | [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md), [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md), [`docs/adr/0007-sprite-frame-combat-runtime.md`](adr/0007-sprite-frame-combat-runtime.md) | Runtime consome hitboxes, hurtboxes e origem de projectile do manifesto com fallback para o greybox. |
| Feito local | Arte final dos seis lutadores | Acabamento e verificação concluídos | [Laudos e vídeos atuais](../assets/candidates/README.md), [cobertura](19-sprite-production-coverage.md) | 120 clips e 378 quadros, dez golpes por personagem. Go recebeu outra identidade semirrealista. Arte padrão, comparação com env=0, fontes e combate preservados. |
| Concluído | Release Prototype 0.1 | Corte v0.1.0-prototype.3: seleção, reações, energia e fluxo de partida | [Processo](06-release-process.md), [notas da versão](releases/v0.1.0-prototype.3.md), [ADR 0019](adr/0019-playtest-distribution.md) | Cinco formatos de pacote; continuar o playtest humano de GPU, controles e áudio. A correção de caminhos Windows da prototype.2 permanece incluída. |

## Ritual de manutenção

Antes de abrir branch:

- verificar **Agora / Próximo / Depois** e as filas de to-do e bugfix;
- confirmar se existe issue ou PR para a frente ativa;
- se nao existir, criar issue pequena ou atualizar este backlog.

Antes de mergear PR:

- atualizar este backlog se a mudanca concluiu, criou ou reordenou trabalho, preservando IDs e registrando evidência ao concluir um item;
- atualizar o roadmap especializado quando houver detalhe tecnico de uma frente;
- registrar no [`CHANGELOG.md`](../CHANGELOG.md) mudancas relevantes.

Depois de mergear:

- fechar ou comentar a issue relacionada;
- decidir explicitamente qual linha da tabela vira o novo **Agora**.

## Legenda de t-shirt sizing

- **XS**: muito pequeno.
- **S**: pequeno.
- **M**: médio.
- **L**: grande.
- **XL**: muito grande; evitar no protótipo.
- **?**: precisa de investigação.

## Prototype 0.1 — Greybox Fighting Slice

Status do primeiro greybox:

- Mergeado na `main`.
- Cobre o núcleo mínimo jogável com sprites placeholder, cenário bitmap e tela de preferências.
- Ainda não fecha Prototype 0.1 como release; serve para playtest, arte inicial e discussão de feeling.

| Item | Tamanho | Prioridade | Status | Observação |
|---|---:|---:|---|---|
| Criar projeto Rust | S | Alta | Feito | Cargo project básico |
| Configurar Raylib/Raylib-rs | M | Alta | Feito | Validado com checks locais |
| Criar janela e loop principal | S | Alta | Feito | Primeiro teste visual |
| Desenhar arena simples | S | Alta | Feito | Arenas Sirius, Fortaleza Tech Coast e Java Street em rotação |
| Criar entidade Player | M | Alta | Feito | Posição, velocidade, vida e estado |
| Implementar input local | M | Alta | Feito | Teclado e gamepad quando disponível |
| Movimento horizontal | M | Alta | Feito | Esquerda/direita com suavização inicial |
| Gravidade e pulo | M | Alta | Feito | Pulo vertical e diagonal |
| Direção/facing do personagem | S | Média | Feito | Olhar para adversário |
| Soco fraco e forte | M | Alta | Feito | Estados separados |
| Chute | M | Alta | Feito | Usado por jogador e CPU |
| Especial projectile | M | Alta | Feito | `ProjectileSpec` por personagem para Rust, Duke, Go, C, Python e C++ |
| Defesa e abaixar | M | Alta | Feito | Inclui leitura visual em sprite |
| Hurtbox | M | Alta | Feito | Ajustável por estado/personagem no código |
| Hitbox | L | Alta | Feito | Área ofensiva temporária |
| Detecção hitbox/hurtbox | L | Alta | Feito | Coração do combate |
| Aplicação de dano | M | Alta | Feito | Flags permitem invencibilidade de P1/P2 |
| Barra de vida | S | Alta | Feito | HUD opcional |
| Condição de vitória | S | Alta | Feito | Vida <= 0 |
| Reinício da partida | S | Média | Feito | Tecla R |
| Debug draw | M | Média | Feito | Toggle de hitbox/hurtbox |
| Runtime de sprites | M | Média | Feito | Atlas + manifesto JSON v1 |
| IA de playtest | M | Média | Feito | P1/P2, perfis diferentes, ataques variados |
| Polimento de timing | M | Alta | Backlog | Ataques, projectile, spawn e IA ainda precisam tuning; acompanhar TODO-003 a TODO-005. |
| Arte dos lutadores | L | Alta | Seis concluídos e verificados | Rust, Duke/Java, Go, C, Python e C++ revisados no runtime. Go antigo arquivado. Cenários e VFX de protótipo ficam fora deste escopo. |

## Fora do backlog inicial

Registro histórico do corte inicial. Arte final e expansão para os seis lutadores
foram autorizadas posteriormente pelo usuário e constam da frente ativa acima.

| Item | Motivo |
|---|---|
| Online multiplayer | Complexidade XL |
| Sistema de combo | Depende do feeling básico |
| Vários personagens | Depende da abstração mínima |
| Arte final | Exclusão inicial superada pela produção autorizada; veja estado atual acima |
| Trilha sonora | Não prova gameplay |
| Menu completo | Não prova gameplay |
| IA avançada | A IA atual é apenas playtest, não desafio competitivo |
