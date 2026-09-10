# ADR 0025 — Peças substituíveis e evacuação da rua

## Status

Aceito para o goal solicitado em 10 de setembro de 2026.

## Contexto

Pessoas e carros já eram sobrepostos ao bairro em atlas separados, mas
recortes, tamanhos, poses e instâncias estavam acoplados ao renderer.
O pedido exige trocar e reaproveitar peças, enriquecer a identidade brasileira
e encerrar toda circulação depois que a EP provoca uma evacuação.

## Decisão

Separar catálogo de peças e composição de cena em dados da aventura. Cada
peça referencia imagem, recorte, apoio, tamanho e frames; instâncias usam IDs
estáveis, posição e plano. Um mesmo catálogo permite PNG individual ou atlas,
sem obrigar nova arte a usar dimensões/ordem do PNG anterior. Renderização
carrega esses dados em `adventure/engine`, com validação do contrato em um
módulo da aventura que não conhece Raylib. Textos usam o catálogo existente.

Manter a reação em `AmbientState`: registrar a fase de susto, congelar a
origem de cada trajetória e amostrar saída sem wrap. Carros aceleram até
sair; ciclistas abandonam bicicletas e escapam a pé; o menino conserva sua
fuga. Objetos abandonados são amostras decorativas persistentes. Retry e
restart restauram esse estado; pausa omite updates. Áudio observa os marcos.

## Consequências

Peças podem ser trocadas e instanciadas sem alterar o cenário pintado ou as
regras de combate. O contrato valida caminhos locais, dimensões, referências,
apoios e frames. A composição inclui apenas o necessário ao prólogo; não é
um sistema genérico de cenas, asset pipeline, ECS ou editor. Reuso fica no
domínio da aventura, preservando a separação da ADR 0021.

[Escopo e critérios](../31-brazilian-street-evacuation.md).
