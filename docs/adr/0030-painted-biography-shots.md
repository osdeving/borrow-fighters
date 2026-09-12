# ADR 0030 — Quadros completos nas biografias

Status: aceita para a revisão visual solicitada pelo usuário.

## Contexto

A montagem de móveis, pessoas e veículos separados nas biografias de Duke e
Old C produziu incoerências de perspectiva, iluminação e proporção. O usuário
priorizou o acabamento do prólogo e autorizou cenas completas semelhantes às
de Ada, Python e C++, inclusive a chegada de Duke como quadro estático.

## Decisão

Usar uma ilustração completa por tomada destas duas biografias. Cada personagem
tem duas tomadas com ações/enquadramentos distintos. Preservar as identidades
existentes e uma câmera discreta interpolada sobre cada quadro. A limousine
já está estacionada paralela à guia; o quadro comunica a chegada sem deslocar
uma peça através de uma perspectiva incompatível.

Catálogo, trilhos da câmera, duração e legendas continuam externos. Substituir
uma tomada não afeta a outra. O mundo jogável conserva fachadas, adereços,
personagens e colisões separados: a decisão vale para quadros biográficos,
cuja perspectiva e iluminação se beneficiam de composição única.

## Consequências

Detalhes pintados dentro de um quadro são corrigidos na imagem correspondente;
textos narrativos e movimentos da câmera continuam editáveis em JSON.
A verificação exige capturas do renderer e inspeção das transições, escala,
apoios, mãos, oclusão e área ocupada pelas legendas. Geração de uma imagem
não substitui a conferência da cena real.
