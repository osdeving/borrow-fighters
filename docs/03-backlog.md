# 03 — Backlog Inicial

## Fonte de verdade

Este documento e a fonte de verdade para **o que vem agora**.

Rodada concluída: [reações próprias Python × C++](25-python-cpp-contact-reactions.md).
64 desenhos novos e resposta sincronizada a cada contato, incluindo os oito da
rajada. 346 testes, Fmt/Clippy e 64 cenários renderizados nos dois sentidos.
[Vídeo e evidências](evidence/python-cpp-reactions/README.md).
Próximo corte solicitado: aplicar o padrão de reações aos demais personagens
em outra rodada, após revisar o piloto. Custo, raridade, contra-jogo e vantagens
por arena permanecem fora desta entrega.

Rodada anterior: [reações e transformações](24-reactions-and-transformations.md).
Python gigante, expansão de C++ e mutação persistente de Rust continuam
integradas. O playtest rejeitou a leitura das reações genéricas, e o piloto acima
substitui essa apresentação para Python/C++. A validação anterior está preservada
como [registro histórico](evidence/reactions-transformations/README.md).

Rodada anterior concluída: [supers autorais e identidade sonora](23-authored-super-sequences.md).
Garbage Collector, Ownership Eclipse, General Protection Fault e Footgun
integrados, com 321 testes Rust e 41 verificações de controles aprovados.
[Evidência visual e sonora](evidence/authored-supers/README.md).
Próximo corte: balancear custo/raridade,
janela de captura e contra-jogo após o playtest destas animações.

Rodada anterior concluída: [polimento da apresentação, Brasil cotidiano e seis
especiais cinematográficos](22-presentation-and-brazilian-stage-life.md), solicitado
em 8 de setembro de 2026. Implementação integrada; 300 testes Rust, formatação,
Clippy estrito e links/YAML aprovados. [Evidência gráfica e controles](evidence/presentation-polish/README.md).
Próximo corte após entrega: playtest humano e fechamento
do checklist de release, incluindo pausa, resolução e remapeamento de controles.

Roadmaps especializados continuam existindo, mas devem apontar para este backlog quando uma frente virar trabalho ativo:

- combate e balanceamento: [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md);
- sprites, atlas e ferramenta de inspecao: [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md);
- arte e mood: [`docs/07-art-direction.md`](07-art-direction.md);
- audio: [`docs/14-audio-pipeline.md`](14-audio-pipeline.md);
- processo, PRs e GitHub: [`docs/05-governance.md`](05-governance.md).

Regra operacional:

1. toda frente ativa deve aparecer na tabela **Agora / Proximo / Depois** abaixo;
2. toda tarefa aceita deve ter issue ou PR relacionado quando sair de ideia para execucao;
3. toda mudanca que altera comandos, processo, formato de dados, roadmap ou contribuicao deve atualizar este backlog ou o roadmap especializado correspondente;
4. se uma decisao criar padrao duradouro, registrar ADR.

## Agora / Proximo / Depois

