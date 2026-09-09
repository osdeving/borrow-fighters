# C — ações para o MVP

O candidato contém **22 clips e 71 quadros**. Foram criadas três sequências pela ferramenta integrada `imagegen`: especial próprio `signature_special`, reação `knockdown` e rasteira `sweep` baixa. O lançamento binário anterior continua no clip `special`.

| Ação | Resultado | Temporização |
| --- | --- | --- |
| Pointer Lance | Preparação, avanço com ponteiro ciano na mão livre, retração e guarda. O livro permanece sob o outro braço. | 300 / 100 / 183 / 184 ms; 767 ms, ativo nos ticks 18–23. |
| Knockdown | Perda de equilíbrio, apoio lateral, corpo no chão e recuperação em joelho. O livro acompanha o corpo nos quatro desenhos. | 100 / 100 / 250 / 150 ms; 600 ms. |
| Segfault Sweep | Rasteira efetiva junto ao chão; mão livre apoia o corpo enquanto a perna estende e recolhe. | 200 / 150 / 125 / 125 ms; 600 ms, ativo nos ticks 12–20. |

Os retângulos e pivôs são explícitos nos `action.json`. A escala é uniforme por ação: `268/504` no especial/queda e `0,45` na rasteira. A fonte inicial do especial apontava na altura do ombro; a v2 redesenhou o braço ativo na altura da cintura/peito baixo. A ponta do ponteiro, fonte aproximada `(1140,393)` contra pivô `(727,644)`, projeta para `(+219,6,-133,5)`, dentro da faixa inicialmente definida para o golpe. O centro da sola da rasteira `(1080,582)`, contra pivô `(705,614)`, projeta para `(+168,75,-14,4)`, dentro da nova faixa física de `−42,67..0 px` acima do chão. São medidas de inspeção da arte; não substituem captura de contato no runtime.

Foram conferidos os quatro desenhos por ação no tamanho do jogo, alpha em fundos claro/escuro e espelhamento nos GIFs. A cabeça e o livro preservam a identidade; os símbolos espelham junto do personagem, conforme o renderer existente. A reação caída permite distinguir rasteira/agarrão dos hits em pé.

- [Especial quadro a quadro](../../candidates/c/review/signature_special-frames.png), [queda](../../candidates/c/review/knockdown-frames.png) e [rasteira](../../candidates/c/review/sweep-frames.png).
- [Especial com alpha claro/escuro](signature_special/mvp-alpha-review.png), [queda](knockdown/mvp-alpha-review.png) e [rasteira](sweep/mvp-alpha-review.png).
- [Fontes, prompts e hashes](mvp-generation.json); [preservação de 59 quadros anteriores](mvp-preservation.json).

A ferramenta retornou RGB apesar do pedido de transparência. Especial v2 e rasteira usam o helper de magenta existente, sem contorno específico de Rust. A queda usa a extração de fundo claro conectado à borda; contornos, cabelo branco, camiseta e solas foram conferidos sobre fundos claro/escuro. As fontes originais permanecem intactas. Nenhuma pose foi criada por processamento de pixels.

O [snapshot anterior](snapshots/2026-09-07-before-mvp/production.json) preserva as entradas antigas. Os **59 quadros dos outros 19 clips** mantêm exatamente os mesmos bytes RGBA, pivôs, durações e metadados; apenas suas posições no atlas podem mudar. A substituição da rasteira acompanha a revisão de geometria baixa no combate.

Reprodução após geração das fontes:

```sh
python3 tools/art/remove_generated_magenta.py assets/production/c/signature_special/source-v2.png assets/production/c/signature_special/keyed-v2.png
python3 tools/art/remove_generated_checkerboard.py assets/production/c/knockdown/source-v1.png assets/production/c/knockdown/keyed-v1.png
python3 tools/art/remove_generated_magenta.py assets/production/c/sweep/source-mvp-v1.png assets/production/c/sweep/keyed-mvp-v1.png
python3 tools/art/prepare_reviewed_actions.py assets/production/c/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/c/production.json
python3 tools/art/render_sprite_review.py assets/candidates/c/c-fighter.sprite.json
```

## Revisão no renderer real

Foram executados os **15 cenários nos dois lados** a 1280×720, usando o mesmo `World`, seletor de clips e renderer do Training. As 30 situações concluíram com 22 acertos e 8 bloqueios. Cada ator utilizou o clip solicitado em todas as 270 capturas; no tick 199 os dois estavam no chão e fora de hitstun/blockstun, retornando a idle ou à guarda prevista pelo cenário.

A inspeção abriu os 30 quadros de contato, os golpes novos, os dois tipos de guarda e as reações de dano. A queda foi conferida em quatro estágios, incluindo uma captura adicional no contato +30 para enxergar a recuperação, também espelhada. Não foi encontrado corte de silhueta, fundo residual, penetração do apoio no chão ou retângulo de debug. A conferência cruzada incluiu os sprites de guarda e dano dos cinco personagens públicos nos dois sentidos. Isso registra revisão gráfica determinística; não substitui uma sessão longa de playtest humano.

- [Relatório com estados, resultados e hashes](../../candidates/c/review/mvp/runtime-review.json).
- [Especial em contato](../../candidates/c/review/mvp/signature-contact.png) e [espelhado](../../candidates/c/review/mvp/signature-contact-mirrored.png).
- [Rasteira baixa](../../candidates/c/review/mvp/sweep-contact.png), [anti-aéreo contra salto](../../candidates/c/review/mvp/anti-air-contact.png) e [reação ao agarrão](../../candidates/c/review/mvp/throw-reaction.png).
- [Guarda em pé](../../candidates/c/review/mvp/standing-block.png), [agachada](../../candidates/c/review/mvp/crouching-block.png) e [dano comum recebido](../../candidates/c/review/mvp/received-normal-hit.png).
- [Derrubado](../../candidates/c/review/mvp/received-knockdown.png), [recuperando](../../candidates/c/review/mvp/received-recovery.png) e [recuperando espelhado](../../candidates/c/review/mvp/received-recovery-mirrored.png).

Doze imagens foram curadas no repositório. As capturas brutas ficaram em `/tmp/borrow-fighters-duke-c-mvp-final/`; o relatório preserva os estados de todos os quadros, os nomes originais e os hashes. Para repetir em uma sessão gráfica, execute:

```sh
cargo run --example capture_showcase_review -- c /tmp/c-showcase-forward
cargo run --example capture_showcase_review -- c /tmp/c-showcase-reverse --reverse
```
