# Julia retida — pintura pareada e contato

O cafetão e Julia usam uma pintura conjunta enquanto o braço dela está preso.
Ele fica à esquerda, voltado para C++ fora do quadro; Julia permanece atrás,
à direita, com expressão assustada. Cada personagem tem dois braços, e a mão
dele envolve o antebraço dela. A mesma imagem e malha aparecem na câmera de
gameplay e no plano de profundidade 8 da cinematografia.

As capturas nativas em 3,1x verificam [repouso sobre azul](rest-blue.png) e
[tentativa de saída sobre laranja](pull-orange.png). O [relatório de chroma](chroma-review.json)
registra zero pixels com excesso verde acima de 9/255 nas duas composições.
A inspeção visual verificou contorno de cabelo, punhos, mangas, sapatos, bolsa,
escala e contato. Não há checkerboard, fundo verde ou membros adicionais.

Os [prompts e a proveniência](../../../../assets/adventure/chapters/cpp-augusta/source/paired-restraint.provenance.json)
registram o uso do `image_gen` integrado e o hash da pintura escolhida.
Pedidos iniciais de alpha produziram checkerboards pintados e foram rejeitados.
A arte final usa chroma verde; o shader remove esse fundo somente no desenho,
preservando o PNG original e limpando a contaminação nas bordas filtradas.
O carregador exige a peça `restrained-pair` e um shader válido com o uniforme
próprio, para uma recarga incompleta não fazer o casal desaparecer.

O ponto de pegada acompanha o socket da história. Julia tenta avançar 22px,
e o cafetão acompanha 12px para não comprimir os antebraços pintados. Pés e
pernas usam recortes de malha separados pela área transparente. A respiração
e a inclinação acompanham a tensão; todos os deslocamentos retornam a zero.
A escala preserva aproximadamente 194px para ele e 174px para Julia.

```sh
cargo test --no-default-features --features adventure --lib \
  adventure::engine::production::restraint::tests
```

A regressão verifica o contato fixo e a ausência de inversão da malha nos
antebraços durante 33 amostras da tentativa. O teste de encenação do capítulo
confere também ordem, alcance dos braços e retorno das posições. A suíte da
aventura passou com 199 testes na integração desta peça; a verificação final
do capítulo completa a revisão da câmera 3D e do restante da história.
