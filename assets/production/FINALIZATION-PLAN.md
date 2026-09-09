# Plano de continuação da arte — concluído em 7 de setembro de 2026

## Objetivo e ponto de partida

Retomar o pedido de evoluir os seis personagens existentes para arte final, com Rust como piloto, preservando identidade, fontes, placeholders e regras de combate. Trabalho local, sem commit nem push. O goal permanece ativo enquanto houver produção ou verificação necessária.

O lote anterior, registrado no commit `90299c0`, entregou seis candidatos com 20 clips por personagem e 373 quadros. Os dez golpes atuais estão implementados; produzir novas mecânicas não faz parte deste trabalho. A [matriz de cobertura](../../docs/19-sprite-production-coverage.md) e os laudos individuais registram o que foi verificado e os defeitos restantes.

## Alteração de escopo solicitada durante a execução

O usuário interrompeu explicitamente a versão atual de Go por estar caricata demais em relação ao restante do jogo e pediu outra com mais realismo. A produção dessa versão foi abortada. Go passa a ser a exceção autorizada à preservação estrita da identidade anterior: estabelecer uma nova referência com anatomia/pelagem e acabamento mais naturais, depois produzir suas 20 ações com essa referência fixa. Preservar a versão descartada como histórico, sem promovê-la; colisão, golpes e métricas físicas continuam iguais. Duke e os outros personagens seguem o plano existente.

## Plano de execução

1. **Recuperar contexto:** conferir o prompt original, as correções do usuário no histórico local, os prompts por ação e os PNGs reais. Preservar as referências principais em cada geração. Recuperada a autorização anterior para remover o fundo localmente e revisar o alpha.
2. **Refinar Rust piloto:** corrigir leitura de contato da varredura, agarrão e chute aéreo; revisar o ciclo de caminhada, consistência e transições. Gerar revisões em arquivos versionados, mantendo as fontes anteriores. Revisar recortes e pontos de apoio explícitos antes de exportar.
3. **Validar piloto completo:** conferir as 20 ações, incluindo em movimento, ambas as orientações, escala real e fundos claros/escuros. Usar Combat Lab com colisão baseline, World e Sprite Studio conforme disponíveis. Reprovar revisões que piorarem identidade ou continuidade. Registrar evidência específica antes de propagar o refinamento.
4. **Aplicar aos demais:** revisar Duke, Go, C, Python e C++, priorizando ações com defeitos observáveis nos laudos e painéis. Manter o que já funciona; cada alteração deve resolver um problema identificado. Revisar também os projéteis separados, distinguindo arte gerada de reaproveitada.
5. **Conferir integração e entrega:** reconstruir atlas PNG + JSON com as ferramentas existentes; conferir os 120 clips e dez golpes de cada personagem no runtime. Depois de revisar os seis conjuntos, promover a apresentação nova ao início padrão, com comparação explícita e fallback preservados, conforme a [decisão do ADR 0011](../../docs/adr/0011-reviewed-art-default.md). Rodar os checks pertinentes e atualizar matriz, galeria, proveniência e laudos. Informar exatamente qualquer limitação de verificação.

## Critérios de encerramento

- Identidade, proporções, roupa e acessórios coerentes entre ações. Go usa o redesign explicitamente solicitado nesta execução; os demais preservam as referências anteriores.
- Cada golpe mostra preparação, contato e recuperação no timing existente.
- Apoio e pivôs estáveis, sem salto involuntário de posição nem recortes de corpo.
- Superfície ativa do desenho coerente com hitboxes baseline; emissão perto de mão, arma ou outro emissor real. Dano, alcance, corpo físico e frame data preservados.
- Transparência real, sem fundo pintado, franja de recorte ou guias nos PNGs runtime.
- Sequências e transições examinadas nos dois sentidos e no tamanho de jogo; testes estruturais não substituem revisão visual.
- Arquivos de trabalho por ação, mapeamento explícito e evidência de revisão preservados. O estado de candidato só muda quando houver evidência visual e funcional suficiente; o nome do goal não aprova arte automaticamente.

## Registro desta continuação

- Inventário conferido: seis manifests presentes, 120 clips e 373 quadros; testes focados de combate, seleção e reprodução passaram antes dos novos ajustes.
- Rust piloto refinado e validado. Duke, C, Python e C++ também concluíram as revisões, capturas World/Lab e partidas nativas com resultado; seus laudos atuais estão nas pastas de produção.
- Go novo concluído: 20 ações/60 quadros, outra identidade semirrealista, salto v2 corrigido, World completo final, Lab, Studio e partida nativa repetidos. Os 263 arquivos antigos foram preservados e os seis contratos físicos conferidos. Ver [laudo final](go/finalization-review.md).

