# C++ — ações do MVP

O conjunto integrado e revisado para o MVP tem **22 clips e 70 quadros**. Esta revisão acrescenta
`signature_special` e `knockdown`, substitui a rasteira alta anterior por uma
rasteira nos tornozelos e preserva os desenhos, mapas e preparados anteriores.
A integração visual do especial e da queda foi revisada no runtime pelo agente
principal e aceita para este MVP. Os registros históricos de candidatos permanecem
preservados.

| Ação | Quadros | Duração | Leitura revisada |
| --- | --- | --- | --- |
| Template Arc (`signature_special`) | 4 | 700 ms / 42 ticks | Preparação plantada, palma diagonal ascendente com arco violeta, recuperação e prontidão. |
| Vector Low (`sweep`) | 4 | **517 ms / aproximadamente 31 ticks** | Tronco dobrado e apoio baixo durante toda a ação; bota atinge o tornozelo. |
| Knockdown (`knockdown`) | 4 | 600 ms / 36 ticks | Queda para trás, corpo deitado, subida ajoelhada e retorno à guarda. |

O especial usa escala uniforme `268/541`. A palma ativa fica aproximadamente
`(+75,8, -295,2)` em relação ao centro do corpo e ao chão; palma, dedos e arco
estão dentro da geometria atual do Template Arc. A rasteira e a queda usam
`268/562`, mantendo a anatomia de aproximadamente 268 px em pé. A rasteira tem
silhueta baixa de aproximadamente 116–118 px, compatível com o corpo agachado de
128 px, e a bota ativa está em `(+165,0, -10,5)`, dentro da faixa baixa de colisão.
Não foram criados metadados de combate nos quadros exportados.

As três ações foram geradas com a ferramenta integrada `imagegen`. Os prompts,
fontes, variantes e recortes explícitos ficam nas respectivas pastas. A primeira
rasteira do MVP já acertava baixo, mas mantinha o tronco alto; a revisão v4 dobra
mais o corpo sem diminuir a anatomia. O especial chegou em RGB com xadrez pintado.
A remoção inicial deixou bolsões de fundo dentro do arco, portanto a ferramenta
trocou somente o fundo por verde antes da extração local de alpha. A preparação
verde preserva a blusa branca e o efeito violeta que um filtro magenta alteraria.
O script de preparação e a tentativa descartada permanecem registrados.

A inspeção em fundos claro/escuro e nas duas orientações está nos previews:
[especial](../../candidates/cpp/review/mvp-alpha-signature_special.png),
[rasteira](../../candidates/cpp/review/mvp-alpha-sweep.png) e
[queda](../../candidates/cpp/review/mvp-alpha-knockdown.png).

Foram renderizados os **15 cenários contextuais nas duas orientações**, com 135
capturas por lado, sem fallback de animação: onze acertos e quatro bloqueios
reais em cada execução. As defesas e os resultados usam o mesmo `World` da luta.
A queda da C++ também foi capturada nas duas orientações recebendo uma rasteira
real da Python. Os dois testes de `sprite_candidates` passaram, incluindo todas
as fronteiras de startup, ativo e recuperação em ticks reais.

O repositório retém onze PNGs escolhidos e os resultados de todos os cenários:
[relatório do lado esquerdo](../../candidates/cpp/review/mvp/showcase-review-left.json),
[relatório espelhado](../../candidates/cpp/review/mvp/showcase-review-right.json),
[queda recebida](../../candidates/cpp/review/mvp/knockdown-review-left.json) e
[queda espelhada](../../candidates/cpp/review/mvp/knockdown-review-right.json).
Os 270 PNGs completos permanecem em `/tmp/borrow-fighters-cpp-final-left` e
`/tmp/borrow-fighters-cpp-final-right`, fora do conjunto versionado.
O [audit estruturado](mvp-audit.json) registra contagens, hashes e escalas.

Reprodução:

```bash
python3 assets/production/cpp/signature_special/prepare_green_alpha.py
python3 tools/art/prepare_reviewed_actions.py assets/production/cpp/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/cpp/production.json
python3 tools/art/render_sprite_review.py assets/candidates/cpp/cpp-fighter.sprite.json
cargo test --test sprite_candidates
cargo run --example capture_showcase_review -- cpp /tmp/cpp-review-left
cargo run --example capture_showcase_review -- cpp /tmp/cpp-review-right --reverse
cargo run --example capture_showcase_review -- python /tmp/cpp-knockdown-review sweep
```
