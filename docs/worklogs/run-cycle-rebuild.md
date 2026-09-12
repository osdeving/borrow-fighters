# Reconstrução da corrida de Rust

Branch `main`; base commitada `6740d6d`. Preservar todas as revisões anteriores
ainda no workspace. Pedido atual restrito à qualidade da corrida, especialmente
dobras da calça e leitura das pernas/botas nas fases 0, 0,125 e 0,375.

## Plano executado

1. Comparar pipeline atual com fontes primárias de produção de jogos.
2. Diagnosticar anatomia, trajetória dos apoios, textura e sobreposição.
3. Produzir uma solução com pose-base controlada, volume consistente e contato
   sincronizado à distância; validar em tamanho real e câmera ampliada.
4. Conferir ciclo completo nos dois sentidos, partida/parada/inversão, colisão,
   passo/áudio e regras existentes; rodar verificações relevantes.

Root: decisão técnica/arte e integração. `rust_gait`: diagnóstico do rig.
`ep_arrival`: pesquisa primária em `run-cycle-research.md`.
`chapter_combat`: baseline e harness de revisão independente da técnica, em
`docs/evidence/run-cycle-rebuild/`, sem mexer em runtime.

## Diagnóstico inicial

Os recortes de coxa/canela contêm pregas e cuffs muito marcados. Rotacionar e
alargar essas duas peças cria bordas sobrepostas. As medidas do rig mantêm
os joelhos flexionados demais durante o ciclo; a bota/sola reduz ainda mais
o alcance disponível. Simplesmente engrossar a roupa agravou a sobreposição.

## Implementação e primeiro checkpoint visual

A decisão está em [ADR 0031](../adr/0031-continuous-run-leg-mesh.md). Foi criada
uma textura contínua de calça com `image_gen`; fonte, prompts e extração ficam
versionáveis em `assets/adventure/locomotion/source/run-mesh/`. O renderer usa
uma malha de dois ossos com correção de volume no joelho, botas rígidas e faixas
do tronco que protegem cabeça e emblema. O catálogo deixa de carregar os quatro
recortes sobrepostos de coxa/canela. Não houve alteração de física ou combate.

O baseline foi preservado em `docs/evidence/run-cycle-rebuild/before/`:
16 fases, dois sentidos, 1x/2x e 12 segundos de movimento nativo a 60 fps.
O harness v2 usa velocidade nominal nas folhas de poses e velocidade física
real no vídeo, registrando também transições efetivas de ação/direção.

Primeiro proof em `/tmp/run-mesh-first/`: desapareceu a sobreposição de pregas,
mas os tênis ficaram pequenos e a perna distante perdeu leitura na passagem.
A segunda rodada corrigiu recuperação e separação; a terceira recuperou o
tamanho uniforme das botas, recalibrando a âncora para manter o mesmo vetor de
sola. Essa terceira base foi aprovada nas folhas 1x/2x para captura em movimento.

A revisão independente identificou a falta de validação entre as faixas do
tronco e a altura de seu crop. Corrigida na carga, junto da rejeição de alvos
fora do alcance e de compressão excessiva. Contatos atuais: joelhos perto
151,0° e distante 145,5°; mínimos em apoio 118,26° / 116,57°. A renderização não
modifica os alvos conforme velocidade: o relógio segue o deslocamento real,
mantendo apoio durante aceleração e freio. Os PNGs de calça antigos permanecem
como histórico, sem entrar no catálogo/pacote runtime.

## Verificações concluídas

- `cargo fmt --all --check`: passou.
- `cargo clippy --all-targets --all-features -- -D warnings`: passou.
- `cargo test --all-targets --all-features`: 535 passaram, zero falhas;
  dois testes existentes de dispositivo de áudio continuam ignorados.
  Log da sessão: `/tmp/borrow-run-rebuild-tests.log`.
- `python3.13 -m unittest discover -s tools/release -p test_package.py`:
  20 passaram; malha e textura incluídas, recortes aposentados excluídos.
- `python3.13 tools/check_domain_boundaries.py`: passou.
- Links locais dos documentos desta rodada e `git diff --check`: passaram.

