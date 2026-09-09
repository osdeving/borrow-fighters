"""Prepare selected, genuinely generated Duke drawings without painting anatomy.

All transformations are alpha extraction, uniform size normalization, cropping,
and placement. Versioned sources stay unchanged. Runtime timings stay unchanged.
"""

import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFilter


ROOT = Path(__file__).resolve().parent


def clear_gap(image, seed):
    """Remove a visually reviewed enclosed neutral backdrop gap, not costume."""
    rgba = np.asarray(image).copy()
    rgb = rgba[:, :, :3].astype(np.int16)
    neutral = (rgb.min(axis=2) >= 210) & (rgb.max(axis=2)-rgb.min(axis=2) <= 20)
    mask = Image.fromarray(np.where(neutral, 255, 0).astype(np.uint8))
    assert neutral[seed[1], seed[0]], seed
    ImageDraw.floodfill(mask, seed, 128)
    gap = np.asarray(mask) == 128
    rgba[gap] = 0
    for _ in range(2):
        edge = np.asarray(Image.fromarray((gap*255).astype(np.uint8)).filter(ImageFilter.MaxFilter(3))) > 0
        fringe = edge & (rgb.min(axis=2) > 85) & (rgb.max(axis=2)-rgb.min(axis=2) < 30)
        rgba[fringe] = 0
        gap |= fringe
    return Image.fromarray(rgba)


def compose(action, entries, output):
    canvas = Image.new('RGBA', (800*len(entries), 900))
    frames = []
    records = []
    for i, entry in enumerate(entries):
        filename, rect, factor, pivot, duration, phase, seeds = entry
        source = Image.open(ROOT/action/filename).convert('RGBA')
        for seed in seeds:
            source = clear_gap(source, seed)
        crop = source.crop(rect)
        art = crop.resize((round(crop.width*factor), round(crop.height*factor)), Image.Resampling.LANCZOS)
        local_pivot = (round(pivot[0]*factor), round(pivot[1]*factor))
        offset = (i*800+350-local_pivot[0], 780-local_pivot[1])
        assert offset[0] >= i*800 and offset[1] >= 0
        assert offset[0]+art.width <= (i+1)*800 and offset[1]+art.height <= 900
        canvas.alpha_composite(art, offset)
        frames.append({'name':f'{action}_{i:02d}','source_rect':{'x':i*800,'y':0,'w':800,'h':900},'pivot':{'x':350,'y':780},'duration_ms':duration,'phase':phase})
        records.append({'source':filename,'source_rect_xyxy':rect,'uniform_normalization':factor,'source_local_pivot':pivot,'destination_offset':offset,'duration_ms':duration,'phase':phase,'reviewed_background_gap_seeds':seeds})
    canvas.save(ROOT/action/output)
    (ROOT/action/'composition-v1.json').write_text(json.dumps({'operation':'Composition of real generated full poses; no invented/repeated frames or anatomy painting','runtime_scale':0.4169435215946844,'frames':records},indent=2)+'\n')
    (ROOT/action/'action.json').write_text(json.dumps({'sheet':f'{action}/{output}','reviewed':True,'loop':action=='walk','frames':frames},indent=2)+'\n')


if __name__ == '__main__':
    compose('walk', [
        ('keyed-v5.png',(0,0,512,768),1.0,(307,683),112,'near_leg_contact',[(196,443)]),
        ('keyed-v5.png',(512,0,1024,768),1.0,(274,683),113,'far_leg_passage',[(665,445)]),
        ('keyed-v7.png',(0,0,1086,1448),0.49,(535,1287),112,'far_leg_contact',[]),
        ('keyed-v6.png',(0,0,1086,1449),0.49,(565,1325),113,'near_leg_passage',[(296,817)]),
    ], 'selected-v1.png')
    compose('overhead', [
        ('keyed-v6.png',(0,0,682,768),1.0,(327,661),250,'startup',[(198,432)]),
        ('keyed-v8.png',(0,0,1086,1448),0.53,(490,1274),133,'active',[(251,852)]),
        ('keyed-v6.png',(1365,0,2048,768),1.0,(327,661),283,'recovery',[(1566,432)]),
    ], 'selected-v1.png')
    plan_path = ROOT/'review-plan.json'
    plan = json.loads(plan_path.read_text())
    for action in ('walk','overhead','crouch_block'):
        plan['actions'][action]['scale_to_runtime'] = 0.4169435215946844
    plan['actions']['walk']['notes'] = 'Four genuinely different generated poses assembled from v5/v7/v6. v7 preserves opposite-contact v4 with safe magenta matte; checker extraction of v4 damaged white belly and was rejected. Alternating near/far contact and passage. Single-pose sources normalized uniformly0.49, then common runtime scale0.4169435. Original112/113/112/113ms total450ms.'
    plan['actions']['overhead']['notes'] = 'New isolated v8 contact has elbow above descending glove inside original box; v6 startup/recovery preserved. Uniform0.53 normalization of isolated contact, then common runtime0.4169435. Original250/133/283ms total666ms.'
    plan['actions']['crouch_block']['notes'] = 'New v2 compressed low guard. Original nose/glove/feet scale retained; belly compressed and cone inclined. Original80/100/180ms total360ms.'
    spec_path = ROOT/'crouch_block/action.json'
    spec = json.loads(spec_path.read_text())
    spec['sheet'] = 'crouch_block/keyed-v2.png'
    for frame, pivot in zip(spec['frames'],[(370,681),(342,681),(277,681)]):
        frame['pivot'] = {'x':pivot[0],'y':pivot[1]}
    spec_path.write_text(json.dumps(spec,indent=2)+'\n')
    plan['provenance']['refinement'] = 'Targeted real image_gen revisions; other satisfactory actions reused. Rejected source variants and old action/plan snapshots preserved.'
    plan_path.write_text(json.dumps(plan,indent=2)+'\n')
