# 24 — Reações de impacto e transformações

## Plano autorizado

Rodada de 9 de setembro de 2026, posterior ao commit `94f0c7c`. O pedido amplia
as animações explicitamente; custos e balanceamento competitivo continuam fora
desta rodada. Preservar as vozes de Duke/Python aprovadas anteriormente.

1. Auditar seleção, relógios e atlas de reação dos seis personagens. Cada acerto
   precisa interromper a pose anterior e mostrar recoil próprio: leve, pesado,
   baixo, aéreo, projétil, arremesso e especial; bloqueio mantém reação de guarda.
   Supers exigem impacto, deslocamento visual/queda exagerados e recuperação
   legível, sem transformar decoração em novos contatos de dano.
2. Python: sequência autoral longa com preparação, transformação gradual em
   cobra gigante azul/amarela inspirada no mascote Python, crescimento, boca
   aberta, bote e deglutição; reversão, salto alegre com pernas dobradas,
   aterrissagem e sinal de paz voltado ao jogador. Produzir atlas específicos
   com muitos desenhos, mantendo sua identidade. O adversário retorna após o
   efeito para que a luta continue, ferido/caído; captura, chip e KO seguem o World.
3. C++: notebook e digitação; terminal grande com código C++ legível de
   desreferência de ponteiro nulo; bazuca no pé, pulinhos, fúria e corrida/rajada;
   herança visual de Old C com cascata de terminais, tela azul, BIOS e reboot.
   Reaproveitar o desenho de erros de C com mensagens e contexto próprios.
4. Rust: abandonar estruturas derretendo e substituir progressivamente o cenário
   atual por Sirius em blocos. A troca passa a ser estado real da partida,
   persistindo após o super e atualizando nome, música e animações de ambiente.
   Não adicionar vantagem de combate. Definir comportamento coerente para reset,
   próximo round, treino e replay, sem alterar preferências globais do usuário.
5. Integrar desenhos, relógios, sons e visibilidade em luta, Lab e showcase;
   manter pause/frame-step/replay e os comandos fáceis de especiais atuais.
6. Validar contatos e reações de todos os defensores em situações diferentes,
   duas direções, guarda/KO/reset, fases visuais e persistência da arena.
   Executar formatação, Clippy e testes; guardar capturas/vídeo reproduzíveis,
   atualizar catálogo/arquitetura/backlog e commitar a entrega.

## Critérios de aceitação

- Nenhum personagem conserva idle/ataque ao tomar um golpe válido.
- Reação tem progressão visível, não somente um flash ou uma pose estática.
- Python muda de forma com desenhos intermediários, engole no contato e celebra.
- Código de C++ pode ser lido antes do tiro; terminais, BSOD e BIOS fecham o golpe.
- Depois do especial de Rust, a luta continua em Sirius com apresentação e música
  correspondentes; durante a construção os blocos mostram a própria arena nova.
- Dano não depende de FPS nem de tamanho do sprite; chip não encerra a luta.

Decisão estrutural: [ADR 0017](adr/0017-reaction-clocks-and-arena-mutation.md).

## Entrega implementada

| Super | Frames / duração | Dano / chip máximo | Leitura da sequência |
|---|---:|---:|---|
| Ownership Eclipse | 300 / 5 s | 28 / 7 | Revelação em blocos nos frames 11–124; Sirius torna-se arena efetiva no frame 125, pulso no 212 e retorno ao combate na arena nova. |
| Garbage Collector | 350 / 5,83 s | 32 / 8 | Roteiro da rodada 23 preservado; alvo agora tem reação ampliada de voo/impacto/recuperação. |
| General Protection Fault | 320 / 5,33 s | 32 / 8 | Roteiro de C preservado; reação pós-BIOS animada com relógio real. |
| Undefined Behavior: Footgun | 632 / 10,53 s | 36 / 10 | Notebook nos frames 8–147; tiro no 184; pulinhos/raiva; corrida 280–343; rajada 344–414, final no 424; terminais no 450, BSOD no 498, BIOS no 538, reboot no 566. |
| import devour | 528 / 8,8 s | 32 / 8 | Preparação nos frames 8–61; transformação 62–157; crescimento 158–229; boca 230–269; bote 270–303; engole no 304; retorno no 388; salto 400–443; paz 444–487. |

O alvo continua pertencendo ao World enquanto está oculto (Python, frames 304–387).
A sucção usa o sprite real do adversário, com rotação e redução de escala;
o retorno segue a reação física, sem um segundo arco visual independente.
Guarda capturada reduz o dano e não permite KO por chip; KO por acerto aguarda o
fim do roteiro. O tiro no próprio pé de C++ continua cômico, sem dano próprio.

### Reações

A causa principal era o tempo de queda fixado em 0,1 s durante supers, tanto no
Fighter quanto no renderer. O novo `ReactionVisualState` reinicia no contato;
`frame_for_fighter_state` distribui os desenhos pelo tempo disponível, evitando
que o hitstun termine antes das poses finais. Movimento visual de recoil,
compressão e rotação acompanha esse relógio, sem alterar caixas de colisão.

