(def bowmen @{
  'cost 1
  'card-image "assets/image1.png"
  'movement (std/plus 1)
  'attack (array/join (std/plus 1) (std/plus 2))
  'movement-points 2


  'abilities @[]
  'attack-strength 3
  'defense 1

  'description "A bowmen. He has a bow a few arrors and a mission"

  'display-image-asset-string "missing"
})
