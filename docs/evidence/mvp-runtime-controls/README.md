# Verificação gráfica de controles e debug

O [resultado](result.json) registra 20 verificações aprovadas no executável Raylib em 8 de setembro de 2026 UTC. A execução usou Xephyr com janela pai não mapeada: teclado, mouse e fechamento foram enviados apenas ao display aninhado. `BORROW_FIGHTERS_SPRITE_CANDIDATES=0` manteve a arte baseline enquanto os novos atlas eram exportados; estas capturas validam controles e debug, sem substituir a revisão da arte final.

O showcase foi aberto com:

```bash
cargo run -- --showcase --character rust --move throw --repeat --reverse
```

Após o primeiro frame visível, a automação verificou pausa estável, `.` avançando de 42 para 43, `Home`/`Enter` reiniciando em pausa, `Tab`/`Shift+Tab`, repetição com `L`, inversão com `X`, personagens com `PageDown`/`PageUp`, `Esc` voltando ao menu e fechamento da janela com saída 0. A preservação dos quatro cenários de defesa ao trocar personagem recebeu uma correção posterior a estas capturas, coberta pelo teste `showcase_page_navigation_preserves_all_defense_scenarios_and_playback_options` em [app.rs](../../../src/app.rs).

Na luta, a automação desativou as duas CPUs por Options, aproximou os lutadores e confirmou dano real com `T` do Player 1 e Backslash do Player 2. Também executou defesa e rasteira. Os pixels das caixas permaneceram ausentes durante acerto, bloqueio e queda quando debug estava desligado. Options habilitou caixas e limites e depois removeu ambos sem reiniciar a luta:

| Debug ligado | Debug desligado novamente |
|---|---|
| [Captura original](debug-on.png): 1.368 pixels verdes de hurtbox e 395 pixels na borda esquerda | [Captura original](debug-off.png): zero pixels dessas duas categorias |

O `result.json` conserva as contagens e confirmações individuais. As duas imagens são cópias integrais do framebuffer, sem tratamento. Não foi testado gamepad físico; o binding de RT foi apenas inspecionado no código. Reprodução de áudio também não foi avaliada.

Um ensaio inicial do harness enviou comandos antes de terminar o carregamento de assets e capturou imagens pretas. Suas comparações não eram válidas: a execução final passou a aguardar o título renderizado antes de enviar teclas. A fase de luta já tinha renderização ativa. A asserção de borda do harness também foi ajustada para a coluna real de rasterização OpenGL, x41, em vez de x42. Essas correções do ensaio não exigiram mudanças de produção; o JSON registra a limitação explicitamente.
