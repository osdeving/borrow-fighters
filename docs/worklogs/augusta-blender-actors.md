# Augusta — humanos e veículos 3D no Blender

## Pedido e limites

Em 12/09/2026 o usuário autorizou baixar e instalar Blender e substituir os
humanos e carros da Augusta por modelos 3D. A cena, cenário, enquadramentos,
sequência, movimentos e identidade visual devem permanecer os mesmos. O objetivo
é corrigir volume, articulação e orientação dos corpos, não redesenhar o capítulo.

## Retomada

- Branch: `feat/augusta-blender-actors`; base limpa `27ca4bb`.
- Leia este diário e inspecione status/log antes de repetir trabalho.
- A versão pintada e sua evidência permanecem como referência comparativa.
- Root coordena commits e integração; não descartar mudanças dos agentes.
- Skills repo-atlas, art-direction e rust-gamedev; README, docs 07/08/38/39,
  ADRs 0001/0003/0021/0032/0033 consultados.

## Trabalho em andamento

- Root: instalação Blender, continuidade, animação, docs/pacote e modelo security.
- `cpp_arm`: fontes Blender, geometria/rig/identidade dos humanos; piloto C++.
- `augusta_staging`: veículos concluídos; entregador humano sobre a scooter.
- `augusta_ambient`: carregamento/renderização de GLB nas câmeras existentes.

Modelos usam metros; Blender Z para cima e frente -Y, exportados em glTF com
Y para cima e frente +Z. Pivô no chão. GLB deve conter materiais e animações;
arquivos `.blend` e scripts são fontes de autoria e não dependências do jogo.

## Instalação

Blender 4.5.4 LTS Linux x64 baixado do domínio oficial `download.blender.org`.
Arquivo e checksums em `/tmp/borrow-blender-install`. Instalação local em
`/home/willams/.local/opt/blender-4.5.4-linux-x64`, comando
`/home/willams/.local/bin/blender`; validação SHA-256 antes da extração.

- SHA-256 confirmado: `2e6ef8e99fc36327270429ddc8e7bad2859dd878a5a137d2e0bf0f02f6792505`.
- `blender --version`: 4.5.4 LTS, build `b3efe983cc58`.
- Render headless Cycles CPU concluído com sucesso em
  `/tmp/borrow-blender-install/smoke.png`; instalação funcional.
- Launcher desktop: `/home/willams/.local/share/applications/blender-borrow.desktop`.
- Baseline de continuidade: `docs/evidence/augusta-3d-pilot/baseline.json`
  registra SHA-256 de onze arquivos de cenário, história e câmera.

## Checkpoint — piloto nativo e animação

- C++ e os três tipos de veículo carregaram como GLB no próprio Raylib;
  piloto ativado por `BORROW_AUGUSTA_MODELS_3D=1`. A pintura anterior continua
  disponível como referência enquanto o elenco é ampliado.
- Captura nativa de 720 quadros em `/tmp/augusta-cpp3d-opening-v5`; comparação
  com o filme anterior confirmou os onze hashes fixos e todos os campos
  originais da telemetria nos 720 quadros. Isso verifica continuidade de
  estado/cenário, não aprova a qualidade dos modelos.
- Laboratório nativo `/tmp/augusta-cpp3d-lab-v5`: 120 quadros, 40 folhas e seis
  ampliações; mostrou necessidade de corrigir botas/cadarços e braços na corrida.
- `tools/blender/actor_motion.py` resolve braços/pernas sobre o esqueleto real,
  conserva durações da C++, exporta 60 quadros/s e separa movimento local da
  orientação/rota já existente. Apoios usam a distância percorrida no runtime.
- Medição de 65 fases por ação em doze ações: alvos de mãos e tornozelos
  alcançados com erro abaixo de 0,000001m. Revisão visual continua necessária
  para roupa, cabelo, contato e interpenetração. Corrida e dedos em refinamento.
- `tools/blender/render_motion.py` permite renderizar somente ação/fases
  escolhidas no Blender, sem executar campanha nem exportar todos os modelos.
- Empacotador passa a seguir catálogos GLB, exigir texturas/buffers embutidos e
  excluir `.blend` da distribuição. Doze testes de referências passaram.
- Integração Rust teve build release, fmt, Clippy focado e três testes de
  catálogo/tempo/IK aprovados pelo agente; validação final aguarda elenco completo.
- Fontes humanas: base/rig MPFB e roupas CC0 selecionadas, com URLs/hashes em
  `assets/adventure/production-3d/humans/provenance.json`; ferramenta externa
  em `/tmp/borrow-fighters-mpfb2`, pacotes em `/tmp/borrow-fighters-mh-assets`.

## Checkpoint — modelos e export isolado

- Commits: `6825f7b` (decisão/baseline), `190124c` (movimento/export/pacote),
  `188ea36` (veículos Blender e validação).
