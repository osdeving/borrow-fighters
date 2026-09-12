# Caminhada e chute de Rust

[Prévia nativa de 7 segundos](rust-walk-native.mp4), com saída do quarto e
caminhada nas duas direções na rua. O renderer Raylib foi amostrado a partir
dos relógios de cena e da simulação de exploração; saída em 1280×720, 30 fps.
Este recorte é uma verificação visual sem áudio e não representa entrada física
de controle. A revisão integrada do capítulo cobre a partida completa.

[Oito apoios de caminhada](walk-poses-native.png) ·
[Preparação, extensão e recuperação do chute](kick-poses-native.png) ·
[Caminho pela frente da cama](morning-740.png).

Os apoios, caminho de saída, comprimento de passada e clips são externos.
Distância parada não avança a caminhada. Percursos em profundidade normalizam
cada novo passo, conservando a fase anterior; diminuir o personagem não
recalcula toda a distância histórica. A extensão visual do chute coincide com
os ticks 10–16 do contato físico.

A captura confirmou altura aparente próxima à pose original: cerca de 170 px
em escala de gameplay, sem halo magenta ou fragmentos das linhas vizinhas do
atlas. Os PNGs continuam sendo desenhos discretos; deslocamento, câmera e
assentamento em torno dos apoios são interpolados. Não há deformação esquelética.

Verificação: `cargo fmt --all`, 119 testes da biblioteca adventure e
`cargo clippy --all-targets --all-features -- -D warnings` passaram após o
ajuste final do relógio de passada.
