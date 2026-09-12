# Laboratório externo de personagens

O binário `borrow-actor-lab` abre diretamente um `character.json`. Ele usa os
mesmos contratos, carregador, interpolação, desenho e simulação consumidos pelo
capítulo C++ Augusta. Não carrega a campanha, a rua ou recursos do Rust.

```sh
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --validate
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --clip run
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --actor assets/adventure/actors/security/character.json --clip light-1
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --enemy assets/adventure/actors/erratic/character.json
```

`--validate` verifica o pacote sem abrir janela: versões, referências, geometria,
tempos de golpes e clipes, existência, decodificação e dimensões PNG e recortes
dentro das imagens. Um caminho externo
é aceito da mesma forma que os pacotes incluídos no projeto; nenhuma seleção de
personagem é embutida no laboratório.

## Inspeção e recarga

- Espaço pausa ou reproduz; Tab seleciona o próximo clipe.
- Na inspeção pausada, as setas avançam ou recuam um tick; a barra permite examinar
  qualquer fase. `F` troca a direção. As duas instâncias mostram a escala nativa e
  uma ampliação identificada na tela.
- Enter alterna para a arena. A/D movem, W pula, J encadeia golpes leves, V chuta,
  K gira, L dispara Linker e Q defende ou apara no começo da defesa. I solicita um
  golpe do oponente. Na arena pausada, seta direita avança um tick.
- Sem `--enemy`, a arena usa uma cópia de treino do próprio personagem. Um pacote
  de oponente com `ai` executa suas decisões normais. Um pacote alternativo precisa
  de identidade própria.
- F3 exibe esqueleto e caixas; R reinicia o teste. F5 constrói um candidato completo
  antes de substituir a sessão. Arquivos inválidos mantêm o pacote anterior e
  mostram a causa do erro. A inspeção preserva clipe, fase, direção e pausa após
  uma recarga válida; se a duração mudar, preserva a fase proporcional. Na arena,
  a recarga reinicia o combate e mostra essa confirmação explicitamente.

Os textos da ferramenta ficam em
[`production-lab.json`](../../../../assets/adventure/production-lab.json).

## Contratos compartilhados

`character.json` versão 1 referencia `combat.json`, `rig.json` e `clips.json`
relativos ao seu próprio diretório. O pacote guarda dimensões físicas, movimento,
defesa, golpes disponíveis e eventual AI. O arquivo de combate declara frequência
de 60 ticks/s, preparação, atividade, recuperação, dano, reação, caixas e
encadeamentos. Os intervalos são `[início, fim)`, relativos ao começo do golpe;
as caixas locais têm origem nos pés, X positivo para a frente e Y negativo acima
do chão. Um golpe tem no máximo um contato por alvo a cada execução.

O domínio não abre arquivos ou janelas: recebe um `Arc<CombatCatalog>` validado.
`Simulation::tick(Input)` devolve eventos únicos; o laboratório pode usar
`tick_with_controls` para dirigir um oponente sem criar outra implementação.
Snapshots de atores possuem IDs estáveis, posição, velocidade, HP, direção,
ação, idade da ação e distância realmente percorrida no chão. Ataques usam
`Actor::clip_id()` e o relógio do golpe; corrida pode usar distância no clipe.
Rigs `frames` usam clipes temporais com `stride_pixels: null`; a seleção de seus
frames não depende de uma velocidade presumida. Rigs `skeletal` podem dirigir
a corrida pela distância de apoio.

Cada vista esquelética pode declarar `body_leg_blend`: a faixa do quadril em que
o tecido pintado acompanha as coxas, preservando o tronco acima dela.
`leg_alpha_half_width` registra a meia largura visível da perna dentro de sua
imagem, para que margens transparentes não deixem sobras do torso no encaixe.
Esses ajustes permanecem no JSON e reutilizam as mesmas pinturas.

O campo `yaw_interpolation` usa `shortest` por padrão: a passagem de 330° para
0° percorre 30°, mantendo o golpe voltado ao alvo. Um giro completo declara
`unwrapped` e registra o trajeto acumulado, como 330° → 720°; isso permite editar
a rotação sem introduzir voltas acidentais nos demais clipes.

`begin_encounter` valida todos os spawns antes de substituir a onda e preserva HP
e posição do protagonista. `spawn_enemy(..., false)` permite encenar uma chegada:
o ator vivo inativo impede vitória, mas não possui caixas ou ataques. Ativação e
posicionamento são operações públicas. `reset_player` restaura HP e remove onda,
projéteis e ações pendentes para o checkpoint escolhido pelo host.

