# Captura World — go

Entrada, contagem, caminhada nos dois sentidos, retorno ao idle e arco completo de um salto, nas duas orientações simultaneamente, renderizados por `draw_fight` a partir da simulação real. Inputs determinísticos; não é um playtest manual de teclado. HP final preservado.

O vídeo usa o tempo da simulação (20 fps de apresentação); as capturas completas e o estado de cada um dos 585 ticks estão em `target/sprite-production/go-motion-final`. Nenhum deslocamento foi desenhado manualmente no vídeo. Manifesto carregado foi comparado ao candidato antes da captura e permaneceu imutável.

[Vídeo](runtime-motion.mp4), [medidas do salto](motion-summary.json) e [laudo do personagem](../../../../production/go/VERIFICATION.md). Este cenário verifica movimento e salto; não testa emissão de especial ou a origem de projétil. A comparação do manifesto descreve o estado da execução, sem afirmar que capturas históricas incorporam revisões posteriores de outros clips.
