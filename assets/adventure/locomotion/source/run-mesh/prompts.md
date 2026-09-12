# Fonte da malha da calça — ferramenta integrada imagegen

Referência de identidade/roupa: `../../../rust-actions.png`, primeiro idle.
Imagem inicial: `exec-7d5c3a1c-e9b0-4a2d-b0fc-22b700ef4bae.png` na pasta
pessoal de geração; a versão selecionada e reutilizável é `leg-keyed.png`.
O jogo usa `../../run-mesh/leg.png`; não depende da pasta pessoal.

## Geração inicial (stylized-concept)

Use case: stylized-concept, production asset for a 2D skeletal animation mesh. Reference ONLY the cloth/color/ink style of Rust's cargo trousers in the first idle character in the attached game sprite sheet. Generate ONE continuous trouser-leg sprite from high upper thigh to ankle, WITHOUT a boot, WITHOUT any other body parts. Neutral straight vertical relaxed bind pose, side view facing right, one large cargo pocket on upper outside thigh. Whole uninterrupted length of charcoal olive-grey cargo fabric, muscular but ordinary pants volume, gentle taper at ankle. Width-to-length about 1:3.4. Smooth clean broad cel-shaded fabric planes like the original idle, only TWO OR THREE subtle folds near knee and ankle, absolutely no dense crisscross wrinkles. Crucial: uninterrupted cloth across the knee, NO knee pad, NO cuff or ring or dark joint socket at knee, NO horizontal seam separating thigh from calf, NO black openings at upper end; silhouette should be a closed rounded cap at top hidden under pelvis when rigged. A single modest cuff only at very bottom ankle, slightly wider than pant ankle, not rolled. Strong outer contour, consistent soft warm light from upper left, controlled simple internal linework, no rendering noise. It will be deformed by a 2-bone weighted mesh, so keep middle knee area very smooth and simple. Center the isolated full-length sprite large in portrait canvas with a true transparent RGBA background, no checkerboard, no ground shadow, no text, no sheet of multiple parts, do not draw the original character.

## Correção de fundo e dobra do tornozelo

Edit this exact single trouser-leg sprite. Preserve the full leg silhouette, proportions, cargo pocket, dark grey olive color, light direction, single continuous fabric through the knee. Replace EVERY background pixel, including all painted white/gray checkerboard and wavy patterns, with perfectly uniform solid RGB #ff00ff magenta, no gradient or shadow outside the leg. There must be NO checkerboard remaining. This is a keyed production source for deterministic sprite extraction. Inside the leg, simplify the cluster of dark folds just above the ankle cuff into ONE soft fold; preserve the modest ankle cuff. Knee remains uninterrupted smooth fabric, no new seams, pads, rings or divisions. Exactly one isolated continuous leg as in reference, uncropped. No text.

A saída selecionada foi `exec-168f2de9-d2e5-4b95-a296-826214515e1b.png`.
A extração determinística por ImageMagick remove somente a chave magenta,
recorta o limite transparente e neutraliza RGB invisível. Resultado: 332×1156,
recorte de origem 332×1156+425+102. Desenho e iluminação não são sintetizados
pelo importador.
