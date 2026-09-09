# Tentativa local de navegação — inconclusiva

Execução em 9 de setembro de 2026, 19:20:44–19:23:09 UTC, com o binário Linux
otimizado da revisão `62e947c`, SHA-256
`cfde9dbdc0fb3b40c56742dc83cb81e3460b48df181eabbbddc34f8438172b7c`.
O processo recebeu `--fight`, assets do repositório e dados de usuário isolados
em `/tmp/borrow-fighters-native-app-flow`, no display WSLg `:0`.

O [harness](../../../tools/review/capture_match_flow_x11.py) criou o processo
49267, conferiu `_NET_WM_PID` antes de enviar cada tecla por `XSendEvent` e
capturou apenas seu framebuffer com F12. Nenhum processo Windows existente,
foco global ou configuração do desktop foi alterado.

## Divergência observada

| Captura original | Estado observado na imagem |
| --- | --- |
| [02 — pausa](app-flow-local-02-pause.png) | Rust × Java, vida cheia, opção **Continuar** selecionada. |
| [03 — pausa após espera](app-flow-local-03-pause-held.png) | Mesma luta, mas **Menu** selecionado, embora o roteiro só tenha esperado e enviado F12 entre as capturas. |
| [04 — após Enter](app-flow-local-04-resumed.png) | Seleção Linker com Rust e Old C já confirmados; não é a luta retomada esperada. |

As imagens posteriores chegaram a uma luta CPU × CPU de Rust contra Python
em Tech Coast Beacon, um resultado e uma revanche. Isso não valida a sequência
solicitada: a divergência inicial impede atribuir todas as transições às entradas
registradas, e a execução foi interrompida antes de completar três partidas.

**Causa não confirmada.** A consulta somente de leitura a `XListInputDevices`
mostrou ponteiro e teclado XWayland no display compartilhado. `/dev/input`
estava ausente e `/sys/class/input` vazio; os logs do jogo não registraram
gamepad. Entrada externa é uma hipótese, não uma conclusão. Não houve leitura
de eventos físicos, interação com dispositivos, teste de gamepad ou audição.

## Encerramento e rastreabilidade

Após identificar a divergência, SIGINT foi enviado somente ao harness próprio
49262, com sua linha de comando conferida. Seu cleanup enviou
`WM_DELETE_WINDOW` ao processo 49267, que terminou com código 0. O harness
terminou com 130 por interrupção; ambos os PIDs deixaram de existir.

- [Eventos brutos](app-flow-local-events.jsonl): entradas, horários e hashes.
- [Resumo bruto](app-flow-local-summary.json): `success: false`, sem erros X11;
  os caminhos absolutos dos demais PNGs apontam para a tentativa temporária.
- Os três PNGs acima foram copiados sem modificação. Os demais PNGs desta
  tentativa não integram evidência de aceite.

A sequência completa deve ser revisada novamente no artefato
`native-app-flow-linux` do CI, que usa Xvfb isolado. Esta tentativa não comprova
três partidas, teste físico de controles nem qualidade sonora.
