# Revisão de navegação após retomada — 10/09/2026

O [pedido](../../../29-story-terminal-menu.md) foi recuperado do histórico local
após o reinício da máquina. A implementação anterior foi preservada e integrada:
textos dos controles, confirmação final, avanço por trecho e skip total.

## Resultado na janela

- [Prompt final](screenshots/held-enter-completion.png): mantém título/subtítulo
  e pede uma nova tecla, clique ou botão. Segurar Enter por três segundos e
  soltá-lo conserva a espera; um novo Enter ou clique confirma.
- [Menu após confirmação](screenshots/02-main-menu.png): mesma janela e PID,
  Modo História reservado e demais destinos preservados.
- [Pausa](screenshots/paused-skip-controls.png): pular tudo abre o menu também
  durante o combate pausado.
- [Ajuda de Ada](ada-controls/screenshots/ada-skip-controls.png): revelar a mensagem,
  avançar trecho, pular tudo e pausar permanecem visíveis no rodapé. A captura
  complementar passou [oito checks](ada-controls/native-checks.json).
- [Vídeo nativo](title-to-menu-native.mp4): final, confirmação e navegação pelos
  destinos do menu. Captura em 10 fps, sem áudio, sem editar os pixels.

Os [81 checks X11](native-checks.json) passaram, incluindo os segmentos de Ada,
manhã e apresentação; skip do encontro sem inventar vitória/pesar; skip total
em Ada, encontro ativo, pausa e apresentação; fechamento/limite de frames;
janela oculta voltando visível; e revisão determinística completando a espera
de 180 frames mais 24 de fade. Catálogo e binário conservaram seus hashes
durante toda a rodada. [Eventos](events.jsonl) e [amostras da janela](window-frames.jsonl)
registram as entradas e a transição. Os resultados de cada sessão estão em `sessions/`.

A [revisão visual](visual-review.json) lista os PNGs efetivamente conferidos.
O relatório automático mantém seus campos originais de revisão pendente;
não é usado como substituto da inspeção visual.

## Verificações de código e isolamento

Os [resultados Rust](rust-checks.json) registram Fmt, Clippy com warnings como
erro e a matriz: 432 testes com ambos os modos, 395 somente luta,
38 aventura/core e 3 somente core. Fronteiras e suas 54 fixtures passaram com
Python 3.13; o Python 3.8 padrão do host não atende ao checker.
Os [seis checks da aventura isolada](standalone-checks.json) confirmam conclusão
local por skip, replay da apresentação, skip durante pausa, retry do encontro,
skip individual do encontro e saída limpa.

Os 68 valores preexistentes no catálogo do usuário foram preservados. Foram
acrescentadas oito entradas `navigation.*`, validadas ao carregar/recarregar.

## Reproduzir

```sh
cargo build --all-features --bins
python3.13 tools/review/capture_story_menu_x11.py \
  --executable target/debug/borrow-story \
  --output-directory /tmp/borrow-story-navigation-new
```

O diretório deve ser novo. Os eventos sintéticos são enviados apenas à janela
do processo filho. Não houve teste de controle físico nem avaliação auditiva;
os logs verificam abertura e encerramento dos dispositivos sem falha.
O [diário](../../../worklogs/rust-adventure-prologue.md) guarda o prompt
recuperado e as instruções de retomada.
