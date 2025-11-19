(def zombie @{
'cost 2
'card-image "assets/image3.png"
'movement (std/plus 1)
'movement-points 1
'attack (array/join (std/cross 1) (std/plus 1))
'abilities @["dig"]
'attack-strength 2
'defense 1
'on-play @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  )]
'on-turn-begin 
 @[(table
  'action (fn [game card-id] (std/apply-effect game 'weakening 2 (std/from-current-position game card-id (std/plus 1))))
  'timing @['now]
  )]


'description "He has seen better days"

'display-image-asset-string "missing"})
