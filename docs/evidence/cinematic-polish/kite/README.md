# Pipa, menino e vida cotidiana — revisão nativa

Captura em 12/09/2026 do renderer Raylib a 1280×720, com a rua de 4608 px
e os assets modulares atuais. A tomada estabelece a rotina antes da ameaça:
pipa até 3,5 s; menino/calçada aos 6 s; moradores, caramelo, bicicletas e
trânsito perto da mercearia até 13 s; fade cobrindo a troca de tomada;
Rust reaparece aos 15 s e recebe controle aos 18 s.

[Vídeo contínuo — 18,60 s](kite-establishment.mp4) ·
[Trilho usado](camera-track.json) · [Telemetria](preview-telemetry.jsonl)

| Quadro extraído do vídeo nativo | Momento observado |
| --- | --- |
| [Pipa](kite-01-kite.png) | Close mantido, com movimento da linha e da pipa. |
| [Menino](kite-02-child.png) | A câmera desce sem apressar a brincadeira; carro e bicicleta continuam passando. |
| [Vizinhos](kite-03-neighbours.png) | Comércio aberto, lojista, cliente e cachorro em rotina normal. |
| [Vida cotidiana](kite-04-ordinary-life.png) | Ônibus e ciclista atravessam naturalmente a composição. |
| [Rust](kite-05-rust.png) | Enquadramento após o corte coberto, ainda sem controles. |
| [Gameplay](kite-06-gameplay.png) | Câmera final coincide com a exploração, sem EP antecipada. |

Os [6 checks da prévia](preview-checks.json) passaram: cadência observada de
18,049 s para 18 s de simulação, ambiente vivo durante o bloqueio do combate,
entrega exata de câmera, vídeo válido e integridade do binário/textos.
Os [9 checks funcionais](functional-checks.json) passaram separadamente,
incluindo inputs antecipados, pausa durante o fade, avanço local e restart.
O teste funcional usa comandos de teclado enviados somente à janela do processo.
O vídeo contínuo não contém pausas nem pedidos de screenshot; os PNGs acima
foram extraídos posteriormente para não interromper sua cadência.

A troca de âncora ocorre nos ticks 840–841 sob preto completo. Os testes
unitários verificam que qualquer mudança descoberta é rejeitada, limitam
deslocamento e zoom entre quadros visíveis, e confirmam que a duração
externa também governa bloqueio/liberação e avanço. Passaram os 5 testes
de arrival, os 16 de Story e os 4 de landscape. A queda rápida da EP não
foi alterada nesta revisão.

O som do vídeo foi **reconstruído offline** usando os WAVs originais de ar
e trânsito e os estados registrados; [relatório da mixagem](mix-review.json).
Não é captura do dispositivo de áudio. As imagens e tempos são da execução
nativa; não há validação de controle físico nem aprovação visual humana.

Binário de referência: SHA-256
`0e6c0f1495423d52fc59b53ec737502d82a1fd1d67986477970237daf2331869`.
O gatilho de screenshot do modo de revisão foi movido de 180 para 570 após
esta captura; essa mudança não afeta `--capture` nem o renderer filmado.

Reprodução a partir do checkout:

```sh
cargo build --no-default-features --features adventure --bin borrow-adventure
python3.13 tools/review/capture_kite_arrival_x11.py --output-directory /tmp/kite-preview --preview-only --mute
python3.13 tools/review/mix_adventure_review_audio.py /tmp/kite-preview
python3.13 tools/review/capture_kite_arrival_x11.py --output-directory /tmp/kite-functional --mute
```

O [roteiro](../../../../tools/review/capture_kite_arrival_x11.py) preserva os
assets originais. [Especificação da cena](../../../32-cinematic-neighbourhood-arrival.md).
