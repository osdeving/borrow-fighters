# Carga, chute e duas EPs

Capturas do renderer nativo de aventura, usando os mesmos dados de mundo,
atores e comandos do jogo. A [revisão completa](review.json) atravessou o
capítulo por comandos públicos até `Complete`: todas as caixas quebradas,
duas EPs derrotadas e checkpoint final preservado. A captura completa foi
silenciosa; não é evidência de áudio.

- [Carga intacta](cargo-intact.png): peças vermelhas e azuis, tábuas soltas e apoio físico.
- [Chute e destruição](kick-and-debris.png): bota estendida, fragmentos e poeira de contato.
- [Caminho aberto](cargo-cleared.png): remoção dos sólidos e sobras independentes no chão.
- [Duas EPs](two-eps.png): corpos, intenções e duas barras de vida.

Os três quadros da carga foram refeitos após ajustar recortes, volumes de apoio
e inclinações; não houve regravação dos pixels dos assets. O material de produção
permanece em `assets/adventure/street/props.png`. As imagens usam o quadro real
renderizado e não montagens ilustrativas.

Reprodução completa sem carregar o save pessoal:

```sh
cargo run --bin borrow-adventure -- --start chapter --review /tmp/borrow-chapter-review
```

O runner usa `review-campaign.json` dentro do diretório da captura. Para verificar
entradas de teclado e som nativo, `tools/review/capture_chapter_x11.py` acompanha
agora os objetos por ID, socos/chute e o inimigo vivo mais próximo.

Verificações específicas: janela ativa do chute e alcance, contato consumido
uma vez, pilha bloqueando pulos antes da demolição, caixas caindo ao perder o
apoio, retry restaurando ambas as EPs, vitória apenas após ambas perderem vida,
objetos/fontes/textos incluídos no pacote e caminho automático até o fim.
