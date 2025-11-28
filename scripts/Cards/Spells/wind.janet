(def wind @{
'cost 2
'on-play  @{
  :action @{
    :type "move-creature"
    :direction [0 1]
  }
  :timing "now"
  :speed spell/slow
  :target ["single-tile"]
  }
'description "Blows away a card"

'display-image-asset-string "missing"
})
