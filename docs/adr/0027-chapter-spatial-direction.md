# ADR 0027 — Capítulo com geometria, encenação e peças independentes

- Status: aceita para o capítulo autorizado pelo usuário.
- Data: 2026-09-10.

## Contexto

A aventura precisa continuar a partir do menu, com exploração, interação e
animação in-game. O usuário exige modularidade, peças substituíveis e uma
representação matemática do cenário. O prólogo existente permanece revisável;
expandir seu `Story` e renderer indefinidamente misturaria dois fluxos distintos.

## Decisão

Criar `adventure/chapter/` para modelo espacial, progressão, encenação e save,
com estado puro em ticks de 60 Hz. `adventure/engine/chapter/` interpreta esse
estado visualmente; uma aplicação própria hospeda o loop sem regras de luta.
O host `presentation` recebe um pedido opaco do menu e chama a aventura;
o domínio fighting continua sem importar código de adventure.

Geometria e POIs são dados validados: extensão/chão, obstáculos, regiões de
interação, posições e caminhos. O renderer e os triggers usam a mesma origem
espacial. Caminhos de aproximação resolvem a diferença entre o passeio jogável
e a profundidade de porta/carro. Limites da câmera fazem parte do modelo.

Animação híbrida: clips de poses com apoios constantes para ações corporais,
keyframes/interpolação para translação/câmera e sockets por pose para acessórios.
O celular é uma peça separada; seu painel ampliado lê a mesma fase da conversa.
Dados de poses/clips não decidem progressão narrativa; a máquina de fases
escolhe a ação, e seu relógio determina frame, gesto e eventos sonoros.

Save guarda checkpoints versionados e fatos narrativos mínimos; não serializa
recursos Raylib nem imagens. Escrita atômica preserva o último save válido.
Sair durante uma conversa retoma de um ponto seguro. Pular o prólogo não gera
estatística de vitória; iniciar o capítulo usa seu estado narrativo canônico.

## Alternativas

Um sistema esquelético completo traria rigging/importação e deformação sem
necessidade demonstrada para este capítulo. Um ECS ou editor adicionaria
infraestrutura além dos poucos atores necessários. Vídeo pré-renderizado ou
páginas ilustradas perderiam continuidade e edição individual das peças.

## Consequências

É possível substituir um gesto, adereço ou região do mapa separadamente.
Testes exercitam caminhos, triggers, controle, mensagens e checkpoints sem
janela; revisão nativa continua necessária para contato visual e expressividade.
O escopo é o [capítulo 33](../33-after-the-silence.md), mantendo o isolamento
da [ADR 0021](0021-isolated-adventure-experiment.md).