| Janela | Frente | Status | Registro | Proxima acao |
|---|---|---|---|---|
| Concluído | Reações próprias Python × C++ | Implementado e verificado | [Entrega 25](25-python-cpp-contact-reactions.md), [evidências](evidence/python-cpp-reactions/README.md), [ADR 0018](adr/0018-contact-reaction-profiles.md) | 64 desenhos novos, reação individual à rajada, 64 cenários nos dois sentidos e vídeo do par. |
| Próximo | Reações próprias dos demais personagens | Rodada posterior ao piloto | [Padrão 25](25-python-cpp-contact-reactions.md#aplicar-o-padrão-na-próxima-rodada) | Estender desenhos e validação por contato após o playtest de Python/C++. |
| Concluído | Reações e transformações | Cinco supers autorais e reações dos seis defensores integrados e verificados | [Plano 24](24-reactions-and-transformations.md), [ADR 0017](adr/0017-reaction-clocks-and-arena-mutation.md), [evidências](evidence/reactions-transformations/README.md) | Playtest humano das animações e dos sons. 335 testes, Clippy estrito, formatação e teste separado de áudio ao vivo aprovados; custo, raridade e vantagens por arena ficam para outra rodada. |
| Concluído | Supers autorais e vozes distintas | Quatro roteiros integrados, novos atlas e áudio por fase | [Plano23](23-authored-super-sequences.md), [ADR0016](adr/0016-authored-super-sequences.md) | Playtest humano das animações e timbres; depois calibrar captura, custo e raridade. 321 testes e 41 verificações de controles aprovados. |
| Concluído | Tipografia, cenários vivos e segundo especial | Fontes incorporadas, memes animados e seis cinematográficas com contato local | [Plano e entrega](22-presentation-and-brazilian-stage-life.md), [ADR 0015](adr/0015-cinematic-presentation-and-stage-life.md) | Playtest humano do acabamento e do risco/recompensa dos novos golpes; 300 testes e revisão gráfica registrados. |
| Concluído | Arremessos e especiais extraordinários | Cinco assinaturas, arremessos e reações integrados e verificados | [Goal local e critérios](21-signature-spectacle-and-throws.md), [ADR 0014](adr/0014-throws-launches-and-signature-effects.md) | 290 testes Rust; 40 cenas gráficas nos dois lados, 25 verificações de controles e 60 lutas de CPU. |
| Concluído | MVP: showcase contextual, especiais e coerência de combate/arte | Cinco selecionáveis verificados; sem conteúdo novo para Go | [Plano e critérios](20-mvp-combat-showcase.md) | Corrigir debug, demonstrar contato real com oponente contextual, completar ações/reações e validar balanceamento/arte. |
| Feito local | Cursor livre e menus por mouse | Corrigido e verificado | [Playtest](10-greybox-playtest.md), [guia técnico](12-technical-combat-guide.md#mouse-e-fechamento-da-janela), [ADR 0012](adr/0012-shared-menu-pointer-layout.md) | Cursor sem centralização por quadro; hover, cliques, saída pelo menu e fechamento nativo verificados em janela isolada. 242 testes Rust aprovados. |
| Feito local | Runtime de `frames[].combat` | Feito | [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md), [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md), [`docs/adr/0007-sprite-frame-combat-runtime.md`](adr/0007-sprite-frame-combat-runtime.md) | Runtime consome hitboxes, hurtboxes e origem de projectile do manifesto com fallback para o greybox. |
| Proximo | Calibracao de sprite/hitbox | Revisão de gameplay separada do acabamento | [`docs/11-sprite-pipeline.md`](11-sprite-pipeline.md), [`docs/12-technical-combat-guide.md`](12-technical-combat-guide.md), [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) | A arte refinada preserva caixas, origem e regras baseline por `combat_manifest`; eventual revisão de hurtboxes por pose depende de playtest e não é necessária para justificar os desenhos. |
| Proximo | Feeling e balanceamento | Planejado | [`docs/13-combat-design-roadmap.md`](13-combat-design-roadmap.md), [`docs/15-character-combat-matrix.md`](15-character-combat-matrix.md) | Playtestar a demo Rust x Duke x C x Python x C++ com Combat Lab, mantendo Go fora do menu publico e ajustando frame data por dados, nao por achismo. |
| Feito local | Arte final dos seis lutadores | Acabamento e verificação concluídos | [Laudos e vídeos atuais](../assets/candidates/README.md), [cobertura](19-sprite-production-coverage.md) | 120 clips e 378 quadros, dez golpes por personagem. Go recebeu outra identidade semirrealista. Arte padrão, comparação com env=0, fontes e combate preservados. |
| Depois | Ferramenta visual clicavel | Aberto | [`docs/16-sprite-combat-viewer-roadmap.md`](16-sprite-combat-viewer-roadmap.md) | Avaliar `raygui` somente se atalhos e texto ficarem insuficientes. |
| Depois | Release Prototype 0.1 | Aberto | [`docs/06-release-process.md`](06-release-process.md) | Criar milestone/release checklist quando o slice tiver playtest minimo e assets candidatos. |

## Ritual de manutencao

Antes de abrir branch:

- verificar esta tabela;
- confirmar se existe issue ou PR para a frente ativa;
- se nao existir, criar issue pequena ou atualizar este backlog.

Antes de mergear PR:

- atualizar este backlog se a mudanca concluiu, criou ou reordenou trabalho;
- atualizar o roadmap especializado quando houver detalhe tecnico de uma frente;
- registrar no [`CHANGELOG.md`](../CHANGELOG.md) mudancas relevantes.

Depois de mergear:

- fechar ou comentar a issue relacionada;
- decidir explicitamente qual linha da tabela vira o novo **Agora**.

## Legenda de t-shirt sizing

- **XS**: muito pequeno.
- **S**: pequeno.
- **M**: médio.
- **L**: grande.
- **XL**: muito grande; evitar no protótipo.
- **?**: precisa de investigação.

## Prototype 0.1 — Greybox Fighting Slice

Status do primeiro greybox:

- Mergeado na `main`.
- Cobre o núcleo mínimo jogável com sprites placeholder, cenário bitmap e tela de preferências.
- Ainda não fecha Prototype 0.1 como release; serve para playtest, arte inicial e discussão de feeling.

| Item | Tamanho | Prioridade | Status | Observação |
|---|---:|---:|---|---|
| Criar projeto Rust | S | Alta | Feito | Cargo project básico |
| Configurar Raylib/Raylib-rs | M | Alta | Feito | Validado com checks locais |
| Criar janela e loop principal | S | Alta | Feito | Primeiro teste visual |
| Desenhar arena simples | S | Alta | Feito | Arenas Sirius, Fortaleza Tech Coast e Java Street em rotação |
| Criar entidade Player | M | Alta | Feito | Posição, velocidade, vida e estado |
| Implementar input local | M | Alta | Feito | Teclado e gamepad quando disponível |
| Movimento horizontal | M | Alta | Feito | Esquerda/direita com suavização inicial |
| Gravidade e pulo | M | Alta | Feito | Pulo vertical e diagonal |
| Direção/facing do personagem | S | Média | Feito | Olhar para adversário |
| Soco fraco e forte | M | Alta | Feito | Estados separados |
| Chute | M | Alta | Feito | Usado por jogador e CPU |
| Especial projectile | M | Alta | Feito | `ProjectileSpec` por personagem para Rust, Duke, Go, C, Python e C++ |
| Defesa e abaixar | M | Alta | Feito | Inclui leitura visual em sprite |
| Hurtbox | M | Alta | Feito | Ajustável por estado/personagem no código |
| Hitbox | L | Alta | Feito | Área ofensiva temporária |
| Detecção hitbox/hurtbox | L | Alta | Feito | Coração do combate |
| Aplicação de dano | M | Alta | Feito | Flags permitem invencibilidade de P1/P2 |
| Barra de vida | S | Alta | Feito | HUD opcional |
| Condição de vitória | S | Alta | Feito | Vida <= 0 |
| Reinício da partida | S | Média | Feito | Tecla R |
| Debug draw | M | Média | Feito | Toggle de hitbox/hurtbox |
| Runtime de sprites | M | Média | Feito | Atlas + manifesto JSON v1 |
| IA de playtest | M | Média | Feito | P1/P2, perfis diferentes, ataques variados |
| Polimento de timing | M | Alta | Em andamento | Ataques, projectile, spawn e IA ainda precisam tuning |
| Arte dos lutadores | L | Alta | Seis concluídos e verificados | Rust, Duke/Java, Go, C, Python e C++ revisados no runtime. Go antigo arquivado. Cenários e VFX de protótipo ficam fora deste escopo. |

## Fora do backlog inicial

Registro histórico do corte inicial. Arte final e expansão para os seis lutadores
foram autorizadas posteriormente pelo usuário e constam da frente ativa acima.

| Item | Motivo |
|---|---|
| Online multiplayer | Complexidade XL |
| Sistema de combo | Depende do feeling básico |
| Vários personagens | Depende da abstração mínima |
| Arte final | Exclusão inicial superada pela produção autorizada; veja estado atual acima |
| Trilha sonora | Não prova gameplay |
| Menu completo | Não prova gameplay |
| IA avançada | A IA atual é apenas playtest, não desafio competitivo |
