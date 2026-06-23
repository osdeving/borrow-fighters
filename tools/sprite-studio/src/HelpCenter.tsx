type HelpCenterProps = {
  open: boolean;
  onClose: () => void;
};

export function HelpCenter({ open, onClose }: HelpCenterProps) {
  if (!open) {
    return null;
  }

  return (
    <div className="help-backdrop" role="dialog" aria-modal="true">
      <section className="help-window">
        <header className="help-header">
          <div>
            <h2>Guia do Sprite Studio</h2>
            <p>
              Como abrir um atlas, ajustar escala, pivot e caixas de combate,
              salvar o manifesto e entender o que muda no jogo.
            </p>
          </div>
          <button type="button" onClick={onClose}>
            Close
          </button>
        </header>

        <div className="help-content">
          <article>
            <h3>Como o Studio funciona</h3>
            <p>
              O Studio não edita o PNG do atlas. Ele abre um arquivo
              <code> *.sprite.json</code>, lê o PNG apontado por
              <code> image</code> e salva dados que o runtime do jogo consome
              depois: escala, pivot, duração de frames, hurtboxes, hitboxes e
              origem de projétil.
            </p>
            <DataFlowDiagram />
            <div className="help-callout">
              <strong>Fonte de verdade:</strong> quando você salva o manifesto
              real, o próximo carregamento do jogo, Combat Lab ou Sprite Viewer
              usa esses valores. Não precisa recompilar para trocar JSON ou PNG,
              mas precisa reiniciar/recarregar a cena que já estava aberta.
            </div>
          </article>

          <article>
            <h3>Casos de uso comuns</h3>
            <div className="workflow-grid">
              <WorkflowCard
                title="Carregar algo pronto"
                steps={[
                  "Use os presets Rust, Duke, Go ou C.",
                  "Escolha um clip na lateral ou um frame na timeline.",
                  "Confira pivot, escala e marcadores de combat metadata.",
                ]}
              />
              <WorkflowCard
                title="Criar um manifesto novo"
                steps={[
                  "Hoje não existe botão New.",
                  "Copie um manifesto parecido, troque image, frames e clips.",
                  "Abra o novo JSON pelo campo Path ou por Open.",
                ]}
              />
              <WorkflowCard
                title="Ajustar um golpe"
                steps={[
                  "Vá ao clip do golpe.",
                  "Escolha modo Hitbox ou Hurtbox e clique no canvas.",
                  "Arraste a caixa, ajuste alças e valide antes de salvar.",
                ]}
              />
              <WorkflowCard
                title="Preparar review"
                steps={[
                  "Rode Run Runtime Validation.",
                  "Exporte PNG Review para mostrar a imagem anotada.",
                  "Exporte Review JSON com notas e dados do frame.",
                ]}
              />
            </div>
          </article>

          <article>
            <h3>O que cada caixa significa</h3>
            <p>
              Em jogo de luta, o sprite é a imagem. O gameplay usa retângulos
              invisíveis. Eles não precisam copiar o desenho perfeitamente; eles
              precisam parecer justos e legíveis para quem joga.
            </p>
            <BoxDictionaryDiagram />
            <div className="concept-grid">
              <ConceptCard
                name="Pushbox / collision box"
                text="Corpo físico do personagem. Define espaço ocupado, empurrão corpo-corpo, chão e limites de arena. No Studio ela vem de assets/tuning/character-body-metrics.json, não de frames[].combat."
              />
              <ConceptCard
                name="Hurtbox"
                text="Área vulnerável. Se a hitbox inimiga tocar aqui, o personagem apanha. Pode mudar por frame, por exemplo em crouch, pulo ou chute."
              />
              <ConceptCard
                name="Hitbox"
                text="Área ofensiva do golpe. Só deve existir nos frames ativos. Hitbox grande demais vira golpe fantasma; pequena demais faz o golpe parecer quebrado."
              />
              <ConceptCard
                name="Pivot"
                text="Âncora do frame. No jogo, prende o desenho ao corpo lógico. Normalmente fica no chão, entre os pés ou no ponto de apoio."
              />
              <ConceptCard
                name="Bounds / source crop"
                text="Retângulos de leitura do atlas. Ajudam a ver recorte, espaço vazio e faixa visual, mas não causam dano nem colisão por si só."
              />
              <ConceptCard
                name="Projectile origin"
                text="Ponto de nascimento do projétil. Ele alinha o poder com mão, arma ou emissor visual. A caixa de dano do projétil ainda vem do ProjectileSpec em Rust."
              />
            </div>
          </article>

          <article>
            <h3>Exemplo: hitbox de golpe</h3>
            <p>
              Se você carrega um manifesto, escolhe o frame ativo de um chute e
              ajusta a caixa vermelha para cobrir só o pé, isso significa que o
              golpe só acerta quando essa área tocar a hurtbox do oponente. A
              perna desenhada pode passar um pouco da caixa, mas a parte que
              causa dano precisa bater com o que o jogador entende como impacto.
            </p>
            <StrikeExampleDiagram />
            <ul>
              <li>
                Caixa vermelha longe do pé: o golpe acerta antes de encostar.
              </li>
              <li>
                Caixa vermelha curta demais: o pé atravessa o inimigo sem hit.
              </li>
              <li>
                Hurtbox verde no braço/perna esticada cria risco de punição.
              </li>
            </ul>
          </article>

          <article>
            <h3>Exemplo: projétil</h3>
            <p>
              No fluxo atual, o Studio edita a origem do projétil no frame do
              personagem. A caixa ofensiva do projétil não está no
              <code> *.sprite.json</code>; ela vem de
              <code> ProjectileSpec</code> no código Rust, com largura, altura,
              dano, velocidade e stun. Por isso você pode ver uma cruz roxa no
              frame <code> special</code>, mas não uma hitbox vermelha no
              projectile.
            </p>
            <ProjectileExampleDiagram />
            <div className="help-callout">
              Se você mover a origem roxa para a mão, o projétil nasce mais
              alinhado com a animação. Se quiser mudar o tamanho da caixa que dá
              dano no projétil, isso ainda é tuning de <code> ProjectileSpec</code>
              e não edição visual do manifesto.
            </div>
          </article>

          <article>
            <h3>Por que uma caixa não aparece?</h3>
            <p>
              Nem todo frame já tem metadata própria. Isso não significa
              necessariamente erro de carregamento.
            </p>
            <ul>
              <li>
                Se o frame não tem <code>frames[].combat.hitboxes</code>, o
                Studio não desenha hitbox vermelha para ele.
              </li>
              <li>
                Se o frame não tem <code>frames[].combat.hurtboxes</code>, o
                jogo usa a hurtbox base gerada pelo corpo físico do personagem.
              </li>
              <li>
                Se o frame tem só <code>projectile_origin</code>, ele serve para
                posicionar o disparo, não para definir a caixa de dano do
                projétil.
              </li>
              <li>
                Nos placeholders atuais, Rust já tem algumas hitboxes de
                soco/chute no manifesto; Duke, Go e C ainda dependem mais de
                fallback runtime.
              </li>
            </ul>
          </article>

          <article>
            <h3>Salvar e ver no jogo</h3>
            <p>
              Save JSON escreve o manifesto aberto. Save Metrics escreve o
              arquivo de corpo físico. Esses dois arquivos têm efeitos
              diferentes no runtime.
            </p>
            <SaveFlowDiagram />
            <div className="concept-grid">
              <ConceptCard
                name="*.sprite.json"
                text="Afeta atlas, clips, duração, scale, pivot, hitboxes, hurtboxes e origem de projétil por frame."
              />
              <ConceptCard
                name="character-body-metrics.json"
                text="Afeta largura do corpo, altura em pé e altura abaixada. Isso guia pushbox, colisão corpo-corpo e hurtbox base."
              />
              <ConceptCard
                name="Autosave"
                text="Grava rascunho em target/sprite-studio-autosave/. Não altera o manifesto real usado pelo jogo."
              />
              <ConceptCard
                name="Backup"
                text="Antes de salvar o arquivo real, o Studio copia a versão anterior para target/sprite-studio-backups/."
              />
            </div>
          </article>

          <article>
            <h3>Comandos principais</h3>
            <div className="shortcut-grid">
              <Shortcut label="Selecionar frame" value="Clique na timeline" />
              <Shortcut label="Frame anterior/próximo" value="← / →" />
              <Shortcut label="Zoom" value="Mouse wheel no canvas ou +/-" />
              <Shortcut label="Ajuda" value="F1" />
              <Shortcut label="Criar box" value="Modo Hurtbox/Hitbox + clique" />
              <Shortcut label="Mover item" value="Arrastar pivot, origem ou box" />
              <Shortcut label="Redimensionar box" value="Arrastar as alças" />
              <Shortcut label="Undo" value="Ctrl+Z" />
              <Shortcut label="Redo" value="Ctrl+Shift+Z ou Ctrl+Y" />
              <Shortcut label="Salvar manifesto" value="Ctrl+S" />
            </div>
          </article>
        </div>
      </section>
    </div>
  );
}

