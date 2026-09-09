# Duke — ações para o MVP

O candidato contém **22 clips e 70 quadros**. Foram criadas três sequências pela ferramenta integrada `imagegen`: especial próprio `signature_special`, reação `knockdown` e rasteira `sweep` baixa. O projétil anterior continua no clip `special`.

| Ação | Resultado | Temporização |
| --- | --- | --- |
| GC Slam | Quatro desenhos: mãos acima da cabeça, contato descendente com traço de circuitos, acompanhamento baixo e retorno à guarda. | 366 / 117 / 192 / 192 ms; 867 ms, ativo nos ticks 22–28. |
| Knockdown | Queda para trás, apoio na aterrissagem, pose baixa e recuperação. O primeiro pivô mantém um chão virtual abaixo dos pés suspensos. | 100 / 100 / 250 / 150 ms; 600 ms. |
| GC Sweep | Perna estendida junto ao chão; sapato atinge tornozelo/canela e recolhe mantendo a postura baixa. | 216 / 167 / 125 / 125 ms; 633 ms, ativo nos ticks 13–22. |

Os retângulos e pivôs são explícitos nos `action.json`. A escala é uniforme por ação: 0,5 no especial/rasteira e 0,56 na queda, preservando a leitura do nariz e dos membros. O contato manual do sapato da rasteira, fonte `(1080,606)` contra pivô `(734,634)`, projeta para `(+173,-14)` relativo ao chão do corpo. Essa faixa corresponde à nova caixa baixa de `−42,67..0 px`. No especial, o centro dos punhos fica aproximadamente em `(+75,-50)`; o traço descendente cobre a trajetória acima deles. Essas medidas são inspeção da arte, não alegação de playtest interativo.

Foram conferidos os quatro desenhos de cada sequência no tamanho do runtime, alpha em fundos claro/escuro e espelhamento nos GIFs. A fase de queda mantém o corpo reconhecível, sem recorrer ao hit em pé. Cone e vapor continuam acima do tronco comprimido da rasteira; o vapor é detalhe visual.

- [Especial quadro a quadro](../../candidates/duke/review/signature_special-frames.png), [queda](../../candidates/duke/review/knockdown-frames.png) e [rasteira](../../candidates/duke/review/sweep-frames.png).
- [Especial com alpha claro/escuro](signature_special/mvp-alpha-review.png), [queda](knockdown/mvp-alpha-review.png) e [rasteira](sweep/mvp-alpha-review.png).
- [Fontes, prompts e hashes](mvp-generation.json); [preservação de 58 quadros anteriores](mvp-preservation.json).

A ferramenta retornou RGB com xadrez apesar do pedido de transparência. As primeiras extrações das duas ações novas ficaram apenas como evidência: eliminavam parte do vapor. As revisões v2 trocaram o fundo por magenta através de `imagegen`; o helper existente extraiu alpha sem a opção específica de contorno Rust. Fontes v1/v2 e prompts permanecem no disco. Nenhuma pose foi inventada por processamento de pixels.

O [snapshot anterior](snapshots/2026-09-07-before-mvp/production.json) preserva as entradas antigas. Os **58 quadros dos outros 19 clips** conservam exatamente os mesmos bytes RGBA, pivôs, durações e metadados; somente suas posições no atlas podem mudar. A substituição da rasteira é intencional e acompanha a revisão física do golpe.

Reprodução após geração das fontes:

```sh
python3 tools/art/remove_generated_magenta.py assets/production/duke/signature_special/source-v2.png assets/production/duke/signature_special/keyed-v2.png
python3 tools/art/remove_generated_magenta.py assets/production/duke/knockdown/source-v2.png assets/production/duke/knockdown/keyed-v2.png
python3 tools/art/remove_generated_magenta.py assets/production/duke/sweep/source-mvp-v1.png assets/production/duke/sweep/keyed-mvp-v1.png
python3 tools/art/prepare_reviewed_actions.py assets/production/duke/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/duke/production.json
python3 tools/art/render_sprite_review.py assets/candidates/duke/duke-fighter.sprite.json
```

## Revisão no renderer real

Foram executados os **15 cenários nos dois lados** a 1280×720, usando o mesmo `World`, seletor de clips e renderer do Training. As 30 situações concluíram com 22 acertos e 8 bloqueios. Cada ator utilizou o clip solicitado em todas as 270 capturas; no tick 199 os dois estavam no chão e fora de hitstun/blockstun, retornando a idle ou à guarda prevista pelo cenário.

A inspeção abriu os 30 quadros de contato, os golpes novos, os dois tipos de guarda e as reações de dano. A queda foi conferida em quatro estágios, incluindo uma captura adicional no contato +30 para enxergar a recuperação, também espelhada. Não foi encontrado corte de silhueta, fundo residual, penetração do apoio no chão ou retângulo de debug. A conferência cruzada incluiu os sprites de guarda e dano dos cinco personagens públicos nos dois sentidos. Isso registra revisão gráfica determinística; não substitui uma sessão longa de playtest humano.

- [Relatório com estados, resultados e hashes](../../candidates/duke/review/mvp/runtime-review.json).
- [Especial em contato](../../candidates/duke/review/mvp/signature-contact.png) e [espelhado](../../candidates/duke/review/mvp/signature-contact-mirrored.png).
- [Rasteira baixa](../../candidates/duke/review/mvp/sweep-contact.png), [anti-aéreo contra salto](../../candidates/duke/review/mvp/anti-air-contact.png) e [reação ao agarrão](../../candidates/duke/review/mvp/throw-reaction.png).
- [Guarda em pé](../../candidates/duke/review/mvp/standing-block.png), [agachada](../../candidates/duke/review/mvp/crouching-block.png) e [dano comum recebido](../../candidates/duke/review/mvp/received-normal-hit.png).
- [Derrubado](../../candidates/duke/review/mvp/received-knockdown.png), [recuperando](../../candidates/duke/review/mvp/received-recovery.png) e [recuperando espelhado](../../candidates/duke/review/mvp/received-recovery-mirrored.png).

Doze imagens foram curadas no repositório. As capturas brutas ficaram em `/tmp/borrow-fighters-duke-c-mvp-final/`; o relatório preserva os estados de todos os quadros, os nomes originais e os hashes. Para repetir em uma sessão gráfica, execute:

```sh
cargo run --example capture_showcase_review -- duke /tmp/duke-showcase-forward
cargo run --example capture_showcase_review -- duke /tmp/duke-showcase-reverse --reverse
```
