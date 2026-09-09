# Go / Gopher — referência principal preservada

Este arquivo é um **recorte reaproveitado do asset existente**, sem nova geração, redesign, ampliação ou aprovação como arte final.

- Fonte: `assets/placeholder/go-fighter-atlas.png`.
- Recorte explícito (left, top, right, bottom): `(0, 0, 384, 256)`.
- Quadro: primeiro idle; Go usa `idle_0` do atlas runtime na proporção corrigida.
- [Master preservado](master-existing.png).

## Identidade

Corpo azul/ciano estreito, ventre azul claro, orelhas arredondadas, olhos grandes brancos/pupilas pretas, focinho creme e dois dentes. Luvas escuras, faixa escura na cintura e pés claros.

## Paleta e acabamento

Ciano/azul, azul pálido, creme, preto/carvão. Manter contorno escuro legível, luz consistente e sombreamento coerente com a referência; melhorar recorte sem redesenhar rosto/roupa.

## Proporção

Usar esta proporção estreita já corrigida, não a silhueta baixa/larga da fonte original. Runtime scale1.44; corpo físico humanoide padrão não muda.

## Limites da referência

Recorte exato de idle_0 do runtime; outros quadros Go estão cortados, em particular idle_4 e taunt. O master preserva pixels existentes e não é arte nova.

Usar este master em todas as futuras gerações, junto de sequências revisadas quando útil, para evitar deriva de identidade. A produção das novas sequências começou após a validação técnica do piloto Rust. O estado atual do candidato e suas ressalvas estão no [laudo de revisão](../VERIFICATION.md).

Inspeção ampliada: até `idle_0` tem parte dos pés cortada no limite inferior da célula (alpha chega a y255/256). O master fixa proporção/rosto, não aprova esse corte. [Complemento de pés da fonte original](footwear-existing.png) foi recortado de `assets/references/go-sprite-altas.png` em `(35,10,105,75)` sem reescala; usar somente para reconstruir os pés sem recuperar a proporção larga da fonte.
