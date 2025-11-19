(def soldier @{

'cost 1
'card-image "assets/image2.png"
'movement-points 3

'movement(std/plus 1)
'attack (std/plus 1)

'abilities @["fly" "dig"]
'attack-strength 3
'defense 3

'on-draw @[]
'on-play @[]
'on-discard @[]
'on-ability @[]
'on-turn-begin @[]
'on-turn-end @[]

'description "A soldier drafted from the finest of peasents"

'display-image-asset-string "missing"})