Fontes:
[`spec.rs`](../../../../src/adventure/production/spec.rs),
[`sim.rs`](../../../../src/adventure/production/sim.rs),
[`animation.rs`](../../../../src/adventure/production/animation.rs),
[`carregador`](../../../../src/adventure/engine/production/assets.rs) e
[`desenho compartilhado`](../../../../src/adventure/engine/production/actors.rs).

## Revisão nativa reproduzível

O diretório deve estar novo ou vazio. É necessário um display nativo funcional e
FFmpeg disponível no `PATH`; a janela pode permanecer oculta.

```sh
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --hidden --review /tmp/cpp-lab-review
```

[`verify.py`](verify.py) executa essa captura, compara hashes das fontes antes e
depois, verifica frequência/contagem/duração e decodificação do vídeo, valida o
snapshot exportado e abre novamente sua janela a partir de `/tmp`:

```sh
python3 docs/evidence/production-pipeline/lab/verify.py --output /tmp/cpp-lab-audited
```

A revisão padrão possui 1.080 frames, a 60 fps, com um `Simulation::tick` por
frame: repouso, aceleração, parada, reversão, salto e arena com comandos comuns.
Nenhum HP ou resultado é forçado. A gravação é silenciosa e não duplica frames
para simular uma frequência maior. `--frames N` limita a captura; o resultado
registra tanto a contagem de frames quanto a de ticks.

O resultado inclui:

- `clip-*.png`: doze fases de todos os clipes, nas duas direções; em clipes cíclicos,
  as fases 0 e 1 mostram a mesma pose de fechamento.
- `zoom-*.png`: oito fases ampliadas de repouso, corrida e giro, quando esses
  clipes existem; até 2x, com a escala efetiva indicada em cada quadro.
- `simulation-*.png`, `telemetry.jsonl` e `lab-simulation-60fps.mp4`: contexto de
  movimento e combate, comandos, eventos, HP, fase, deslocamento e direção.
- `invocation.json` e `result.json`: frequência, duração e conclusão da captura.
- `source/player/` e eventual `source/enemy/`: cópias dos JSONs e de todas as imagens
  referenciadas, com caminhos relativos preservados.

Esse snapshot é um pacote executável por si:

```sh
cargo run --no-default-features --features adventure --bin borrow-actor-lab -- --actor /tmp/cpp-lab-review/source/player/character.json --validate
```

A revisão visual deve verificar: identidade e proporções em frente, perfil e
costas; mãos, quadris e membros sem emendas aparentes; tênis acompanhando a canela
durante a recuperação; contato dos pés com o chão e continuidade no começo,
parada e reversão; ataques legíveis no intervalo ativo; ausência de deformação
grosseira nas mudanças de vista. Uma captura concluída não constitui aprovação
automática da arte.

## Captura verificada da C++

A [auditoria final](cpp-review-audit.json) registra captura, FFprobe,
decodificação completa, validação do pacote exportado e reabertura nativa a partir
de `/tmp`, todos com saída 0. Não houve alteração das fontes durante a gravação.
O [vídeo](cpp-review/lab-simulation-60fps.mp4) contém 1.080 frames, cada um
correspondente a um novo tick da simulação, em 18 segundos, 1280×720 a 60 fps;
o arquivo tem 757.420 bytes.

Os comandos da arena produziram contatos dos três golpes leves, chute, giro e
Linker, encerrando o encontro com vitória sem modificar HP ou resultado. O
[snapshot do personagem](cpp-review/source/player/character.json) foi validado e
aberto novamente como pacote externo. Os pacotes `security` e `erratic` também
passaram pela CLI `--validate` e por abertura nativa de dois frames.

A inspeção visual incluiu corrida e giro nas duas direções, repouso ampliado,
parada e reversão, as recuperações de Linker e chute nos frames 480 e 780 e os
frames 662, 669 e 676 extraídos do vídeo para conferir o encaixe do quadril durante
o giro. Exemplos para revisão:

- [Corrida ampliada](cpp-review/zoom-run-right.png) e
  [direção oposta](cpp-review/zoom-run-left.png).
- [Giro ampliado](cpp-review/zoom-spin-left.png), com o tecido do quadril
  acompanhando as coxas.
- [Recuperação do Linker](cpp-review/simulation-0480.png) e
  [recuperação do chute](cpp-review/simulation-0780.png), voltadas ao alvo.

As regressões automatizadas verificam alcance e apoio das pernas em 4.097 fases,
continuidade dos cotovelos, giro acumulado e recuperação angular dos demais
clipes; o laboratório testa ainda preservação da fase no F5, pacotes alternativos,
rejeição de conteúdo inválido e contatos reais do combo completo. A inspeção dos
frames complementa essas verificações geométricas e de comportamento.
