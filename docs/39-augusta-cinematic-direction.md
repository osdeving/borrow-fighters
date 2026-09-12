# 39 — Augusta: encenação e câmera cinematográfica

A revisão transforma a apresentação do capítulo em tomadas dirigidas, com
perspectiva 3D real, mantendo o combate lateral e a identidade ilustrada.
O cenário é o mesmo mapa: letreiro Limiar, porta, fachadas vizinhas e posições
dos personagens não mudam de lugar quando a câmera corta.

## Sequência

1. **Bairro, 20 segundos:** grua sobre as fachadas, travelling pela vida noturna,
   apresentação do Limiar, Julia retida e revelação de C++.
2. **Aproximação jogável:** C++ caminha até o conflito. Ao chegar, a câmera
   mostra Julia reconhecendo a amiga e tentando se soltar, antes da conversa.
3. **Tentativa de Julia, 7 segundos:** close de reação, detalhe da pegada no
   braço e plano que relaciona C++ ao par. Julia fica à direita e atrás do
   cafetão, considerando C++ à esquerda. Ela tem 22 anos, conforme o catálogo.
4. **Confronto:** planos próximos acompanham quem fala, mantendo o eixo da rua.
5. **Seguranças, 9 segundos:** a porta abre e três homens saem por ela em
   intervalos. O primeiro contorna C++ pelo primeiro plano. Nenhum ataca antes
   da entrega ao jogador; os apoios finais coincidem com a simulação.
6. **Chegada das EPs, 8 segundos:** ameaça no alto, reação assustada do cafetão,
   fuga dele e queda das erráticas. O bairro evacua. O cafetão já saiu quando
   o combate começa; derrotar as EPs permite seguir diretamente ao resgate.
7. **Resgate e saída:** conversa com planos próximos e deslocamento acompanhado.

As conversas continuam sob controle do jogador. **Enter / A** avança falas;
**Backspace / View** conclui a atuação atual. Pular uma tomada leva ao estado
final daquela encenação e não elimina inimigos. **Esc / Start** pausa o relógio.
Derrota e retomada das EPs conservam a rua evacuada e o cafetão ausente.

## Continuidade e estilo

O palco usa fachadas pintadas sobre volumes rasos, chão texturizado, calçada
e meio-fio em profundidade. Os atores conservam as pinturas e os rigs usados
no gameplay. A câmera faz grua, travelling, aproximação e arcos oblíquos na
frente da rua. Personagens continuam ilustrados em planos; esta etapa não
cria modelos humanos 3D completos nem uma câmera livre de 360 graus.

O braço posterior da C++ tinha seu alvo atrás do tronco. A pose de repouso
e os retornos agora deixam as duas mãos visíveis. As imagens originais foram
preservadas. O contato de Julia tem uma pintura conjunta própria, compartilhada
pelos planos da cinemática e pelo gameplay. Uma malha acompanha a tentativa de
saída e mantém o punho preso; o shader retira o fundo verde da fonte em runtime.
As passadas dos NPCs articulam recortes das pinturas existentes, com pernas,
apoios e contrapasso, sem redesenhar o rosto e a roupa a cada quadro.

Há 22 adultos decorativos, roupas/cabelos/bolsas variados, grupos conversando,
pessoas nas mesas, mulheres na vida noturna, telefone, vasos e cinco veículos,
incluindo táxi e entrega de moto. São desenhos geométricos de fundo, com
membros articulados; não substituem a arte pintada dos protagonistas.
O mesmo relógio os mantém em posição entre cortes. Ao surgir a ameaça,
fogem sem reaparecer durante o encontro.

Porta, passos, ruptura e pânico têm Foley original sintetizado, sem vozes.
O tráfego se apaga nos seis primeiros segundos da ameaça; o ar continua.
A pausa suspende os sons e a retomada não repete cues abandonados.

## Arquivos de autoria

| Arquivo | Responsabilidade |
| --- | --- |
| [chapter.json](../assets/adventure/chapters/cpp-augusta/chapter.json) | Durações das atuações |
| [world.json](../assets/adventure/chapters/cpp-augusta/world.json) | Porta, posições e fachadas |
| [texts.json](../assets/adventure/chapters/cpp-augusta/texts.json) | Falas e legendas |
| [augusta/chapter.rs](../src/adventure/augusta/chapter.rs) | Contato, trajetórias, fuga e progressão |
| [augusta/cinema.rs](../src/adventure/augusta/cinema.rs) | Tomadas, lentes, trilhos e cortes |
| [augusta/ambient.rs](../src/adventure/augusta/ambient.rs) | Vida noturna e evacuação determinística |
| [production/cinema.rs](../src/adventure/engine/production/cinema.rs) | Palco e projeção 3D |
| [production/restraint.rs](../src/adventure/engine/production/restraint.rs) | Atuação pareada com contato |
| [production/nightlife.rs](../src/adventure/engine/production/nightlife.rs) | Figurantes, móveis e veículos |
| [production/frame_motion.rs](../src/adventure/engine/production/frame_motion.rs) | Passadas dos NPCs com a pintura existente |
| [clips.json](../assets/adventure/actors/cpp/clips.json) | Duas mãos legíveis em repouso e recuperações |

A câmera é autoria Rust localizada; alterar trilhos requer recompilar. Os
assets/tempos/textos continuam externos. A [ADR 0033](adr/0033-augusta-cinematic-stage.md)
registra a decisão, e o [diário](worklogs/augusta-cinematic-direction.md) mantém
comandos, revisões e instruções de recuperação.

```sh
cargo run -- --start augusta
# Captura reproduzível, com perfil próprio no diretório da revisão:
cargo run -- --start augusta --hidden --review /tmp/augusta-film --frames 24000
python3 tools/review/mix_augusta_review_audio.py /tmp/augusta-film
```

O modo de revisão atravessa o capítulo com controles normais, sem forçar dano
ou vitória. Telemetria registra tomadas, poses de chegada, NPCs e relógio de
ameaça. O áudio da revisão é reconstruído dos cues e amostras do runtime.
As evidências e verificações finais são registradas no diário de trabalho.