O preflight do harness v3 em `/tmp/rust-run-context-preflight-2/` teve saída 0,
sem alteração de hashes. Foram inspecionadas as capturas de prólogo e capítulo,
confirmando a arte na escala e câmera do mundo.

## Primeira captura completa e retomada

`docs/evidence/run-cycle-rebuild/after/` contém a primeira captura completa: saída 0,
129 hashes estáveis, vídeo de 12 s / 720 quadros / 60 fps / 1280×900 e decode
FFmpeg sem erro. A telemetria confirma `delta_x = velocity_x / 60` nos 720 ticks.
Foram inspecionados ciclos nos dois sentidos, as fases apontadas e quadros de
partida/parada/reversão. O baseline permaneceu intocado. Revisão independente
aprovou tecido, botas, silhueta e preservação de identidade nas folhas.

Não regenerar a arte para ajustar poses: usar `run-rig.json` para os alvos e
proporções, `run-mesh.json` para pesos/bandas e catálogo para âncoras. Consultar
o [relatório visual](../evidence/run-cycle-rebuild/README.md). O harness recusa
sobrescrever evidências; novas capturas devem usar outro diretório vazio.
Os arquivos desta rodada permanecem no workspace junto das revisões anteriores.

## Feedback do usuário: orientação do tênis em recuperação

O usuário apontou `after/transition-359.png` (fase 0,5742512, sentido esquerdo):
a bota continua quase horizontal enquanto a canela já mudou de direção. Isso
quebra a leitura da ligação do tornozelo. O ciclo precisa de mais uma correção
antes da entrega. `after/` fica preservado como referência exata do feedback.

`rust_gait` ajusta orientação da bota durante o voo/recuperação sem mudar o
apoio no chão. `chapter_combat` adiciona folha de fases próximas de 0,5742512,
com os dois sentidos, e a próxima captura será `after-ankle/`. Root revisa os
quadros, integração e documentação. Não sobrescrever nenhuma evidência anterior.

A revisão também constatou os cortes já existentes na fronteira idle/run e
no giro. O ciclo em corrida é interpolado; esses cortes de estado não foram
substituídos nesta rodada de tecido/pose. Não descrevê-los como interpolados.

Primeira prova da correção do tornozelo: `/tmp/run-ankle-first/`, aprovada nas
oito fases de recuperação nos dois sentidos e no ciclo 1x. O contrato
`air_ankle` alinha a orientação à canela somente no voo, após resolver IK,
conservando posições de quadril/joelho/tornozelo. Rotação e sola retornam ao
guia de apoio antes do contato. Uma varredura independente de 10.001 fases
com a máscara alpha confirmou folga mínima de 1,35 px / 1,81 px durante a
correção ativa. O caso de perfil editado que atravessava o chão levou a uma
rejeição adicional na carga. A captura `after-ankle/` aguarda o congelamento
desse último ajuste e os testes.

Verificação após correção do tornozelo: `cargo fmt --all --check` e
`cargo clippy --all-targets --all-features -- -D warnings` passaram.
`cargo test --all-targets --all-features` passou com 538 testes, zero falhas e
os mesmos dois testes de dispositivo de áudio ignorados. Log:
`/tmp/borrow-run-ankle-tests.log`. As três regressões novas cobrem orientação
sem mover juntas/apoios, continuidade dos blends e rejeição de tuning que
coloca a sola abaixo do piso.

Captura final `after-ankle/` concluída: exit 0, 129 hashes estáveis, MP4 de
12,000 s / 720 quadros / 60 fps / 1280×900 / 4.728.646 bytes, decode FFmpeg
exit 0. O `timeline.csv` é idêntico ao de `after/`; os quadros 359–361 e as
folhas de recuperação foram revisados. As dez PNGs estáticas coincidem byte
a byte com o proof aprovado. `before/` e `after/` permaneceram intactos,
verificados por SHA-256. Correções solicitadas de tecido, anatomia e orientação
do tênis concluídas; nenhuma regeneração da arte foi necessária para o ajuste
do tornozelo. Evidências e documentação atualizadas para o encerramento do goal.
