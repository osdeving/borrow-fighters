"""Register six generated Go actions and render staging-only diagnostics.

Preparation only: explicit crops, transparent margins and pivots. No anatomy
painting, per-pose scaling, runtime export or combat/timing modification.
"""

import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
OUTPUT = REPO/'target/art/go-replacement-b'

# Crops refer to the exact generated sheet. Pivots use the same global source
# coordinates before transparent margin placement. Height is measured once per
# action from a normal upright/chamber pose, not from the active action's alpha.
CONFIG = {
    'punch_light': {'version':4, 'cuts':[0,570,1260,1882], 'pivots':[(274,786),(899,786),(1503,786)], 'height':754, 'reference_frame':0, 'contact':[1095,398,1158,435], 'notes':'Short bent-elbow body jab. v1 was too high/long; v2 had no reach; v3 was still high. v4 selects the final compact contact.'},
    'punch_heavy': {'version':2, 'cuts':[0,570,1320,1881], 'pivots':[(301,785),(896,774),(1568,785)], 'height':725, 'reference_frame':0, 'contact':[1232,415,1299,475], 'notes':'Low body cross with visible weight transfer. v2 lowered the fist; normal anatomy scale is shared across all three poses.'},
    'kick': {'version':1, 'cuts':[0,600,1260,1881], 'pivots':[(267,777),(880,777),(1495,777)], 'height':742, 'reference_frame':0, 'contact':[1164,550,1213,645], 'notes':'Low front kick with readable heel/sole pads, chamber and recoil. Supporting paw stays at the ground anchor.'},
    'sweep': {'version':2, 'cuts':[0,627,1321,1881], 'pivots':[(300,746),(867,728),(1560,770)], 'height':712, 'reference_frame':2, 'contact':[1210,400,1318,465], 'notes':'Supported elevated sweep; v2 lowers and turns the striking foot to the existing box. Perspective contact plane lies between planted palm and rear claws (about3px above/below runtime ground). v3 was rejected because it weakened fur detail and did not materially improve support.'},
    'overhead': {'version':1, 'cuts':[0,610,1270,1881], 'pivots':[(340,798),(957,795),(1524,795)], 'height':701, 'reference_frame':2, 'contact':[1122,427,1184,493], 'notes':'Raised windup, descending hammerfist, guard recovery. Body stays the same size despite the raised hand extending above the head.'},
    'throw': {'version':2, 'cuts':[0,610,1230,1880], 'pivots':[(300,811),(890,811),(1510,811)], 'height':780, 'reference_frame':0, 'contact':[1088,270,1130,331], 'notes':'Two separated bent-arm grips. v2 lowered and extended the leading grasp into the short throw range; rear support hand naturally sits behind the contact hand.'},
}


