# Projétil Duke — substituição localizada

`source-existing.png` preserva o projétil anterior, com halo branco e clarão amarelo cortado na borda esquerda. A extração de alpha isolada não resolveria o corte. `source-v1.png` é uma nova imagem produzida pelo `image_gen` integrado, usando o projétil anterior e a referência fixa do Duke; o pedido completo está em [prompt-v1.txt](prompt-v1.txt).

A nova textura mantém grãos de café marrons, contornos escuros e rastros creme. São nove grãos legíveis; a dispersão horizontal é mais compacta que a anterior. Não há clarão colado ao personagem nem recorte lateral.

A ferramenta entregou RGB com quadriculado apesar do pedido de alpha. O processamento local autorizado usou:

```sh
python3 tools/art/remove_generated_checkerboard.py assets/production/duke/projectile/source-v1.png assets/production/duke/projectile/keyed-v1.png
python3 assets/production/duke/projectile/prepare.py
```

O segundo comando aplica um único fator uniforme `116/817 = 0.1419828641` ao conteúdo e o centraliza numa textura de 158×132, com quatro pixels de margem simétrica adicionais ao canvas anterior de 150×124. O conteúdo ocupa 106×116; em runtime, com o fator fixo `0.6`, ocupa aproximadamente 63,6×69,6. Nenhuma origem, dimensão física, colisão ou duração foi alterada.

Ver [metadados reproduzíveis](production.json), [comparação de alpha em fundos claro/escuro](alpha-review-v1.png) e [captura real após disparo](../finalization-evidence/lab-special-f018-idle.png). Esta entrega utiliza **nova geração**.
