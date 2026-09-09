"""Review the real native Studio via its AT-SPI UI and capture its X11 window."""
from pathlib import Path
import gi,json,time,subprocess,ctypes as C

gi.require_version('Atspi','2.0');from gi.repository import Atspi,GLib
root=Path('/home/willams/borrow-fighters');out=root/'target/art/go-finalization-studio';win=0xa00003
app=next(Atspi.get_desktop(0).get_child_at_index(i) for i in range(Atspi.get_desktop(0).get_child_count()) if Atspi.get_desktop(0).get_child_at_index(i).get_name()=='borrow-fighters-sprite-studio')
def nodes(o=app):
 o.clear_cache();yield o
 for i in range(o.get_child_count()):yield from nodes(o.get_child_at_index(i))
def settle():
 end=time.monotonic()+.18
 while time.monotonic()<end:
  while GLib.MainContext.default().pending():GLib.MainContext.default().iteration(False)
  time.sleep(.015)
def press(name):
 b=next(o for o in nodes() if o.get_role_name()=='push button' and o.get_name()==name)
 assert b.get_action_iface().do_action(0);settle()
x=C.CDLL('libX11.so.6');x.XOpenDisplay.restype=C.c_void_p;x.XOpenDisplay.argtypes=[C.c_char_p];d=x.XOpenDisplay(None);x.XResizeWindow.argtypes=[C.c_void_p,C.c_ulong,C.c_uint,C.c_uint];x.XFlush.argtypes=[C.c_void_p]
def expose():
 x.XResizeWindow(d,win,1280,821);x.XFlush(d);settle();x.XResizeWindow(d,win,1280,820);x.XFlush(d);settle()
manifest=json.loads((root/'assets/candidates/go/go-fighter.sprite.json').read_text())
by_clip={v['name']:v for v in manifest['clips']}
for o in nodes():
 if o.get_role_name()=='entry' and o.get_name()=='' and 'Text' in o.get_interfaces():
  text=o.get_text_iface().get_text(0,-1)
  if text.startswith('{'):
   loaded=json.loads(text);assert loaded==manifest,'UI JSON differs from candidate';(out/'loaded-ui-manifest.json').write_text(text+'\n');break
else:raise AssertionError('Missing actual UI JSON preview')
# Presentation only: remove bounds tint while keeping pivot and metadata inspectors.
for o in nodes():
 if o.get_role_name()=='check box' and o.get_name()=='Bounds' and o.get_state_set().contains(Atspi.StateType.CHECKED):o.get_action_iface().do_action(0);break
records=[]
for clip in by_clip:
 names=by_clip[clip]['frames'];press(f'{clip} {len(names)} frames')
 for index,name in enumerate(names):
  if index:press('Next frame')
  expose()
  file=out/f'{clip}-{index:02}.png';subprocess.run(['import','-window',hex(win),str(file)],check=True)
  values=[o.get_name() for o in nodes() if o.get_role_name()=='entry' and o.get_name().startswith(('Pivot X','Pivot Y','Duration ms','Manifest scale'))]
  zoom=next(o.get_name() for o in nodes() if o.get_role_name()=='slider' and o.get_name().startswith('Zoom'))
  assert '1.00' in zoom,zoom
  records.append({'clip':clip,'frame':name,'ui_frame_index':index+1,'ui_values':values,'zoom':zoom,'capture':file.name})
  print(clip,name,values,flush=True)
(out/'review-captures.json').write_text(json.dumps({'method':'Native AT-SPI action of real clip buttons and Next frame; native X11 window PNG via ImageMagick import. Resize 1px requests repaint in WSL Weston. No manifest or metrics writes. No continuous playback claimed.','window_id':hex(win),'loaded_manifest_matches_candidate':True,'captures':records},indent=2)+'\n')
