"""Load the candidate with the actual Open dialog through native accessibility."""
import gi,time

gi.require_version('Atspi','2.0');from gi.repository import Atspi,GLib
app=next(Atspi.get_desktop(0).get_child_at_index(i) for i in range(Atspi.get_desktop(0).get_child_count()) if Atspi.get_desktop(0).get_child_at_index(i).get_name()=='borrow-fighters-sprite-studio')
def nodes(o=app):
 o.clear_cache();yield o
 for i in range(o.get_child_count()):yield from nodes(o.get_child_at_index(i))
def settle():
 until=time.monotonic()+.6
 while time.monotonic()<until:
  while GLib.MainContext.default().pending():GLib.MainContext.default().iteration(False)
  time.sleep(.015)
def obj(role,name,parent=app):return next(o for o in nodes(parent) if o.get_role_name()==role and o.get_name()==name)
obj('push button','Open').get_action_iface().do_action(0);settle()
dlg=next(o for o in nodes() if o.get_role_name()=='file chooser')
obj('toggle button','assets',dlg).get_action_iface().do_action(0);settle()
for name in ['candidates','go','go-fighter.sprite.json']:
 table=obj('table','Files',dlg).get_table_iface()
 matches=[]
 for row in range(table.get_n_rows()):
  cell=table.get_accessible_at(row,0)
  if any(o.get_name()==name for o in nodes(cell)):matches.append(row)
 assert len(matches)==1,(name,matches)
 table.add_row_selection(matches[0]);settle();obj('push button','Open',dlg).get_action_iface().do_action(0);settle();print('opened',name,flush=True)
settle()
path=next(o for o in nodes() if o.get_role_name()=='entry' and o.get_name().startswith('Path ')).get_text_iface().get_text(0,-1)
assert path=='assets/candidates/go/go-fighter.sprite.json',path
z=next(o for o in nodes() if o.get_role_name()=='slider' and o.get_name().startswith('Zoom'));assert z.get_value_iface().set_current_value(1.0)
for name in ['Grid','Bounds','Scale guide']:
 b=obj('check box',name)
 if b.get_state_set().contains(Atspi.StateType.CHECKED):b.get_action_iface().do_action(0);settle()
print('Loaded actual candidate and set presentation-only zoom=1, overlays off.')
