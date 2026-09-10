# 33 — Depois do silêncio

Goal autorizado em 10/09/2026. Branch `feature/prologue-scene-improvements`;
referência anterior preservada em **`81c50e6`**.

## Capítulo aprovado

O Modo História continua a aventura de Rust depois do primeiro encontro.
Seleção livre permanece em Versus; capítulos de outros protagonistas ficam
para depois. A transição retorna à rua evacuada, com carro amassado, bicicletas
caídas e mercearia fechada. Rust verifica o motorista, conversa com o lojista
e outros moradores e obtém uma ligação com a investigação de Python.

Um morador apresenta o trabalho de Python. Rust tira o celular e escreve;
uma ampliação da tela aparece ao lado dele, mantendo o mundo e sua atuação
visíveis. Mensageiro com cabeçalho verde, balões e sinais de entrega, sem
nome ou logotipo de aplicativo. Conversa solicitada, nesta ordem:

1. Rust: `voltou a acontecer.`
2. Python: `sim, eu senti algo...`
3. Rust: `Vou resolver isso.`

Rust guarda o telefone e segue. A conversa sugere um contato anterior, sem
inventar a autoria da mensagem de Ada nem a causa das erráticas. Python é
professora; sua localização atual permanece aberta. O capítulo segue por
trechos curtos do bairro, com obstáculo legível, interação e um encontro cujo
objetivo é proteger uma passagem. O fim dá direção à investigação seguinte.

## Encenação e estrutura

- Animação dentro do jogo: poses, gesto, deslocamento, olhar, câmera e som.
  O capítulo não usa páginas de graphic novel nem retratos como substitutos
  da atuação. Legendas curtas e painel do telefone são interfaces sobre o mundo.
- Atores, adereços, animações e composição têm dados separados; pivôs e sockets
  permitem trocar poses ou aparelho sem repintar a cena. Reutilizar a rua e seus
  moradores. Só novos gestos, motorista e adereços necessários recebem arte.
- Modelo espacial explícito: coordenadas, chão, bounds, obstáculos, POIs,
  regiões de interação, rotas de aproximação e câmera limitada ao cenário.
  F3 mostra a geometria; triggers usam esses dados, nunca pixels da imagem.
- Direção por fases e relógio fixo; aproximação da porta/carro move o ator
  fisicamente por pontos de caminho, incluindo profundidade e escala.
- Gameplay, encenação, telefone, persistência, input, render e áudio possuem
  responsabilidades próprias. Sem adicionar uma engine, ECS ou editor genérico.
- História oferece continuar, novo capítulo e revisão do prólogo; checkpoints
  persistem. Prólogo pulado não registra uma vitória jogada. Retry restaura o
  encontro local sem repetir conversas; pausa conserva atores e mensagens.

## Verificação e entrega

Rever clique real no menu, entrada/saída da campanha, prólogo versus pulo,
save/continue/retry, ordem das interações, bloqueio de ações durante atuação,
geometria/câmera, obstáculos, contato e entrega de controle. Conferir na janela
as mãos/celular, leitura da conversa, porta e motorista. Gravar uma prévia com
áudio nativo e evidência funcional; rodar fmt, Clippy, testes e fronteiras.

Disco: inspecionar o Windows, remover somente temporários comprovadamente
dispensáveis e registrar antes/depois. Não apagar dados pessoais, conversas,
assets, saves ou checkpoints; não parar WSL/Codex para compactar discos virtuais.

[ADR 0027](adr/0027-chapter-spatial-direction.md) ·
[Diário](worklogs/after-the-silence.md).