| Situação | Reação |
|---|---|
| Leve, chute, projétil | Hit com três desenhos, recoil e retorno. |
| Pesado, overhead, assinatura, cinematográfico local Go | HeavyHit mais amplo. |
| Rasteira | Knockdown, impacto no piso e recuperação. |
| Anti-air, assinatura que lança, golpe em alvo aéreo | Launched, voo real e pouso. |
| Arremesso | Thrown com captura, voo e aterrissagem. |
| Super autoral | Voo acentuado, impacto no piso, pose caída e get-up; KO permanece caído. |
| Defesa em pé/baixa | Recoil de guarda, preservando postura e chip. |

Os seis defensores usam os desenhos de reação existentes; Go usa seu próprio
hit com transformação de corpo para voo/queda onde não há atlas dedicado.
A velocidade horizontal dos supers considera o espaço até a borda, mantendo
margem para a silhueta. Acertos aéreos pesados preservam o voo em andamento.

### Arena e código

`World::arena_override` muda para Sirius no frame 125 e permanece até o World ser
recriado. Nome/local, textura, vida ambiente e música consultam a arena efetiva.
A faixa nova é preparada pausada e começa no cursor zero ao terminar o super.
Reset/replay restauram a arena base; o próximo round mantém a rotação da seleção
original e o menu não é sobrescrito pela mutação. Não há multiplicador de dano.
Para perceber a transição, comece em outra arena, por exemplo Java Street.

O terminal de C++ mostra:

```cpp
int main() {
    int* alvo = nullptr;
    *alvo = 42;
}
```

A escrita pelo ponteiro nulo tem comportamento indefinido conforme os
[operadores unários do draft de C++](https://eel.is/c++draft/expr.unary.op).
O jogo apenas apresenta o texto e encena a falha; não executa esse programa.

### Produção

- Python: quatro atlas com oito desenhos cada, total de 32, para transformação,
  serpente, reversão e comemoração. Prompts, origens e alpha ficam junto aos PNGs,
  com [guia de produção e pivôs](../assets/production/super-sequences/python/README.md).
- C++: seis desenhos de notebook, reutilizando bazuca/rajada e a composição de
  terminais/BSOD/BIOS de C com mensagens próprias.
  [Registro do notebook e pivôs](../assets/production/super-sequences/cpp/README.md#notebook-da-rodada-24).
- Dez sons de fase produzidos de gravações CC0 já licenciadas; vozes aprovadas
  de Duke/Python preservadas. [Procedência](../assets/audio/production-transformations-2026-09-09.json).

As fases usam o mesmo World na luta, no showcase e no preview autoral do Lab.
Pausa, avanço unitário e replay respeitam esse relógio. O Lab convencional
continua sendo inspeção do ator/dummy; não virou outra implementação de combate.

### Ajustes da revisão

Reiniciar Python após interromper o super entre transformação e crescimento
podia inverter os sons dessas duas fases. `SuperStart` agora reinicia os cursores
dos bindings `super.*`, mantendo a variação independente das vozes comuns.
Um teste de regressão cobre o replay após essa interrupção.

O banner mostra um resumo do roteiro — por exemplo, “Transformação / Devorar /
Celebrar” — em vez de apresentar a arena natal como se fosse o cenário atual.
A identidade da arena exibida no HUD continua vindo da arena efetiva da luta.

### Verificação e demonstração

Verificação final: **335 testes aprovados, um teste de áudio ao vivo ignorado
pela suíte padrão e nenhuma falha**, em 39 alvos de `cargo test --all-targets`.
`cargo fmt --all --check` e `cargo clippy --all-targets --all-features -- -D warnings`
também passaram. O teste de áudio ao vivo foi executado separadamente e aprovado.
Esses checks usaram o target padrão restaurado; `cargo run` funciona normalmente.

A [demonstração com áudio](../assets/showcase/reactions-transformations-2026-09-09.mp4)
acompanha o [índice de evidências e reprodução](evidence/reactions-transformations/README.md).
Esse índice reúne as capturas e o resultado da verificação final da rodada.

A revisão visual confirmou boca, bote, retorno, salto e paz de Python;
notebook, código e terminais de C++; e blocos, cenário final e HUD de Rust.
Uma revisão independente de oito quadros incluiu Python e C++ para a esquerda,
BIOS e reações dos seis defensores.

A [matriz de reações](evidence/reactions-transformations/reactions/README.md)
cobre 1.704 cenários de contato com seis defensores, duas direções e colisões com
e sem metadata. Os [registros de controles](evidence/reactions-transformations/controls/README.md)
incluem seis verificações de Rust e dez de Python, além de inspeções das fases.

A [observação do player de áudio](evidence/reactions-transformations/audio-stream-review.json)
verifica pausa, retomada e a troca de Java Street para Sirius: a faixa de Sirius
permanece no início durante o super, avança após sua conclusão e o reset restaura
a faixa da arena selecionada.

A [recuperação após o reinício](evidence/reactions-transformations/recovery-2026-09-09.md)
preserva o pedido original, o ponto da interrupção e a primeira verificação de
334 testes, anterior ao teste adicional de regressão do replay sonoro.
