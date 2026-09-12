# Regressão da chegada da EP

O binário final desta revisão passou nas onze verificações nativas do
[roteiro de captura](../../../../tools/review/capture_ep_arrival_x11.py).
[Resultados](native-checks.json).

Conferidos: bloqueio de input durante a descida, pausa de todos os relógios,
retorno da câmera enquanto a EP ainda está no ar, impacto iniciando o tumulto,
recuperação, continuidade da trajetória, retry e avanço seguro. A descida
permanece com 58 ticks; a ampliação da sequência da pipa não a desacelera.

[EP ainda no ar](airborne.png) · [Contato e poeira](impact.png).

Reprodução em sessão gráfica:

```sh
cargo build --bin borrow-adventure
python3.13 tools/review/capture_ep_arrival_x11.py --executable target/debug/borrow-adventure --output-directory /tmp/cinematic-polish-ep --mute
```

A captura usa eventos de teclado X11 na janela pertencente ao processo de
teste. O SHA do binário final está no [relatório geral](../verification.json).
