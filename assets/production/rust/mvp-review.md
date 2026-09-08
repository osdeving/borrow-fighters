# Rust — revisão visual do MVP

Arte integrada e revisada para esta rodada: **22 clips / 79 quadros**.
Borrow Break, queda/recuperação e rasteira baixa possuem quatro desenhos novos por ação.
Especial e rasteira duram respectivamente 633 / 533 ms, com fase ativa alinhada
em todos os ticks ao `MoveSpec`. A queda percorre fall/grounded/get_up/recover em
100/220/180/100 ms, acompanhando os 36 frames de recuperação protegida.

Os [prompts/fontes/hashes](mvp-generation.json) e [recortes e pivôs](production.json)
registram a produção. Todas as poses são da ferramenta integrada de imagens; o
processamento local somente extrai matte e prepara os quadros, conforme autorização
registrada no [README de produção](../README.md#alpha-e-escala). As fontes e a
rasteira anterior permanecem preservadas. Os outros **67 quadros anteriores**
conservam pixels, pivôs e tempos: [comparação](mvp-preservation.json).

A revisão verificou identidade, roupa/acessórios, apoio no chão, cortes completos,
alpha em fundo claro/escuro e espelhamento. A escala é uniforme por ação, sem ajustar
cada pose à própria caixa de alpha. Os previews reproduzem os tempos reais:
[especial](../../candidates/rust/review/signature_special-frames.png),
[rasteira](../../candidates/rust/review/sweep-frames.png) e
[queda](../../candidates/rust/review/knockdown-frames.png).

Foram renderizadas **30 demonstrações reais** (15 cenários nos dois lados), com
270 capturas em Raylib/World: 22 acertos e oito bloqueios. O helper rejeita clip
faltante em vez de aprovar fallback. O [registro de estados e resultados](../../candidates/rust/review/mvp/showcase-validation.json)
retém todas as amostras; dez PNGs selecionados acompanham o relatório. O root
inspecionou os 30 contatos em painéis e amostras em resolução1280×720. Anti-air
intercepta o salto, ataques aéreos acertam durante a descida, rasteira/slide
atingem as pernas, agarrão derruba e guardas permanecem na postura correta.
A reação deste personagem como vítima também foi conferida na rotação do elenco.

O Borrow Break tem palmas na altura do peito/ombro; a caixa foi calibrada para
essa região mantendo contato com guarda baixa. A rasteira usa apoio sentado e
pé junto ao chão. Na queda, a última guarda ainda flexiona os joelhos antes do idle.

As novas ações permanecem uma animação de MVP com quatro poses principais;
não há interpolação de membros. O espelhamento dos símbolos acompanha a convenção
existente. Essas características não alteram timing nem volumes físicos.

Reprodução:

```sh
python3 tools/art/prepare_reviewed_actions.py assets/production/rust/review-plan.json
python3 tools/art/build_reviewed_sprite_atlas.py assets/production/rust/production.json
python3 tools/art/render_sprite_review.py assets/candidates/rust/rust-fighter.sprite.json
cargo run --example capture_showcase_review -- rust /tmp/rust-review
cargo run --example capture_showcase_review -- rust /tmp/rust-review-reverse --reverse
```
