# Proporções e travessia de Rust

[Comparação idle/corrida na mesma escala](idle-run-native.png) ·
[Poses de costas e de frente](crossing-poses-native.png) ·
[Travessias nativas, 18,73 segundos](crossing-native.mp4).

A roupa da corrida ganhou volume transversal: `thigh_width=1.28` e
`shin_width=1.25` ajustam coxa e canela sem alterar o comprimento dos ossos.
As botas aumentaram 16%; seus sockets de sola continuam determinando o contato.
A folha compara a pose parada original com quatro fases da corrida em escala
1,7, sob a mesma câmera. Velocidade, aceleração, freio, stride e física permanecem
iguais. Os bolsos/calças e botas têm volume mais próximo do corpo parado.

Cada segmento de aproximação escolhe sua vista pela direção em profundidade:
costas para ir ao passeio distante, frente para voltar e lateral nos segmentos
horizontais. São 16 PNGs novos independentes, com [fonte, prompt e extração
preservados](../../../../assets/adventure/locomotion/README.md). A caminhada usa
poses desenhadas por distância percorrida; posição, câmera e escala em
profundidade continuam contínuas. A orientação muda na virada do caminho e
retoma o corpo de diálogo ao chegar, sem relocar o personagem.

[Ida ao motorista](DriverApproach-Back.png) ·
[Retorno do motorista](DriverReturn-Front.png) ·
[Ida à mercearia](ShopApproach-Back.png) ·
[Retorno da mercearia](ShopReturn-Front.png) ·
[Ida ao vizinho](NeighbourApproach-Back.png) ·
[Retorno do vizinho](NeighbourReturn-Front.png).

O vídeo usa renderer Raylib e comandos públicos do capítulo: corre até cada
região, interage, atravessa a rua, conversa e retorna. O harness avança falas
após um segundo para concentrar a revisão nas rotas. A introdução é pulada pelo
comando público. Não altera posições, mapas, triggers ou save. O vídeo não tem
áudio e não representa teclado/controle físico.

Nos 562 frames, houve 80 amostras de costas e 80 de frente, sem orientação
contrária ao deslocamento vertical. A maior mudança de posição entre frames
foi 15,17 px, correspondente à corrida de 455 px/s amostrada em 30 fps. O
[resumo](summary.json) e a [telemetria](crossing.jsonl) registram as transições.

Validação: 135 testes adventure, `cargo fmt --all`, Clippy de todos os targets e
features com `-D warnings`, e `git diff --check`. O teste novo percorre a ida e
a volta da rota real da mercearia, confirma vistas e conserva a posição final;
os testes do rig continuam verificando apoios e comprimentos em mil fases.

Reprodução em uma sessão gráfica com Rust e FFmpeg:

```sh
python3 docs/evidence/cinematic-polish/gait/capture.py
```

`--poses-only` gera somente as folhas. O [harness](native.rs) fica fora da
entrada normal do jogo.
