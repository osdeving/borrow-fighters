# Evidência histórica do Combat Lab — python

[Relatório](capture-report.json) e amostras selecionadas da captura GPU de poses. As imagens completas ficam localmente em `target/art/python-combat-lab-projectile-final`. O manifesto visual carregado foi comparado ao candidato daquela execução. A inspeção e as limitações estão no [laudo](../../../../production/python/review.md).

Esta captura antecede a correção que faz o Lab consultar o combate baseline. Os nove golpes deste personagem usam MoveSpec, portanto a evidência de contato continua válida. O especial era criado por `Projectile::from_fighter`, sem consultar a origem baseline: sua posição aparente aqui não comprova a origem usada no World. A verificação da origem foi concluída na [recaptura do especial com baseline](../runtime-baseline-special/README.md). As imagens continuam úteis para verificar a textura e as poses; a palavra `final` no nome da pasta identifica aquela rodada histórica.
