# Verificação de controles das sequências autorais

41 verificações aprovadas com inputs XTest reais em Xephyr isolado. O [resultado detalhado](result.json) registra cada checagem, hashes, ajustes das métricas de pixels e os limites de atribuição do binário durante um rebuild concorrente.

- Showcase de C e Duke: pausa, frame-step, Home/Enter, inversão de lados, repetição longa e Escape.
- Combat Lab de C: pausa, step, reset em pausa e durante reprodução, BIOS e retorno completo.
- Luta C × Rust: CPUs desligadas no menu; Y à distância causa 32 HP, U reduz a 8 HP de chip, R cancela e restaura a vida, Escape retorna ao menu.
- Build final: fonte Barlow no SUPER LAB; F9 inicia gravação; BIOS cobre REC e a interface inteira; F10 finaliza o arquivo sem desenhar a mensagem sobre BIOS.

Capturas retidas:

1. [F9 com REC visível antes da sequência](final-01-recording-ready.png).
2. [BIOS cobrindo a tela durante gravação](final-02-BIOS-recording-hidden.png).
3. [SUPER LAB com Barlow e frame-step após reset](final-03-lab-Barlow-font.png).
4. [Luta após o super bloqueado: Rust com 92/100 HP](fight-c-rust-07-guard-restored.png).

Os demais screenshots e logs ficam em `/tmp/borrow-authored-controls/archive`. O teste de F9/F10 usou FFmpeg real com uma fonte silenciosa substituindo Pulse somente no wrapper de QA; a captura foi enviada para `/tmp`, preservando a gravação audiovisual concorrente. A pausa real da música foi verificada separadamente no [registro de streams](../audio-stream-review.json). Não houve mudança de produção nesta tarefa de QA.
