"""Mechanically extract and pack generated C++ RGBA reactions; never draw art.

Opaque cores identify each existing sprite. Nearest-core expansion assigns every
original nontransparent edge pixel to one sprite without changing its RGBA value.
Only uniform per-sheet scaling and packing follow. Requires Pillow and NumPy.
"""
from pathlib import Path
import copy, hashlib, json
import numpy as np
from PIL import Image
from components import connected_runs

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
CANDIDATE=ROOT/'assets/candidates/cpp'
MANIFEST=CANDIDATE/'cpp-fighter.sprite.json'
CELL=384
sha=lambda data:hashlib.sha256(data).hexdigest()
SHEETS=[
 ('hits',['reaction_head','reaction_body','reaction_low'],0.73,[[240,605,930,1310],[230,590,946,1310],[235,602,947,1310]]),
 ('guards',['reaction_guard_high','reaction_guard_low'],0.53,[[262,710,1139,1580],[260,710,1135,1580]]),
 ('air',['reaction_launch','reaction_fall','reaction_rise'],0.76,None),
]

def extract(path, expected):
    source=np.asarray(Image.open(path).convert('RGBA'))
    cores=[c for c in connected_runs(np.where(source[:,:,3]>=240,255,0)) if c['count']>10000]
    assert len(cores)==expected,(path,len(cores))
    cores.sort(key=lambda c:(c['box'][1]+c['box'][3])/2)
    cores=[c for row in range(expected//4) for c in sorted(cores[row*4:row*4+4],key=lambda c:c['box'][0])]
    owner=np.zeros(source.shape[:2],dtype=np.uint8)
    for index,core in enumerate(cores,1):
        for y,x0,x1 in core['runs']:owner[y,x0:x1]=index
    needed=source[:,:,3]>0
    for radius in range(256):
        if not np.any(needed & (owner==0)):break
        before=owner.copy()
        for dy,dx in [(0,1),(0,-1),(1,0),(-1,0)]:
            adjacent=np.roll(before,(dy,dx),(0,1))
            if dy>0:adjacent[:dy]=0
            if dy<0:adjacent[dy:]=0
            if dx>0:adjacent[:,:dx]=0
            if dx<0:adjacent[:,dx:]=0
            fill=(owner==0)&(adjacent>0);owner[fill]=adjacent[fill]
    assert not np.any(needed & (owner==0)),path
    extracted=[];assigned=0
    for index,core in enumerate(cores,1):
        selected=needed & (owner==index);ys,xs=np.where(selected)
        box=[int(xs.min()),int(ys.min()),int(xs.max()+1),int(ys.max()+1)]
        x0,y0,x1,y1=box
        pixels=source[y0:y1,x0:x1].copy();inside=selected[y0:y1,x0:x1]
        pixels[~inside]=0
        assert np.array_equal(pixels[inside],source[y0:y1,x0:x1][inside])
        assigned+=int(np.count_nonzero(pixels[:,:,3]))
        extracted.append((Image.fromarray(pixels),box,core['box']))
    assert assigned==int(np.count_nonzero(source[:,:,3])),path
    return extracted,{'source':path.name,'sha256':sha(path.read_bytes()),'dimensions':[source.shape[1],source.shape[0]],'native_alpha':True,'source_nontransparent_pixels':assigned,'all_original_nontransparent_pixels_preserved_before_scaling':True,'edge_assignment_radius_pixels':radius}

manifest=json.loads(MANIFEST.read_text());before=copy.deepcopy(manifest)
old_frames=[f for f in manifest['frames'] if not f['name'].startswith('reaction_')]
old_clips=[c for c in manifest['clips'] if not c['name'].startswith('reaction_')]
manifest['frames']=copy.deepcopy(old_frames);manifest['clips']=copy.deepcopy(old_clips)
atlas=Image.new('RGBA',(4*CELL,8*CELL))
metadata=[];sources=[];actions={};row_index=0
for sheet,clips,scale,anchors in SHEETS:
    sprites,source=extract(HERE/f'rgba-{sheet}.png',len(clips)*4);sources.append(source)
    for row,clip in enumerate(clips):
        durations=[70,85,85,90]
        if 'guard' in clip:durations=[60,70,70,80]
        if clip=='reaction_fall':durations=[80,100,100,160]
        if clip=='reaction_rise':durations=[90,90,110,110]
        prepared=Image.new('RGBA',(4*CELL,CELL));names=[];production_frames=[]
        for col in range(4):
            original,box,core=sprites[row*4+col]
            original.save(HERE/f'{clip}_{col:02d}-crop.png')
            resized=original.resize((round(original.width*scale),round(original.height*scale)),Image.Resampling.LANCZOS)
            assert resized.width+20<=CELL and resized.height+20<=CELL,(clip,resized.size)
            pad=10
            pivot_x=((anchors[row][col] if anchors else (core[0]+core[2])/2)-box[0])*scale+pad
            pivot_y=(core[3]-box[1])*scale+pad
            # Alpha 1/255 residue from the generator can extend far beyond the
            # visible body. Measure support without editing any alpha samples.
            visible=np.asarray(resized)[:,:,3]>16
            ys,xs=np.where(visible)
            bounds=(int(xs.min()),int(ys.min()),int(xs.max()+1),int(ys.max()+1))
            if clip!='reaction_launch':pivot_y=bounds[3]+pad
            pivot={'x':round(pivot_x),'y':round(pivot_y)}
            dest=(col*CELL+pad,row_index*CELL+pad)
            atlas.paste(resized,dest);prepared.paste(resized,(col*CELL+pad,pad))
            name=f'{clip}_{col:02d}';names.append(name)
            frame={'name':name,'clip':clip,'duration_ms':durations[col],'image':'cpp-reactions-own-2026-09-09.png','pivot':pivot,'frame':{'x':col*CELL,'y':row_index*CELL,'w':CELL,'h':CELL},'trimmed_bounds':{'x':bounds[0]+pad,'y':bounds[1]+pad,'w':bounds[2]-bounds[0],'h':bounds[3]-bounds[1]}}
            manifest['frames'].append(frame)
            production_frames.append({'name':name,'source_rect':{'x':col*CELL,'y':0,'w':CELL,'h':CELL},'pivot':pivot,'duration_ms':durations[col]})
            metadata.append({'name':name,'source':f'rgba-{sheet}.png','source_rect':box,'opaque_core_bounds':core,'uniform_scale':scale,'pivot':pivot,'crop_sha256':sha((HERE/f'{name}-crop.png').read_bytes()),'crop_rgba_sha256':sha(original.tobytes()),'scaled_size':[resized.width,resized.height]})
        manifest['clips'].append({'name':clip,'loop':False,'frames':names})
        prepared.save(HERE/f'{clip}-prepared.png')
        actions[clip]={'sheet':f'{clip}-prepared.png','reviewed':True,'loop':False,'frames':production_frames}
        row_index+=1
atlas_path=CANDIDATE/'cpp-reactions-own-2026-09-09.png';atlas.save(atlas_path)
assert manifest['frames'][:len(old_frames)]==old_frames
assert manifest['clips'][:len(old_clips)]==old_clips
assert {k:v for k,v in manifest.items() if k not in ('frames','clips')}=={k:v for k,v in before.items() if k not in ('frames','clips')}
provenance_note='The optional reaction_* clips use cpp-reactions-own-2026-09-09.png; prompts, original RGBA sheets, support bounds and production provenance: ../../production/cpp/reactions-own-2026-09-09/generation.json'
if provenance_note not in manifest['notes']:manifest['notes'].append(provenance_note)
MANIFEST.write_text(json.dumps(manifest,indent=2)+'\n')
(HERE/'production.json').write_text(json.dumps({'schema':'borrow-fighters.production.v1','character':'cpp','scale':1.0,'provenance':{'generator':'built-in imagegen','status':'candidate; distinct articulated reactions reviewed in source sheets; runtime review follows separately','combat':'No combat metadata. All prior candidate frames/clips and main fields retained.'},'actions':actions},indent=2)+'\n')
report={'generator':'built-in imagegen','date':'2026-09-09','method':'Generated three original reaction sheets from C++ identity references. The generator initially returned RGB checkerboards; separate built-in imagegen background-extraction calls produced real RGBA sheets. The packer assigns original edge pixels to opaque sprite cores, retaining every nontransparent source RGBA pixel before uniform sheet scaling; no background removal, redraw or synthesized pose occurs in Python.','reference_images':['../reference/master-existing.png','../idle/source-v1.png'],'sources':sources,'frames':metadata,'output':str(atlas_path.relative_to(ROOT)),'output_sha256':sha(atlas_path.read_bytes()),'manifest':str(MANIFEST.relative_to(ROOT)),'manifest_sha256':sha(MANIFEST.read_bytes()),'prior_frames_preserved':len(old_frames),'prior_clips_preserved':len(old_clips),'added_clips':8,'added_drawings':32,'new_frames_have_combat_metadata':False,'source_prompts':['prompt-hits.txt','prompt-guards.txt','prompt-air.txt'],'background_extraction_prompts':['prompt-extract-hits.txt','prompt-extract-guards.txt','prompt-extract-air.txt']}
report['visible_support_measurement']='trimmed_bounds and grounded pivots use the bounds of alpha > 16/255. This excludes nearly invisible remote residue from support measurements without changing any image alpha.'
report['contour_review']='Thin red/gold fringe is present in the original generated RGBA edges. Additional built-in cleanup attempts either lost true alpha or restored the same fringe with identity/color drift. Original RGBA was retained; runtime arena inspection found the fringe discreet and the articulated reactions clear. No local alpha/color cleanup was applied.'
report['rejected_cleanup_attempts']=[{'file':name,'sha256':sha((HERE/name).read_bytes()),'reason':reason} for name,reason in [('source-hits-clean-v2.png','RGB checkerboard; unusable transparency'),('source-hits-clean-v3.png','RGB checkerboard; unusable transparency'),('rgba-hits-clean-v2-rejected.png','True RGBA but fringe persists and colors/pose details drift; original preferred')]]
report['original_generation_sources']=[{'file':f'source-{sheet}.png','sha256':sha((HERE/f'source-{sheet}.png').read_bytes()),'prompt':f'prompt-{sheet}.txt','selected_rgba':f'rgba-{sheet}.png'} for sheet,_,_,_ in SHEETS]
report['cleanup_prompt_history']=[{'prompt':'prompt-clean-hits.txt','input':'rgba-hits.png','output':'source-hits-clean-v2.png'},{'prompt':'prompt-extract-clean-hits.txt','input':'source-hits-clean-v2.png','output':'source-hits-clean-v3.png'},{'prompt':'prompt-extract-hits.txt','input':'source-hits-clean-v2.png','output':'rgba-hits-clean-v2-rejected.png'}]
(HERE/'generation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'atlas':str(atlas_path.relative_to(ROOT)),'size':atlas.size,'frames_added':32,'prior_frames_preserved':len(old_frames),'prior_clips_preserved':len(old_clips)}))