### Histórico recuperado

O anexo desta retomada é idêntico ao pedido inicial de 7 de setembro. No histórico do mesmo projeto, o usuário autorizou em seguida: “Sim, remover localmente e revisar”. Depois apontou pontos brancos nas bordas e sugeriu magenta. Essa autorização continua válida para preparar alpha; as fontes RGB permanecem preservadas e o jogo recebe PNG RGBA, sem chroma key em tempo de renderização. A autorização posterior de commit dizia respeito ao lote anterior; nesta continuação vale o pedido reenviado de não fazer commit nem push.

### Rust — refinamento integrado e verificado

| Ação | Revisão | Mudança |
|---|---|---|
| walk | [v8](rust/walk/review-v8.md) | Quatro poses com alternância real das pernas, 400 ms, escala uniforme baseada no corpo de referência. |
| sweep | [fonte v6](rust/sweep/source-v6.png) | Pé ativo mais baixo que v2 e dentro da área baseline; v5 rejeitada por baixar demais. Alpha preparado por [script rastreável](rust/sweep/prepare-v6.py). |
| throw | [fonte v2](rust/throw/source-v2.png) | Cotovelos dobrados e mão de contato dentro do alcance curto existente. |
| air_kick | [fonte v2](rust/air_kick/source-v2.png) | Perna/bota ativas redesenhadas para coincidir com a box; restaurada a âncora aérea original da sequência. |
| air_punch | [calibração](rust/air_punch/pivot-calibration-2026-09-07.md) | Somente pivô virtual do quadro ativo, removendo queda comum de cabeça/cintura de aproximadamente 22 px; PNG reaproveitado. |

As primeiras 57 capturas nativas confirmaram contato de sweep/throw/air_kick com o baseline. Depois da integração de walk/air_punch passaram os 17 testes focados de candidatos, seleção e reprodução. A revisão completa posterior produziu 276 capturas no Lab, 650 no World com dez golpes nas duas orientações, 24 quadros no Studio nativo, recaptura do projétil limpo e uma partida entre CPUs até o resultado. O [laudo atual](rust/finalization-review.md) distingue o que foi exercitado de cada limite de evidência.

### Prioridades originais da auditoria — registro histórico

| Personagem | Ações a refinar | Problema observado |
|---|---|---|
| Duke | walk, crouch_block, overhead | Passadas repetidas; guarda baixa excessivamente alta; contato de overhead com leitura horizontal. |
| Go | walk, sweep, air_kick | Ciclo de duas poses; sola de sweep na borda superior; contato aéreo a alinhar. |
| C | walk, sweep, punch_heavy | Passagem irregular; sola acima da box; livro do soco forte pouco alinhado ao contato. |
| Python | walk, kick, sweep | Alternância de pernas; sapato abaixo da box do chute; sola e recolhimento da varredura. |
| C++ | walk, air_kick, crouch_block | Continuidade de caminhada; bota aérea abaixo da box; transição abrupta de defesa baixa. |

As prioridades vieram de PNGs e capturas reais, conferidos com [contact-contract.json](contact-contract.json). A box de sweep permanece aproximadamente entre −136 e −96 px da âncora: desenhar a sola rente ao chão não corresponderia à mecânica atual. Detalhes menores e projéteis reaproveitados serão aceitos ou refinados conforme inspeção, sem gerar revisões arbitrárias apenas para aumentar a quantidade de arquivos.

As linhas de Duke, C, Python e C++ foram resolvidas e verificadas nos respectivos [laudos Duke](duke/finalization-review.md), [C](c/finalization-review.md), [Python](python/finalization-review.md) e [C++](cpp/finalization-review.md). A linha original de Go foi supersedida pelo redesign de todas as ações, conforme a alteração de escopo acima.

## Entrega concluída

Os seis conjuntos atuais totalizam 120 clips/378 quadros e os dez golpes existentes por personagem. PNGs, JSONs, fontes, prompts, alpha, pivôs, tempos, painéis/GIFs e evidências nativas estão preservados. A [galeria atual](../candidates/README.md) reúne os vídeos e laudos. A arte revisada é padrão, com comparação e fallback conforme o ADR 0011.

Passaram 234 testes Rust após o último atlas, sete testes das ferramentas Python, cargo fmt, Clippy com warnings negados e validação de caminhos Markdown. Combate, placeholders e métricas físicas permanecem sem diferenças. Não houve commit nem push. As ressalvas menores de estilo/apoio estão explicitadas nos laudos, sem pendência de clip ou golpe neste escopo.
