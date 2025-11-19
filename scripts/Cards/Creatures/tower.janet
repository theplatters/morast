(def tower @{'cost 1
'card-image "assets/image3.png"

'movement @[]
'movement-points 0

'attack (array/join (std/cross 1 ) (std/cross 2) (std/plus 2) (std/plus 1))

'abilities @["fly" "dig"]

'attack-strength 3
'defense 2


'description "Stands tall over the lands"

'display-image-asset-string "missing"})
