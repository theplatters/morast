(def wind @{
'cost 2
'on-play  @{
  'action @{
    'type "move-creature"
    'direction [1 0]
  }
  'timing "now"
  'speed spell/slow
  'target ["single-tile"]
  }
'description "Blows away a card"

'display-image-asset-string "missing"
})
