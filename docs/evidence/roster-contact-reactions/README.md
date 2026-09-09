# Rust, Duke, C e Go — reações por contato

Os quatro personagens receberam **128 desenhos**, com oito perfis de quatro
poses: cabeça, corpo, perna, guarda alta, guarda baixa, lançamento, queda e
recuperação. A [produção](../../../assets/production/roster-reactions-2026-09-09/README.md)
registra fontes, recortes, escala e pivôs.

## Verificação dos assets

A [auditoria](asset-review.json) compara os candidatos com `38d81ae` e confirma
128 recortes RGBA distintos, transparência, ausência de clipping nas bordas e
preservação dos 334 frames, 95 clips e PNGs anteriores. Escala dos manifests e
pivôs continuam registrados. A [inspeção das pranchas](source-visual-review.json)
examinou os desenhos sobre fundo claro/escuro; a captura abaixo acrescenta o
uso real das poses pelo World.

## Resultado da captura integrada

A execução [34394289582](https://github.com/osdeving/borrow-fighters/actions/runs/34394289582),
no commit `4f1631fad02fdf24184e8117c0a05332da843690`, passou na auditoria completa
com Raylib/Xvfb no Linux. O [laudo original do runner](capture-verification.json)
e o [registro de origem e revisão](runtime-review.json) documentam:

| Verificação | Resultado |
|---|---|
| Cenários completos | 120, entre Rust, Duke, C e Go, dois sentidos e dano ligado/desligado |
| Cobertura por defensor/lado/dano | Oito perfis e todos os 32 desenhos |
| PNGs conferidos no runner | 2.920 cabeçalhos válidos, em 1280×720 |
| Rajada de C++ | 128 janelas completas, incluindo 1.280 PNGs consecutivos |
| KO | Oito cenários mantendo a última pose de queda no piso por ao menos 60 ticks após o super |
| Dano desligado | HP permanece em 1 e dano em zero durante todos os contatos protegidos |
| Amostra baixada | 360 PNGs originais decodificados e registrados por SHA-256; 32 preservados nesta pasta |

Cada um dos oito contatos da rajada, nos ticks 344–414 a cada dez ticks,
reinicia a reação de cabeça. A resposta percorre os quatro desenhos em nove
frames e mantém a última pose até o próximo contato. As 8.720 observações
registram a posição física, clip, desenho, idade, duração e vida do defensor.

O apoio no piso é exigido quando o corpo está `grounded` e fora de captura.
Durante 11 ticks de cada arremesso, a captura suspende o alvo antes de começar
o voo; esse intervalo não representa apoio no chão. A correção do auditor
preserva as exigências de aterrissagem, recuperação e KO.

## Vídeo e imagens preservados

O [vídeo contínuo](contacts-right.mp4) reúne os 32 cenários do sentido principal,
com dano ligado: **111,8 segundos, 6.708 frames a 60 fps, sem áudio**. A cópia foi
recomprimida para reduzir o tamanho, conservando a sequência, a duração e a
cadência; não usa interpolação. O [índice dos desenhos revisados](video-frame-selection.json)
e os relatórios de estado identificam os frames e intervalos de cada cenário. Os PNGs abaixo mantêm os bytes originais da captura.

| Defensor | Corpo | Cabeça | Baixo | Guarda alta | Guarda baixa | Captura/lançamento | Queda | Recuperação |
|---|---|---|---|---|---|---|---|---|
| Rust | [PNG](frames/rust-right-body.png) | [PNG](frames/rust-protected-right-head.png) | [PNG](frames/rust-left-low.png) | [PNG](frames/rust-protected-left-guardhigh.png) | [PNG](frames/rust-protected-right-guardlow.png) | [PNG](frames/rust-left-launch.png) | [PNG](frames/rust-right-fall.png) | [PNG](frames/rust-protected-left-rise.png) |
| Duke | [PNG](frames/duke-right-body.png) | [PNG](frames/duke-protected-right-head.png) | [PNG](frames/duke-left-low.png) | [PNG](frames/duke-protected-left-guardhigh.png) | [PNG](frames/duke-protected-right-guardlow.png) | [PNG](frames/duke-left-launch.png) | [PNG](frames/duke-right-fall.png) | [PNG](frames/duke-protected-left-rise.png) |
| C | [PNG](frames/c-right-body.png) | [PNG](frames/c-protected-right-head.png) | [PNG](frames/c-left-low.png) | [PNG](frames/c-protected-left-guardhigh.png) | [PNG](frames/c-protected-right-guardlow.png) | [PNG](frames/c-left-launch.png) | [PNG](frames/c-right-fall.png) | [PNG](frames/c-protected-left-rise.png) |
| Go | [PNG](frames/go-right-body.png) | [PNG](frames/go-protected-right-head.png) | [PNG](frames/go-left-low.png) | [PNG](frames/go-protected-left-guardhigh.png) | [PNG](frames/go-protected-right-guardlow.png) | [PNG](frames/go-left-launch.png) | [PNG](frames/go-right-fall.png) | [PNG](frames/go-protected-left-rise.png) |

`protected` identifica dano desligado. Os arquivos brutos de estado foram
preservados sem alteração de conteúdo, compactados em gzip:
[direita](source-reports/right.json.gz), [esquerda](source-reports/left.json.gz),
[direita protegida](source-reports/protected-right.json.gz) e
[esquerda protegida](source-reports/protected-left.json.gz).
[SHA256SUMS](SHA256SUMS) permite conferir os arquivos permanentes.

## Revisão visual e limites

A comparação das poses extraídas do vídeo mostra articulação própria nos quatro
personagens: cabeça/tronco recuam, braços mudam de posição e joelhos absorvem o
contato; queda e levantamento têm etapas corporais distintas. A amostra de PNGs
originais acrescenta os dois sentidos e as condições de dano. A revisão combina
comparação estática com os relógios exportados; não atribui aprovação humana de
fluidez a 60 fps, de áudio ou de gamepad físico.

Permanecem limites de acabamento que não alteram a auditoria mecânica: no ápice
do arremesso, o HUD encobre parte da cabeça/tronco; a guarda em pé de Go mantém
as mãos altas mesmo quando este soco acerta o abdômen; o primeiro recuo de Duke
na rajada deixa um pequeno espaço entre a faísca e a cabeça. Go também apresenta
variação de focinho/volume facial entre desenhos iniciais de recuperação.
O vídeo cobre apenas o sentido principal com dano ligado; as outras condições
têm amostras PNG e relatórios completos de estado.

## Reprodução

O [exemplo de captura](../../../examples/capture_roster_contacts.rs) aplica
comandos reais no `World`, com métricas corporais e manifests de combate do jogo,
e desenha os dois atores com `draw_fight`. Nenhum perfil é forçado. `--video`
grava cada tick do framebuffer a 60 fps e exige `ffmpeg`; o exemplo não captura
áudio. `--reverse`, `--no-damage`, `--defender` e `--barrage-only` permitem recortes
de revisão. As saídas precisam estar em diretórios vazios.

Com Rust, dependências Raylib e display disponível (Xvfb também funciona):

```sh
CARGO_BUILD_JOBS=1 cargo test --test roster_contact_reactions --test reaction_matrix --test contact_reactions --test paired_reaction_sprites --test no_damage_reactions
cargo run --release --example capture_roster_contacts -- --output /tmp/roster-contact-right --video /tmp/roster-contact-right.mp4
cargo run --release --example capture_roster_contacts -- --reverse --output /tmp/roster-contact-left
cargo run --release --example capture_roster_contacts -- --no-damage --output /tmp/roster-contact-protected-right
cargo run --release --example capture_roster_contacts -- --no-damage --reverse --output /tmp/roster-contact-protected-left
python3 tools/art/audit_roster_contact_assets.py --baseline 38d81ae --report /tmp/roster-asset-review.json
python3 tools/art/audit_roster_contact_capture.py --input /tmp/roster-contact-right --input /tmp/roster-contact-left --input /tmp/roster-contact-protected-right --input /tmp/roster-contact-protected-left --report /tmp/roster-contact-verification.json
```

O auditor exige a matriz completa de PNGs gerada pelos comandos acima; a seleção
compacta do artefato ou desta pasta não substitui esses arquivos. A matriz de
regressão de contatos cobre 96 combinações de defensor, slot, lado, guarda e dano;
a matriz geral continua cobrindo 1.704 combinações de ataques e defensores.
