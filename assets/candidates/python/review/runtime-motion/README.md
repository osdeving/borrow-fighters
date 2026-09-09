# Captura World — python

Entrada, contagem, avanço/recuo, retorno ao idle e salto completo, nas duas orientações simultaneamente, renderizados por `draw_fight` a partir da simulação real. Inputs determinísticos; não é playtest manual de teclado. HP final preservado.

O vídeo preserva o tempo da simulação (20 fps de apresentação); capturas e estados de cada um dos 585 ticks ficam localmente em `target/sprite-production/python-motion-clean-final`. O manifesto carregado foi comparado ao candidato antes da captura e permaneceu imutável. Revisões posteriores de outros clips e recorte de pixels isolados são detalhadas no laudo do personagem.

[Vídeo](runtime-motion.mp4), [medidas do salto](motion-summary.json) e [laudo do personagem](../../../../production/python/review.md). Este cenário verifica movimento e salto; não testa emissão de especial ou a origem de projétil. A comparação do manifesto descreve o estado da execução, sem afirmar que capturas históricas incorporam revisões posteriores de outros clips.
