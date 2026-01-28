(def merchant @{
'cost 3
'movement-points 2
'card-image "assets/image.png"

'movement (std/plus 1)
'attack (std/plus 1)

'abilities @["fly" "dig"]
'attack-strength 1
'defense 1


'on-turn-end @{
  'action @{
    'type "get-gold"
    'amount 4
  }
  'timing "now"
  'speed  spell/slow
  }

'on-turn-end @{
  'action @{
    'type "get-gold"
    'amount 4
  }
  'timing "now"
  'speed spell/slow 
  }

'description "Get 4 gold at the start and end of a turn"

'display-image-asset-string "missing"
})
