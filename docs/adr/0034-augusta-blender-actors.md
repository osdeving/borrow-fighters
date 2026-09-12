# ADR 0034 — Humanos e veículos 3D na Augusta

Status: implementação experimental autorizada em 12/09/2026.

## Contexto

Os personagens pintados da ADR 0033 têm volume limitado a planos. O usuário
identificou problemas de orientação e articulação em atuação cinematográfica,
e autorizou modelos Blender mantendo a mesma cena, movimentos e identidade.

## Decisão

Produzir humanos e veículos no Blender e exportar modelos animados glTF/GLB
para o adaptador Raylib da aventura. Compartilhar os mesmos modelos entre
cinemática e câmera lateral. Preservar o domínio de história, combate, diálogos,
trajetórias, relógios, enquadramentos e assets do cenário.

Começar com um piloto da C++ e conferir a identidade em closes e movimentos
antes de ampliar o elenco. Fontes editáveis e sua proveniência permanecem em
`assets/adventure/production-3d`; somente recursos declarados para reprodução
em `assets/adventure/models` entram na distribuição.

## Consequências

O palco permanece em Rust/Raylib; Blender é a ferramenta de produção. A
integração precisa carregar malhas, materiais e animações, calibrar escala e
sincronizar gestos aos mesmos estados. Um esqueleto não impede interpenetração
automaticamente: contato, apoio, orientação e silhueta exigem revisão de frames.

Preservar o renderer pintado como comparação durante o piloto permite medir
a alteração sem reescrever regras ou perder a referência visual existente.

Referências: [ADR 0033](0033-augusta-cinematic-stage.md),
[diário](../worklogs/augusta-blender-actors.md),
[arquitetura](../08-code-architecture.md).
