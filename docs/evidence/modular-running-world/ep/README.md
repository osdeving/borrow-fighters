# Queda rápida da primeira EP

[Prévia contínua](ep-fast-arrival.mp4) ·
[aproximação no céu](ep-01-sky-body.png) ·
[última queda no plano jogável](ep-04-gameplay-fall.png) ·
[impacto no chão](ep-05-impact-and-dust.png).

A EP se aproxima pequena, cresce até a escala de gameplay e acelera durante
toda a queda: 320 a 777 px/s, em 58 ticks (0,97 s). A câmera termina de abrir
no tick 40; os últimos 212 px caem em 18 ticks antes do apoio de mão/joelho.
Rastros acompanham a velocidade real, e o contato dispara poeira, fragmentos,
tremor e tumulto no mesmo marco do relógio.

**11/11 verificações nativas aprovadas**, incluindo comandos antecipados,
pausa, continuidade, contato/reação, recuperação, derrota/retry e skip local.
O [relatório completo](native-checks.json) registra essa revisão focal no
cenário anterior à ampliação do mapa. O movimento recebe a posição do inimigo
e o deslocamento da câmera; não armazena X absoluto no roteiro.

A prévia foi registrada numa segunda execução sem F12 ou pausa durante a
queda: **0,954 s reais para 0,950 s de simulação observada**. O
[relatório da prévia](preview-checks.json) confere a cadência, e a
[telemetria](landing-telemetry.jsonl) preserva os pontos de câmera/corpo.
O vídeo usa imagens nativas. Seu áudio é reconstruído dos WAVs e da telemetria,
conforme o [relatório do mixer](mix-review.json); não é gravação do dispositivo.

```sh
cargo build --no-default-features --features adventure --bin borrow-adventure
cp target/debug/borrow-adventure /tmp/borrow-adventure-fast-ep
python3 tools/review/capture_ep_arrival_x11.py \
  --executable /tmp/borrow-adventure-fast-ep \
  --output-directory /tmp/ep-checks --display :0
python3 tools/review/capture_ep_arrival_x11.py \
  --executable /tmp/borrow-adventure-fast-ep \
  --output-directory /tmp/ep-preview --display :0 --preview-only
python3 tools/review/mix_adventure_review_audio.py /tmp/ep-preview
```

O binário temporário impede substituição por builds paralelos durante a captura.
Os mesmos sprites foram reutilizados; configuração, perspectiva, trajetória e
efeitos permanecem editáveis em `assets/adventure/street/ep-arrival.json`.
