# ADR 0003 — Arquitetura inicial de código Rust + Raylib

## Status

Proposto.

## Contexto

O projeto ainda está em pré-produção, mas precisa de uma direção clara para o primeiro scaffold de código.

O risco principal é cair em um dos extremos:

- um `main.rs` grande demais, difícil de testar e manter;
- uma arquitetura de engine/ECS grande demais antes de existir gameplay validado.

O protótipo 0.1 precisa provar combate básico com dois personagens placeholder, mantendo espaço para testes e evolução.

## Decisão

Adotar uma arquitetura inicial de **pacote Rust único** com:

- `src/main.rs` fino;
- `src/lib.rs` expondo módulos testáveis;
- módulos internos por domínio: `engine`, `game`, `combat`, `characters`, `scenes`, `ui`, `math`;
- Raylib isolado tanto quanto possível em `engine/*` e na borda de aplicação;
- regras de combate preferencialmente independentes de Raylib;
- dados de golpes próximos em tabela simples `MoveSpec` sob `combat`, com `CharacterSpec` em `characters` e `AttackKind` como camada runtime de compatibilidade enquanto os personagens ainda compartilham golpes;
- loop com input, fixed update e render;
- documentação de módulo no topo de cada arquivo Rust novo, incluindo o sistema maior via `System:`;
- rastreio técnico de sistemas de combate em `docs/12-technical-combat-guide.md`.

Não adotar workspace multi-crate, ECS genérico, sistema de plugins, scripting ou asset pipeline complexo no protótipo 0.1.

## Alternativas consideradas

### `main.rs` único

Prós:

- mais rápido para começar;
- menos arquivos;
- fácil de copiar exemplos de Raylib.

Contras:

- dificulta testes;
- mistura Raylib, regras de jogo, UI e colisão;
- tende a virar dívida cedo em jogo de luta.

### Workspace multi-crate

Prós:

- separação forte entre engine, jogo e ferramentas;
- bom para projetos maiores.

Contras:

- cerimônia cedo demais;
- aumenta custo de navegação;
- não é necessário antes de validar o protótipo.

### ECS desde o início

Prós:

- pode ajudar quando houver muitas entidades/sistemas;
- padrão comum em engines modernas.

Contras:

- aumenta abstração antes da dor real;
- pode esconder regras de combate que precisam ser explícitas;
- desloca foco do jogo para infraestrutura.

## Consequências

### Positivas

- Mantém `main.rs` simples.
- Permite testar regras de combate sem janela.
- Dá um mapa claro para contribuidores.
- Evita engine própria prematura.
- Deixa Raylib como dependência útil, não como centro do domínio.

### Negativas

- Cria mais arquivos que o mínimo absoluto.
- Exige disciplina para não espalhar Raylib por todos os módulos.
- Algumas fronteiras podem mudar depois do primeiro protótipo.
- Fixed timestep precisa ser implementado com cuidado para não complicar cedo.

## Critério de revisão

Revisar esta decisão se:

- o protótipo 0.1 não avançar por excesso de estrutura;
- testes de gameplay não trouxerem benefício real;
- Raylib exigir outro desenho de posse/vida útil dos recursos;
- o projeto precisar de ferramentas ou editor;
- o número de personagens/sistemas justificar outra arquitetura.
