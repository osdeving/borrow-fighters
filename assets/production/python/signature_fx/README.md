# Efeitos de import antigravity

[fx-plan.json](fx-plan.json) registra seis recortes, pivôs, durações e escalas.
Execute `python3 assets/production/python/signature_fx/build_atlas.py` da raiz para
reconstruir o [manifest runtime](../../../candidates/python/python-signature-fx.sprite.json)
e seu PNG. [export-audit.json](export-audit.json) registra hashes das entradas/saídas.

`projectile` usa dois quadros de serpente vertical em loop70/70ms, escala uniforme
427/900. Mede aproximadamente190×427world, com pivô na ponta que toca o chão;
a forma circular inicial foi substituída via imagegen, preservando azul/ouro.

`impact` usa quatro desenhos de colapso/dispersão55/80/100/120ms, escala uniforme0,57.
O eixo de dispersão fica aproximadamente213world acima da âncora no chão, para
colapsar perto do centro do campo vertical. Padding transparente acomoda esse pivô,
sem alterar os desenhos ou aplicar outra trajetória.

A preparação red-only preserva o azul da serpente e o dourado, removendo resíduos
do matte apenas junto à transparência. Todas as fontes e primeiros alphas foram
mantidos. Não há arte procedural ou ajuste automático de escala no renderer.
