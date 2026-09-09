# Python — revisão visual do MVP

Arte integrada e revisada para esta rodada: **22 clips / 70 quadros**.
Serpent Slide, queda/recuperação e rasteira baixa possuem quatro desenhos novos por ação.
Especial e rasteira duram respectivamente 650 / 483 ms, com fase ativa alinhada
em todos os ticks ao `MoveSpec`. A queda percorre fall/grounded/get_up/recover em
100/220/180/100 ms, acompanhando os 36 frames de recuperação protegida.

Os [prompts/fontes/hashes](mvp-generation.json) e [recortes e pivôs](production.json)
registram a produção. Todas as poses são da ferramenta integrada de imagens; o
processamento local somente extrai matte e prepara os quadros, conforme autorização
registrada no [README de produção](../README.md#alpha-e-escala). As fontes e a
rasteira anterior permanecem preservadas. Os outros **58 quadros anteriores**
conservam pixels, pivôs e tempos: [comparação](mvp-preservation.json).

A revisão verificou identidade, roupa/acessórios, apoio no chão, cortes completos,
alpha em fundo claro/escuro e espelhamento. A escala é uniforme por ação, sem ajustar
cada pose à própria caixa de alpha. Os previews reproduzem os tempos reais:
[especial](../../candidates/python/review/signature_special-frames.png),
[rasteira](../../candidates/python/review/sweep-frames.png) e
[queda](../../candidates/python/review/knockdown-frames.png).

Foram renderizadas **30 demonstrações reais** (15 cenários nos dois lados), com
270 capturas em Raylib/World: 22 acertos e oito bloqueios. O helper rejeita clip
faltante em vez de aprovar fallback. O [registro de estados e resultados](../../candidates/python/review/mvp/showcase-validation.json)
retém todas as amostras; dez PNGs selecionados acompanham o relatório. O root
inspecionou os 30 contatos em painéis e amostras em resolução1280×720. Anti-air
intercepta o salto, ataques aéreos acertam durante a descida, rasteira/slide
atingem as pernas, agarrão derruba e guardas permanecem na postura correta.
A reação deste personagem como vítima também foi conferida na rotação do elenco.

O slide tem perna estendida com a cobra acompanhando o tornozelo; a rasteira usa
apoio de mão e recolhimento diferentes. A pose baixa muda a altura aparente sem
encolher a anatomia. Blusa branca, saia, saltos e cobra foram preservados no alpha.

As novas ações permanecem uma animação de MVP com quatro poses principais;
não há interpolação de membros. O espelhamento dos símbolos acompanha a convenção
existente. Essas características não alteram timing nem volumes físicos.

Reprodução:

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/python/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/python/production.json
python3 tools/art/render_sprite_review.py assets/candidates/python/python-fighter.sprite.json
cargo run --example capture_showcase_review -- python /tmp/python-review
cargo run --example capture_showcase_review -- python /tmp/python-review-reverse --reverse
```
