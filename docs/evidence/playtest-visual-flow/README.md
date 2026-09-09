# Seleção Linker, energia e fluxo de partida

As imagens desta página vêm do renderer Raylib a 1280 × 720, com os assets do
jogo. Não são mockups. O [plano da entrega](../../26-playtest-visual-completion.md)
define o escopo; as reações dos lutadores têm uma
[revisão própria](../roster-contact-reactions/README.md).

## Seleção e movimento

[![Seleção Linker: Rust e Java](roster-rust-java.png)](roster-motion.mp4)

O [vídeo de seleção](roster-motion.mp4) contém 150 frames a 60 fps, sem
interpolação nem áudio. Mostra a abertura dos painéis, navegação, rotação da
prévia aleatória e confirmação. A captura determinística não exercita entrada
física: ela desenha os mesmos estados e assets usados pela aplicação.

- [Old C e Python](roster-old-c-python.png) e [Python e C++](roster-python-cpp.png):
  retratos, corpo inteiro animado e identificação dos dois jogadores.
- [Abertura](selection-state-005.png), [random](selection-state-065.png) e
  [escolhas confirmadas](selection-state-120.png).
- A revisão quadro a quadro observou abertura progressiva e escolha estabilizada
  após confirmar. O teste com os cinco manifests reais verifica escala e pivô
  constantes durante idle, nos dois sentidos, dentro dos painéis.

O [registro de UI](ui-review.json) identifica o código, backend gráfico e hashes
dos arquivos. Captura local com WSLg/Mesa D3D12 na GPU NVIDIA RTX 4070 Laptop;
as mesmas cenas também são capturadas pelo CI em `visual-review-linux`.

## Energia, pausa e resultado

- [Energia inicial](fight-energy.png): medidores ligados ao estado do combate.
- [Energia pronta](energy-ready.png): cinco contatos reais de C carregam a barra
  de 50 a 100; o exemplo exige esse valor antes de capturar.
- [Energia consumida](energy-spent.png): o cinematográfico aceito deixa a barra
  em zero, com o título abaixo do HUD. O exemplo verifica o custo no World.
- [Pausa](fight-pause.png): continuar, reiniciar, trocar personagens e menu.
- [Resultado](fight-result.png): revanche, seleção e menu abaixo dos lutadores.

Os testes de estado cobrem entrada neutra entre telas, confirmação dos dois
jogadores, random estável, vagas futuras, conservação de arena/confronto e
energia limitada ou livre nas ferramentas. A navegação na janela é registrada
separadamente: a [revisão nativa isolada](app-flow-ci/README.md) confirmou três
partidas, duas revanches, pausa/retomada, reinício, random, vagas futuras e
controle dos dois lados. Preserva as 31 capturas originais, entradas e hashes.
A [tentativa no desktop compartilhado](app-flow-local-attempt.md) ficou
inconclusiva e não conta como aceite; o teste isolado posterior passou.

## Áudio e limites

O [teste de pausa com dispositivo real](audio-pause.json), com
[saída integral](audio-pause.txt), passou em Raylib/miniaudio e PulseAudio/WSLg.
Verificou cursor da música, pausa de partida combinada com cinematográfico,
retomada, cancelamento, troca de faixa e sons de UI durante pausa. Usou sondas
silenciosas e música com volume zero; não constitui audição subjetiva.

Old C usa temporariamente as gravações de Rust autorizadas no pedido. A
[comparação antes/depois e procedência](../../../assets/audio/review/old-c-fallback-2026-09-09/README.md)
registra as fontes CC0 e a preservação dos outros personagens.

Teclado sintético, estado e geometria compartilhada não substituem um controle
físico. Não houve teste de gamepad físico nem aprovação humana de timbre nesta
execução. Balanceamento fino continua fora desta rodada.

## Verificações de código

O [registro dos checks](checks.json) resume fmt, Clippy estrito e 393 testes
aprovados, com a [saída dos testes](tests-output.txt.gz) preservada e seu hash.
Os dois testes de áudio que exigem dispositivo ficam fora da execução padrão;
a verificação nativa de pausa está registrada acima.
