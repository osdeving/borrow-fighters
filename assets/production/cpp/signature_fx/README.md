# Efeitos de Undefined Bazooka

[fx-plan.json](fx-plan.json) registra seis recortes, pivôs, durações e escalas.
Execute `python3 assets/production/cpp/signature_fx/build_atlas.py` da raiz para
reconstruir o [manifest runtime](../../../candidates/cpp/cpp-signature-fx.sprite.json)
e seu PNG. [export-audit.json](export-audit.json) registra hashes das entradas/saídas.

`projectile` usa dois quadros em loop, cerca de130×70px incluindo rastro, escala
uniforme130/455. A âncora está no corpo do foguete; o rastro fica atrás. O eixo
pintado já desce cerca de24°: o renderer subtrai24° antes de aplicar o ângulo da
velocidade física. Espelhamento inverte corretamente o eixo e a compensação.

`impact` usa os quatro desenhos inferiores da fonte v2,35/55/50/43ms, escala
uniforme347/407. A explosão visual tem cerca de347×184px e usa pivô na base/chão;
sua chama e símbolos são cosméticos acima da região física de pernas. A variante
extra no alto da v2 não integra o atlas. A primeira fonte alta foi substituída.

A preparação red-only preserva ouro, preto e cores internas, removendo resíduos
do matte apenas junto à transparência. Todas as fontes e primeiros alphas foram
mantidos. Não há arte procedural ou ajuste automático de escala no renderer.
