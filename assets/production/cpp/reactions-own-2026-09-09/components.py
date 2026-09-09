from PIL import Image
import numpy as np

def connected_runs(alpha):
 runs=[]; parents=[]; prev=[]
 def find(i):
  while parents[i]!=i:
   parents[i]=parents[parents[i]];i=parents[i]
  return i
 for y,row in enumerate(alpha>0):
  edges=np.flatnonzero(np.diff(np.r_[False,row,False]));cur=[]
  for x0,x1 in zip(edges[::2],edges[1::2]):
   i=len(runs);parents.append(i);runs.append((y,int(x0),int(x1)));cur.append(i)
   for p in prev:
    _,px0,px1=runs[p]
    if px1 < x0:continue
    if px0 > x1:break
    a=find(i);b=find(p)
    if a!=b:parents[a]=b
  prev=cur
 groups={}
 for i,run in enumerate(runs):groups.setdefault(find(i),[]).append(run)
 out=[]
 for rs in groups.values():
  out.append({'runs':rs,'count':sum(x1-x0 for y,x0,x1 in rs),'box':[min(x0 for y,x0,x1 in rs),min(y for y,x0,x1 in rs),max(x1 for y,x0,x1 in rs),max(y for y,x0,x1 in rs)+1]})
 return out
if __name__=='__main__':
 for name in ['hits','guards','air']:
  im=Image.open('assets/production/cpp/reactions-own-2026-09-09/rgba-'+name+'.png');a=np.asarray(im)[:,:,3];c=connected_runs(a)
  print(name,im.mode,im.size,[(p['count'],p['box']) for p in sorted(c,key=lambda p:p['box'][1]) if p['count']>500],flush=True)
