# C — caminhada selecionada na continuação

Foi selecionada a folha [source-v3.png](source-v3.png), gerada pela ferramenta
integrada de imagem a partir da referência original fixa e da folha v2. O
[prompt](prompt-v3.txt) pede a inversão explícita da sobreposição das coxas,
mantendo rosto, cabelo, livro e roupa. A fonte anterior e o mapeamento original
permanecem preservados.

A sequência usa quatro corpos completos: contato da perna próxima, saída do pé
distante do chão, contato da perna distante e passagem da perna próxima. O
segundo quadro mostra o calcanhar distante levantado atrás do apoio; ele é uma
chave de saída do pé, não foi declarado falsamente como passagem à frente. Os
contatos têm a coxa próxima indo em direções opostas. A elevação exagerada do
joelho da primeira produção foi removida.

V2 repetia a mesma perna nos quatro desenhos. V4 e v5 mudaram a perna de apoio
ao tentar mover o pé distante para a frente; v6 retornou outro contato e mudou
a guarda. Essas alternativas foram rejeitadas. Nenhum membro foi montado a
partir de imagens diferentes.

O [action.json](action.json) contém quatro retângulos explícitos de 512 × 768,
pivôs X292/276/255/200 e Y701/696/695/697, definidos pelo eixo da cintura e pela
sola de apoio. A escala uniforme anterior `268/630` e os quatro tempos de
100 ms foram preservados. O topo da figura varia aproximadamente 268–270 px
acima do chão; não há compensação no corpo físico ou velocidade.

O magenta foi removido por `tools/art/remove_generated_magenta.py`, sem a opção
específica do Rust. A fonte RGB está intacta. Foram inspecionados o
[alpha sobre fundos claro e escuro](alpha-review-v3.png), os
[quadros na escala de runtime](review/walk-frames.png) e as duas orientações no
[GIF de diagnóstico](review/walk.gif). Livro, dedos, cabelo e sapatos estão
inteiros, sem resíduos evidentes nesses previews.

A integração e a revisão temporal no World são registradas no laudo final de C.
Esta seleção estática não substitui essa conferência funcional.
