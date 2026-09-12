# Corridinha de Rust

[Vídeo nativo de oito segundos](rust-run-native.mp4) ·
[Doze amostras do ciclo interpolado](run-poses-native.png).

O vídeo usa o renderer Raylib, os assets atuais e a simulação real de exploração
em 1280×720, 30 fps. Mostra arrancada, corrida para os dois lados, freio e
retomada na rua modular. É uma revisão visual sem áudio, com entradas
determinísticas do harness; não simula teclas do sistema nem uma partida completa.

Rust corre a 455 px/s. A aceleração alcança essa velocidade em nove ticks
(0,15 s); o freio leva oito ticks ou menos e percorre menos de 30 px. Ao inverter
a direção, a orientação acompanha o deslocamento durante a frenagem. Aproximações
de NPCs, quarto e pesar continuam usando caminhada cuidadosa.

A corrida combina nove PNGs independentes, sockets anatômicos e chaves externas.
Um ciclo percorre 272 px e alterna dois apoios com suspensão entre eles. A sola
avança para trás na mesma distância que o personagem percorre para frente,
preservando o contato durante aceleração. Os joelhos são resolvidos a partir
dos comprimentos da coxa/canela; o tronco comprime no apoio e os braços fazem
contrabalanço. A revisão reduziu o recolhimento do pé e a amplitude dos braços
por JSON, deixando a bota e a canela traseiras legíveis. As pernas ainda se
sobrepõem brevemente ao cruzar, como aparece na folha; a técnica usa recortes
com sobreposição, sem deformação contínua do tecido.

Os 12 PNGs anteriores de caminhada/chute foram preservados. A [procedência da
arte](../../../../assets/adventure/locomotion/README.md) inclui a fonte do rig,
o prompt e o script de extração. Configuração e imagens recarregam por F5;
os sons do capítulo seguem o comprimento de cada marcha e não reiniciam no ar.

Verificações: 129 testes da biblioteca adventure, `cargo fmt --all` e
`cargo clippy --all-targets --all-features -- -D warnings`. O teste físico
percorre mil fases e verifica apoios alternados, suspensão, pés acima do chão
e comprimentos de membros; os testes de movimento verificam velocidade,
freio, inversão e troca de marcha. Os testes de áudio verificam contatos por
distância, incluindo retomada durante suspensão.

Reproduzir em uma sessão gráfica com Rust, Raylib e FFmpeg disponíveis:

```sh
python3 docs/evidence/modular-running-world/gait/capture.py
```

`--poses-only` atualiza somente a folha. O [harness nativo](native.rs) é isolado
da entrada normal do jogo e não modifica saves.
