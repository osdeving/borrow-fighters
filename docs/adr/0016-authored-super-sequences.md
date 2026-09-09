# ADR 0016 — Sequências autorais de super

## Status

Aceito para a expansão solicitada em 9 de setembro de 2026.

## Contexto

O snapshot de um ataque local implementado na ADR0015 não representa coleta por
clones, reboot de BIOS, perseguição e rajada de contatos ou música suspensa.
O usuário pediu expressamente sequências longas com captura confirmada na entrada.

## Decisão

Adicionar estado explícito de sequência ao `World`, com um único proprietário,
alvo, guarda capturada, relógio e eventos de dano/áudio. O update normal cede à
sequência até seu término; cenas e renderer apenas consomem o estado. Não criar
engine de cutscenes, scripts, ECS ou sistema genérico de timeline.

Separar a pausa inicial da sequência animada: gameplay normal fica suspenso,
mas o relógio autoral avança para tocar fases, áudio e poses. BIOS e eclipse são
camadas de apresentação, sem misturar coordenadas da UI com cálculo de dano.
Os golpes anteriores e os cinematográficos de Go/Python preservam suas regras.

Utilizar atlas opcionais separados para clones/lixo e poses da Footgun, com
recortes autorais e pivôs. O áudio tem cues de fase no manifesto e pausa real da
música, retomando a faixa existente ao sair, reiniciar ou terminar a sequência.
Estado compartilhado entre luta, showcase e captura permite verificar o mesmo
resultado sem renderização nos testes e com áudio nas evidências.

## Consequências

- Captura garantida e chip são decisões temporárias de protótipo para os quatro
  supers, substituindo a localidade inicial da rodada anterior nesses casos.
- O dano fica testável e independente de sprites, brilho, clone ou tela azul.
- Produção de imagens e sons cresce conforme solicitado; fontes e licenças ficam
  no repositório, sem novo pipeline para artistas.
- Medidor, custo, raridade e contrajogo competitivo ficam para playtest posterior.

Veja [roteiros e critérios](../23-authored-super-sequences.md).
