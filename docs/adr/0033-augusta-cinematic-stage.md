# ADR 0033 — Palco e câmera cinematográfica da Augusta

Status: aceita para a ampliação autorizada em 12/09/2026.

## Contexto

A encenação lateral do capítulo não mostra a tentativa de saída de Julia,
a origem dos seguranças ou a reação do cafetão à chegada das erráticas.
O usuário pediu atuação, apresentação do bairro, closes, movimento de câmera
e 3D onde viável, com continuidade espacial e visual entre planos e gameplay.

## Decisão

Usar a mesma composição e os mesmos atores ilustrados em um palco Raylib 3D
durante as tomadas dirigidas. Fachadas têm posições e bases fixas; rua, calçada,
soleira e volumes de apoio dão profundidade à perspectiva. A câmera percorre
gruas, travellings e arcos na frente da rua, sem atravessar as pinturas para
exibir costas inexistentes. Não há troca de engine ou dependência do Versus.

O modelo puro da Augusta mantém relógios, poses, contato e trajetórias;
o módulo de câmera produz enquadramentos determinísticos. O adaptador gráfico
desenha os mesmos assets e habitantes tanto no palco quanto no gameplay.
Combate fica retido durante atuação e passa ao jogador com inimigos apoiados
nas posições físicas finais. Skip conclui a encenação, sem conceder vitória.

## Consequências

Há perspectiva e profundidade 3D reais, com personagens ilustrados em planos:
este recorte não é um elenco de modelos 3D completos. A restrição de arco evita
falsas vistas e mantém o eixo C++ → cafetão → Julia. Não se criam pinturas
independentes por ângulo que possam contradizer o cenário.

Capturas nativas e revisão dos cortes são necessárias além dos testes puros.
Pausa, retry e checkpoints preservam as regras da aventura; saves antigos
continuam reconhecidos. A câmera é autoria localizada, sem editor ou framework.

Referências: [arquitetura](../08-code-architecture.md),
[piloto Augusta](../38-cpp-augusta-production.md),
[diário](../worklogs/augusta-cinematic-direction.md).