function WorkflowCard({
  title,
  steps,
}: {
  title: string;
  steps: string[];
}) {
  return (
    <div className="workflow-card">
      <strong>{title}</strong>
      <ol>
        {steps.map((step) => (
          <li key={step}>{step}</li>
        ))}
      </ol>
    </div>
  );
}

function ConceptCard({ name, text }: { name: string; text: string }) {
  return (
    <div className="concept-card">
      <strong>{name}</strong>
      <span>{text}</span>
    </div>
  );
}

function Shortcut({ label, value }: { label: string; value: string }) {
  return (
    <div className="shortcut">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function DataFlowDiagram() {
  return (
    <svg className="help-diagram" viewBox="0 0 760 260" role="img">
      <rect className="diagram-bg" x="0" y="0" width="760" height="260" />
      <rect className="diagram-card" x="36" y="72" width="144" height="86" />
      <rect className="diagram-card" x="222" y="72" width="144" height="86" />
      <rect className="diagram-card accent" x="408" y="54" width="156" height="122" />
      <rect className="diagram-card" x="606" y="72" width="118" height="86" />
      <path className="diagram-arrow" d="M180 115 L222 115" />
      <path className="diagram-arrow" d="M366 115 L408 115" />
      <path className="diagram-arrow" d="M564 115 L606 115" />
      <text x="62" y="106">atlas PNG</text>
      <text x="58" y="132">imagem</text>
      <text x="248" y="106">*.sprite.json</text>
      <text x="250" y="132">dados</text>
      <text x="436" y="94">Sprite Studio</text>
      <text x="434" y="124">edita JSON</text>
      <text x="430" y="152">valida/exporta</text>
      <text x="626" y="106">jogo</text>
      <text x="620" y="132">runtime</text>
    </svg>
  );
}

function BoxDictionaryDiagram() {
  return (
    <svg className="help-diagram" viewBox="0 0 760 300" role="img">
      <rect className="diagram-bg" x="0" y="0" width="760" height="300" />
      <line className="diagram-floor" x1="58" y1="246" x2="704" y2="246" />
      <rect className="diagram-body" x="298" y="74" width="112" height="172" />
      <rect className="diagram-push" x="315" y="92" width="76" height="154" />
      <rect className="diagram-hurt" x="324" y="88" width="58" height="142" />
      <rect className="diagram-hit" x="392" y="118" width="128" height="42" />
      <circle className="diagram-projectile" cx="510" cy="139" r="10" />
      <circle className="diagram-pivot" cx="354" cy="246" r="9" />
      <text x="300" y="68">sprite</text>
      <text x="418" y="146">hitbox</text>
      <text x="228" y="116">hurtbox</text>
      <text x="218" y="210">pushbox</text>
      <text x="376" y="270">pivot</text>
      <text x="526" y="126">origem</text>
    </svg>
  );
}

function StrikeExampleDiagram() {
  return (
    <svg className="help-diagram" viewBox="0 0 760 300" role="img">
      <rect className="diagram-bg" x="0" y="0" width="760" height="300" />
      <line className="diagram-floor" x1="54" y1="244" x2="706" y2="244" />
      <g transform="translate(120 0)">
        <rect className="diagram-body" x="96" y="76" width="82" height="168" />
        <rect className="diagram-hurt" x="108" y="86" width="58" height="145" />
        <rect className="diagram-hit ghost" x="206" y="120" width="154" height="48" />
        <text x="114" y="62">ruim: fantasma</text>
      </g>
      <g transform="translate(450 0)">
        <rect className="diagram-body" x="20" y="76" width="82" height="168" />
        <rect className="diagram-hurt" x="32" y="86" width="58" height="145" />
        <rect className="diagram-hit" x="92" y="132" width="96" height="40" />
        <text x="28" y="62">bom: cobre impacto</text>
      </g>
    </svg>
  );
}

function ProjectileExampleDiagram() {
  return (
    <svg className="help-diagram" viewBox="0 0 760 300" role="img">
      <rect className="diagram-bg" x="0" y="0" width="760" height="300" />
      <line className="diagram-floor" x1="54" y1="244" x2="706" y2="244" />
      <rect className="diagram-body" x="176" y="82" width="94" height="162" />
      <rect className="diagram-hurt" x="188" y="96" width="66" height="132" />
      <circle className="diagram-projectile" cx="300" cy="132" r="10" />
      <path className="diagram-arrow" d="M310 132 C362 116 398 116 452 132" />
      <rect className="diagram-hit" x="454" y="114" width="78" height="36" />
      <text x="274" y="104">origem no frame</text>
      <text x="442" y="98">caixa runtime</text>
      <text x="462" y="168">ProjectileSpec</text>
    </svg>
  );
}

function SaveFlowDiagram() {
  return (
    <svg className="help-diagram" viewBox="0 0 760 260" role="img">
      <rect className="diagram-bg" x="0" y="0" width="760" height="260" />
      <rect className="diagram-card accent" x="54" y="54" width="154" height="122" />
      <rect className="diagram-card" x="270" y="34" width="174" height="76" />
      <rect className="diagram-card" x="270" y="140" width="174" height="76" />
      <rect className="diagram-card" x="516" y="86" width="180" height="92" />
      <path className="diagram-arrow" d="M208 94 L270 72" />
      <path className="diagram-arrow" d="M208 136 L270 178" />
      <path className="diagram-arrow" d="M444 72 L516 116" />
      <path className="diagram-arrow" d="M444 178 L516 140" />
      <text x="82" y="96">Save</text>
      <text x="76" y="126">no Studio</text>
      <text x="292" y="70">*.sprite.json</text>
      <text x="292" y="94">visual/frame combat</text>
      <text x="292" y="176">body metrics</text>
      <text x="292" y="200">pushbox/base</text>
      <text x="544" y="122">recarregar jogo</text>
      <text x="552" y="148">ou laboratório</text>
    </svg>
  );
}
