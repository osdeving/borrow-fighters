# ADR 0029 — Mundo lateral extensível e fachadas por instância

Status: aceita para a revisão autorizada da aventura.

## Contexto

Rust passa a correr. O deslocamento exige ruas mais longas e intervalos claros entre acontecimentos. A pintura única e o limite físico de 2200 pixels impediam aumentar os espaços de forma consistente. Objetos narrativos não podem deslizar com o parallax.

## Decisão

Manter dados tipados de mundo, sem introduzir ECS. O capítulo conserva seu arquivo de geometria autoritativo; o prólogo recebe um mapa físico externo equivalente, com limites, spawn e marcos nomeados. O combate recebe os limites do espaço que o possui.

Um catálogo de paisagem contém fachadas individuais, fundos distantes e peças de chão. A composição externa possui instâncias com coordenadas e apoio na mesma base. Os fundos repetem com parallax; fachadas, adereços, personagens e acontecimentos compartilham coordenadas de mundo. O renderizador recorta a faixa visível e não amplia uma imagem para cobrir o comprimento do mapa.

A vizinhança existente continua como um conjunto local ancorado no mapa: mercearia, porta, pessoas, trânsito e suas consequências mantêm o encaixe original. A abertura da pipa pode percorrer essa rua antes de entregar a câmera ao jogador. POIs do capítulo reutilizam a mesma ancoragem.

## Consequências

Comprimento, distância entre acontecimentos e posição de cada fachada ficam editáveis por arquivo. Alterar uma peça não exige refazer o cenário inteiro. Validação rejeita limites inválidos e referências ausentes. A física continua independente de Raylib e as fronteiras entre aventura e luta são preservadas.
