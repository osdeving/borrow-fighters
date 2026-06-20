# ADR 0001 — Stack inicial: Rust + Raylib

## Status

Proposto.

## Contexto

O projeto começa como um jogo 2D de luta com escopo pequeno, foco em protótipo local e objetivo de aprendizado técnico.

A stack inicial precisa permitir:

- loop de jogo simples;
- renderização 2D;
- input;
- colisão;
- prototipação rápida;
- controle razoável sobre arquitetura;
- evolução para uma micro-engine própria.

## Decisão

Usar inicialmente:

- Rust como linguagem principal;
- Raylib/Raylib-rs como biblioteca gráfica e de input;
- arquitetura simples e evolutiva;
- placeholders para arte e som.

## Alternativas consideradas

### Bevy

Prós:

- ECS nativo.
- Ecossistema Rust forte.
- Mais recursos de engine.

Contras:

- Pode introduzir complexidade cedo.
- Pode deslocar o foco para aprendizado da engine.

### Macroquad

Prós:

- Simples.
- Boa para protótipos 2D.

Contras:

- Menos alinhada com a intenção de aprender estrutura mais baixa de game loop.

### Godot

Prós:

- Produtiva.
- Editor visual.
- Ótima para jogos 2D.

Contras:

- Menos alinhada com o objetivo de aprender Rust e micro-engine própria.

## Consequências

### Positivas

- Controle maior sobre o loop.
- Boa base para aprender arquitetura de jogo.
- Baixa cerimônia para renderizar coisas simples.
- Permite começar com caixas e placeholders.

### Negativas

- Mais coisas precisarão ser implementadas manualmente.
- Menos tooling visual.
- Pode exigir decisões arquiteturais antes da hora.

## Critério de revisão

Esta decisão deve ser revisada se:

- Raylib-rs travar o desenvolvimento no ambiente principal;
- a criação de ferramentas ficar mais importante que o jogo;
- o projeto precisar de editor visual;
- o protótipo demonstrar que a engine própria está virando gargalo.
