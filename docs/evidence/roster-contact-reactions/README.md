# Rust, Duke, C e Go — reações por contato

Os quatro personagens receberam **128 desenhos**, com oito perfis de quatro
poses: cabeça, corpo, perna, guarda alta, guarda baixa, lançamento, queda e
recuperação. A [produção](../../../assets/production/roster-reactions-2026-09-09/README.md)
registra fontes, recortes, escala e pivôs.

## Verificação dos assets

A [auditoria](asset-review.json) compara os candidatos com `38d81ae` e confirma:

- 128 recortes RGBA distintos, com transparência e sem clipping nas bordas;
- 334 frames e 95 clips anteriores preservados, incluindo os bytes dos PNGs;
- escala dos manifests preservada; pivôs dentro das células;
- medidas de altura aparente e do pixel mais baixo em relação ao apoio.

A [inspeção das pranchas](source-visual-review.json) examinou todos os desenhos
sobre fundo claro/escuro. Cabeça, braços, tronco e pernas mudam entre impacto,
compressão e recomposição. As poses de voo, queda e levantamento preservam os
acessórios e o apoio visível. Essa etapa verifica a arte isolada; as capturas de
contato no runtime e os testes integrados ainda estão em execução nesta rodada.

## Captura e matriz reproduzíveis

O [exemplo de captura](../../../examples/capture_roster_contacts.rs) aplica
comandos reais no `World`, com métricas corporais e manifests de combate do jogo,
e desenha os dois atores com `draw_fight`. Ele exporta PNGs sem retoque e um JSON
com contatos, HP, perfil, idade da reação, desenho, pivô e posição física. Nenhum
perfil é forçado para produzir a evidência. O exemplo não captura áudio.

`--video PATH.mp4` também grava todos os ticks diretamente do framebuffer a
60 fps, sem interpolação. Requer o `ffmpeg` já usado pelas outras ferramentas
de captura, limitado a uma thread. O JSON identifica o intervalo exato de
frames de cada cenário. Sem essa opção, apenas os PNGs selecionados são lidos.

As execuções completas gravam os 80 frames consecutivos das oito pancadas de
C++ por defensor; `--barrage-only` permite repetir somente essa sequência.
`--no-damage` mantém o alvo em 1 HP com dano desligado; `--reverse`
espelha o confronto e `--defender` limita a Rust, Duke, C ou Go. As saídas precisam
estar em diretórios vazios para preservar capturas anteriores.

Com Rust, dependências Raylib e um display disponível (Xvfb também funciona):

```sh
CARGO_BUILD_JOBS=1 cargo test --test roster_contact_reactions --test reaction_matrix --test contact_reactions --test paired_reaction_sprites --test no_damage_reactions
cargo run --example capture_roster_contacts -- --output /tmp/roster-contact-right --video /tmp/roster-contact-right.mp4
cargo run --example capture_roster_contacts -- --reverse --output /tmp/roster-contact-left
cargo run --example capture_roster_contacts -- --no-damage --output /tmp/roster-contact-protected-right
cargo run --example capture_roster_contacts -- --no-damage --reverse --output /tmp/roster-contact-protected-left
python3 tools/art/audit_roster_contact_assets.py --baseline 38d81ae --report /tmp/roster-asset-review.json
python3 tools/art/audit_roster_contact_capture.py --input /tmp/roster-contact-right --input /tmp/roster-contact-left --input /tmp/roster-contact-protected-right --input /tmp/roster-contact-protected-left --report /tmp/roster-contact-verification.json
```

A matriz nova verifica cada pancada em 96 combinações de defensor, slot, lado,
guarda e dano ligado/desligado, além de KO e apoio no piso. A matriz geral
continua cobrindo 1.704 combinações de ataques e defensores. Resultados efetivos
e capturas revisadas serão registrados aqui após a execução integrada.

O auditor de captura exige 120 cenários completos: os oito perfis e os 32
desenhos de cada defensor em ambos os sentidos, separadamente com dano ligado
e desligado. São 128 janelas de rajada, com 1.280 PNGs consecutivos; o KO deve
preservar a pose final no chão por pelo menos 60 ticks após o cinematográfico.
Esses números são critérios de verificação, não resultados já observados.