- CPP geometria v6 + movimento v6 exportados com trinta ações estáveis,
  mãos fechadas, cotovelos baixos na guarda e contrapasso na corrida.
  Laboratório `/tmp/augusta-cpp3d-lab-v6` aprovado para integração do par.
- `export_actions.py` reexporta apenas movimentos de uma fonte `.blend`
  pronta. Fonte salva com compressão nativa: C++ reduziu de 106 MiB para
  26 MiB sem remover geometria, texturas ou ações. `.blend1` é backup local;
  `.obj` de produção tem exceção explícita ao ignore de objetos nativos.
- `validate_humans.py` examina buffers/ossos/pesos/ações a 60 Hz sem Blender
  ou jogo. CPP: 116551 vértices, 217364 triângulos; Julia e broker drafts
  também passaram. Aprovação geométrica não substitui revisão de figurino.
- Veículos finalizados: três fontes, três GLBs, nove ângulos; checagem GLB
  e reimportação/skin Blender aprovadas. Corpo permanece imóvel e as rodas
  fecham o ciclo com erro inferior a 0,000001m. Fontes/reports junto aos veículos.
- Julia/broker em ajuste de roupa antes de liberar captura da pegada.
  Oito figurinos com cpp_arm; security com root; rider com staging.
- Entregador tem pernas e assento conferidos; staging ajusta apenas mãos e
  direção da cabeça no ramo `riding` de `actor_motion.py`, coordenado com root.
- Telefone 3D acompanha `hand_r` só nos dois figurantes que telefonam;
  cinco testes focais e Clippy passaram. Próximo build incorpora essa mudança.
- Revalidar e capturar a cena completa somente depois de estabilizar elenco,
  contato e materiais. Fontes fixas do cenário/domínio continuam sem alteração.

## Próximos marcos

1. Concluir correções de roupa/cabelo/botas e movimentos da C++ no piloto.
2. Comparar close e corrida nativos usando a exportação atualizada.
3. Completar humanos/carros, poses/contatos e reprodução no gameplay.
4. Rever quadros consecutivos e tomadas equivalentes; validar fontes e pacote.
5. Executar fmt, Clippy e testes pertinentes à integração, registrar resultados
   e commitar etapas coerentes. Não confundir uma renderização bonita isolada
   com o resultado dentro do jogo.

## Checkpoint — elenco completo e contato

- Catálogo humano agora contém treze entradas: C++, Julia, broker, security,
  oito figurinos e entregador. GLBs somam aproximadamente 255 MiB com os três
  veículos; fontes `.blend` e texturas selecionadas ficam fora do pacote.
- Piloto nativo de 1900 quadros em `/tmp/augusta-3d-pair-pilot-v7` preservou
  os onze hashes e todos os campos originais de telemetria. A pegada falhou
  visualmente: os braços externos não alcançavam o contato sem esticar.
- Corrigidos para os braços internos (broker direito/Julia esquerdo), com
  deslocamento anatômico da palma. Dezoito quadros focais nativos em
  `/tmp/augusta-3d-grip-internal-v1`, incluindo 744–746 e 1519–1521, mostraram
  contato contínuo e telemetria idêntica. Não foram movidos atores ou câmera.
- Os carros cobrem parcialmente `cpp-intervenes` em 27,9 e 29,4 segundos;
  a mesma passagem já existe no filme pintado. Câmera/faixas foram mantidas
  conforme a exigência de continuidade.
- Entregador: glTF reimportado em cinco fases, palmas a cerca de 21 mm dos
  centros das manoplas (raio 24 mm), solas a 0,255 mm do apoio. Animação
  fecha sem deslocamento do veículo; relatório junto à fonte do entregador.
- Revisão isolada encontrou mão acima da cabeça em `phone` e dedos dentro
  da saia em `seated`. `actor_motion.py` agora coloca o punho na mandíbula,
  cotovelo baixo e dedos sobre as coxas. Renders em
  `/tmp/augusta-crowd-motion-v2`; sete crowd em reexportação de ações apenas.
- CPP/security: aros circulares atravessavam punhos. Staging ajusta aros à
  seção anatômica existente, preservando curvas e demais malhas por hashes;
  ajusta também aba cargo esquerda em 2 mm. LongCoat recebe máscara de pele
  sob a manga no cotovelo. CPP/Julia recebem ajuste isolado de raiz de cabelo.
- Laboratório aceita `--models`, preserva fase ao ampliar e guarda catálogo
  e GLBs em snapshot portátil. Teste de reabertura removeu as fontes originais.
- `python3 -m unittest discover -s tools/release -p test_package.py`: 37 testes
  passaram. Créditos MakeHuman selecionados acompanham o pacote em
  `assets/adventure/models/NOTICES.md`.
- Próximo: congelar correções pontuais, validar os treze GLBs finais, rever
  crowd nativo e filmar os 8238 quadros. Rust final e pacote portátil pendentes.
