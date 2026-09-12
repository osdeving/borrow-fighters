# Chegada cinematográfica da primeira EP

Captura nativa em 12/09/2026: **11/11 verificações aprovadas** no
[`native-checks.json`](native-checks.json). A revisão executa o binário de
aventura numa janela X11 própria, injeta comandos reais e lê a telemetria.

[Prévia da queda e do impacto](ep-arrival.mp4) ·
[corpo no céu](ep-01-sky-body.png) ·
[queda no enquadramento jogável](ep-04-gameplay-fall.png) ·
[contato no chão](ep-05-impact-and-dust.png) ·
[recuperação](ep-06-grounded-recovery.png).

O vídeo conserva imagens do renderer nativo. O áudio da prévia é **reconstruído
da telemetria e dos WAVs originais**, com o mesmo observador de marcos, pausa e
descarte; não é uma gravação do dispositivo de som. A revisão tocou os sons
nativamente, mas isso não mede latência do dispositivo nem aprovação auditiva.
Detalhes no [`mix-review.json`](mix-review.json).

Foram conferidos: câmera no céu antes do tumulto, bloqueio de movimento/pulo/
ataques durante a tomada, pausa de todos os relógios, retorno ao plano jogável
antes de tocar o chão, pose de apoio e reação dos moradores no mesmo tick,
trajetória contínua, entrega de combate com vida preservada, derrota real,
retry sem repetir as tomadas e reinício/avanço local sem inventar vitória.
A [telemetria da aterrissagem](landing-telemetry.jsonl) conserva o trecho usado
na verificação da continuidade. O vídeo inclui uma pausa deliberada de revisão.

Reprodução, da raiz do repositório:

```sh
cargo build --no-default-features --features adventure --bin borrow-adventure
cp target/debug/borrow-adventure /tmp/borrow-adventure-ep-snapshot
python3 tools/review/capture_ep_arrival_x11.py \
  --executable /tmp/borrow-adventure-ep-snapshot \
  --output-directory /tmp/ep-review --display :0
python3 tools/review/mix_adventure_review_audio.py /tmp/ep-review
```

A cópia temporária impede que um build paralelo substitua o executável durante
o controle de integridade da captura. A origem visual continua sendo o jogo.
Não foram gerados novos fundos ou corpos para a chegada: poses e cenário
aprovados foram reutilizados como assets independentes.