def run():
    OUTPUT.mkdir(parents=True, exist_ok=True)
    report = {'status':'six staged actions reviewed statically; integrator still runs final Lab/World', 'generator':'built-in image_gen', 'master':'reference/master.png', 'candidate_exported':False, 'actions':{}}
    for action, cfg in CONFIG.items():
        folder = ROOT/action
        spec = json.loads((folder/'action-planning.json').read_text())
        image = Image.open(folder/f'keyed-v{cfg["version"]}.png').convert('RGBA')
        # The last jab generation has isolated matte pixels at empty canvas
        # corners. These bands are outside every reviewed figure and whisker.
        if action == 'punch_light':
            draw = ImageDraw.Draw(image)
            for rect in [(0,0,24,image.height),(1830,0,image.width,image.height),(0,800,image.width,image.height)]:
                draw.rectangle(rect,fill=(0,0,0,0))
            image.save(folder/'keyed-v4-clean.png')
        canvas = Image.new('RGBA',(2400,920))
        scale = 264/cfg['height']
        mappings = []
        for i, (frame,pivot,x0,x1) in enumerate(zip(spec['frames'],cfg['pivots'],cfg['cuts'],cfg['cuts'][1:])):
            art = image.crop((x0,0,x1,image.height))
            dx = (800-art.width)//2
            dy = 42
            canvas.alpha_composite(art,(i*800+dx,dy))
            frame['source_rect'] = {'x':i*800,'y':0,'w':800,'h':920}
            frame['pivot'] = {'x':pivot[0]-x0+dx,'y':pivot[1]+dy}
            mappings.append({'source_rect_xyxy':[x0,0,x1,image.height],'source_global_pivot':list(pivot),'destination_offset':[i*800+dx,dy],'prepared_local_pivot':frame['pivot'],'duration_ms':frame['duration_ms'],'phase':frame['phase']})
        canvas.save(folder/'sheet-v1.png')
        spec.update(source=f'{action}/source-v{cfg["version"]}.png',sheet=f'{action}/sheet-v1.png',reviewed=True,status='staged artwork; static review complete; runtime review by integrator',source_body_height_reference_px=cfg['height'],suggested_scale_to_runtime=scale,scale_to_runtime=scale)
        spec['references']={'new_master':'reference/master.png','new_idle':None,'old_pose_art_allowed':False}
        spec['review_notes']=cfg['notes']+' Explicit crops/pivots are recorded in preparation.json; no timing or combat changes.'
        (folder/'action.json').write_text(json.dumps(spec,indent=2)+'\n')
        source_file=folder/f'source-v{cfg["version"]}.png'
        preparation={'operation':'Cropping and transparent margin placement only; one uniform runtime scale per action','source':source_file.name,'source_sha256':hashlib.sha256(source_file.read_bytes()).hexdigest(),'source_size':list(image.size),'sheet':'sheet-v1.png','sheet_size':list(canvas.size),'scale_to_runtime':scale,'source_body_height_reference_px':cfg['height'],'height_reference_frame':cfg['reference_frame'],'frames':mappings}
        (folder/'preparation.json').write_text(json.dumps(preparation,indent=2)+'\n')
        cards=[]
        for i,frame in enumerate(spec['frames']):
            sprite=canvas.crop((i*800,0,(i+1)*800,920))
            sprite=sprite.resize((round(sprite.width*scale),round(sprite.height*scale)),Image.Resampling.LANCZOS)
            px,py=round(frame['pivot']['x']*scale),round(frame['pivot']['y']*scale)
            card=Image.new('RGB',(960,430),(23,28,37));d=ImageDraw.Draw(card);d.rectangle((480,0,960,430),fill=(211,214,212))
            for center,flip in [(220,False),(740,True)]:
                art=sprite.transpose(Image.Transpose.FLIP_LEFT_RIGHT) if flip else sprite
                x=center-(sprite.width-px) if flip else center-px
                card.paste(art,(x,385-py),art)
                if frame['phase']=='active':
                    side='left' if flip else 'right';box=spec['timing_reference'][f'active_box_{side}_relative_to_anchor']
                    d.rectangle((center+box['x'],385+box['y'],center+box['x']+box['w'],385+box['y']+box['h']),outline=(220,70,65),width=1)
                d.line((center-5,385,center+5,385),fill=(220,200,60),width=2)
            d.line((0,385,960,385),fill=(80,110,105));d.text((12,12),f'{action} / {frame["phase"]} / {frame["duration_ms"]}ms / runtime size',fill='white');d.text((492,12),'mirrored / light background',fill='black')
            card.save(OUTPUT/f'{action}-{i}.png');cards.append(card)
        cards[0].save(OUTPUT/f'{action}.gif',save_all=True,append_images=cards[1:],duration=[f['duration_ms'] for f in spec['frames']],loop=0)
        montage=Image.new('RGB',(1440,430),(23,28,37))
        for i,card in enumerate(cards):montage.paste(card.crop((0,0,480,430)),(480*i,0))
        montage.save(OUTPUT/f'{action}-frames.png')
        x0,y0,x1,y1=cfg['contact'];px,py=cfg['pivots'][1]
        contact=[round((x0-px)*scale,3),round((y0-py)*scale,3),round((x1-px)*scale,3),round((y1-py)*scale,3)]
        report['actions'][action]={'action':f'{action}/action.json','sheet':f'{action}/sheet-v1.png','source':f'{action}/source-v{cfg["version"]}.png','scale_to_runtime':scale,'source_body_height_reference_px':cfg['height'],'durations_ms':[f['duration_ms'] for f in spec['frames']],'total_ms':sum(f['duration_ms'] for f in spec['frames']),'contact_landmark_right_xyxy':contact,'contact_box_right':spec['timing_reference']['active_box_right_relative_to_anchor'],'notes':cfg['notes'],'diagnostic':f'target/art/go-replacement-b/{action}.gif'}
    (ROOT/'batch-b-review.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))


if __name__=='__main__':
    run()
