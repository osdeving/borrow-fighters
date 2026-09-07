# Python — revisão do candidato

Produzidas as **20 ações e 60 quadros selecionados**, com referência fixa da mulher adulta original: cabelo preto, camisa branca completa, saia preta, saltos e cobra azul/amarela. A largura mais esguia é intencional para preservar essa identidade; a altura idle fica próxima de 268 px. A referência antiga de roupa curta/saia azul foi usada apenas para entender o cavalete da entrada.

- [Manifesto](../../candidates/python/python-fighter.sprite.json), [panorama](../../candidates/python/review/overview.png) e [soco fraco](../../candidates/python/review/punch_light.gif)
- [Plano e escalas por ação](review-plan.json), [mapeamento explícito](production.json)
- [Novo projétil separado](../../candidates/python/python-projectile.png) e [sua preparação](projectile/production.json)

## Produção e alpha

As folhas foram geradas com imagegen real, preservando prompts e versões rejeitadas. As 20 ações usam fundo magenta removido offline, com descontaminação de borda sem o filtro específico do Rust. O projétil novo foi retornado diretamente em RGBA. Pequenos pixels isolados nos cantos vazios de seis folhas foram removidos; coordenadas e contagens ficam em [alpha-corner-cleanup.json](alpha-corner-cleanup.json). Esses pixels não pertenciam ao corpo; um deles contaminava o limite alpha do jump sem representar pé abaixo do chão.

Cada ação usa um único fator de escala. Pivôs aéreos alinham a cintura e mantêm a trajetória sob controle do World. AirPunch inclui 32 px transparentes abaixo da folha para conter o pivô virtual ativo, preservando todos os pixels da fonte. A normalização não estica poses individualmente.

## Inspeção de combate

A primeira captura completa do Lab produziu 263 PNGs em 19 contextos com manifesto visual idêntico ao candidato daquela execução. A inspeção dos nove contatos encontrou o jab v4 alto demais; v5 baixa o braço ativo sem alterar os pés, deslocamento físico ou timing. Heavy, overhead, anti-air, air-punch, air-kick e throw mostram mão/pé dentro de parte da caixa existente. Kick cruza com tornozelo/sapato a parte inferior da caixa; sweep usa canela/tornozelo enquanto a sola fica acima. Essas duas leituras continuam candidatas a refinamento.

O especial já emite no primeiro frame. A boca da cobra foi desenhada para alcançar o ponto baseline (+205,33; −157,33), seguida de recolhimento e retorno à guarda. Esse alinhamento foi estimado na fonte. A captura revelou que a textura antiga de projétil era um recorte da personagem caída. Foi gerado um data stream azul/amarelo separado, ativado apenas com o opt-in dos candidatos. Canvas de 144 × 68 corresponde a 86,4 × 40,8 px na fórmula de desenho existente; o envelope visual antigo inadequado foi substituído deliberadamente, e hitbox de 85,33 × 40 px, velocidade, dano, emissão e origem continuam intactos.

As capturas anteriores do Lab criavam o projétil por `Projectile::from_fighter`, sem consultar a origem baseline. Elas documentam a textura e as poses, mas não comprovam a origem usada no World.

A [recaptura corrigida do especial](../../candidates/python/review/runtime-baseline-special/README.md), com 16 amostras e ambos os manifestos conferidos, confirma a origem baseline e o efeito de dados saindo da boca da cobra. O recorte antigo da personagem caída não aparece; essa pendência técnica foi encerrada. Os nove golpes usam MoveSpec, portanto a inspeção de seus contatos continua válida. Python é o único personagem com projétil novo neste lote; Rust, Duke, Go, C e C++ reaproveitam os cinco projéteis originais.

## Continuidade e limitações

O cenário World conferiu entrada/contagem, avanço/recuo, retorno ao idle e salto completo nas duas orientações, com HP 96 preservado. A cabeça mantém posição coerente entre subida/queda e as pernas se estendem antes do pouso. A primeira sequência completa fica em `target/sprite-production/python-motion-final`; a captura limpa final e o projétil corrigido são registrados na cobertura global.

Idle tem respiração muito sutil. Walk seleciona contato/passagem/contato/passagem, mas a alternância das pernas ainda é pouco clara em algumas poses. Spawn preserva o cavalete/gráfico, com variação de aproximadamente 6% na altura do primeiro desenho. O cavalete desaparece ao terminar a intro conforme o fluxo atual; não foi acrescentado objeto persistente de cenário. Fontes ainda apresentam variações discretas de rosto, mãos e cabeça/cauda da cobra; vitória v2 remove uma cabeça duplicada que aparecia na cauda.

As capturas determinísticas não são um playtest manual de teclado. Resultado, guardas e reação foram examinados no Lab e os relógios compartilhados têm regressões automatizadas. O conjunto continua candidato artístico.
