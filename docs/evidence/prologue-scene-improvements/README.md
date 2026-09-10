# Quarto e rua de Rust — primeira rodada

Experimento da [entrega 30](../../30-prologue-scene-improvements.md) na branch
`feature/prologue-scene-improvements`, em 10 de setembro de 2026.

[Vídeo da manhã e da rua](morning-and-street.mp4), 59,334 segundos de execução
nativa capturada, sem áudio. O vídeo inclui os comandos de revisão e pausas;
não representa uma sequência cinematográfica com tempo editado. O último
ajuste de altura do texto do pôster foi conferido numa captura curta posterior;
as três imagens 090/390/650 do quarto abaixo já mostram esse ajuste.

| Momento | Imagem |
|---|---|
| Rust deitado; setup e luz matinal | [Quarto — início](screens/morning-090.png) |
| Rust sentado; pôsteres e terminal | [Quarto — sentado](screens/morning-390.png) |
| Rust em pé, apoiado no chão | [Quarto — em pé](screens/morning-650.png) |
| Dois ciclistas e garoto na calçada | [Rua tranquila](screens/street-02-calm-later.png) |
| Aproximação antes da ameaça | [Antes da EP](screens/street-04-before-ep.png) |
| Garoto percebe a errática | [Susto](screens/street-05-boy-startle.png) |
| Mão aberta e linha livre | [Soltura da pipa](screens/street-06-release-video.png) |
| Garoto foge e a pipa segue pelo vento | [Corrida](screens/street-06-run.png) |
| Limite direito da câmera | [Fim da panorâmica](screens/street-09-right-camera-clamp.png) |

O quarto conserva os apoios da manhã. A rua separa garoto (calçada, y420),
ciclistas (y467/480) e Rust (y580), com canteiro entre ciclovia e faixa jogável.
Os figurantes acompanham o parallax do cenário, sem corpos, dano ou colisão.
A errática aparece quando `enemy_awake` dispara o susto do garoto.

## Resultados

- [Matriz Rust](rust-checks.json): 437 testes conjuntos, 43 aventura, 395 luta,
  3 core; Fmt e Clippy com warnings negados.
- 56 fixtures de fronteira e dez testes de empacotamento aprovados.
- [15 checks nativos](native-checks.json), [seis checks da encenação](ambient-checks.json),
  [cinco de entrada direta/câmera](direct-entry-checks.json) e
  [cinco da revisão final do quarto](room-checks.json): 31 no total.
- Pausa conserva relógios e posições, inclusive durante a fuga. Retry inicia
  o susto no checkpoint; restart restaura a rua calma; skip não fabrica vitória.
- Staging com 227 assets verificado, inclusive inicialização de RustMorning
  fora do checkout. Os 76 textos existentes foram preservados, com quatro
  novas chaves apenas para os pôsteres e o terminal.
- [Resumo](verification.json), [telemetria](telemetry.jsonl),
  [eventos de entrada](native-events.jsonl) e [resultado da sessão](result.json).

## Reproduzir

```sh
cargo build --locked --bin borrow-adventure --bin borrow-story
python3.13 tools/review/capture_prologue_scenes_x11.py \
  --output-directory /tmp/prologue-scene-review
cargo run -- --start morning
cargo run -- --start encounter
```

O script reutiliza o harness X11 existente, envia comandos somente à janela
do próprio processo e requer ffmpeg. Grava a execução em tempo real, observando
a telemetria; não depende de xdotool. As rotinas de câmera/entrada e a última
captura do quarto complementaram a sessão principal.

As artes foram produzidas pelo `image_gen` integrado e mantêm procedência e
[prompts completos](../../../assets/adventure/ART-PROVENANCE.md). São candidatas
para este experimento. A revisão do agente confirma composição e comportamento;
avaliação humana de fluidez, controles físicos e som continua no playtest.
