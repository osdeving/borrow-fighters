# Rust — reações por contato

32 desenhos novos em oito clips, produzidos por `image_gen.imagegen` em 2026-09-09.
Goggles, cabelo castanho espetado, moletom preto com detalhes laranja, calça cargo e ferramentas presas ao cinto.

Referência de continuidade: [arte existente](../idle/prepared.png).
As folhas superiores contêm cabeça, corpo, baixo e guarda alta; as inferiores
contêm guarda baixa, lançamento, queda e recuperação. Cada linha tem quatro
poses articuladas, começando pelo impacto.

- [Prompt superior](prompt-upper.txt) e [extração alpha](prompt-upper-alpha.txt).
- [Prompt inferior](prompt-lower.txt) e [extração alpha](prompt-lower-alpha.txt).
- [Gerações originais e aceitação](generation-records.json).
- [Fonte superior](upper-source.png) e [RGBA superior](upper-alpha.png).
- [Fonte inferior](lower-source.png) e [RGBA inferior](lower-alpha.png).
- [Revisão em escala do jogo](review-game-scale.png).
- [Auditoria da exportação](export-audit.json) e [baseline preservado](preserved-baseline.json).
- [Atlas runtime](../../../candidates/rust/rust-contact-reactions-atlas.png).

A [documentação comum](../../roster-reactions-2026-09-09/README.md) registra o método,
pivôs, escalas por sequência, reprodução e limites da revisão. Nenhum desenho foi
produzido apenas girando o sprite antigo. As imagens de revisão não são consumidas
pelo jogo; o manifest aponta para o atlas RGBA novo.

A primeira extração inferior continuou RGB: [tentativa descartada](lower-alpha-attempt.png).
O [prompt da segunda extração](prompt-lower-alpha-retry.txt) produziu o alpha aceito.
