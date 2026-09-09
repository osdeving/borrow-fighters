# ADR 0011 — Arte revisada como apresentação padrão

## Status

Aceito e verificado localmente em 7 de setembro de 2026. Os seis conjuntos concluíram seus laudos na [rodada de refinamento](../../assets/production/FINALIZATION-PLAN.md); trabalho sem commit nem push.

## Contexto

O lote anterior integra 20 clips por personagem, mas exige `BORROW_FIGHTERS_SPRITE_CANDIDATES=1` para exibi-los. Assim, iniciar normalmente o jogo continua mostrando a arte anterior mesmo quando o trabalho novo já foi exportado. O pedido atual inclui concluir a produção, integração e verificação, preservando os assets existentes como referência e fallback.

## Decisão

Depois da revisão visual e funcional dos seis personagens, usar os atlas revisados como apresentação padrão. Manter os caminhos existentes em `assets/candidates/` para preservar referências e ferramentas; o nome histórico da pasta não é uma avaliação automática da qualidade.

- Sem a variável de ambiente, procurar o conjunto revisado completo.
- `BORROW_FIGHTERS_SPRITE_CANDIDATES=1` continua selecionando os conjuntos novos explicitamente.
- `BORROW_FIGHTERS_SPRITE_CANDIDATES=0` permite comparar os placeholders originais. Valores inválidos também escolhem os originais.
- Manifesto, textura ou clips inválidos/ausentes mantêm o fallback por personagem.
- Aplicar a mesma seleção às texturas opcionais de projétil.
- `combat_manifest`, corpo, dano, hitboxes, hurtboxes, timing e origem dos projéteis continuam no baseline, como no [ADR 0010](0010-reviewed-action-sprite-production.md).

Não adicionar formato de atlas, estado de combate ou sistema de assets. A mudança é a escolha visual inicial do carregador existente, verificada também iniciando o aplicativo sem a variável.

## Verificação necessária para promoção

Concluir os laudos por personagem, conferir cobertura e contato, ambas as orientações e transições no runtime. Verificar inicialização padrão, comparação com `0`, fallback e preservação dos dados de combate. Atualizar README, pipeline, matriz e comandos de revisão junto da mudança.

O [teste nativo do carregador](../../assets/production/runtime-verification/README.md) passou em dez casos de seleção/fallback, com comparação integral dos manifestos e preservação do baseline. Os seis laudos de arte foram concluídos, incluindo o novo Go, sua correção de salto e recaptura final. O teste de carregamento complementa essa revisão visual.
