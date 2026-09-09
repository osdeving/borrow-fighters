# Fluxo de três partidas — revisão do aplicativo no CI

**Sequência verificada nas imagens:** seleção, pausa/retomada, reinício,
três resultados e duas revanches. Os 31 PNGs originais foram abertos e
inspecionados, além da conferência de decodificação, dimensões e SHA-256.
As expectativas do harness não foram tratadas como prova dos estados.

Fonte: [execução 34394289582](https://github.com/osdeving/borrow-fighters/actions/runs/34394289582),
artefato `native-app-flow-linux` / `10121652772`, commit
`4f1631fad02fdf24184e8117c0a05332da843690`. Execução entre 19:34:56 e
19:39:35 UTC de 9 de setembro de 2026, em Linux/Raylib com Xvfb isolado.
O binário tem SHA-256
`42f8eb6afd6709d0db68e9eb23660333c897b49a14d56d2adff951919d7738d8`.

## Estados observados

| Capturas | Resultado da inspeção |
| --- | --- |
| [01 — luta inicial](screenshots/01-initial-fight.png) | Rust × Java em Sirius, vida cheia e energia 50/100 de cada lado. |
| [02 — pausa](screenshots/02-pause.png), [03 — espera](screenshots/03-pause-held.png), [04 — retomada](screenshots/04-resumed.png) | Continuar permanece selecionado durante a espera; poses visíveis e HUD preservados. Após Enter, a mesma luta avança sem o painel de pausa. |
| [05 — reiniciar](screenshots/05-restart-highlight.png), [06 — entrada reiniciada](screenshots/06-restarted-entry.png) | Rust estava com 90/100 de vida e 58/100 de energia; a nova entrada restaura vida cheia e energia 50/100. Java e Sirius são preservados. |
| [07 — elenco](screenshots/07-roster.png) | Cinco personagens disponíveis, Random e duas vagas futuras. |
| [08 — vaga futura](screenshots/08-future-slot.png), [09 — confirmação recusada](screenshots/09-future-rejected.png) | A vaga permanece indisponível, sem confirmação nem início de partida. |
| [10 — Random](screenshots/10-random-preview.png), [11 — resolvido](screenshots/11-random-confirmed.png), [12 — estável](screenshots/12-random-stable.png) | A prévia mostra Old C; a confirmação resolve Duke, que permanece confirmado enquanto a pose de espera muda. |
| [13 — ambos confirmados](screenshots/13-both-confirmed.png) | Duke × Old C, ambos prontos, botão Lutar habilitado. |
| [14 — modo local](screenshots/14-local-mode.png), [15 — local confirmado](screenshots/15-local-both-confirmed.png) | Trocar o modo apaga ambas as confirmações; após F e Enter registrados, C++ e Old C aparecem confirmados em Duelo Local. |
| [16 — demo pronta](screenshots/16-demo-ready.png), [17 — entrada](screenshots/17-demo-entry.png) | CPU × CPU, Rust × Old C e Tech Coast Beacon/Fortaleza. A entrada usa os mesmos personagens e arena. |

## Três partidas e duas revanches

Todas as partidas terminam com **Rust venceu**, Rust com 95/100 de vida e
Old C com 0/104. O resultado apresenta Revanche, Trocar personagens e Menu.
A IA determinística repete esse resultado nas três partidas.

| Partida | Pausa e retomada | Resultado efetivo | Revanche |
| --- | --- | --- | --- |
| 1 | [Pausa](screenshots/18-match-1-pause.png) · [Retomada](screenshots/18-match-1-resumed.png) | [Resultado](screenshots/19-match-1-result-candidate.png) | [Nova entrada](screenshots/20-rematch-1-entry.png) |
| 2 | [Pausa](screenshots/18-match-2-pause.png) · [Retomada](screenshots/18-match-2-resumed.png) | [Resultado](screenshots/19-match-2-result-candidate.png) | [Nova entrada](screenshots/20-rematch-2-entry.png) |
| 3 | [Pausa](screenshots/18-match-3-pause.png) · [Retomada](screenshots/18-match-3-resumed.png) | [Resultado](screenshots/19-match-3-result-candidate.png) | Não solicitada após a terceira partida. |

As duas revanches preservam Rust, Old C e Tech Coast Beacon e restauram vida
100/100 e 104/104, com energia 50/100 para ambos. Cada retomada mostra a
contagem chegando a Fight, sem voltar à seleção ou trocar o confronto.
As capturas intermediárias [1](screenshots/18-match-1-mid.png),
[2](screenshots/18-match-2-mid.png) e [3](screenshots/18-match-3-mid.png)
já mostram o resultado concluído.

A revelação da entrada cobre temporariamente as bordas do HUD em 17 e nas
duas imagens de revanche. As três capturas de retomada mostram o HUD completo;
não foi observado recorte persistente depois da transição.

## Alcance e rastreabilidade

O harness criou o processo 5790, enviou teclado simulado por `XSendEvent`
apenas à janela cujo `_NET_WM_PID` correspondia ao processo e capturou seu
framebuffer com F12. Terminou sem erros X11, com código 0. Não houve ensaio
de mouse ou gamepad físicos. As capturas do modo local antes/depois e o log
demonstram a sequência F/Enter; não registram uma imagem entre essas duas teclas.

**Áudio não validado nesta execução:** o runner não tinha dispositivo ALSA,
e o jogo registrou `audio disabled`. Não há audição nem comprovação de pausa
sonora por esse artefato. As imagens também não comprovam todos os frames
intermediários da entrada. Esta revisão identifica explicitamente seu commit;
não afirma ter executado alterações de código posteriores.

- [Revisão por captura, hashes e horários](visual-review.json).
- [Metadados originais do artefato](artifact-metadata.json).
- [Eventos originais](events.jsonl) e [resumo original do harness](summary.json).
- [Log do jogo](stdout.log) e [limitação ALSA](stderr.log).
- [Tentativa anterior no desktop compartilhado](../app-flow-local-attempt.md),
  mantida como inconclusiva; sua divergência não ocorreu nesta sequência.

Os PNGs e logs foram preservados sem retoque ou recompressão. Caminhos
absolutos no resumo original identificam o runner; as cópias estão em
`screenshots/`, com os mesmos nomes e hashes.
